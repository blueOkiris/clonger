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

enum WindowState {
    Document,
    Dictionary,
    Examples
}

pub struct ClongerWindow {
    state : WindowState,
    ipa_view_open : bool
}

impl Default for ClongerWindow {
    fn default() -> Self {
        Self {
            state : WindowState::Document,
            ipa_view_open : true
        }
    }
}

impl App for ClongerWindow {
    fn name(&self) -> &str {
        return "Clonger";
    }

    fn update(&mut self, ctx : &CtxRef, frame : &mut Frame<'_>) {
        create_menu_bar(self, ctx, frame);
        create_side_navigation(self, ctx, frame);

        CentralPanel::default().show(ctx, |ui| {
            match self.state {
                WindowState::Document => {
                    ui.heading("Document");

                }, WindowState::Dictionary => {
                    ui.heading("Dictionary");

                }, WindowState::Examples => {
                    ui.heading("Examples");

                }
            }
        });

        if self.ipa_view_open {
            create_ipa_view(self, ctx, frame);
        }
    }
}

fn create_ipa_view(
        me : &mut ClongerWindow, ctx : &CtxRef, frame : &mut Frame<'_>,) {
    Window::new("IPA Typer").show(ctx, |ui| {
        ui.label("Windows can be moved by dragging them.");
        
    });
}

fn create_side_navigation(
        me : &mut ClongerWindow, ctx : &CtxRef, frame : &mut Frame<'_>,) {
    SidePanel::right("View").show(ctx, |ui| {
        ui.heading("View");

        if ui.button("Toggle IPA").clicked() {
            me.ipa_view_open = !me.ipa_view_open;
        }
        if ui.button("Document").clicked() {
            me.state = WindowState::Document;
        }
        if ui.button("Dictionary").clicked() {
            me.state = WindowState::Dictionary;
        }
        if ui.button("Examples").clicked() {
            me.state = WindowState::Examples;
        }
    });
}

fn create_menu_bar(
        me : &mut ClongerWindow, ctx : &CtxRef, frame : &mut Frame<'_>,) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            menu::menu(ui, "File", |ui| {
                if ui.button("New").clicked() {
                    
                }
                if ui.button("Save").clicked() {
                    
                }
                if ui.button("Save As").clicked() {
                    
                }
                if ui.button("Open").clicked() {
                    
                }
                if ui.button("Exit").clicked() {
                    frame.quit();
                }
            });
            
            menu::menu(ui, "View", |ui| {
                if ui.button("Toggle IPA").clicked() {
                    me.ipa_view_open = !me.ipa_view_open;
                }
                if ui.button("Document").clicked() {
                    me.state = WindowState::Document;
                }
                if ui.button("Dictionary").clicked() {
                    me.state = WindowState::Dictionary;
                }
                if ui.button("Examples").clicked() {
                    me.state = WindowState::Examples;
                }
            });
        });
    });
}