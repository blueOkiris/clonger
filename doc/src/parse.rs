/*
 * Author: Dylan Turner
 * Description: Parse text to understand how to apply styling
 */

use gtk::{
    TextIter, TextBuffer, traits::TextBufferExt
};

pub struct TextSection {
    pub start: TextIter,
    pub end: TextIter,
    pub style: Option<String>
}

/*
 * Markdown-like styling
 * * <text> * == italics
 * _ <text> _ == bold
 * ~ <text> ~ == underline
 * # <text> # == header
 * ! <url > ! == image (cannot be combined with styles)
 */
pub fn parse_style_sections(buff: &TextBuffer) -> Vec<TextSection> {
    let mut sections = Vec::new();
    let mut start = buff.start_iter();
    let mut sect = parse_text_section(buff, &mut start, &mut sections);
    while !sect.is_none() {
        sections.push(sect.unwrap());
        sect = parse_text_section(buff, &mut start, &mut sections);
    }
    sections
}

/*
 * Could probably clean this up. Not much code reuse
 * A few things that make it hard:
 *  - Need to be able to parse some tags within other tags
 *  - Need to distinguish between normal and styled
 *  - Need to reset for multiple parsing
 */
fn parse_text_section(
        buff: &TextBuffer, start: &mut TextIter,
        sections: &mut Vec<TextSection>) -> Option<TextSection> {
    if start.offset() >= buff.end_iter().offset() {
        return None
    }

    match start.char().expect("Failed to get char from iter!") {
        '*' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            let mut end_others = Vec::new();
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '*' {
                // Could use match here, but it makes code reuse awful
                if start.char().expect("Failed to get char from iter!") == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                } else if start.char().expect("Failed to get char from iter!")
                            == '_'
                        || start.char().expect("Failed to get char from iter!")
                            == '~'
                        || start.char().expect("Failed to get char from iter!")
                            == '#'
                        || start.char().expect("Failed to get char from iter!")
                            == '$' {
                    // We add a new one, but also ignore to fully get currnt

                    let mut sub_start = buff.iter_at_offset(start.offset());
                    if !end_others.contains(&sub_start.offset()) {
                        let sub_sect = parse_text_section(
                            buff, &mut sub_start, sections
                        );
                        end_others.push(sub_start.offset() - 1);
                        if !sub_sect.is_none() {
                            sections.push(sub_sect.unwrap());
                        }
                    }
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("italic"))
            })
        }, '_' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            let mut end_others = Vec::new();
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '_' {
                // Could use match here, but it makes code reuse awful
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                } else if start.char().expect("Failed to get char from iter!")
                            == '*'
                        || start.char().expect("Failed to get char from iter!")
                            == '~'
                        || start.char().expect("Failed to get char from iter!")
                            == '#'
                        || start.char().expect("Failed to get char from iter!")
                            == '$' {
                    // We add a new one, but also ignore to fully get currnt

                    let mut sub_start = buff.iter_at_offset(start.offset());
                    if !end_others.contains(&sub_start.offset()) {
                        let sub_sect = parse_text_section(
                            buff, &mut sub_start, sections
                        );
                        end_others.push(sub_start.offset() - 1);
                        if !sub_sect.is_none() {
                            sections.push(sub_sect.unwrap());
                        }
                    }
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("bold"))
            })
        }, '~' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            let mut end_others = Vec::new();
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '~' {
                // Could use match here, but it makes code reuse awful
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                } else if start.char().expect("Failed to get char from iter!")
                            == '*'
                        || start.char().expect("Failed to get char from iter!")
                            == '_'
                        || start.char().expect("Failed to get char from iter!")
                            == '#'
                        || start.char().expect("Failed to get char from iter!")
                            == '$' {
                    // We add a new one, but also ignore to fully get currnt

                    let mut sub_start = buff.iter_at_offset(start.offset());
                    if !end_others.contains(&sub_start.offset()) {
                        let sub_sect = parse_text_section(
                            buff, &mut sub_start, sections
                        );
                        end_others.push(sub_start.offset() - 1);
                        if !sub_sect.is_none() {
                            sections.push(sub_sect.unwrap());
                        }
                    }
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("underline"))
            })
        }, '#' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            let mut end_others = Vec::new();
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '#' {
                // Could use match here, but it makes code reuse awful
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                } else if start.char().expect("Failed to get char from iter!")
                            == '*'
                        || start.char().expect("Failed to get char from iter!")
                            == '_'
                        || start.char().expect("Failed to get char from iter!")
                            == '~'
                        || start.char().expect("Failed to get char from iter!")
                            == '$' {
                    // We add a new one, but also ignore to fully get currnt

                    let mut sub_start = buff.iter_at_offset(start.offset());
                    if !end_others.contains(&sub_start.offset()) {
                        let sub_sect = parse_text_section(
                            buff, &mut sub_start, sections
                        );
                        end_others.push(sub_start.offset() - 1);
                        if !sub_sect.is_none() {
                            sections.push(sub_sect.unwrap());
                        }
                    }
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("header"))
            })
        }, '$' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            let mut end_others = Vec::new();
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '$' {
                // Could use match here, but it makes code reuse awful
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                } else if start.char().expect("Failed to get char from iter!")
                            == '*'
                        || start.char().expect("Failed to get char from iter!")
                            == '_'
                        || start.char().expect("Failed to get char from iter!")
                            == '~'
                        || start.char().expect("Failed to get char from iter!")
                            == '#' {
                    // We add a new one, but also ignore to fully get currnt

                    let mut sub_start = buff.iter_at_offset(start.offset());
                    if !end_others.contains(&sub_start.offset()) {
                        let sub_sect = parse_text_section(
                            buff, &mut sub_start, sections
                        );
                        end_others.push(sub_start.offset() - 1);
                        if !sub_sect.is_none() {
                            sections.push(sub_sect.unwrap());
                        }
                    }
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("subheader"))
            })
        }, '!' => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        == '!' {
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = buff.iter_at_offset(start.offset());
            let url = buff.text(
                &buff.iter_at_offset(sect_start.offset() + 1),
                &buff.iter_at_offset(sect_end.offset() - 1), true
            ).expect("Failed to get buffer text!").to_string();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("image:") + &url)
            })
        }, _ => {
            let sect_start = buff.iter_at_offset(start.offset());
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char().expect("Failed to get char from iter!")
                        != '*'
                    && start.char().expect("Failed to get char from iter!")
                        != '_'
                    && start.char().expect("Failed to get char from iter!")
                        != '~'
                    && start.char().expect("Failed to get char from iter!")
                        != '#'
                    && start.char().expect("Failed to get char from iter!")
                        != '$'
                    && start.char().expect("Failed to get char from iter!")
                        != '!' {
                if start.char().expect("Failed to get char from iter!")
                            == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            let sect_end = buff.iter_at_offset(start.offset());
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: None
            })
        }
    }
}
