// Thanks to https://github.com/pjmlp/gwc-rs/ for showing the way forward with the GUI

extern crate gtk;

use clap::Parser;

// use password::PasswordType;
mod password;

use gui::GWCApp;

mod gui;

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
