/*
 * Author: Dylan Turner
 * Description: Collect modules and start clonger app
 */

mod app;
mod plugin;

use app::init_app;

fn main() {
    init_app();
}
