/*
 * Author: Dylan Turner
 * Description: Create and launch main window using GTK and set up plugins
 */

use gtk4::{
    Application, ApplicationWindow,
    prelude::{
        ApplicationExt, WidgetExt, ApplicationExtManual,
    }
};
use std::env::set_var;

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

        setup_gui(&win);
    
        win.show();
    });
    app.run();
}

fn setup_gui(win: &ApplicationWindow) {
    // TODO: Add global data for file name and display it in a label
    // TODO: Add content area where plugin pages will load
    // TODO: Track changes & update file name based on if plugin changes (bool)
    // TODO: Add keyboard shortcuts for creating new, opening, and saving files
    // TODO: Create app pages from plugins and connect their events
    // TODO: Create sub windows from plugins and connect their events
}
