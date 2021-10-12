/*
 * Author: Dylan Turner
 * Description: Collect modules and start app
 */

mod app;

use eframe::{ NativeOptions, run_native };

fn main() {
    let app = app::ClongerWindow::default();
    let native_ops = NativeOptions::default();
    run_native(Box::new(app), native_ops);
}
