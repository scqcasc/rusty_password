use rand::Rng;

fn get_charset(simple: bool) -> String {
    if simple {
        return String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789!+=%#*");
    } else {
        return String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789)(*^%$#@!~{}[]+=");
    }
}

pub fn get_password(simple: bool, length: i32) -> String {
    let binding = get_charset(simple);
    let charset: &[u8] = binding.as_bytes();
    
    let password_len = length;
    let mut rng = rand::thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
        return password;
}