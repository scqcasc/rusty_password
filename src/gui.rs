use gtk::glib::Propagation;
use gtk::{prelude::*, SpinButton};
use gtk::{
    AboutDialog, IconSize, Image, Label, Menu, MenuBar, MenuItem, SeparatorToolItem, ToolButton,
    Toolbar, ToolbarStyle, Window, WindowType, Entry, Adjustment, CheckButton
};
use std::borrow::Borrow;
use std::rc::Rc;
use crate::password::PasswordType;

// mod password;

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
pub struct GWCApp {
  
    /// a kind of handle for the Gtk+ window
    window: Option<Rc<Window>>,

    /// option for password type
    password_type: Option<Rc<GtkPasswdArray>>,

    /// url_tb
    url_tb: Option<Rc<Entry>>,

    // notes_tb
    notes_tb: Option<Rc<Entry>>,

    // username_tb
    username_tb: Option<Rc<Entry>>,

    // password_tb
    password_tb: Option<Rc<Entry>>,

    //pass_size_spin
    password_size_spin: Option<Rc<SpinButton>>,

    // encryption_key_tb
    encryption_key_tb: Option<Rc<Entry>>,

    

}


impl GWCApp {
    /// Provides a new instance of the GWC application
    pub fn new() -> GWCApp {
        GWCApp {
            window: None,
            password_type: None,
            url_tb: None,
            notes_tb: None,
            username_tb: None,
            password_tb: None,
            password_size_spin: None,
            encryption_key_tb: None,
        }
    }

    fn get_pass_type(data: &Rc<GtkPasswdArray>) -> PasswordType {
        for pw_type in &data.types {
            if pw_type.radio_buton.is_active() {
                return pw_type.password_type.clone();
            }
        }
        return PasswordType::Complex;
        /* `Option<PasswordType>` value */
    }

    pub fn set_password(_win: &Rc<Window>, lbl: &Rc<Entry>, pass_type: PasswordType, size: &Rc<gtk::SpinButton>) {
        
        let p = crate::password::Password {
            password_type: pass_type,
            password_length: size.value_as_int(),
        };
        let pass_str: String = p.get_a_password();
        lbl.set_focus_on_click(true);
        lbl.set_text(&pass_str);
        
    }

    /// Responsible for initializing the application state, including the whole UI
    pub fn init(&mut self) {

        // input_grid contains the input fields
        let input_grid = gtk::Grid::new();
        input_grid.set_row_spacing(10);
        input_grid.set_column_spacing(10);
        input_grid.set_margin(5);

        // tool_grid contains the tools
        let tool_grid = gtk::Grid::new();
        tool_grid.set_row_spacing(10);
        tool_grid.set_column_spacing(10);
        tool_grid.set_margin(5);

        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        
        // Labels
        let password_label = Label::new(Some("Password"));
        let url_label = Label::new(Some("URL"));
        let notes_label = Label::new(Some("Notes"));
        let username_label = Label::new(Some("Username"));
        let password_size_label = Label::new(Some("Password Size"));
        let encypt_key_label = Label::new(Some("Encryption Key"));
        
        // Widgets
        let win = Window::new(WindowType::Toplevel);
        let rbgs: Vec<GtkPasswordTypes> = self.build_rbg();
        let rbgs_array: GtkPasswdArray = GtkPasswdArray { types: rbgs };
        let url_tb: Entry = Entry::new();
        let notes_tb: Entry = Entry::new();
        let username_tb: Entry = Entry::new();
        let password_tb: Entry = Entry::new();
        let adjustment: Adjustment = gtk::Adjustment::new (15.0, 0.0, 100.0, 1.0, 5.0, 0.0);
        let password_size_spin: SpinButton = SpinButton::new(Some(&adjustment),1.0, 0);
        let encryption_key_tb: Entry = Entry::new();
        let toggle_visibility_cb = CheckButton::with_label("Show passwords");

        encryption_key_tb.set_visibility(false);
        password_tb.set_visibility(false);

        // this toggles visibility of password fields
        {
            let encryption_key_tb = encryption_key_tb.clone();
            let password_tb = password_tb.clone();

            toggle_visibility_cb.connect_toggled(move |cb| {
                encryption_key_tb.set_visibility(cb.is_active());
                password_tb.set_visibility(cb.is_active());

            });
        }

        // main win properties
        win.set_title("Rusty Password");
        win.set_position(gtk::WindowPosition::Center);
        win.set_size_request(500, 400);
        win.connect_delete_event(|_, _| {
            gtk::main_quit();
            Propagation::Stop
        });

        // The fields must be updated for the helper methods to work as expected.
        self.window = Some(Rc::new(win));
        self.password_type = Some(Rc::new(rbgs_array));
        self.url_tb = Some(Rc::new(url_tb));
        self.notes_tb = Some(Rc::new(notes_tb));
        self.username_tb = Some(Rc::new(username_tb));
        self.password_tb = Some(Rc::new(password_tb));
        self.password_size_spin = Some(Rc::new(password_size_spin));
        self.encryption_key_tb = Some(Rc::new(encryption_key_tb));


        let pt: PasswordType = GWCApp::get_pass_type(
            &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                .unwrap()
                .clone(),
        );
        GWCApp::set_password(
            &self.window.clone().unwrap(),
            &self.password_tb.clone().unwrap(),
            pt,
            &self.password_size_spin.clone().unwrap(),
        );

        // create the application menu
        let menu_bar = self.init_menus();
        v_box.pack_start(&menu_bar, false, false, 0);

        // followed by the toolbar
        let tool_bar = self.init_toolbar();
        v_box.pack_start(&tool_bar, false, false, 0);

        // Create the complexity radio buttons
        let radio_container: gtk::Box = self.init_extra_tools();
        tool_grid.attach(&radio_container, 0,1,1,1);
        tool_grid.attach(&toggle_visibility_cb, 3, 1,10,1);

        
        // add the password size spinner
        if let Some(ref password_size_spin) = self.password_size_spin {
            tool_grid.attach(&password_size_label, 1, 1, 1, 1);
            tool_grid.attach(password_size_spin.as_ref(), 2, 1, 1, 1);
        }

        // add the encryption_key_tb
        if let Some(ref encryption_key_tb) = self.encryption_key_tb {
            tool_grid.attach(&encypt_key_label, 1, 2, 1, 1);
            tool_grid.attach(encryption_key_tb.as_ref(), 2, 2,25,1);
        }

        // add the url tb
        if let Some(ref url_tb) = self.url_tb {
            input_grid.attach(&url_label, 0, 0, 1, 1);
            input_grid.attach(url_tb.as_ref(), 1, 0, 55, 1);
        }

        // add the username tb
        if let Some(ref username_tb) = self.username_tb {
            input_grid.attach(&username_label, 0, 1, 1, 1);
            input_grid.attach(username_tb.as_ref(), 1, 1, 55, 1);
        }
        // add the notes tb
        if let Some(ref notes_tb) = self.notes_tb {
            input_grid.attach(&notes_label, 0, 2, 1, 1);
            input_grid.attach(notes_tb.as_ref(), 1, 2, 55, 1);
        }

        // Create the password label
        if let Some(ref password_tb) = self.password_tb {
            input_grid.attach(&password_label, 0, 3, 1, 1);
            input_grid.attach(password_tb.as_ref(),1, 3, 55, 1);
        }

        if let Some(ref w) = self.window {
            w.add(&v_box);
            v_box.add(&tool_grid);
            v_box.add(&input_grid);
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
        if let Some(ref label) = self.password_tb {
            if let Some(ref win) = self.window {
                if let Some(ref size) = self.password_size_spin{
                    // let pt: Vec<GtkPasswordTypes> = self.build_rbg();
                    let pt: PasswordType = GWCApp::get_pass_type(
                        &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                            .unwrap()
                            .clone(),
                    );
                    let w = win.to_owned().clone();
                    let l = label.clone();
                    let s = size.clone();
                
                    new_passwd.connect_activate(move |_| {
                        GWCApp::set_password(&w, &l, pt.clone(), &s);
                    });
                }
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
    fn build_rbg(&self) -> Vec<GtkPasswordTypes> {
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

        let view_button_image = Image::from_icon_name(Some("view-reveal"), IconSize::LargeToolbar.into());
        let view_button = ToolButton::new(Some(&view_button_image), Some("View"));
        toolbar.insert(&view_button,2);
        let sep = SeparatorToolItem::new();
        toolbar.insert(&sep, 3);
        let quit_btn_image =
            Image::from_icon_name(Some("application-exit"), IconSize::LargeToolbar.into());
        let quit_btn = ToolButton::new(Some(&quit_btn_image), Some("Quit"));
        toolbar.insert(&quit_btn, 4);

        // This connects the new_passwd menu item with the set_password method
        if let Some(ref label) = self.password_tb {
            if let Some(ref win) = self.window {
                if let Some(ref size) = self.password_size_spin{

                    // let pt: Vec<GtkPasswordTypes> = self.build_rbg();
                    let pt: PasswordType = GWCApp::get_pass_type(
                        &<Option<Rc<GtkPasswdArray>> as Clone>::clone(&self.password_type)
                            .unwrap()
                            .clone(),
                    );

                    let w = win.to_owned().clone();
                    let l = label.clone();
                    let s = size.clone();

                    new_pass_button.connect_clicked(move |_| {
                        GWCApp::set_password(&w, &l, pt.clone(), &s);
                    });
                }
            }
        }

        quit_btn.connect_clicked(|_| {
            gtk::main_quit();
        });

        toolbar
    }
}