//public-key encryption algorithm enables secure data transmission, encrypting it with public key and decrypting it with private key

fn caesar_cypher(msg: String, cShift: u8) -> String {
    let encrypted_msg = "";
    for c in msg.chars() {
        let temp = c as u8;
        temp = temp + cShift;
        encrypted_msg = encrypted_msg + (temp as &str);
    }

    return encrypted_msg;
}

fn main() {
    let msg = String::from("Hello there");
    let encrypted_msg = caesar_cypher(msg,1);
    println!("{}", encrypted_msg);

}
