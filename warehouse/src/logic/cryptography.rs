use argon2::{self, Config};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_salt() -> [u8; 8] {
    rand::thread_rng().gen::<[u8; 8]>()
}

pub fn hash_and_salt(item: String, salt: &[u8]) -> Result<String, argon2::Error> {
    argon2::hash_encoded(item.as_bytes(), salt, &Config::default())
}

pub fn verify_encryption(hash: String, item_bytes: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(&hash, item_bytes)
}

pub fn verify_ip_encrypted(to_check_ip: String, database_ip: String, database_salt: &[u8]) -> bool {
    hash_and_salt(to_check_ip, database_salt).unwrap() == database_ip
}

pub fn generate_code() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub fn parse_string_salt_to_byte_vector(byte_vec_string: String) -> Vec<u8> {
    let modified = byte_vec_string.replace("[", "").replace("]", "");

    let split_modified = modified
        .split(", ")
        .into_iter()
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    split_modified
}
