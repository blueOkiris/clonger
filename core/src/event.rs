/*
 * Author: Dylan Turner
 * Description:
 *  - A lot of gtk stuff happens in closures
 *  - This allows us to send data out of the closures and handle it
 *    Bc Rust ownership won't let us pass data into a closure
 *    and unborrow it again
 */

#[derive(Debug)]
pub enum AsyncEventType {
    KeyPressed,
    KeyReleased
}

pub struct AsyncEvent {
    pub event_type: AsyncEventType,

    // Key pressed and key released data
    pub key: String,
    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
    pub shift_pressed: bool,
    pub super_pressed: bool,
    pub active_tab: String
}

impl AsyncEvent {
    pub fn key_pressed(
            key: String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool,
            active_tab: String) -> Self {
        Self {
            event_type: AsyncEventType::KeyPressed,
            key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed,
            active_tab
        }
    }

    pub fn key_released(
            key: String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool,
            active_tab: String) -> Self {
        Self {
            event_type: AsyncEventType::KeyReleased,
            key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed,
            active_tab
        }
    }
}
