use std::env;
use gtk::prelude::*;
use gtk::{glib, Application};
use clap::Parser;

const APP_ID: &str = "org.scq.Rusty_Password";

///Set up cli arg
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// length of password to create
    #[arg(short, long, default_value_t = 15)]
    length: i32,
}

fn main() { 
    let empty: Vec<String> = vec![];
    let args = Args::parse();
    let app = Application::builder().application_id(APP_ID).build();
    app.run_with_args(&empty);

    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~{}[]+=";
    let password_len = args.length;
    let mut rng = rand::thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}