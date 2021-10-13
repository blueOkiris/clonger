/*
 * Author: Dylan Turner
 * Description: Define the IPA typing popup window and functionality
 */

use crate::app::ClongerWindow;
use eframe::egui::{ CtxRef, Window, Pos2, TextEdit, TextStyle };
use eframe::epi::Frame;

pub fn create_ipa_view(
        win : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    Window::new("IPA Typer").default_pos(
        Pos2::new(800000.0, 800000.0) // Put at bottom of screen
    ).scroll(true).show(ctx, |ui| {
        ui.label("Type into here with shortcuts to create IPA:");
        ui.add_sized(
            ui.available_size(),
            TextEdit::multiline(&mut win.ipa_tb)
                .desired_rows(6)
                .text_style(TextStyle::Monospace)
        );
    });
}
