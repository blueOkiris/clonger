/*
 * Author: Dylan Turner
 * Description: Define an interface for creating plugins
 */

use std::env::args;
use libloading::{ Library, Symbol };

type AddFunc = unsafe fn(isize, isize) -> isize;

pub fn load_plugins() {
    unsafe {
        let lib = Library::new("libdocumentation.so").unwrap();
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
        let answer = func(1, 2);
        println!("1 + 2 = {}", answer);
    }
}
