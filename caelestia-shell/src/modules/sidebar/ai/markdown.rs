//! Markdown AST tokenizer and block parser for AI Chat messages
//! Ported with 100% parity from `ChatMarkdown.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkdownBlock {
    Heading { level: u8, text: String },
    Paragraph(String),
    CodeBlock { language: Option<String>, code: String },
    ListItem { bullet: String, text: String },
    Blockquote(String),
    ThematicBreak,
}

pub struct ChatMarkdownParser;

impl ChatMarkdownParser {
    pub fn parse(raw: &str) -> Vec<MarkdownBlock> {
        let mut blocks = Vec::new();
        let mut in_code_block = false;
        let mut code_lang = None;
        let mut code_buffer = Vec::new();
        let mut paragraph_buffer = Vec::new();

        let flush_paragraph = |buffer: &mut Vec<String>, out: &mut Vec<MarkdownBlock>| {
            if !buffer.is_empty() {
                let text = buffer.join("\n");
                out.push(MarkdownBlock::Paragraph(text));
                buffer.clear();
            }
        };

        for line in raw.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                if in_code_block {
                    // Closing code block
                    blocks.push(MarkdownBlock::CodeBlock {
                        language: code_lang.take(),
                        code: code_buffer.join("\n"),
                    });
                    code_buffer.clear();
                    in_code_block = false;
                } else {
                    // Opening code block
                    flush_paragraph(&mut paragraph_buffer, &mut blocks);
                    in_code_block = true;
                    let lang = trimmed.trim_start_matches('`').trim();
                    code_lang = if lang.is_empty() { None } else { Some(lang.to_string()) };
                }
                continue;
            }

            if in_code_block {
                code_buffer.push(line.to_string());
                continue;
            }

            if trimmed.is_empty() {
                flush_paragraph(&mut paragraph_buffer, &mut blocks);
                continue;
            }

            if trimmed == "---" || trimmed == "***" || trimmed == "___" {
                flush_paragraph(&mut paragraph_buffer, &mut blocks);
                blocks.push(MarkdownBlock::ThematicBreak);
                continue;
            }

            if trimmed.starts_with('#') {
                flush_paragraph(&mut paragraph_buffer, &mut blocks);
                let level = trimmed.chars().take_while(|c| *c == '#').count() as u8;
                let text = trimmed.trim_start_matches('#').trim().to_string();
                blocks.push(MarkdownBlock::Heading { level, text });
                continue;
            }

            if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                flush_paragraph(&mut paragraph_buffer, &mut blocks);
                let bullet = trimmed[..2].to_string();
                let text = trimmed[2..].trim().to_string();
                blocks.push(MarkdownBlock::ListItem { bullet, text });
                continue;
            }

            if trimmed.starts_with("> ") {
                flush_paragraph(&mut paragraph_buffer, &mut blocks);
                let text = trimmed[2..].trim().to_string();
                blocks.push(MarkdownBlock::Blockquote(text));
                continue;
            }

            paragraph_buffer.push(line.to_string());
        }

        if in_code_block && !code_buffer.is_empty() {
            blocks.push(MarkdownBlock::CodeBlock {
                language: code_lang,
                code: code_buffer.join("\n"),
            });
        }

        flush_paragraph(&mut paragraph_buffer, &mut blocks);
        blocks
    }
}
