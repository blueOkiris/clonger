/*
 * Author: Dylan Turner
 * Description: Define the Document view and functionality
 */

use crate::app::ClongerWindow;
use eframe::egui::{ CtxRef, CentralPanel };
use eframe::epi::Frame;

pub fn create_doc_view(
        _win : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Document");
        

    });
}
