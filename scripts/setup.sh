#!/usr/bin/env bash

set -euo pipefail
export CAELESTIA_SETUP_RUNNING=1

tput civis 2>/dev/null || true

BUNDLE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPTS_DIR="$BUNDLE_DIR/scripts"
export BUNDLE_DIR
export INSTALL_START_EPOCH="$(date +%s)"

export PATH="$HOME/.local/bin:$PATH"

exec 9>"${XDG_RUNTIME_DIR:-/tmp}/caelestia-setup.lock"
flock -n 9 || { echo "Another Caelestia setup is already running."; exit 1; }

# shellcheck source=scripts/lib/privileges.sh
source "$SCRIPTS_DIR/lib/privileges.sh"
# shellcheck source=scripts/lib/packages.sh
source "$SCRIPTS_DIR/lib/packages.sh"
# shellcheck source=scripts/lib/download.sh
source "$SCRIPTS_DIR/lib/download.sh"

run_arch_pacman_install() {
    local -a pkgs=("$@")
    local -a pacman_args=(-S --needed --noconfirm)

    if (( ${#pkgs[@]} == 0 )); then
        return 0
    fi

    caelestia_sudo pacman -Sy --noconfirm >/dev/null 2>&1 || echo "[WARN]  Failed to refresh pacman sources before install. Continuing..."
    caelestia_sudo pacman "${pacman_args[@]}" "${pkgs[@]}" && return 0

    echo "[WARN]  pacman install failed. Refreshing sources and retrying once..."
    caelestia_sudo pacman -Sy --noconfirm >/dev/null 2>&1 || true
    caelestia_sudo pacman "${pacman_args[@]}" "${pkgs[@]}"
}

normalize_line_endings_first() {
    local -a crlf_files=()
    local convert_choice=""

    mapfile -t crlf_files < <(
        find "$BUNDLE_DIR" -path "$BUNDLE_DIR/.git" -prune -o -type f -name '*.sh' -print0 | \
            xargs -0 grep -Il $'\r' 2>/dev/null || true
    )

    if (( ${#crlf_files[@]} == 0 )); then
        return 0
    fi

    echo "[WARN]  Detected ${#crlf_files[@]} file(s) with CRLF line endings."
    while true; do
        read -r -p "Convert all files under this repo to LF with dos2unix? [Y/n]: " convert_choice
        convert_choice="${convert_choice:-y}"

        case "${convert_choice,,}" in
            y|yes)
                if ! command -v dos2unix >/dev/null 2>&1; then
                    echo "[WARN]  dos2unix is not installed. Attempting to install it now..."
                    case "$BASE_DISTRO" in
                        arch)
                            run_arch_pacman_install dos2unix || return 1
                            ;;
                        fedora)
                            caelestia_sudo dnf install -y dos2unix || return 1
                            ;;
                        debian)
                            caelestia_sudo apt-get update && caelestia_sudo apt-get install -y dos2unix || return 1
                            ;;
                        *)
                            echo "[WARN]  Could not detect distro for automatic dos2unix installation."
                            return 1
                            ;;
                    esac
                    echo "[OK]    dos2unix installed."
                fi

                (
                    cd "$BUNDLE_DIR" || exit 1
                    printf '%s\0' "${crlf_files[@]}" | xargs -0 -r dos2unix --
                ) || return 1

                echo "[OK]    Line endings normalized to LF."
                return 0
                ;;
            n|no)
                echo "[WARN]  Skipping line ending normalization by user choice."
                return 0
                ;;
            *)
                echo "Please answer with y or n."
                ;;
        esac
    done
}

if ! normalize_line_endings_first; then
    echo "[FATAL] Line ending normalization step failed. Aborting installer." >&2
    exit 1
fi

BIN="$BUNDLE_DIR/caelestia-install"

tui_version() {
    tr -d '[:space:]' < "$BUNDLE_DIR/installer/data/tui.version" 2>/dev/null || true
}

release_tag() {
    sed -nE 's/^[[:space:]]*VERSION=//p' "$BUNDLE_DIR/.github/version.env" 2>/dev/null | tr -d '[:space:]'
}

fetch_installer() {
    local arch version tag url status=0
    arch="$(uname -m)"

    if [[ -n "${CAELESTIA_INSTALL_BIN:-}" && -x "${CAELESTIA_INSTALL_BIN}" ]]; then
        echo "[INFO]  Using the installer at ${CAELESTIA_INSTALL_BIN}."
        BIN="$CAELESTIA_INSTALL_BIN"
        return 0
    fi

    case "$arch" in
        x86_64|aarch64) ;;
        *)
            echo "[FATAL] There is no prebuilt installer for ${arch}." >&2
            return 1
            ;;
    esac

    version="$(tui_version)"
    tag="$(release_tag)"
    if [[ -z "$version" || -z "$tag" ]]; then
        echo "[FATAL] This checkout does not say which installer version it needs." >&2
        return 1
    fi

    url="https://github.com/ladybug-me/caelestia-kde/releases/download/${tag}/caelestia-install-${arch}-v${version}"
    rm -f "$BIN"
    if ! fetch_asset "$url" "$BIN" --max-time 60 2>/dev/null; then
        # `dev` names the release it is about to become, so between the version bump and the tag
        # being published there is nothing to download. A checkout can build the same binary
        # instead of stopping the install over a release that is not due yet.
        if build_installer; then
            return 0
        fi
        echo "[FATAL] Could not download the installer for v${version}." >&2
        return 1
    fi

    verify_download "$url" "$BIN" || status=$?
    if [[ "$status" -eq 1 ]]; then
        echo "[FATAL] The downloaded installer failed its checksum." >&2
        rm -f "$BIN"
        return 1
    fi
    if [[ "$status" -eq 2 ]]; then
        echo "[FATAL] The release publishes no checksum for the installer." >&2
        rm -f "$BIN"
        return 1
    fi
    if [[ "$status" -ne 0 ]]; then
        echo "[FATAL] The installer could not be verified (status ${status})." >&2
        rm -f "$BIN"
        return 1
    fi
    chmod +x "$BIN"
    return 0
}

# The installer this checkout's own sources make. A prebuilt one is preferred because it needs no
# toolchain, so this only runs when there is nothing published to download.
build_installer() {
    local manifest="$BUNDLE_DIR/installer/Cargo.toml" built="$BUNDLE_DIR/installer/target/release/caelestia-install"
    if [[ ! -f "$manifest" ]]; then
        return 1
    fi
    if ! command -v cargo >/dev/null 2>&1; then
        echo "  This checkout has no published installer yet, and building it needs cargo." >&2
        echo "  Install Rust (https://rustup.rs), or set CAELESTIA_INSTALL_BIN to a built installer." >&2
        return 1
    fi

    # The spinner is the download's; a build draws its own progress on the same line.
    if declare -F stop_spinner >/dev/null 2>&1; then
        stop_spinner
    fi
    echo "[INFO]  This checkout names a release that is not published yet;"
    echo "[INFO]  building the installer from the checkout instead."

    rm -f "$built"
    if ! cargo build --release --manifest-path "$manifest"; then
        echo "[FATAL] Building the installer from this checkout failed." >&2
        return 1
    fi
    if [[ ! -x "$built" ]]; then
        echo "[FATAL] The build finished but produced no installer at $built." >&2
        return 1
    fi
    cp -f "$built" "$BIN" && chmod +x "$BIN"
}

installer_matches_checkout() {
    local version
    version="$(tui_version)"
    if [[ "$("$BIN" --version 2>/dev/null)" != "$version" ]]; then
        echo "[FATAL] The downloaded installer does not report v${version}." >&2
        # Only a file this script fetched is cleared; a binary the caller named is theirs.
        if [[ -z "${CAELESTIA_INSTALL_BIN:-}" ]]; then
            rm -f "$BIN"
        fi
        return 1
    fi
    return 0
}

start_spinner() {
    echo -n "Preparing Caelestia installer"
    {
        while true; do
            printf "."
            sleep 0.5
            printf "."
            sleep 0.5
            printf "."
            sleep 0.5
            printf "\b\b\b   \b\b\b"
        done
    } &
    SPINNER_PID=$!
}

stop_spinner() {
    [[ -n "${SPINNER_PID:-}" ]] || return 0
    kill "$SPINNER_PID" 2>/dev/null || true
    wait "$SPINNER_PID" 2>/dev/null || true
    SPINNER_PID=""
    echo ""
}

start_spinner
# The full cleanup trap lands later; until then a Ctrl-C must still stop
# the spinner and hand the terminal back.
trap 'stty sane 2>/dev/null || true; tput cnorm 2>/dev/null || true; kill "${SPINNER_PID:-}" 2>/dev/null || true' EXIT

if ! fetch_installer || ! installer_matches_checkout; then
    stop_spinner
    echo ""
    echo "  The installer is a published download. Check your network connection,"
    echo "  and that the release named by .github/version.env is still available."
    echo "  A checkout with Rust can build it: cargo build --release --manifest-path installer/Cargo.toml"
    echo "  or point CAELESTIA_INSTALL_BIN at an installer binary."
    echo ""
    echo "Press Enter to close this window..."
    read -r
    exit 1
fi
stop_spinner
echo "[OK]    Using the installer binary v$(tui_version)."

cleanup_install_state() {
    stty sane 2>/dev/null || true
    kill "${SPINNER_PID:-}" 2>/dev/null || true
    rm -f "${ERR_LOG:-}" 2>/dev/null || true
    tput cnorm 2>/dev/null || true
    printf '\033[0m\033[?1049l\033[?25h' 2>/dev/null || true
}
trap cleanup_install_state EXIT

if [[ ! -x "$BIN" ]]; then
    echo ""
    echo "============================================================"
    echo "  FATAL: The installer binary is missing: $BIN"
    echo "============================================================"
    echo ""
    echo "Press Enter to close this window..."
    read -r
    exit 1
fi

export CAELESTIA_INSTALL_LOG="${XDG_CACHE_HOME:-$HOME/.cache}/caelestia/install.log"

# Under set -e a plain invocation would kill this script on the installer's first
# non-zero exit, making the diagnostic block below unreachable exactly when it matters.
ERR_LOG="$(mktemp "${TMPDIR:-/tmp}/caelestia-installer-err.XXXXXX.log")"
_exit_code=0
"$BIN" "$@" 2>"$ERR_LOG" || _exit_code=$?

_diag_title=""
case $_exit_code in
    0) ;;
    1) _diag_title="INSTALLER FAILED: a step failed" ;;
    2) _diag_title="INSTALLER CALLED INCORRECTLY (bad arguments)" ;;
    3) echo "Stopped at your request. Nothing was changed by it." ;;
    4) _diag_title="THE INSTALLER COULD NOT RUN HERE" ;;
    5) _diag_title="INSTALLER FAILED: a bug in the installer" ;;
    130) echo "The install was interrupted." ;;
    *) _diag_title="INSTALLER FAILED (exit code: $_exit_code)" ;;
esac

if [[ -n "$_diag_title" ]]; then
    stty sane 2>/dev/null || true
    tput cnorm 2>/dev/null || true
    printf '\033[0m\033[?1049l\033[?25h' 2>/dev/null || true

    echo ""
    echo "============================================================"
    echo "  $_diag_title"
    echo "============================================================"
    echo ""

    if [[ -s "$ERR_LOG" ]]; then
        echo "--- stderr output ---"
        cat "$ERR_LOG"
        echo "--- end stderr ------"
        echo ""
    else
        echo "(no stderr output captured)"
        echo ""
    fi

    if [[ -s "$CAELESTIA_INSTALL_LOG" ]]; then
        echo "--- last 20 lines of the install log ---"
        tail -n 20 "$CAELESTIA_INSTALL_LOG"
        echo "--- end install log ---"
        echo ""
        echo "Full log: $CAELESTIA_INSTALL_LOG"
        echo ""
    fi

    if [[ $_exit_code -ge 128 ]]; then
        echo "Exit code $_exit_code means the installer was killed by signal $((_exit_code - 128))."
        echo ""
    fi

    echo "Press Enter to close this window..."
    read -r
fi

exit $_exit_code
