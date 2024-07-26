// change this to a more OO model.  
// enum Password that impliments methods to generate the password and
// ensure one of each type is included
// Doing something like
//fn check_is_in(letter: char, sub_list: &str) -> bool {
//    if sub_list.contains(letter) {
//        println!("{letter} found");
//        return true;
//    }
//    return false;
//}
//fn main() {
//    let mut main_binding: String = String::from("");
//    let binding: &str = "abc";
//    let binding2: &str = "ABC";
//    
//    main_binding.push_str(binding);
//    main_binding.push_str(binding2);
//    
//    let password: String = String::from("aBd");
//    
//    for l in password.chars() {
//        if check_is_in(l, binding) {
//            break;
//        }
//        
//    }
//    
//    for l in password.chars() {
//        if check_is_in(l, binding2) {
//            break;
//        }
//    }
//}




use rand::Rng;

pub enum PasswordType {
    Simple,
    Complex,
}

enum PasswordContents<'a> {
    Lc(&'a str),
    Uc(&'a str),
    Num(&'a str),
    Sc(&'a str),
    Scext(&'a str),
}

pub fn set_pw_contents(pw_contents: PasswordContents) {
    match pw_contents {
        PasswordContents::Lc("abcdefghijklmnopqrstuvwxyz"),
        PasswordContents::Uc("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        
    }
}
pub struct Password {
    pub password_type: PasswordType,
    pub password_length: i32,
}

impl Password {
    pub fn get_a_password(&self) -> String {
        match self.password_type { 
            PasswordType::Complex => {
                let my_pass = self.get_password(false, self.password_length);
                return my_pass;
            },
            PasswordType::Simple => {
                let my_pass = self.get_password(true, self.password_length);
                return my_pass;
            }
    }
}

    pub fn get_password(&self, simple: bool, length: i32) -> String {
        let binding = self.get_charset(simple);
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


    fn get_charset(&self, simple: bool) -> String {
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
}


