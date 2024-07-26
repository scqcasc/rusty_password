use rand::Rng;

fn get_charset(simple: bool) -> String {
    let mut main_string: String = String::from("");
    let lc: &str = "abcdefghijklmnopqrstuvwxyz";
    let uc: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let num: &str = "0123456789";
    let sc: &str= "!+=%#*";
    let scext: &str = "@!~{}[]()^";
    main_string.push_str(lc);
    main_string.push_str(uc);
    main_string.push_str(num);
    main_string.push_str(sc);

    if simple {
        return main_string;
    } else {
        main_string.push_str(scext);
        return main_string;
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