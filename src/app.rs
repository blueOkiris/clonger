/*
 * Author: Dylan Turner
 * Description: Create and launch main window using GTK and set up plugins
 */

use gtk4::{
    Application, ApplicationWindow,
    prelude::{ ApplicationExt, WidgetExt, ApplicationExtManual }
};
use std::env::set_var;

const APP_ID: &'static str = "com.blueokiris.Clonger";
const WIN_TITLE: &'static str = "Clonger";
const WIN_DEF_WIDTH: i32 = 800;
const WIN_DEF_HEIGHT: i32 = 600;
const GSK_RENDERER: &'static str = "cairo";

pub fn init_app() {
    set_var("GSK_RENDERER", GSK_RENDERER);
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(WIN_DEF_WIDTH).default_height(WIN_DEF_HEIGHT)
            .title(WIN_TITLE)
            .build();
        
        // TODO: Set up gui

        win.show();
    });
    app.run();
}
