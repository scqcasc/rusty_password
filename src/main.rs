use gtk::prelude::*;
use gtk::{Application};
use clap::Parser;
mod password;

const APP_ID: &str = "org.scq.Rusty_Password";

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

    /// no GUI
    #[arg(short, long, default_value_t = false)]
    gui: bool,
}



fn main() { 
    let args = Args::parse();
    if args.gui {
        let empty: Vec<String> = vec![];
        let app = Application::builder().application_id(APP_ID).build();
        app.run_with_args(&empty);
        println!("run with gui")
    }else{
        let my_password = password::get_password(args.simple, args.length);
        println!("{:?}", my_password);
    }
    
}