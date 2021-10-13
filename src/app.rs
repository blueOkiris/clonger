/*
 * Author: Dylan Turner
 * Description: Define the app window
 */

use eframe::epi::{ App, Frame };
use eframe::egui::{ CtxRef, TopBottomPanel, SidePanel };
use eframe::egui::menu;

use crate::{ ipaview, docview, dictview, exview };

enum WindowState {
    Document,
    Dictionary,
    Examples
}

pub struct ClongerWindow {
    state : WindowState,
    ipa_view_open : bool,
    
    pub ipa_tb : String
}

impl Default for ClongerWindow {
    fn default() -> Self {
        Self {
            state : WindowState::Document,
            ipa_view_open : true,
            ipa_tb : String::new()
        }
    }
}

// Main functions for app construction
impl App for ClongerWindow {
    fn name(&self) -> &str {
        return "Clonger";
    }

    fn update(&mut self, ctx : &CtxRef, frame : &mut Frame<'_>) {
        create_menu_bar(self, ctx, frame);
        create_side_navigation(self, ctx, frame);

        match self.state {
            WindowState::Document => docview::create_doc_view(self, ctx, frame),
            WindowState::Dictionary =>
                dictview::create_dict_view(self, ctx, frame),
            WindowState::Examples => exview::create_ex_view(self, ctx, frame),
        }

        if self.ipa_view_open {
            ipaview::create_ipa_view(self, ctx, frame);
        }
    }
}

fn create_side_navigation(
        win : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    SidePanel::right("View").show(ctx, |ui| {
        ui.heading("View");

        if ui.button("Toggle IPA").clicked() {
            win.ipa_view_open = !win.ipa_view_open;
        }
        if ui.button("Document").clicked() {
            win.state = WindowState::Document;
        }
        if ui.button("Dictionary").clicked() {
            win.state = WindowState::Dictionary;
        }
        if ui.button("Examples").clicked() {
            win.state = WindowState::Examples;
        }
    });
}

fn create_menu_bar(
        win : &mut ClongerWindow, ctx : &CtxRef, frame : &mut Frame<'_>,) {
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
                    win.ipa_view_open = !win.ipa_view_open;
                }
                if ui.button("Document").clicked() {
                    win.state = WindowState::Document;
                }
                if ui.button("Dictionary").clicked() {
                    win.state = WindowState::Dictionary;
                }
                if ui.button("Examples").clicked() {
                    win.state = WindowState::Examples;
                }
            });
        });
    });
}