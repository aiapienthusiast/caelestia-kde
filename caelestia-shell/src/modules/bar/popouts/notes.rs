//! Notes Popout
//! Parity with `shell/modules/bar/popouts/Notes.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_button::IconButton;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NoteType {
    #[default]
    Text,
    List,
    Audio,
    Image,
}

impl NoteType {
    pub fn icon(&self) -> &'static str {
        match self {
            NoteType::Text => "subject",
            NoteType::List => "checklist",
            NoteType::Audio => "mic",
            NoteType::Image => "image",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoteItem {
    pub id: u64,
    pub note_type: NoteType,
    pub title: String,
    pub body: String,
    pub items: Vec<(bool, String)>,
    pub updated_str: String,
}

#[derive(Clone, Debug)]
pub struct NotesPopoutState {
    pub notes: Vec<NoteItem>,
    pub selected_index: usize,
}

impl Default for NotesPopoutState {
    fn default() -> Self {
        Self {
            notes: vec![
                NoteItem {
                    id: 1,
                    note_type: NoteType::Text,
                    title: "Architecture Goals".into(),
                    body: "Recursively adapt modules to new GPUI components ensuring 100% parity with QML.".into(),
                    items: Vec::new(),
                    updated_str: "Just now".into(),
                },
                NoteItem {
                    id: 2,
                    note_type: NoteType::List,
                    title: "Component Checklist".into(),
                    body: "".into(),
                    items: vec![
                        (true, "List Row & Styled Text".into()),
                        (true, "SplitButton & IconButton".into()),
                        (true, "Popout Wrappers".into()),
                        (false, "Dashboard parity".into()),
                    ],
                    updated_str: "10m ago".into(),
                },
            ],
            selected_index: 0,
        }
    }
}

pub struct NotesPopout;

impl NotesPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &NotesPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &NotesPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let selected_note = state.notes.get(state.selected_index);

        let content = div()
            .flex()
            .flex_col()
            .w(px(560.0))
            .gap(px(SPACING_SMALL))
            // Header Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        StyledText::new("Notes")
                            .text_size(px(FONT_SIZE_BODY_LARGE))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(c.on_surface)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(SPACING_EXTRA_SMALL))
                            .child(
                                IconButton::new("subject")
                                    .with_colours(c.clone())
                                    .into_element(),
                            )
                            .child(
                                IconButton::new("checklist")
                                    .with_colours(c.clone())
                                    .into_element(),
                            )
                            .child(
                                IconButton::new("mic")
                                    .with_colours(c.clone())
                                    .into_element(),
                            )
                            .child(
                                IconButton::new("image")
                                    .with_colours(c.clone())
                                    .into_element(),
                            ),
                    ),
            )
            // Divider
            .child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(rgba(c.outline_variant)),
            )
            // Master-Detail 2-Pane Section
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(SPACING_MEDIUM))
                    .h(px(340.0))
                    // Left Pane: Notes List
                    .child(
                        div()
                            .id("notes_list_pane")
                            .w(px(220.0))
                            .h_full()
                            .flex()
                            .flex_col()
                            .gap(px(SPACING_EXTRA_SMALL))
                            .overflow_y_scroll()
                            .children(state.notes.iter().enumerate().map(|(idx, note)| {
                                let is_selected = idx == state.selected_index;
                                let bg_col = if is_selected {
                                    rgba(c.secondary_container)
                                } else {
                                    rgba(c.surface_container_low)
                                };
                                let fg_col = if is_selected {
                                    rgba(c.on_secondary_container)
                                } else {
                                    rgba(c.on_surface)
                                };

                                div()
                                    .id(SharedString::from(format!("note_item_{}", note.id)))
                                    .w_full()
                                    .p(px(PADDING_SMALL))
                                    .rounded(px(if is_selected { ROUNDING_LARGE } else { ROUNDING_MEDIUM }))
                                    .bg(bg_col)
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(SPACING_EXTRA_SMALL))
                                            .child(
                                                MaterialIcon::new(note.note_type.icon())
                                                    .text_size(px(ICON_SIZE_SMALL))
                                                    .text_color(rgba(c.on_surface_variant)),
                                            )
                                            .child(
                                                StyledText::new(note.title.clone())
                                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(fg_col),
                                            ),
                                    )
                                    .child(
                                        StyledText::new(note.updated_str.clone())
                                            .text_size(px(FONT_SIZE_LABEL_SMALL))
                                            .text_color(rgba(c.outline)),
                                    )
                            })),
                    )
                    // Right Pane: Active Note Editor / Viewer
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .p(px(PADDING_MEDIUM))
                            .rounded(px(ROUNDING_LARGE))
                            .bg(rgba(c.surface_container))
                            .flex()
                            .flex_col()
                            .justify_between()
                            .child(
                                if let Some(note) = selected_note {
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(SPACING_SMALL))
                                        .child(
                                            StyledText::new(note.title.clone())
                                                .text_size(px(FONT_SIZE_TITLE_MEDIUM))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgba(c.on_surface)),
                                        )
                                        .child(
                                            if !note.body.is_empty() {
                                                div().child(
                                                    StyledText::new(note.body.clone())
                                                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                                        .text_color(rgba(c.on_surface_variant)),
                                                )
                                            } else {
                                                div()
                                            },
                                        )
                                        .children(note.items.iter().map(|(done, text)| {
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(SPACING_SMALL))
                                                .child(
                                                    MaterialIcon::new(if *done {
                                                        "check_box"
                                                    } else {
                                                        "check_box_outline_blank"
                                                    })
                                                    .text_size(px(ICON_SIZE_SMALL))
                                                    .text_color(if *done {
                                                        rgba(c.primary)
                                                    } else {
                                                        rgba(c.outline)
                                                    }),
                                                )
                                                .child(
                                                    StyledText::new(text.clone())
                                                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                                        .text_color(rgba(c.on_surface)),
                                                )
                                        }))
                                } else {
                                    div().child(
                                        StyledText::new("No note selected")
                                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                            .text_color(rgba(c.on_surface_variant)),
                                    )
                                },
                            )
                            // Editor Bottom Actions
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_end()
                                    .child(
                                        IconButton::new("delete_outline")
                                            .with_colours(c.clone())
                                            .into_element(),
                                    ),
                            ),
                    ),
            );

        PopoutWrapper::render("Notes", "edit_note", center_x, content, c)
    }
}
