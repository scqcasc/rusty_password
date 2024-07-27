// Thanks to https://github.com/pjmlp/gwc-rs/ for showing the way forward with the GUI


extern crate gtk;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{Window, WindowType, Label, Menu, MenuBar, MenuItem, IconSize, Image, AboutDialog, Toolbar, ToolButton,
    ToolbarStyle, SeparatorToolItem};
use clap::Parser;
use std::rc::Rc;
use std::borrow::Borrow;

mod password;


///Set up cli arg
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
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

/// The application window state
#[derive(Debug, Clone)]
struct GWCApp {
    /// the window label used to display the counters
    passwd_label: Option<Rc<Label>>,

    /// a kind of handle for the Gtk+ window
    window : Option<Rc<Window>>
}

impl GWCApp {

    /// Provides a new instance of the GWC application
    pub fn new() -> GWCApp {
        GWCApp { passwd_label: None, window: None }
    }

    
    pub fn set_password(_win:&Rc<Window>, lbl : &Rc<Label>) {
        let p = password::Password {
            password_type: password::PasswordType::Complex,
            password_length: 15,
        };
        let pass_str: String = p.get_a_password();
        let pass_mu: String= format!("<span size='16pt'>{}</span>",&pass_str);
        lbl.set_selectable(true);
        lbl.set_focus_on_click(true);
        lbl.set_markup(pass_mu.as_str());
    }

    /// Responsible for initializing the application state, including the whole UI
    pub fn init(&mut self) {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let  password = Label::new(None);
        let win = Window::new(WindowType::Toplevel);
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
        GWCApp::set_password(&self.window.clone().unwrap(), &self.passwd_label.clone().unwrap());

        // create the application menu
        let menu_bar = self.init_menus();
        v_box.pack_start(&menu_bar, false, false, 0);

        // followed by the toolbar
        let tool_bar = self.init_toolbar();
        v_box.pack_start(&tool_bar, false, false, 0);

        // Create the password label
        if let Some (ref lbl) = self.passwd_label {
            v_box.pack_start(lbl.as_ref(), true, true, 0);
        }
        
        if let Some (ref w) = self.window {
            w.add(&v_box);
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
    fn init_menus (&self) -> MenuBar {
        let menu = Menu::new();
        let menu_bar = MenuBar::new();
        let file = MenuItem::with_label("File");
        let quit = MenuItem::with_label("Quit");
        let new_passwd: MenuItem = MenuItem::new();
        let new_passwd_label: Label = Label::new(Some("New Password"));
        let pass_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        pass_box.pack_start(&new_passwd_label, false, true, 0);
        new_passwd.add(&pass_box);

        // menu.append(&file_item);
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
        if let Some (ref label) = self.passwd_label {
            if let Some (ref win) = self.window {
                let w = win.to_owned().clone();
                let l = label.clone();
                new_passwd.connect_activate(move |_| {
                    GWCApp::set_password(&w, &l);
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

    /// Creates the application toolbar
    fn init_toolbar(&self) -> Toolbar {
        let toolbar = Toolbar::new();
        toolbar.set_style(ToolbarStyle::Both);

        let new_pass_button_image = Image::from_icon_name(Some("document-new"), IconSize::LargeToolbar.into());
        let new_pass_button = ToolButton::new(Some(&new_pass_button_image), Some("New"));
        toolbar.insert(&new_pass_button, 0);

        let sep = SeparatorToolItem::new();
        toolbar.insert(&sep, 1);

        let quit_btn_image = Image::from_icon_name(Some("application-exit"), IconSize::LargeToolbar.into());
        let quit_btn = ToolButton::new(Some(&quit_btn_image), Some("Quit"));
        toolbar.insert(&quit_btn, 2);


        if let Some (ref label) = self.passwd_label {
            if let Some (ref win) = self.window {
                let w = win.to_owned().clone();
                let l = label.clone();
                new_pass_button.connect_clicked(move |_| {
                    GWCApp::set_password(&w, &l);
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
    }else{
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