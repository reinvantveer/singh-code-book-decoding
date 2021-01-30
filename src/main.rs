use std::fs;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Hash;

fn main() {
    let letter_path = "src/encrypted-letter.txt";
    let mut letter = fs::read_to_string(letter_path).expect(
        &*("source file for encrypted letter not found at ".to_owned() + &letter_path)
    );
    println!("Encrypted letter contents:\n{}", &letter);

    let cipher = load_cipher();

    println!("{}", "Cipher:");
    for (encoded, decoded) in cipher.clone() {
        let encoded_token = encoded
            .into_string().unwrap();

        let occurences = letter
            .matches(&encoded_token)
            .collect::<Vec<_>>()
            .len();

        println!("{}: {}", encoded_token, &decoded.into_string().unwrap());
        println!("Occurrences: {}", &occurences)
    }

    decode_letter(&mut letter, cipher);
}

fn decode_letter(letter: &mut String, cipher: Hash) {
    for (encoded, decoded) in cipher {
        let from_token = encoded.into_string().unwrap();
        let to_token = decoded.into_string().unwrap();
        *letter = letter.replace(&from_token, &to_token);
    }

    println!("\nDecoded letter:");
    println!("{}", letter);
}

fn load_cipher() -> Hash {
    let cipher_path = "src/cipher.yaml";

    let cipher_yaml = fs::read_to_string(cipher_path).expect(
        &*("yaml file for encrypted letter not found at ".to_owned() + &cipher_path)
    );

    let docs = YamlLoader::load_from_str(&cipher_yaml).unwrap();
    let doc = &docs[0];  // The first document in the yaml file
    let cipher = doc["cipher"].clone().into_hash().unwrap();
    cipher
}
