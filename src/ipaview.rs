/*
 * Author: Dylan Turner
 * Description: Define the IPA typing popup window and functionality
 */

use crate::app::ClongerWindow;
use std::collections::HashMap;
use eframe::egui::{ CtxRef, Window, Pos2, TextEdit, TextStyle };
use eframe::epi::Frame;

const IPA_CHARS : &'static [(char, &'static [&'static str])] = &[
    ('a', &[ "ɑ", "æ", "ɐ", "ɑ̃" ])
];

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

        let inp = ui.input();
        let mods = inp.modifiers;
        if !mods.alt {
            win.ipa_cur_char = '\0';
            win.ipa_char_count = 0;
        }
        if resp.changed() {
            // If the user types alt, remove the char and replace from table
            if mods.alt
                    && !mods.ctrl && !mods.shift
                    && !mods.command && !mods.mac_cmd {
                let typed_char = win.ipa_tb.remove(win.ipa_tb.len() - 1);

                let char_map : HashMap<char, &'static [&'static str]> =
                    IPA_CHARS.iter().cloned().collect();
                let new_char_option = char_map.get(&typed_char);
                match new_char_option {
                    None => win.ipa_tb.push(typed_char),
                    Some(ipa_str_arr) => {
                        if win.ipa_cur_char != typed_char {
                            win.ipa_cur_char = typed_char;
                            win.ipa_char_count = 0;
                        } else {
                            let cur_ipa_char = ipa_str_arr[win.ipa_char_count];
                            for _c in cur_ipa_char.chars() {
                                win.ipa_tb.pop();
                            }

                            win.ipa_char_count += 1;
                            if win.ipa_char_count >= ipa_str_arr.len() {
                                win.ipa_char_count = 0;
                            }
                        }

                        let ipa_char = ipa_str_arr[win.ipa_char_count].chars();
                        for c in ipa_char {
                            win.ipa_tb.push(c);
                        }
                    }
                }
            }
        }
    });
}
