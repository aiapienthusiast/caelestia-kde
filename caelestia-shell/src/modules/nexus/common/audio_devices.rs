//! Audio Device List for Nexus Hardware Audio Page
//! Parity with `shell/modules/nexus/common/AudioDeviceList.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::audio::AudioDevice;
use gpui::{div, prelude::*, px, rgb, rgba, CursorStyle, Div, Styled};

/// Audio Device Selector List for Sinks (Outputs) or Sources (Inputs)
#[derive(Clone, Debug)]
pub struct AudioDeviceList {
    pub is_sink: bool,
    pub current_id: Option<u32>,
    pub filter: String,
    pub is_expanded: bool,
}

impl AudioDeviceList {
    pub fn new(is_sink: bool) -> Self {
        Self {
            is_sink,
            current_id: None,
            filter: String::new(),
            is_expanded: true,
        }
    }

    pub fn filter_devices<'a>(&self, devices: &'a [AudioDevice]) -> Vec<&'a AudioDevice> {
        let needle = self.filter.trim().to_lowercase();
        devices
            .iter()
            .filter(|d| {
                if needle.is_empty() {
                    true
                } else {
                    d.name.to_lowercase().contains(&needle)
                        || d.description.to_lowercase().contains(&needle)
                }
            })
            .collect()
    }

    pub fn select(&mut self, id: u32) {
        self.current_id = Some(id);
    }

    pub fn render(&self, devices: &[AudioDevice], colours: &CaelestiaColours) -> Div {
        let title = if self.is_sink { "Output Devices (Speakers / Headphones)" } else { "Input Devices (Microphones)" };
        let default_icon = if self.is_sink { "volume_up" } else { "mic" };
        let filtered = self.filter_devices(devices);

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .p(px(PADDING_MEDIUM))
            .gap(px(SPACING_SMALL));

        // Header
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .text_color(rgb(colours.primary))
                            .child(default_icon),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_TITLE_SMALL))
                            .text_color(rgb(colours.on_surface))
                            .child(title),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(format!("{} available", filtered.len())),
            );

        root = root.child(header);

        if self.is_expanded {
            let mut list_col = div()
                .flex()
                .flex_col()
                .w_full()
                .gap(px(4.0));

            if filtered.is_empty() {
                list_col = list_col.child(
                    div()
                        .p(px(8.0))
                        .text_size(px(12.0))
                        .text_color(rgb(colours.on_surface_variant))
                        .child("No audio devices detected"),
                );
            } else {
                for dev in filtered {
                    let is_selected = self.current_id.map_or(dev.is_default, |id| id == dev.id);
                    let row = div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .p(px(8.0))
                        .rounded(px(ROUNDING_SMALL))
                        .bg(if is_selected {
                            rgba((colours.primary_container << 8) | 0xFF)
                        } else {
                            rgba((colours.surface_container_high << 8) | 0xFF)
                        })
                        .cursor(CursorStyle::PointingHand)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .text_color(if is_selected {
                                            rgb(colours.primary)
                                        } else {
                                            rgb(colours.on_surface_variant)
                                        })
                                        .child(if dev.muted { "volume_off" } else { default_icon }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(if is_selected {
                                                    rgb(colours.on_primary_container)
                                                } else {
                                                    rgb(colours.on_surface)
                                                })
                                                .child(if dev.description.is_empty() {
                                                    dev.name.clone()
                                                } else {
                                                    dev.description.clone()
                                                }),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(colours.on_surface_variant))
                                                .child(format!(
                                                    "ID: {} • Volume: {}%{}",
                                                    dev.id,
                                                    (dev.volume * 100.0).round() as u32,
                                                    if dev.is_default { " (System Default)" } else { "" }
                                                )),
                                        ),
                                ),
                        )
                        .child(
                            if is_selected {
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(20.0))
                                    .h(px(20.0))
                                    .rounded(px(10.0))
                                    .bg(rgb(colours.primary))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(rgb(colours.on_primary))
                                            .child("check"),
                                    )
                            } else {
                                div()
                            },
                        );
                    list_col = list_col.child(row);
                }
            }

            root = root.child(list_col);
        }

        root
    }
}
