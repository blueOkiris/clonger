/*
 * Author: Dylan Turner
 * Description: Define the IPA typing popup window and functionality
 */

use crate::app::ClongerWindow;
use std::collections::HashMap;
use eframe::egui::{ CtxRef, Window, Pos2, TextEdit, TextStyle, Id };
use eframe::epi::Frame;

const IPA_CHARS : &'static [(char, &'static [&'static str])] = &[
    ('a', &[ "ɑ", "æ", "ɐ", "ɑ̃" ]),
    ('b', &[ "β", "ɓ" ]),
    ('c', &[ "ç", "ɕ"]),
    ('d', &[ "ð", "d͡ʒ", "ɖ", "ɗ" ]),
    ('e', &[ "ə", "ɚ", "ɵ" ]),
    ('3', &[ "ɛ", "ɜ", "ɝ", "ɛ̃" ]),
    ('g', &[ "ɠ", "ɢ"]),
    ('h', &[ "ħ", "ɦ", "ɥ", "ɧ", "ʜ" ]),
    ('i', &[ "ɪ", "ɨ", "ɪ̈" ]),
    ('j', &[ "ʝ", "ɟ"]),
    ('l', &[ "ɫ", "ɭ", "ɬ", "ʟ", "ɮ" ]),
    ('m', &[ "ɱ" ]),
    ('n', &[ "ŋ", "ɲ", "ɳ", "ɴ" ]),
    ('o', &[ "ɔ", "œ", "ɒ", "ɔ̃" ]),
    ('0', &[ "ø" ]),
    ('p', &[ "ɸ" ]),
    ('r', &[ "ɾ", "ɹ", "ʁ", "ʀ", "ɻ", "ɽ" ]),
    ('s', &[ "ʃ", "ʂ" ]),
    ('t', &[ "θ", "t͡ʃ", "t͡s", "ʈ" ]),
    ('u', &[ "ʊ", "ʉ" ]),
    ('v', &[ "ʌ", "ʋ" ]),
    ('w', &[ "ɯ", "ʍ" ]),
    ('x', &[ "χ" ]),
    ('y', &[ "ɣ", "ʎ", "ʏ", "ɤ" ]),
    ('z', &[ "ʒ", "ʐ", "ʑ" ]),
    ('2', &[ "ʔ", "ʕ" ]),
    ('q', &[ "ˈ", "ˌ" ]),
    (':', &[ "ː" ]),
    ('4', &[ "ʰ", "ʲ", "ʷ" ])
];
const MAX_DISP_COL : u8 = 6;

pub fn create_ipa_view(
        win : &mut ClongerWindow, ctx : &CtxRef, _frame : &mut Frame<'_>,) {
    Window::new("IPA Typer").default_pos(
        Pos2::new(800000.0, 800000.0) // Put at bottom of screen
    ).scroll(true).show(ctx, |ui| {
        ui.label(format!("{}{}",
            "Press Alt+<key> multiple times based on this table",
            " to enter special characters:"
        ));
        let mut i = 0;
        while i < IPA_CHARS.len() {
            ui.horizontal(|ui,| {
                for col in 0..MAX_DISP_COL {
                    if i >= IPA_CHARS.len() {
                        break;
                    }
                    
                    if col != 0 {
                        ui.label("|");
                    }

                    ui.label(format!("{}", IPA_CHARS[i].0));
                    ui.label("→");
                    for hotkey in IPA_CHARS[i].1 {
                        ui.label(format!("{}", hotkey));
                    }

                    i += 1;
                }
            });
        }

        ui.label("");
        ui.label("Type into here with shortcuts to create IPA:");
        let ipa_tb_last_char = win.ipa_tb.len() -1;
        let editor =
            TextEdit::multiline(&mut win.ipa_tb)
                .desired_rows(8)
                .text_style(TextStyle::Monospace)
                .id(Id::new("ipa_tb"));
        let cursor = TextEdit::<String>::cursor(ui, Id::new("ipa_tb"));
        let cursor_pos = match cursor {
            Some(cursor_pair) => cursor_pair.primary.ccursor.index,
            None => ipa_tb_last_char
        };
        let resp = ui.add_sized(ui.available_size(), editor);
        
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
                // Get last typed character
                let typed_char =
                    win.ipa_tb.chars().nth(cursor_pos).unwrap();

                // Get access to the map of special symbols
                let char_map : HashMap<char, &'static [&'static str]> =
                    IPA_CHARS.iter().cloned().collect();

                // Get list of symbols corresponding to typed char
                let new_char_option = char_map.get(&typed_char);
                match new_char_option {
                    // No alt exists, do nothing
                    None => {},

                    // Alt exists, replace typed key
                    Some(ipa_str_arr) => {
                        // Delete typed character
                        win.ipa_tb = delete_char(&mut win.ipa_tb, cursor_pos);

                        /*
                         * If key is pressed multiple times, cycle & delete old
                         * Else, just put new character
                         */
                        if win.ipa_cur_char != typed_char {
                            win.ipa_cur_char = typed_char;
                            win.ipa_char_count = 0;
                        } else {                            
                            // Remove the old one
                            let cur_char = ipa_str_arr[win.ipa_char_count];
                            for _c in cur_char.chars() {
                                win.ipa_tb.pop();
                            }

                            // Progress to next new_char
                            win.ipa_char_count += 1;
                            if win.ipa_char_count >= ipa_str_arr.len() {
                                win.ipa_char_count = 0;
                            }
                        }

                        // Insert new character at end
                        let new_char = ipa_str_arr[win.ipa_char_count];
                        for c in new_char.chars() {
                            win.ipa_tb.push(c);
                        }
                    }
                }
            }
        }
    });
}

fn delete_char(base_data : &mut String, ind : usize) -> String {
    let pre_c = base_data.chars().take(ind);
    let post_c = base_data.chars().skip(ind + 1);
    let without_c = pre_c.chain(post_c);
    let mut new_str = String::new();
    for c in without_c {
        new_str.push(c);
    }
    return new_str.clone();
}
