/*
 * Author: Dylan Turner
 * Description: Define the app window
 */

use eframe::epi::{ App, Frame };
use eframe::egui::{
    CtxRef, TopBottomPanel, SidePanel, CentralPanel, Window,
    Slider, Layout, Align, Hyperlink,
    github_link_file, warn_if_debug_build
};
use eframe::egui::menu;

pub struct ClongerWindow {
    value : f32,
    label : String
}

impl Default for ClongerWindow {
    fn default() -> Self {
        Self {
            value : 0.0,
            label : String::from("Hello, world!")
        }
    }
}

impl App for ClongerWindow {
    fn name(&self) -> &str {
        return "Clonger";
    }

    fn update(&mut self, ctx : &CtxRef, frame : &mut Frame<'_>) {
        let Self { value, label } = self;

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            menu::bar(ui, |ui| {
                menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add(
                    Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui template");
            ui.hyperlink("https://github.com/emilk/egui_template");
            ui.add(github_link_file!(
                "https://github.com/emilk/egui_template/blob/master/",
                "Source code."
            ));
            warn_if_debug_build(ui);
        });

        if true {
            Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}