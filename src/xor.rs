use std::string::FromUtf8Error;

pub fn xor_encode(data: &[u8], key: &str) -> Vec<u8> {
    if key.len() == 0 {
        return data.to_vec();
    }

    let bytes = key.as_bytes();
    let mut encoded_bytes = Vec::new();

    for i in 0..data.len() {
        encoded_bytes.push(data[i] ^ bytes[i % bytes.len()]);
    }

    encoded_bytes
}

pub fn xor_decode(data: &[u8], key: &str) -> Result<String, FromUtf8Error> {
    if key.len() == 0 {
        return String::from_utf8(data.to_vec());
    }

    let bytes = key.as_bytes();
    let mut decoded_bytes = Vec::new();

    for i in 0..data.len() {
        decoded_bytes.push(data[i] ^ bytes[i % bytes.len()]);
    }

    String::from_utf8(decoded_bytes)
}
