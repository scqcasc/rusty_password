// Thanks to https://github.com/pjmlp/gwc-rs/ for showing the way forward with the GUI

extern crate gtk;

use clap::Parser;
use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{
    AboutDialog, IconSize, Image, Label, Menu, MenuBar, MenuItem, SeparatorToolItem, ToolButton,
    Toolbar, ToolbarStyle, Window, WindowType, Entry
};
use password::PasswordType;
use std::borrow::Borrow;
use std::rc::Rc;

mod password;

///Set up cli arg
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// length of password to create
    #[arg(short, long, default_value_t = 15)]
    length: i32,

    /// simplify possible characters
    #[arg(short, long, default_value_t = false)]
    simple: bool,

    /// Enable GUI
    #[arg(short, long, default_value_t = false)]
    gui: bool,
}
#[derive(Debug, Clone)]
struct GtkPasswordTypes {
    radio_buton: gtk::RadioButton,
    password_type: PasswordType,
}

#[derive(Debug)]
// // probably don't need this
struct GtkPasswdArray {
    types: Vec<GtkPasswordTypes>,
}

/// The application window state
#[derive(Debug, Clone)]
struct GWCApp {
    /// the window label used to display the counters
    passwd_label: Option<Rc<Label>>,

    /// a kind of handle for the Gtk+ window
    window: Option<Rc<Window>>,

    /// option for password type
    password_type: Option<Rc<GtkPasswdArray>>,

    /// url_tb
    url_tb: Option<Rc<Entry>>,

    // notes_tb
    notes_tb: Option<Rc<Entry>>,

    
}

impl GWCApp {
    /// Provides a new instance of the GWC application
    pub fn new() -> GWCApp {
        GWCApp {
            passwd_label: None,
            window: None,
            password_type: None,
            url_tb: None,
            notes_tb: None,
        }
    }

    pub fn get_pass_type(data: &Rc<GtkPasswdArray>) -> PasswordType {
        for pw_type in &data.types {
            if pw_type.radio_buton.is_active() {
                return pw_type.password_type.clone();
            }
        }
        return PasswordType::Complex;
        /* `Option<PasswordType>` value */
    }

    pub fn set_password(_win: &Rc<Window>, lbl: &Rc<Label>, pass_type: PasswordType) {
        let p = password::Password {
            password_type: pass_type,
            password_length: 15,
        };
        let pass_str: String = p.get_a_password();
        let pass_mu: String = format!("<span size='16pt'>{}</span>", &pass_str);
        lbl.set_selectable(true);
        lbl.set_focus_on_click(true);
        lbl.set_markup(pass_mu.as_str());
        println!("{:?}", p.password_type);
    }

    /// Responsible for initializing the application state, including the whole UI
    pub fn init(&mut self) {
        let grid = gtk::Grid::new();
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        // let row_1 = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        // let row_2 = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let row_3 = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let password = Label::new(None);
        let url_label = Label::new(Some("URL   "));
        let notes_label = Label::new(Some("Notes"));
        

        let win = Window::new(WindowType::Toplevel);
        let rbgs: Vec<GtkPasswordTypes> = self.build_rbg();
        let rbgs_array: GtkPasswdArray = GtkPasswdArray { types: rbgs };
        let url_tb: Entry = Entry::new();
        let notes_tb: Entry = Entry::new();
    
        win.set_title("Rusty Password");
        win.set_position(gtk::WindowPosition::Center);
        win.set_size_request(500, 400);
        win.connect_delete_event(|_, _| {
            gtk::main_quit();
            Propagation::Stop
        });

        // The fields must be updated for the helper methods to work as expected.
        self.window = Some(Rc::new(win));
        self.passwd_label = Some(Rc::new(password));
        self.password_type = Some(Rc::new(rbgs_array));
        self.url_tb = Some(Rc::new(url_tb));
        self.notes_tb = Some(Rc::new(notes_tb));

        let pt: PasswordType = GWCApp::get_pass_type(
            &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                .unwrap()
                .clone(),
        );
        GWCApp::set_password(
            &self.window.clone().unwrap(),
            &self.passwd_label.clone().unwrap(),
            pt,
        );

        // create the application menu
        let menu_bar = self.init_menus();
        v_box.pack_start(&menu_bar, false, false, 0);

        // followed by the toolbar
        let tool_bar = self.init_toolbar();
        v_box.pack_start(&tool_bar, false, false, 0);

        // Create the complexity radio buttons
        let radio_container: gtk::Box = self.init_extra_tools();
        v_box.pack_start(&radio_container, false, true, 0);

        // add the url tb
        if let Some(ref url_tb) = self.url_tb {
            grid.attach(&url_label, 0, 0, 1, 1);
            grid.attach(url_tb.as_ref(), 1, 0, 1, 1);

            // row_1.pack_start(&url_label, false,false,10);
            // row_1.pack_start(url_tb.as_ref(), false, true, 10);
        }

        // add the notes tb
        if let Some(ref notes_tb) = self.notes_tb {
            grid.attach(&notes_label, 0, 1, 1, 1);
            grid.attach(notes_tb.as_ref(), 1, 1, 1, 1);

            // row_2.pack_start(&notes_label, false,false,10);
            // row_2.pack_start(notes_tb.as_ref(), false, true, 10);
        }

        // Create the password label
        if let Some(ref lbl) = self.passwd_label {
            row_3.pack_start(lbl.as_ref(), false, false, 10);
        }

        if let Some(ref w) = self.window {
            w.add(&v_box);
            v_box.add(&grid);
            // v_box.add(&row_2);
            v_box.add(&row_3);

        }
    }

    /// Displays the application window
    pub fn show(&self) {
        if let Some(ref win) = self.window {
            win.show_all()
        } else {
            panic!("Window has not been properly initialized");
        }
    }

    /// Creates the application menus
    fn init_menus(&self) -> MenuBar {
        let menu = Menu::new();
        let menu_bar = MenuBar::new();
        let file = MenuItem::with_label("File");
        let quit = MenuItem::with_label("Quit");
        let new_passwd: MenuItem = MenuItem::new();
        let new_passwd_label: Label = Label::new(Some("New Password"));
        let pass_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        pass_box.pack_start(&new_passwd_label, false, true, 0);
        new_passwd.add(&pass_box);
        menu.append(&new_passwd);
        menu.append(&quit);
        file.set_submenu(Some(&menu));
        menu_bar.append(&file);

        // Help menu
        let help_menu = Menu::new();
        let help = MenuItem::with_label("Help");
        let about = MenuItem::with_label("About");
        help_menu.append(&about);
        help.set_submenu(Some(&help_menu));
        menu_bar.append(&help);

        // This connects the new_passwd menu item with the set_password method
        if let Some(ref label) = self.passwd_label {
            if let Some(ref win) = self.window {
                // let pt: Vec<GtkPasswordTypes> = self.build_rbg();
                let pt: PasswordType = GWCApp::get_pass_type(
                    &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                        .unwrap()
                        .clone(),
                );
                let w = win.to_owned().clone();
                let l = label.clone();
                new_passwd.connect_activate(move |_| {
                    GWCApp::set_password(&w, &l, pt.clone());
                });
            }
        }

        quit.connect_activate(|_| {
            gtk::main_quit();
        });

        if let Some(ref win) = self.window {
            let wx = win.to_owned().clone();
            about.connect_activate(move |_| {
                let p = AboutDialog::new();
                const VERSION: &str = env!("CARGO_PKG_VERSION");
                const AUTHORS: &[&str] = &[env!("CARGO_PKG_AUTHORS")];
                const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
                const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
                p.set_authors(AUTHORS);
                p.set_website_label(Some("A GTK/Rust-based strong password generator"));
                p.set_website(Some(HOMEPAGE));
                p.set_version(Some(VERSION));
                p.set_title("About Rusty-Password");
                p.set_comments(Some(DESCRIPTION));
                p.set_transient_for(Some(wx.borrow() as &Window));
                p.run();
                p.close();
            });
        }

        menu_bar
    }

    /// Creates a vector of radio butttons for the form
    pub fn build_rbg(&self) -> Vec<GtkPasswordTypes> {
        let rbc: GtkPasswordTypes = GtkPasswordTypes {
            radio_buton: gtk::RadioButton::with_label("complex"),
            password_type: PasswordType::Complex,
        };
        let rbs: GtkPasswordTypes = GtkPasswordTypes {
            radio_buton: gtk::RadioButton::from_widget(&rbc.radio_buton),
            password_type: PasswordType::Simple,
        };
        let rb_simple_lable: Label = Label::new(Some("simple"));
        rbs.radio_buton.add(&rb_simple_lable);
        let mut rbg: Vec<GtkPasswordTypes> = Vec::with_capacity(2);
        rbg.push(rbc);
        rbg.push(rbs);
        rbg
    }

    // Create the extra tools toolbar
    fn init_extra_tools(&self) -> gtk::Box {
        let toolbar: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let rbg = self.build_rbg();
        for tbi in rbg.iter() {
            toolbar.add(&tbi.radio_buton);
        }
        toolbar
    }

    /// Creates the application toolbar
    fn init_toolbar(&self) -> Toolbar {
        let toolbar = Toolbar::new();
        toolbar.set_style(ToolbarStyle::Both);

        let new_pass_button_image =
            Image::from_icon_name(Some("document-new"), IconSize::LargeToolbar.into());
        let new_pass_button = ToolButton::new(Some(&new_pass_button_image), Some("New"));
        toolbar.insert(&new_pass_button, 0);

        let sep = SeparatorToolItem::new();
        toolbar.insert(&sep, 1);

        let quit_btn_image =
            Image::from_icon_name(Some("application-exit"), IconSize::LargeToolbar.into());
        let quit_btn = ToolButton::new(Some(&quit_btn_image), Some("Quit"));
        toolbar.insert(&quit_btn, 2);

        // This connects the new_passwd menu item with the set_password method
        if let Some(ref label) = self.passwd_label {
            if let Some(ref win) = self.window {
                // let pt: Vec<GtkPasswordTypes> = self.build_rbg();
                let pt: PasswordType = GWCApp::get_pass_type(
                    &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                        .unwrap()
                        .clone(),
                );

                let w = win.to_owned().clone();
                let l = label.clone();
                new_pass_button.connect_clicked(move |_| {
                    GWCApp::set_password(&w, &l, pt.clone());
                });
            }
        }

        quit_btn.connect_clicked(|_| {
            gtk::main_quit();
        });

        toolbar
    }
}

fn main() {
    let args = Args::parse();
    if args.gui {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let mut window = GWCApp::new();
        window.init();
        window.show();
        gtk::main();
    } else {
        let p = password::Password {
            password_type: {
                if args.simple {
                    password::PasswordType::Simple
                } else {
                    password::PasswordType::Complex
                }
            },
            password_length: args.length,
        };
        let my_password: String = p.get_a_password();
        println!("{:?}", my_password);
    }
}
