/*
 * Author: Dylan Turner
 * Description: Define the Dictionary view and functionality
 */

use crate::app::ClongerWindow;
use eframe::egui::{ CtxRef, CentralPanel };
use eframe::epi::Frame;

pub fn create_dict_view(
        _me : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Dictionary");
        
        
    });
}
