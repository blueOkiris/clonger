/*
 * Author: Dylan Turner
 * Description: Basic version of the documentation plugin for clonger
 */

mod parse;

use gtk4::{
    Box, Orientation, ScrolledWindow, TextView, TextBuffer, Label, TextTag,
    prelude::{ BoxExt, TextViewExt, TextBufferExt },
    pango::{ Style, Underline }
};
use std::sync::{ Arc, Mutex };
use crate::parse::parse_style_sections;

type TabBuildFunc = fn(&Label) -> Box;

const NAME: &'static str = "Documentation";
const DEF_MARGIN: i32 = 10;
const WEIGHT: i32 = 700;
const HEADER_SCALE: f64 = 2.1;
const SUBHEADER_SCALE: f64 = 1.3;

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

/*
 * Perhaps not the best way to do this.
 * 
 * We need to be able to init the text view with a buffer AND update it via
 * file stuff. I couldn't think of a good way to do this without static vars
 * 
 * May improve later
 */
static mut BUFFER: Option<Arc<Mutex<TextBuffer>>> = None;
static mut FNAME_LABEL: Option<Arc<Mutex<Label>>> = None;

// This returns a function to be called later, due to Gtk's creation restriction
#[no_mangle]
#[allow(improper_ctypes_definitions)] // to use GTK types across plugin to core
pub extern "C" fn build_tab() -> TabBuildFunc {
    |label| {
        let content = Box::builder()
            .hexpand(true).vexpand(true)
            .orientation(Orientation::Vertical)
            .build();
        let scrollview = ScrolledWindow::builder()
            .hexpand(true).vexpand(true)
            .margin_top(DEF_MARGIN).margin_bottom(DEF_MARGIN)
            .margin_start(DEF_MARGIN).margin_end(DEF_MARGIN)
            .build();
        let text = TextView::builder()
            .accepts_tab(true)
            .build();

        let buff = text.buffer();
        let insert_label = label.clone();
        buff.connect_changed(move |buff| {
            // Implement markdown-like syntax and adjustments
            add_styling(buff);

            // Update the fname bar to show changes
            let mut cur_val = insert_label.text().to_string();
            if cur_val.ends_with("*") {
                return
            }
            cur_val.push_str(" *");
            insert_label.set_text(cur_val.as_str());
        });
        let tag_table = buff.tag_table();
        tag_table.add(&TextTag::builder()
            .name("italic").style(Style::Italic).build());
        tag_table.add(&TextTag::builder().name("bold").weight(WEIGHT).build());
        tag_table.add(&TextTag::builder()
            .name("underline").underline(Underline::Single).build());
        tag_table.add(&TextTag::builder()
            .name("header").scale(HEADER_SCALE).build());
        tag_table.add(&TextTag::builder()
            .name("subheader").scale(SUBHEADER_SCALE).build());

        unsafe {
            BUFFER = Some(Arc::new(Mutex::new(buff)));
            FNAME_LABEL = Some(Arc::new(Mutex::new(label.clone())));
        }

        scrollview.set_child(Some(&text));
        content.append(&scrollview);
        content
    }
}

#[no_mangle]
pub extern "C" fn win_on_key_pressed(
        key: &String,
        ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool,
        clong_file: &mut String, fname: &mut String) -> bool {
    // TODO: Implement key press event

    if ctrl_pressed && key == "s" {
        save_file(clong_file, fname);
    }

    false
}

#[no_mangle]
pub extern "C" fn win_on_key_released(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool) {
    // TODO: Implement key release event
}

fn add_styling(buff: &TextBuffer) {
    buff.remove_all_tags(&buff.start_iter(), &buff.end_iter());
    let sections = parse_style_sections(buff);
    for sect in sections {
        if sect.style != None {
            buff.apply_tag_by_name(
                sect.style.unwrap().as_str(), &sect.start, &sect.end
            );
        }
    }
}

fn save_file(clong_file: &mut String, fname: &mut String) -> bool {
    // TODO manage file name settings

    // Make sure fname matches text view (bc it might not have updated)
    unsafe {
        let label_mut = FNAME_LABEL.to_owned().unwrap();
        let label = label_mut.lock().unwrap();
        *fname = label.text().to_string();
    }

    // Get the current file data for the "DOC" section
    let fsplit = clong_file.split("\r");
    let mut doc_sect = String::new();
    for sect in fsplit {
        if sect.starts_with("[DOC]") {
            doc_sect = String::from(sect);
            break;
        }
    }

    // Update doc
    let old_doc_sect = doc_sect.clone();
    let tb_mut = unsafe { BUFFER.to_owned().unwrap() };
    let tb = tb_mut.lock().unwrap().to_owned();
    let tb_start = tb.start_iter();
    let tb_end = tb.end_iter();
    let text = tb_start.text(&tb_end);
    doc_sect = String::from("[DOC]") + &text.to_string();

    if doc_sect == old_doc_sect {
        return false
    }

    // Check if it doesn't exist yet but file does
    if old_doc_sect.len() < 1 && clong_file.len() > 0 {
        clong_file.push('\r');
        clong_file.push_str(&doc_sect.as_str());
    } else {
        let new_file = clong_file.replace(
            &old_doc_sect.as_str(), &doc_sect.as_str()
        );
        *clong_file = new_file;
    }

    // Keep track of changes
    if fname.ends_with("*") {
        fname.remove(fname.len() - 1);
        fname.remove(fname.len() - 1);
    }

    // Update the actual frame buffer
    unsafe {
        let label_mut = FNAME_LABEL.to_owned().unwrap();
        let label = label_mut.lock().unwrap();
        label.set_text(fname.as_str());
    }

    true
}
