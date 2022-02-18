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
use std::{
    thread::spawn,
    sync::mpsc::{ Sender, Receiver, channel },
    env::set_var
};
use crate::plugin::{ Plugin, load_plugins };
use crate::event::{ AsyncEvent, AsyncEventType };

const APP_ID: &'static str = "com.blueokiris.Clonger";
const GSK_RENDERER: &'static str = "cairo";
const WIN_MIN_WIDTH: i32 = 600;
const WIN_MIN_HEIGHT: i32 = 800;
const WIN_TITLE: &'static str = "Clonger";

pub struct App {
    pub plugins: Vec<Plugin>,
    pub tx: Sender<AsyncEvent>,
    pub rx: Receiver<AsyncEvent>
}

impl App {
    pub fn new() -> Self {
        set_var("GSK_RENDERER", GSK_RENDERER);
        let plugins = load_plugins();
        let (tx, rx) = channel();
        Self { plugins, tx, rx }
    }

    pub fn start(self) {
        // Set up gui and connect plugins to its events
        let gtk_app = Application::builder().application_id(APP_ID).build();
        let setup_tx = self.tx.clone();
        gtk_app.connect_activate(move |app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .title(WIN_TITLE)
                .default_width(WIN_MIN_WIDTH).default_height(WIN_MIN_HEIGHT)
                .build();
            win.set_size_request(WIN_MIN_WIDTH, WIN_MIN_HEIGHT);
            
            Self::setup_gui(app, &win, &setup_tx);

            win.show();
        });

        /*
         * Set up event handling (i.e. plugin calling) receive thread
         * Ideally, we could just call plugin events from the setup_gui
         * closures, but unfortunately, we can't borrow into multiple closures
         * so instead, this async thing is used
         */
        spawn(move || {
            while let Ok(event) = self.rx.recv() {
                self.handle_async_events(event);
            }
        });

        // Start the application
        gtk_app.run();
        gtk_app.quit(); // For cleanup - just in case
    }

    fn setup_gui(
            app: &Application, win: &ApplicationWindow,
            tx: &Sender<AsyncEvent>) {
        // TODO: Add global data for file name and display it in a label
        // TODO: Add content area where plugin pages will load
        // TODO: Track changes & update f name based on if plugin changes (bool)
        // TODO: Add keyboard shortcuts for new, opening, and saving files

        Self::attach_key_event_senders(app, win, tx);

        // TODO: Create app pages from plugins and connect their events
        // TODO: Create sub windows from plugins and connect their events
    }

    fn handle_async_events(&self, event: AsyncEvent) {
        match event.event_type {
            AsyncEventType::KeyPressed => {
                // TODO: Allow modifying the "file" via plugins
                
                for plugin in &self.plugins {
                    plugin.on_key_pressed(
                        &event.key,
                        event.ctrl_pressed, event.alt_pressed,
                        event.shift_pressed, event.super_pressed
                    );
                }

                // TODO: Handle saving here
            }, AsyncEventType::KeyReleased => {
                for plugin in &self.plugins {
                    plugin.on_key_released(
                        &event.key,
                        event.ctrl_pressed, event.alt_pressed,
                        event.shift_pressed, event.super_pressed
                    );
                }
            }
        }
    }

    /*
    * Connect plugin functions to window key events (via tx)
    * These are needed as to save, open, and new you use keyboard shortcuts!
    */
    fn attach_key_event_senders(
            _app: &Application, win: &ApplicationWindow,
            tx: &Sender<AsyncEvent>) {
        let ev_cont = EventControllerKey::new();

        let key_pressed_tx = tx.clone();
        ev_cont.connect_key_pressed(move |_ev_cont, key, _key_code, state| {
            let key_name = String::from(key.name().unwrap().as_str());
            let ctrl_pressed =
                (state.bits() & ModifierType::CONTROL_MASK.bits()) > 0;
            let alt_pressed =
                (state.bits() & ModifierType::ALT_MASK.bits()) > 0;
            let shift_pressed =
                (state.bits() & ModifierType::SHIFT_MASK.bits()) > 0;
            let super_pressed =
                (state.bits() & ModifierType::SUPER_MASK.bits()) > 0;

            key_pressed_tx.send(AsyncEvent::key_pressed(
                key_name,
                ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            )).unwrap();

            Inhibit(false)
        });

        let key_released_tx = tx.clone();
        ev_cont.connect_key_released(move |_ev_cont, key, _key_code, state| {
            let key_name = String::from(key.name().unwrap().as_str());
            let ctrl_pressed =
                (state.bits() & ModifierType::CONTROL_MASK.bits()) > 0;
            let alt_pressed =
                (state.bits() & ModifierType::ALT_MASK.bits()) > 0;
            let shift_pressed =
                (state.bits() & ModifierType::SHIFT_MASK.bits()) > 0;
            let super_pressed =
                (state.bits() & ModifierType::SUPER_MASK.bits()) > 0;

            key_released_tx.send(AsyncEvent::key_released(
                key_name,
                ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            )).unwrap();
        });

        win.add_controller(&ev_cont);
    }
}
