/*
 * Author: Dylan Turner
 * Description:
 *  - Define the Document view and functionality
 *  - Code in the functions highlight, format_from_style taken from
 *      https://github.com/emilk/egui/tree/master/egui_demo_lib/src/easy_mark
 *      which is licensed under MIT 
 */

use crate::app::ClongerWindow;
use eframe::egui::{
    CtxRef, CentralPanel, ScrollArea, Ui, Visuals, TextEdit,
    Align, Color32, Stroke, TextStyle
};
use eframe::egui::text::{ LayoutJob, TextFormat };
use eframe::epi::Frame;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Style {
    pub heading : bool,
    pub quoted : bool,
    pub code : bool,
    pub strong : bool,
    pub underline : bool,
    pub strikethrough : bool,
    pub italics : bool,
    pub small : bool,
    pub raised : bool
}

pub fn create_doc_view(
        win : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Document");
        
        ui.columns(2, |columns| {
            ScrollArea::vertical()
                .id_source("source")
                .show(&mut columns[0], |ui| { // Editor view
                    // Layout control (for wrapping and stuff)
                    let mut layouter =
                        |ui : &Ui, code : &str, wrap_width : f32| {
                            let mut job = highlight(ui.visuals(), code);
                            job.wrap_width = wrap_width;
                            return ui.fonts().layout_job(job);
                        };

                    // Text box
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut win.doc_code)
                            .desired_width(f32::INFINITY)
                            .text_style(TextStyle::Monospace)
                            .layouter(&mut layouter)
                    );
                });
            ScrollArea::vertical()
                .id_source("rendered")
                .show(&mut columns[1], |_ui| {
                    // Rendered markdown

                });
        });
    });
}

// Highlight the markdown within the editor
fn highlight(visuals : &Visuals, mut code : &str) -> LayoutJob {
    let mut job = LayoutJob::default();
    let mut style = Style::default();
    let mut start_of_line = true;

    while !code.is_empty() {
        if start_of_line && code.starts_with("```") {
            let end = code.find("\n```").map_or_else(|| code.len(), |i| i + 4);
            job.append(
                &code[..end],
                0.0,
                format_from_style(
                    visuals,
                    &Style {
                        code : true,
                        ..Default::default()
                    },
                ),
            );
            code = &code[end..];
            style = Default::default();
            continue;
        }

        if code.starts_with('`') {
            style.code = true;
            let end = code[1..]
                .find(&['1', '\n'][..])
                .map_or_else(|| code.len(), |i| i + 2);
            job.append(&code[..end], 0.0, format_from_style(visuals, &style));
            code = &code[end..];
            style.code = false;
            continue;
        }

        let mut skip;
        if code.starts_with('\\') && code.len() >= 2 {
            skip = 2;
        } else if start_of_line && code.starts_with(' ') {
            skip = 1;
        } else if start_of_line && code.starts_with("# ") {
            style.heading = true;
            skip = 2;
        } else if start_of_line && code.starts_with("> ") {
            style.quoted = true;
            skip = 2;
        } else if start_of_line && code.starts_with("- ") {
            skip = 2;
        } else if code.starts_with('*') {
            skip = 1;
            if style.strong {
                job.append(
                    &code[..skip], 0.0, format_from_style(visuals, &style)
                );
                code = &code[skip..];
                skip = 0;
            }
            style.strong ^= true;
        } else if code.starts_with('$') {
            skip = 1;
            if style.small {
                job.append(
                    &code[..skip], 0.0, format_from_style(visuals, &style)
                );
            }
            style.small ^= true;
        } else if code.starts_with('^') {
            skip = 1;
            if style.raised {
                job.append(
                    &code[..skip], 0.0, format_from_style(visuals, &style)
                );
                code = &code[skip..];
                skip = 0;
            }
            style.raised ^= true;
        } else {
            skip = 0;
        }

        let line_end = code[skip..]
            .find('\n')
            .map_or_else(|| code.len(), |i| (skip + i + 1));
        let end = code[skip..]
            .find(&['*', '`', '~', ')', '/', '$', '\\', '<', '['][..])
            .map_or_else(|| code.len(), |i| (skip + i).max(1));
        
        if line_end <= end {
            job.append(
                &code[..line_end], 0.0, format_from_style(visuals, &style)
            );
            code = &code[line_end..];
            start_of_line = true;
            style = Default::default();
        } else {
            job.append(
                &code[..end], 0.0, format_from_style(visuals, &style)
            );
            code = &code[end..];
            start_of_line = false;
        }
    }

    return job;
}

// Set up a chunk of text's color and stuff
fn format_from_style(visuals : &Visuals, style : &Style) -> TextFormat {
    let color = if style.strong || style.heading {
        visuals.strong_text_color()
    } else if style.quoted {
        visuals.weak_text_color()
    } else {
        visuals.text_color()
    };

    let text_style = if style.heading {
        TextStyle::Heading
    } else if style.code {
        TextStyle::Monospace
    } else if style.small | style.raised {
        TextStyle::Small
    } else {
        TextStyle::Body
    };

    let bg = if style.code {
        visuals.code_bg_color
    } else {
        Color32::TRANSPARENT
    };

    let underline = if style.underline {
        Stroke::new(1.0, color)
    } else {
        Stroke::none()
    };

    let strikethrough = if style.strikethrough {
        Stroke::new(1.0, color)
    } else {
        Stroke::none()
    };

    let valign = if style.raised {
        Align::TOP
    } else {
        Align::BOTTOM
    };

    return TextFormat {
        style : text_style,
        color,
        background : bg,
        italics : style.italics,
        underline,
        strikethrough,
        valign
    };
}
