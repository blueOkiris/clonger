/*
 * Author: Dylan Turner
 * Description: Parse text to understand how to apply styling
 */

use gtk4::{ TextIter, TextBuffer, traits::TextBufferExt };

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
 */
pub fn parse_style_sections(buff: &TextBuffer) -> Vec<TextSection> {
    let mut sections = Vec::new();
    let mut start = buff.start_iter();
    let mut sect = parse_text_section(buff, &mut start);
    while !sect.is_none() {
        sections.push(sect.unwrap());
        sect = parse_text_section(buff, &mut start);
    }
    sections
}

fn parse_text_section(
        buff: &TextBuffer, start: &mut TextIter) -> Option<TextSection> {
    if start.offset() >= buff.end_iter().offset() {
        return None
    }

    match start.char() {
        '*' => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '*' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("italic"))
            })
        }, '_' => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '_' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("bold"))
            })
        }, '~' => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '~' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("underline"))
            })
        }, '#' => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '#' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("header"))
            })
        }, '$' => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '$' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            *start = buff.iter_at_offset(start.offset() + 1);
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: Some(String::from("subheader"))
            })
        }, _ => {
            let sect_start = start.clone();
            *start = buff.iter_at_offset(start.offset() + 1);
            while start.offset() != buff.end_iter().offset()
                    && start.char() != '*'
                    && start.char() != '_'
                    && start.char() != '~'
                    && start.char() != '#'
                    && start.char() != '$' {
                if start.char() == '\\'
                        && start.offset() + 1 != buff.end_iter().offset() {
                    *start = buff.iter_at_offset(start.offset() + 1);
                }
                *start = buff.iter_at_offset(start.offset() + 1);
            }
            let sect_end = start.clone();
            Some(TextSection {
                start: sect_start,
                end: sect_end,
                style: None
            })
        }
    }
}
