/*
 * Author: Dylan Turner
 * Description: Collect modules and start clonger app
 */

mod app;
mod plugin;
mod event;

use app::App;

fn main() {
    let app: App = App::new();
    app.start();
}
