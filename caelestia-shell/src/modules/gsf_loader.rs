//! Google Sans Flex Font Loader
//! Parity with `shell/modules/GSFLoader.qml`.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GsfFontLoader {
    asset_font_path: PathBuf,
}

impl Default for GsfFontLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl GsfFontLoader {
    pub fn new() -> Self {
        let asset_font_path = PathBuf::from("assets/google-sans-flex/GoogleSansFlex-VariableFont_GRAD,ROND,opsz,slnt,wdth,wght.ttf");
        Self { asset_font_path }
    }

    /// Checks whether the system provides Google Sans Flex font family.
    pub fn is_system_font_available(&self) -> bool {
        // Under Linux fontconfig, we check common font paths or let GPUI text system resolve
        Path::new("/usr/share/fonts/google-sans-flex").exists()
            || Path::new("/usr/share/fonts/TTF/GoogleSansFlex.ttf").exists()
    }

    /// Resolves the effective font file path if fallback to bundled asset is required.
    pub fn resolve_font_source(&self) -> Option<PathBuf> {
        if self.is_system_font_available() {
            None
        } else if self.asset_font_path.exists() {
            Some(self.asset_font_path.clone())
        } else {
            None
        }
    }
}
