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
        ui.label(format!("{}{}",
            "Press Alt+<key> multiple times based on this table",
            " to enter special characters:"
        ));

        ui.label("Type into here with shortcuts to create IPA:");
        let resp = ui.add_sized(
            ui.available_size(),
            TextEdit::multiline(&mut win.ipa_tb)
                .desired_rows(6)
                .text_style(TextStyle::Monospace)
        );

        if resp.changed() {
            // If the user types alt, remove the char and replace from table
            let inp = ui.input();
            let mods = inp.modifiers;
            if mods.alt
                    && !mods.ctrl && !mods.shift
                    && !mods.command && !mods.mac_cmd {
                win.ipa_tb.remove(win.ipa_tb.len() - 1);
            }
        }
    });
}
