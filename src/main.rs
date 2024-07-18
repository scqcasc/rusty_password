use std::env;
use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.scq.Rusty_Password";

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = &args[1];
    let app = Application::builder().application_id(APP_ID).build();
    app.run();

    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~{}[]+=";
    let password_len: i32 = length.parse().expect("message");
    let mut rng = rand::thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}