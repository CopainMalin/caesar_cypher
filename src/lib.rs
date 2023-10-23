static LETTERS : [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub struct Crypter {
    pub msg: String,
    pub decalage: i8,
}

impl Crypter {
    pub fn new(msg: String, decalage: i8) -> Crypter {
        Crypter {msg, decalage}
    }

    fn find_index(&self, caractere: char) -> i8 {
        let mut index = 0;
        
        for element in LETTERS {
            
            if element == caractere {
                return index;
            } else {
                index+=1;
            }
        }
    
        return 0;
    }

    fn move_char(&self, caractere: char, decalage: i8) -> char {
    
        let index = self.find_index(caractere);
        let new_index = index + decalage;
        
        if (new_index as usize) > LETTERS.len() && new_index > 0 {
            return LETTERS[(new_index as usize) - LETTERS.len()];
        }
    
        else {
            if new_index < 0 {
                return LETTERS[(new_index + (LETTERS.len() as i8)) as usize];
            }
        }
        
        return LETTERS[new_index as usize];
    }

    pub fn encrypt_word(&self, mot: &String, decalage: i8) -> String {
        let char_vec : Vec<char> = mot.chars().collect();
        let mut result = String::from("");
    
        for caractere in char_vec {
            result.push(self.move_char(caractere, decalage));
        }
    
        return result;
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_finder_find_the_right_index() {
        let crypter = Crypter::new(String::from("test"), 0);
        assert_eq!(crypter.find_index('a'), 0);
        assert_eq!(crypter.find_index('z'), 25);
    }
    
    #[test]
    fn move_forward_works() {
        let crypter = Crypter::new(String::from("test"), 0);
         assert_eq!(crypter.move_char('a', 2), 'c');
    }

    #[test]
    fn move_backward_works() {
        let crypter = Crypter::new(String::from("test"), 0);
        assert_eq!(crypter.move_char('z', 2), 'b');
   }

   #[test]
   fn encrypt_word_works() {
        let crypter = Crypter::new(String::from("test"), 0);
        assert_eq!(crypter.encrypt_word(&String::from("baptistz"), 2), String::from("dcrvkuvb"))
   }
   
   #[test]
   fn decrypt_word_works() {
    let crypter = Crypter::new(String::from("test"), 0);
    assert_eq!(crypter.encrypt_word(&String::from("dcrvkuvb"), -2), String::from("baptistz"))
}

}
