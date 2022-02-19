/*
 * Author: Dylan Turner
 * Description: Create and launch main window using GTK and set up plugins
 */

use gtk4::{
    Application, ApplicationWindow, EventControllerKey, Inhibit,
    Notebook, Box, Label,
    Align, Orientation,
    prelude::{
        ApplicationExt, WidgetExt, ApplicationExtManual, BoxExt
    }, gdk::ModifierType,
    traits::GtkWindowExt,
};
use std::{
    thread::spawn,
    sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, channel } },
    env::set_var
};
use crate::plugin::{ Plugin, load_plugins };
use crate::event::{ AsyncEvent, AsyncEventType };

const APP_ID: &'static str = "com.blueokiris.Clonger";
const GSK_RENDERER: &'static str = "cairo";
const WIN_MIN_WIDTH: i32 = 600;
const WIN_MIN_HEIGHT: i32 = 800;
const WIN_TITLE: &'static str = "Clonger";
const WIN_DEF_MARGIN: i32 = 10;

// Data structure for sending info to the gui for updating ui
pub struct GuiUpdateData {
    pub fname: String,
    pub changed: bool
}

pub struct App {
    plugins: Vec<Plugin>,
    plugin_names: Vec<String>,

    // Send event data from gui to logic:
    tx: Sender<AsyncEvent>,
    rx: Receiver<AsyncEvent>,

    file: String, fname: String, changed: bool
}

impl App {
    pub fn new() -> Self {
        set_var("GSK_RENDERER", GSK_RENDERER);
        let (plugins, plugin_names) = load_plugins();
        let (tx, rx) = channel();
        Self {
            plugins, plugin_names,
            tx, rx,
            file: String::new(), fname: String::from("New File *"),
            changed: true
        }
    }

    pub fn start(self) {
        // Set up gui and connect plugins to its events
        let gtk_app = Application::builder().application_id(APP_ID).build();
        let setup_tx = self.tx.clone();
        let setup_fname = self.fname.clone();
        let setup_plugin_names = self.plugin_names.clone();
        let (fname_tx, fname_rx) = channel(); // Reverse of App tx, rx
        let clonable_fname_rx = Arc::new(Mutex::new(fname_rx)).clone();
        gtk_app.connect_activate(move |app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .title(WIN_TITLE)
                .default_width(WIN_MIN_WIDTH).default_height(WIN_MIN_HEIGHT)
                .build();
            win.set_size_request(WIN_MIN_WIDTH, WIN_MIN_HEIGHT);
            
            Self::setup_gui(
                app, &win, &setup_tx, &clonable_fname_rx,
                setup_plugin_names.clone(), setup_fname.clone()
            );
            
            win.show();
        });

        /*
         * Set up event handling (i.e. plugin calling) receive thread
         * Ideally, we could just call plugin events from the setup_gui
         * closures, but unfortunately, we can't borrow into multiple closures
         * so instead, this async thing is used
         */
        let fname_tx_event = fname_tx.clone();
        spawn(move || {
            while let Ok(event) = self.rx.recv() {
                self.handle_async_events(event, &fname_tx_event);
            }
        });

        // Start the application
        gtk_app.run();
        gtk_app.quit(); // For cleanup - just in case
    }

    fn setup_gui(
            _app: &Application, win: &ApplicationWindow,
            tx: &Sender<AsyncEvent>,
            fname_rx: &Arc<Mutex<Receiver<Option<GuiUpdateData>>>>,
            plugin_names: Vec<String>, fname: String) {
        // Main vbox for file name and tabs and such
        let content_box = Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true).vexpand(true)
            .halign(Align::Fill).valign(Align::Fill)
            .spacing(WIN_DEF_MARGIN)
            .build();
        win.set_child(Some(&content_box));

        // Create a view for file name (as well as that there are changes)
        let fname_label = Label::builder()
            .label(fname.as_str())
            .halign(Align::Center).valign(Align::Start)
            .hexpand(true).vexpand(false)
            .margin_top(WIN_DEF_MARGIN)
            .margin_bottom(0)
            .build();
        content_box.append(&fname_label);
        // TODO: Track changes & update f name based on if plugin changes (bool)
        
        Self::create_notebook(&content_box, tx, plugin_names);
        // TODO: Create sub windows from plugins and connect their events

        Self::attach_key_event_senders(win, &fname_label, tx, fname_rx);
        // TODO: Add keyboard shortcuts for new, opening, and saving files
    }

    fn handle_async_events(
            &self, event: AsyncEvent, tx: &Sender<Option<GuiUpdateData>>) {
        match event.event_type {
            AsyncEventType::KeyPressed => {
                // TODO: Allow modifying the "file" via plugins
                let cur_fname = self.fname.clone();
                let cur_changed = self.changed;

                for plugin in &self.plugins {
                    plugin.on_key_pressed(
                        &event.key,
                        event.ctrl_pressed, event.alt_pressed,
                        event.shift_pressed, event.super_pressed
                    );
                }

                // TODO: Handle saving here

                // Use reverse channel to 
                if self.fname != cur_fname || self.changed != cur_changed {
                    tx.send(Some(GuiUpdateData {
                        fname: self.fname.clone(),
                        changed: self.changed
                    })).unwrap();
                } else {
                    tx.send(None).unwrap();
                }
            }, AsyncEventType::KeyReleased => {
                for plugin in &self.plugins {
                    plugin.on_key_released(
                        &event.key,
                        event.ctrl_pressed, event.alt_pressed,
                        event.shift_pressed, event.super_pressed
                    );
                }

                tx.send(None).unwrap();
            }
        }
    }
    
    // For non-window plugins, create a tabbed document from them
    fn create_notebook(
            content_box: &Box, _tx: &Sender<AsyncEvent>,
            plugin_names: Vec<String>) {
        let nb = Notebook::builder()
            .margin_top(0).margin_bottom(WIN_DEF_MARGIN)
            .margin_start(WIN_DEF_MARGIN).margin_end(WIN_DEF_MARGIN)
            .halign(Align::Fill).valign(Align::Fill)
            .hexpand(true).vexpand(true)
            .scrollable(true)
            .build();

        for name in plugin_names {
            // Check if it's a window plugin
            if name.starts_with("w_") {
                continue;
            }

            let page = Box::builder()
                .hexpand(true).vexpand(true).orientation(Orientation::Vertical)
                .build();
            let label = Label::new(Some(&name));
            nb.append_page(&page, Some(&label));
        }

        content_box.append(&nb);
    }

    /*
    * Connect plugin functions to window key events (via tx)
    * These are needed as to save, open, and new you use keyboard shortcuts!
    */
    fn attach_key_event_senders(
            win: &ApplicationWindow, fname_label: &Label,
            tx: &Sender<AsyncEvent>,
            fname_rx: &Arc<Mutex<Receiver<Option<GuiUpdateData>>>>) {
        let ev_cont = EventControllerKey::new();

        let key_pressed_tx = tx.clone();
        let key_pressed_rx = fname_rx.clone();
        let key_pressed_fname_label = fname_label.clone();
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

            match key_pressed_rx.lock().unwrap().recv().unwrap() {
                Some(data) => {
                    key_pressed_fname_label.set_label((
                        data.fname + if data.changed { " *" } else { "" }
                    ).as_str());
                },
                None => {}
            }

            Inhibit(false)
        });

        let key_released_tx = tx.clone();
        let key_released_rx = fname_rx.clone();
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

            match key_released_rx.lock().unwrap().recv().unwrap() {
                Some(_) => {},
                None => {}
            }
        });

        win.add_controller(&ev_cont);
    }
}
