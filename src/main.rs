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
use eframe::egui::Vec2;

fn main() {
    let app = app::ClongerWindow::default();
    let mut native_ops = NativeOptions::default();
    native_ops.initial_window_size = Some(Vec2::new(1280.0, 720.0));
    run_native(Box::new(app), native_ops);
}
