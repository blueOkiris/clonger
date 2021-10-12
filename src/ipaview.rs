/*
 * Author: Dylan Turner
 * Description: Define the IPA typing popup window and functionality
 */

use crate::app::ClongerWindow;
use eframe::egui::{ CtxRef, Window };
use eframe::epi::Frame;

pub fn create_ipa_view(
        _me : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    Window::new("IPA Typer").show(ctx, |ui| {
        ui.label("Windows can be moved by dragging them.");
        
    });
}
