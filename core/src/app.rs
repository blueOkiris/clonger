/*
 * Author: Dylan Turner
 * Description: Create and launch main window using GTK and set up plugins
 */

use gtk4::{
    Application, ApplicationWindow, EventControllerKey, Inhibit,
    prelude::{
        ApplicationExt, WidgetExt, ApplicationExtManual
    }, gdk::ModifierType
};
use std::env::set_var;
use crate::plugin::load_plugins;

const APP_ID: &'static str = "com.blueokiris.Clonger";
const GSK_RENDERER: &'static str = "cairo";
const WIN_MIN_WIDTH: i32 = 600;
const WIN_MIN_HEIGHT: i32 = 800;
const WIN_TITLE: &'static str = "Clonger";

pub fn init_app() {
    set_var("GSK_RENDERER", GSK_RENDERER);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .title(WIN_TITLE)
            .default_width(WIN_MIN_WIDTH).default_height(WIN_MIN_HEIGHT)
            .build();
        win.set_size_request(WIN_MIN_WIDTH, WIN_MIN_HEIGHT);

        setup_gui(&app, &win);
    
        win.show();
    });
    app.run();
}

fn setup_gui(_app: &Application, win: &ApplicationWindow) {
    // TODO: Add global data for file name and display it in a label
    // TODO: Add content area where plugin pages will load
    // TODO: Track changes & update file name based on if plugin changes (bool)
    // TODO: Add keyboard shortcuts for creating new, opening, and saving files

    let plugins = load_plugins(); // TODO: Connect ALL plugin funcs

    let ev_cont = EventControllerKey::new();
    ev_cont.connect_key_pressed(move |_ev_cont, key, _key_code, state| {
        let ctrl_pressed =
            (state.bits() & ModifierType::CONTROL_MASK.bits()) > 0;
        let alt_pressed =
            (state.bits() & ModifierType::ALT_MASK.bits()) > 0;
        let shift_pressed =
            (state.bits() & ModifierType::SHIFT_MASK.bits()) > 0;
        let super_pressed =
            (state.bits() & ModifierType::SUPER_MASK.bits()) > 0;

        for plugin in &plugins {
            plugin.on_key_pressed(
                &String::from(key.name().unwrap().as_str()),
                ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            );
        }

        Inhibit(true)
    });
    win.add_controller(&ev_cont);

    // TODO: Create app pages from plugins and connect their events
    // TODO: Create sub windows from plugins and connect their events
}
