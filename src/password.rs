// generates a strong password making sure there is at least 1 instance of each char type

use rand::Rng;

pub enum PasswordType {
    Simple,
    Complex,
}

enum PasswordContents {
    Lc,
    Uc,
    Num,
    Sc,
    Scext,
}
impl PasswordContents {
    fn value(&self) -> &str {
        match *self {
            PasswordContents::Lc => "abcdefghijklmnopqrstuvwxyz",
            PasswordContents::Uc => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            PasswordContents::Num => "0123456789",
            PasswordContents::Sc => "!+=%#*@^",
            PasswordContents::Scext => "~{}[]()",
    }
}

}
pub struct Password {
    pub password_type: PasswordType,
    pub password_length: i32,
}

impl Password {
    fn check_in_list(&self, password: &String, sub_list: &PasswordContents) -> bool {
        let list = sub_list.value();
        for l in password.chars() {
            if list.contains(l) {
                return true;
            }
        }
        
        return false;
    }

    pub fn get_a_password(&self) -> String {
        match self.password_type { 
            PasswordType::Complex => {
                let my_pass = self.get_password(false, self.password_length);
                return my_pass.unwrap();
            },
            PasswordType::Simple => {
                let my_pass = self.get_password(true, self.password_length);
                return my_pass.unwrap();
            }
    }
}

    pub fn get_password(&self, simple: bool, length: i32) -> Option<String> {
        let binding = self.get_charset(simple);
        let charset: &[u8] = binding.as_bytes();
        
        let password_len = length;
        let mut rng = rand::thread_rng();

        let checks = [
                PasswordContents::Lc, 
                PasswordContents::Uc, 
                PasswordContents::Num, 
                PasswordContents::Sc
            ];
        // do a loop here checking to make sure all the types have a char in
        loop {
            let password: String = (0..password_len)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
            'inner_loop: loop {
                for i in 0..checks.len() {
                    let value = self.check_in_list(&password, &checks[i]);
                    let ready = value;
                    if ready == false {
                        break 'inner_loop;
                    }
                };
                return Some(password.clone());
            };
        };
    }

    fn get_charset(&self, simple: bool) -> String {
        let mut main_string: String = String::from("");        
        main_string.push_str(PasswordContents::value(&PasswordContents::Lc));
        main_string.push_str(PasswordContents::value(&PasswordContents::Uc));
        main_string.push_str(PasswordContents::value(&PasswordContents::Num));
        main_string.push_str(PasswordContents::value(&PasswordContents::Sc));
    
        if simple {
            return main_string;
        } else {
            main_string.push_str(PasswordContents::value(&PasswordContents::Scext));
            return main_string;
        }
    }
}


