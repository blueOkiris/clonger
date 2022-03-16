/*
 * Author: Dylan Turner
 * Description: Basic version of the ipa typing window plugin for clonger
 */

const NAME: &'static str = "w_IPA";

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

#[no_mangle]
pub extern "C" fn win_on_key_pressed(
        key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool,
        _clong_file: &mut String, _fname: &mut String) -> bool {
    // TODO: Implement key press event
    println!("Key press: {}", key);
    false
}

#[no_mangle]
pub extern "C" fn win_on_key_released(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool) {
    // TODO: Implement key release event
}
