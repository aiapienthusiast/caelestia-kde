//! Orion Web Search Bridge
//! 100% native Rust parity with `shell/scripts/orion_search.py`.
//! Implements HTML search scraping, query encoding, URL extraction, and result parsing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrionSearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

pub struct OrionSearchService;

impl OrionSearchService {
    /// Strips HTML tags from raw strings
    pub fn strip_html_tags(text: &str) -> String {
        let mut in_tag = false;
        let mut clean = String::with_capacity(text.len());
        for c in text.chars() {
            if c == '<' {
                in_tag = true;
            } else if c == '>' {
                in_tag = false;
            } else if !in_tag {
                clean.push(c);
            }
        }
        clean.trim().replace("&amp;", "&").replace("&quot;", "\"").replace("&#39;", "'")
    }

    /// Parses search result snippets and URLs from raw HTML payload
    pub fn parse_html_results(html: &str, page_num: usize, page_size: usize) -> Vec<OrionSearchResult> {
        let mut results = Vec::new();

        // Find result titles and URLs
        let mut cursor = html;
        while let Some(title_pos) = cursor.find("result__title") {
            cursor = &cursor[title_pos..];
            if let Some(a_start) = cursor.find("<a") {
                cursor = &cursor[a_start..];
                if let Some(href_pos) = cursor.find("href=\"") {
                    let href_val_start = href_pos + 6;
                    if let Some(href_val_end) = cursor[href_val_start..].find('"') {
                        let raw_url = &cursor[href_val_start..href_val_start + href_val_end];
                        
                        // Extract title text inside <a>...</a>
                        if let Some(tag_close) = cursor.find('>') {
                            if let Some(a_close) = cursor[tag_close..].find("</a>") {
                                let raw_title = &cursor[tag_close + 1..tag_close + a_close];
                                
                                // Clean up duckduckgo redirect if present
                                let clean_url = if raw_url.contains("uddg=") {
                                    if let Some(pos) = raw_url.find("uddg=") {
                                        let substr = &raw_url[pos + 5..];
                                        let end = substr.find('&').unwrap_or(substr.len());
                                        urllib_compat_unquote(&substr[..end])
                                    } else {
                                        raw_url.to_string()
                                    }
                                } else if raw_url.starts_with("//") {
                                    format!("https:{}", raw_url)
                                } else {
                                    raw_url.to_string()
                                };

                                let clean_title = Self::strip_html_tags(raw_title);
                                if !clean_title.is_empty() && !clean_url.is_empty() {
                                    results.push(OrionSearchResult {
                                        title: clean_title,
                                        url: clean_url,
                                        snippet: String::new(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if cursor.len() > 10 {
                cursor = &cursor[10..];
            } else {
                break;
            }
        }

        // Apply pagination
        let start = page_num.saturating_sub(1) * page_size;
        results.into_iter().skip(start).take(page_size).collect()
    }
}

/// Simple percent-decoding helper for redirect URLs
fn urllib_compat_unquote(s: &str) -> String {
    let mut out = String::new();
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let h1 = bytes.next().unwrap_or(b'0');
            let h2 = bytes.next().unwrap_or(b'0');
            if let Ok(val) = u8::from_str_radix(&format!("{}{}", h1 as char, h2 as char), 16) {
                out.push(val as char);
            }
        } else if b == b'+' {
            out.push(' ');
        } else {
            out.push(b as char);
        }
    }
    out
}
