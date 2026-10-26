//! HTTP Requests & File Download Service
//! 100% parity with C++ `Caelestia::Requests`.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use serde_json::Value;

static NEXT_REQUEST_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub request_id: usize,
    pub bytes_received: u64,
    pub bytes_total: u64,
}

pub struct RequestsService {
    active_processes: Mutex<HashMap<usize, Child>>,
}

impl Default for RequestsService {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestsService {
    pub fn new() -> Self {
        Self {
            active_processes: Mutex::new(HashMap::new()),
        }
    }

    /// Perform a synchronous HTTP GET request
    pub fn get(
        &self,
        url: &str,
        headers: Option<&HashMap<String, String>>,
        timeout_secs: Option<u64>,
    ) -> Result<HttpResponse, String> {
        let mut cmd = Command::new("curl");
        cmd.arg("-s").arg("-i").arg("-L"); // silent, include headers, follow redirects

        if let Some(timeout) = timeout_secs {
            cmd.arg("--max-time").arg(timeout.to_string());
        }

        if let Some(hdrs) = headers {
            for (k, v) in hdrs {
                cmd.arg("-H").arg(format!("{}: {}", k, v));
            }
        }

        cmd.arg(url);

        let output = cmd.output().map_err(|e| format!("Failed to execute curl: {}", e))?;
        Self::parse_curl_output(&output.stdout)
    }

    /// Perform a synchronous HTTP POST request
    pub fn post(
        &self,
        url: &str,
        body: &str,
        content_type: &str,
        headers: Option<&HashMap<String, String>>,
        timeout_secs: Option<u64>,
    ) -> Result<HttpResponse, String> {
        let mut cmd = Command::new("curl");
        cmd.arg("-s").arg("-i").arg("-L").arg("-X").arg("POST");

        if let Some(timeout) = timeout_secs {
            cmd.arg("--max-time").arg(timeout.to_string());
        }

        cmd.arg("-H").arg(format!("Content-Type: {}", content_type));

        if let Some(hdrs) = headers {
            for (k, v) in hdrs {
                cmd.arg("-H").arg(format!("{}: {}", k, v));
            }
        }

        cmd.arg("--data").arg(body);
        cmd.arg(url);

        let output = cmd.output().map_err(|e| format!("Failed to execute curl: {}", e))?;
        Self::parse_curl_output(&output.stdout)
    }

    /// Download a file to destination path asynchronously, returning the request ID
    pub fn download(
        &self,
        url: &str,
        dest_path: impl AsRef<Path>,
        timeout_secs: Option<u64>,
    ) -> Result<usize, String> {
        let dest = dest_path.as_ref();
        if let Some(parent) = dest.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let request_id = NEXT_REQUEST_ID.fetch_add(1, Ordering::SeqCst);

        let mut cmd = Command::new("curl");
        cmd.arg("-s").arg("-L").arg("-o").arg(dest);

        if let Some(timeout) = timeout_secs {
            cmd.arg("--max-time").arg(timeout.to_string());
        }

        cmd.arg(url);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let child = cmd.spawn().map_err(|e| format!("Failed to spawn curl download: {}", e))?;

        if let Ok(mut procs) = self.active_processes.lock() {
            procs.insert(request_id, child);
        }

        Ok(request_id)
    }

    /// Cancel an in-flight request by ID
    pub fn cancel(&self, request_id: usize) {
        if let Ok(mut procs) = self.active_processes.lock() {
            if let Some(mut child) = procs.remove(&request_id) {
                let _ = child.kill();
            }
        }
    }

    /// Parse JSON string safely into `serde_json::Value`
    pub fn parse_json(text: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(text)
    }

    /// Format value into JSON string
    pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }

    fn parse_curl_output(raw: &[u8]) -> Result<HttpResponse, String> {
        let text = String::from_utf8_lossy(raw);

        let mut header_section = "";
        let mut body = "";

        // Collect the final redirect HTTP block
        for part in text.split("\r\n\r\n") {
            if part.starts_with("HTTP/") {
                header_section = part;
            } else {
                body = part;
            }
        }

        let mut status = 200;
        let mut headers = HashMap::new();

        for line in header_section.lines() {
            if line.starts_with("HTTP/") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    status = parts[1].parse().unwrap_or(200);
                }
            } else if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }

        Ok(HttpResponse {
            status,
            body: body.to_string(),
            headers,
            success: status >= 200 && status < 300,
        })
    }
}
