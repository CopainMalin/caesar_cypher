use caesar_cypher::Crypter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
    let decalage = args[2].parse::<i8>().unwrap();
    let cypher = Crypter {msg: word.to_string(), decalage: decalage};
    println!("{:}", cypher.encrypt_word(&cypher.msg, cypher.decalage));
}