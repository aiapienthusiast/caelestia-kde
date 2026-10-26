use caelestia_shell::modules::sidebar::ai::*;
use caelestia_shell::modules::sidebar::ai_assistant::MessageRole;

#[test]
fn test_chat_markdown_parser() {
    let raw = r#"# Caelestia Shell

This is a paragraph explaining the feature.

- Item 1
- Item 2

> Note: this is a blockquote

```rust
fn main() {
    println!("Hello World");
}
```
"#;

    let blocks = ChatMarkdownParser::parse(raw);
    assert!(blocks.len() >= 5);

    match &blocks[0] {
        MarkdownBlock::Heading { level, text } => {
            assert_eq!(*level, 1);
            assert_eq!(text, "Caelestia Shell");
        }
        _ => panic!("Expected Heading"),
    }

    match &blocks[1] {
        MarkdownBlock::Paragraph(text) => {
            assert!(text.contains("This is a paragraph"));
        }
        _ => panic!("Expected Paragraph"),
    }

    let code_block = blocks.iter().find(|b| matches!(b, MarkdownBlock::CodeBlock { .. }));
    assert!(code_block.is_some());
    if let Some(MarkdownBlock::CodeBlock { language, code }) = code_block {
        assert_eq!(language.as_deref(), Some("rust"));
        assert!(code.contains("println!(\"Hello World\")"));
    }
}

#[test]
fn test_tool_card_lifecycle() {
    let mut card = ToolCallCardModel::new("call_1", "get_system_stats", "{\"target\":\"cpu\"}");
    assert_eq!(card.status, ToolExecutionStatus::Pending);

    card.complete("{\"usage\": 14.5}", true, 120);
    assert_eq!(card.status, ToolExecutionStatus::Success);
    assert_eq!(card.output_json.as_deref(), Some("{\"usage\": 14.5}"));
    assert_eq!(card.execution_time_ms, Some(120));
}

#[test]
fn test_chat_store_sessions_management() {
    let mut store = ChatStoreManager::default();
    assert_eq!(store.sessions.len(), 1);

    let session_id = store.create_session("Coding assistance");
    assert_eq!(store.sessions.len(), 2);
    assert_eq!(store.active_session_id.as_deref(), Some(session_id.as_str()));

    if let Some(session) = store.active_session_mut() {
        session.add_message(MessageRole::User, "Help me write Rust code");
        session.add_message(MessageRole::Assistant, "Certainly! Let's write safe Rust.");
        assert_eq!(session.messages.len(), 2);
    }

    assert!(store.delete_session(&session_id));
    assert_eq!(store.sessions.len(), 1);
}
