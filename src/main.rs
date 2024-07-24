use gtk::prelude::*;
use gtk::{Application};
use clap::Parser;

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
}

fn get_charset(simple: bool) -> String {
    if simple {
        return String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789!+=&%#*");
    } else {
        return String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789)(*&^%$#@!~{}[]+=");
    }
}

fn main() { 
    let empty: Vec<String> = vec![];
    let args = Args::parse();
    let app = Application::builder().application_id(APP_ID).build();
    app.run_with_args(&empty);

    use rand::Rng;
    
    let binding = get_charset(args.simple);
    let charset: &[u8] = binding.as_bytes();
    
    let password_len = args.length;
    let mut rng = rand::thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    println!("{:?}", password);
}