use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = &args[1];

    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~{}[]+=";
    let PASSWORD_LEN: i32 = length.parse().expect("message");
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}