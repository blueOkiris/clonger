/*
 * Author: Dylan Turner
 * Description: Collect modules and start app
 */

mod app;
mod ipaview;
mod docview;
mod dictview;
mod exview;

use eframe::{ NativeOptions, run_native };

fn main() {
    let app = app::ClongerWindow::default();
    let native_ops = NativeOptions::default();
    run_native(Box::new(app), native_ops);
}
