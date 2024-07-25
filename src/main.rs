// Thanks to https://github.com/pjmlp/gwc-rs/ for showing the way forward with the GUI


extern crate gtk;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{Window, WindowType, Label, Menu, MenuBar, MenuItem, IconSize, Image, AboutDialog, Toolbar, ToolButton,
    ToolbarStyle, SeparatorToolItem, FileChooserDialog, FileChooserAction, ResponseType};
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

    /// Responsible for initializing the application state, including the whole UI
    pub fn init(&mut self) {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let pass_str: String = password::get_password(true, 15);
        let password = Label::new(None);
        let pass_mu = format!("<span size='16pt'>{}</span>",&pass_str);
        password.set_selectable(true);
        password.set_focus_on_click(true);
        password.set_markup(pass_mu.as_str());
        
        
        
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
        

        // create the application menu
        let menu_bar = self.init_menus();
        v_box.pack_start(&menu_bar, false, false, 0);

        // followed by the toolbar
        let tool_bar = self.init_toolbar();
        v_box.pack_start(&tool_bar, false, false, 0);

        // Create the text label for showing the word count
        
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

    ///  Called when the user selects the
    /// File->Open option
    fn on_menu_open(win:&Rc<Window>, _lbl : &Rc<Label>) {
        let filesel = FileChooserDialog::new(Some("Choose a file"), Some(win.as_ref()),
                                                    FileChooserAction::Open);
        filesel.add_buttons(&[
            ("Open", ResponseType::Ok.into()),
            ("Cancel", ResponseType::Cancel.into())
        ]);

        filesel.set_select_multiple(true);
        filesel.run();
        let _file = filesel.filename();
        filesel.close();

    }

    /// Creates the application menus
    fn init_menus (&self) -> MenuBar {
        let menu = Menu::new();
        let menu_bar = MenuBar::new();
        let file = MenuItem::with_label("File");
    
        let quit = MenuItem::with_label("Quit");
        let file_item = MenuItem::new();
        let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let file_image = Image::from_icon_name(Some("document-open"), IconSize::Menu.into());
        let file_label = Label::new(Some("File"));

        file_box.pack_start(&file_image, false, false, 0);
        file_box.pack_start(&file_label, true, true, 0);
        file_item.add(&file_box);

        menu.append(&file_item);
        menu.append(&quit);
        file.set_submenu(Some(&menu));
        menu_bar.append(&file);

        // Extras menu
        let extras_menu = Menu::new();
        let extras = MenuItem::with_label("Extras");
        let about = MenuItem::with_label("About");

        extras_menu.append(&about);
        extras.set_submenu(Some(&extras_menu));
        menu_bar.append(&extras);

        if let Some (ref label) = self.passwd_label {
            if let Some (ref win) = self.window {
                let w = win.to_owned().clone();
                let l = label.clone();

                file_item.connect_activate(move |_| {
                    GWCApp::on_menu_open(&w, &l);
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

                p.set_authors(&["Sean Quaint"]);
                p.set_website_label(Some("A GTK/Rust-based strong password generator"));
                p.set_website(Some("https://github.com/scqcasc/rusty_password"));
                p.set_authors(&["Sean Quaint"]);
                p.set_title("About Rusty-Password");
                p.set_comments(Some("Written while learning about Rust and GTK4"));
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

        let open_btn_image = Image::from_icon_name(Some("document-open"), IconSize::LargeToolbar.into());
        let open_btn = ToolButton::new(Some(&open_btn_image), Some("Open"));
        toolbar.insert(&open_btn, 0);

        let sep = SeparatorToolItem::new();
        toolbar.insert(&sep, 1);

        let quit_btn_image = Image::from_icon_name(Some("application-exit"), IconSize::LargeToolbar.into());
        let quit_btn = ToolButton::new(Some(&quit_btn_image), Some("Quit"));
        toolbar.insert(&quit_btn, 2);


        if let Some (ref label) = self.passwd_label {
            if let Some (ref win) = self.window {
                let w = win.to_owned().clone();
                let l = label.clone();

                open_btn.connect_clicked(move |_| {
                    GWCApp::on_menu_open(&w, &l);
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
        let my_password = password::get_password(args.simple, args.length);
        println!("{:?}", my_password);
    }
    
}