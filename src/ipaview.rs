/*
 * Author: Dylan Turner
 * Description: Define the IPA typing popup window and functionality
 */

use crate::app::ClongerWindow;
use eframe::egui::{ CtxRef, Window, Pos2 };
use eframe::epi::Frame;

pub fn create_ipa_view(
        _me : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    Window::new("IPA Typer").default_pos(
        Pos2::new(800000.0, 800000.0) // Put at bottom of screen
    ).show(ctx, |ui| {
        ui.label("Windows can be moved by dragging them.");
        
    });
}
