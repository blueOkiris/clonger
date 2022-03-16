/*
 * Author: Dylan Turner
 * Description: Create and launch main window using GTK and set up plugins
 */

use gtk::{
    Application, ApplicationWindow, EventControllerKey,
    Notebook, Box, Label,
    Align, Orientation,
    prelude::{
        ApplicationExt, WidgetExt, ApplicationExtManual, BoxExt, ContainerExt,
        LabelExt, NotebookExtManual, NotebookExt
    }, gdk::ModifierType
};
use std::{
    thread::spawn,
    sync::{
        Arc, Mutex, mpsc::{
            Sender, Receiver, channel
        }
    }, env::set_var,
    collections::HashMap
};
use crate::plugin::{
    Plugin, load_plugins, TabBuildFunc
};
use crate::event::{
    AsyncEvent, AsyncEventType
};

const APP_ID: &'static str = "com.blueokiris.Clonger";
const GSK_RENDERER: &'static str = "cairo";
const WIN_MIN_WIDTH: i32 = 600;
const WIN_MIN_HEIGHT: i32 = 800;
const WIN_TITLE: &'static str = "Clonger";
const WIN_DEF_MARGIN: i32 = 10;

pub struct App {
    plugins: Vec<Plugin>,
    plugin_names: Vec<String>,

    // Send event data from gui to logic:
    tx: Sender<AsyncEvent>,
    rx: Receiver<AsyncEvent>,

    /*
     * Basic file format is sections of data split by \r (\n is newline)
     * So like:
     * [Documentation].......
     * ......................
     * ......................\r[Dictionary].......
     * etc
     * So we can split by \r, find the section, and extract or update the info
     */
    clong_file: String,
    fname: String
}

impl App {
    pub fn new() -> Self {
        set_var("GSK_RENDERER", GSK_RENDERER);
        let (plugins, plugin_names) = load_plugins();
        let (tx, rx) = channel();
        Self {
            plugins, plugin_names,
            tx, rx,
            clong_file: String::new(), fname: String::from("New File *")
        }
    }

    pub fn start(mut self) {
        // Create a channel for sending fname updates to title of gui
        let (fname_tx, fname_rx) = channel(); // Reverse of App tx, rx

        // Create a channel for sending tab page content from plugins on init
        let (tab_gui_tx, tab_gui_rx) = channel(); // Reverse of App tx, rx

        // Load up tab_gui_rx with plugin info that we can't do later
        let mut box_map = HashMap::new();
        for plugin in &self.plugins {
            if plugin.name().starts_with("w_") {
                continue;
            }
            let tab_content = plugin.build_tab();
            box_map.insert(plugin.name(), tab_content);
        }
        tab_gui_tx.send(box_map).unwrap();

        // Set up gui and connect plugins to its events
        let gtk_app = Application::builder().application_id(APP_ID).build();
        let setup_tx = self.tx.clone();
        let setup_fname = self.fname.clone();
        let setup_plugin_names = self.plugin_names.clone();
        let clonable_fname_rx = Arc::new(Mutex::new(fname_rx)).clone();
        let clonable_tab_gui_rx = Arc::new(Mutex::new(tab_gui_rx));
        gtk_app.connect_activate(move |app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .title(WIN_TITLE)
                .default_width(WIN_MIN_WIDTH).default_height(WIN_MIN_HEIGHT)
                .can_focus(true)
                .build();
            win.set_size_request(WIN_MIN_WIDTH, WIN_MIN_HEIGHT);
            
            Self::setup_gui(
                app, &win, &setup_tx, &clonable_fname_rx, &clonable_tab_gui_rx,
                setup_plugin_names.clone(), setup_fname.clone()
            );
            
            win.show_all();
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
                //self.handle_async_events(event, &fname_tx_event);
            }
        });

        // Start the application
        gtk_app.run();
        gtk_app.quit(); // For cleanup - just in case
    }

    fn setup_gui(
            _app: &Application, win: &ApplicationWindow,
            tx: &Sender<AsyncEvent>,
            fname_rx: &Arc<Mutex<Receiver<Option<String>>>>,
            tab_gui_rx: &Arc<Mutex<Receiver<HashMap<String, TabBuildFunc>>>>,
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
        content_box.pack_start(&fname_label, false, false, 0);
        // TODO: Track changes & update f name based on if plugin changes (bool)
        
        let nb = Self::create_notebook(
            &content_box, tx, tab_gui_rx, plugin_names, &fname_label
        );

        Self::attach_key_event_senders(win, &fname_label, tx, fname_rx, &nb);
        // TODO: Add keyboard shortcuts for new, opening, and saving files
    }

    fn handle_async_events(
            &mut self, event: AsyncEvent, tx: &Sender<Option<String>>) {
        match event.event_type {
            AsyncEventType::WinKeyPressed => {
                // TODO: Allow modifying the "file" via plugins
                let cur_fname = self.fname.clone();

                for plugin in &self.plugins {
                    if plugin.name() != event.active_tab
                            && !plugin.name().starts_with("w_") {
                        continue;
                    }
                    if plugin.win_on_key_pressed(
                        &event.key,
                        event.ctrl_pressed, event.alt_pressed,
                        event.shift_pressed, event.super_pressed,
                        &mut self.clong_file, &mut self.fname
                    ) {
                        self.fname.push_str(" *");
                    }
                }

                // TODO: Handle saving here

                // Use reverse channel to 
                if self.fname != cur_fname || self.fname.ends_with("*") {
                    tx.send(Some(self.fname.clone())).unwrap();
                } else {
                    tx.send(None).unwrap();
                }
            }, AsyncEventType::WinKeyReleased => {
                for plugin in &self.plugins {
                    if plugin.name() != event.active_tab
                            && !plugin.name().starts_with("w_") {
                        continue;
                    }
                    plugin.win_on_key_released(
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
            tab_gui_rx: &Arc<Mutex<Receiver<HashMap<String, TabBuildFunc>>>>,
            plugin_names: Vec<String>,
            fname_label: &Label) -> Notebook {
        let nb = Notebook::builder()
            .margin_top(0).margin_bottom(WIN_DEF_MARGIN)
            .margin_start(WIN_DEF_MARGIN).margin_end(WIN_DEF_MARGIN)
            .halign(Align::Fill).valign(Align::Fill)
            .hexpand(true).vexpand(true)
            .scrollable(true)
            .build();

        // Get the created widgets from the plugins here
        let rx = tab_gui_rx.lock().unwrap();
        let tab_children = rx.recv().unwrap();

        for name in plugin_names {
            // Check if it's a window plugin
            if name.starts_with("w_") {
                continue;
            }

            let page_func = tab_children.get(&name).unwrap();
            let page = unsafe {
                page_func(fname_label)
            };
            let label = Label::new(Some(&name));
            nb.append_page(&page, Some(&label));
        }

        // TODO: Create sub windows from plugins and connect their events

        content_box.pack_start(&nb, true, true, 0);
        nb
    }

    /*
    * Connect plugin functions to window key events (via tx)
    * These are needed as to save, open, and new you use keyboard shortcuts!
    */
    fn attach_key_event_senders(
            win: &ApplicationWindow, fname_label: &Label,
            tx: &Sender<AsyncEvent>,
            fname_rx: &Arc<Mutex<Receiver<Option<String>>>>,
            nb: &Notebook) {
        let ev_cont = EventControllerKey::new(win);

        let key_pressed_tx = tx.clone();
        let key_pressed_rx = fname_rx.clone();
        let key_pressed_fname_label = fname_label.clone();
        let key_pressed_nb = nb.clone();
        ev_cont.connect_key_pressed(move |_ev_cont, key, _key_code, state| {
            let key_name = String::from(format!("{}", key));
            let ctrl_pressed =
                (state.bits() & ModifierType::CONTROL_MASK.bits()) > 0;
            let alt_pressed =
                (state.bits() & ModifierType::MOD1_MASK.bits()) > 0;
            let shift_pressed =
                (state.bits() & ModifierType::SHIFT_MASK.bits()) > 0;
            let super_pressed =
                (state.bits() & ModifierType::SUPER_MASK.bits()) > 0;

            let page_opt = key_pressed_nb.nth_page(
                key_pressed_nb.current_page()
            );
            let page_title = match page_opt {
                None => String::new(),
                Some(page_wgt) => key_pressed_nb
                    .tab_label_text(&page_wgt).unwrap().to_string()
            };

            key_pressed_tx.send(AsyncEvent::key_pressed(
                key_name,
                ctrl_pressed, alt_pressed, shift_pressed, super_pressed,
                page_title
            )).unwrap();

            match key_pressed_rx.lock().unwrap().recv().unwrap() {
                Some(fname) => {
                    key_pressed_fname_label.set_text(&fname);
                },
                None => {}
            }

            false
        });

        let key_released_tx = tx.clone();
        let key_released_rx = fname_rx.clone();
        let key_released_nb = nb.clone();
        ev_cont.connect_key_released(move |_ev_cont, key, _key_code, state| {
            let key_name = String::from(format!("{}", key));
            let ctrl_pressed =
                (state.bits() & ModifierType::CONTROL_MASK.bits()) > 0;
            let alt_pressed =
                (state.bits() & ModifierType::MOD1_MASK.bits()) > 0;
            let shift_pressed =
                (state.bits() & ModifierType::SHIFT_MASK.bits()) > 0;
            let super_pressed =
                (state.bits() & ModifierType::SUPER_MASK.bits()) > 0;

            let page_opt = key_released_nb.nth_page(
                key_released_nb.current_page()
            );
            let page_title = match page_opt {
                None => String::new(),
                Some(page_wgt) => key_released_nb
                    .tab_label_text(&page_wgt).unwrap().to_string()
            };

            key_released_tx.send(AsyncEvent::key_released(
                key_name,
                ctrl_pressed, alt_pressed, shift_pressed, super_pressed,
                page_title
            )).unwrap();

            match key_released_rx.lock().unwrap().recv().unwrap() {
                Some(_) => {},
                None => {}
            }
        });
    }
}
