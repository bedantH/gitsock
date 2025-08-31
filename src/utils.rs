use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1::EncodeRsaPrivateKey, PublicKeyParts};
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};

fn write_ssh_string(buf: &mut Vec<u8>, data: &[u8]) {
    let len = data.len() as u32;
    buf.extend(&len.to_be_bytes());
    buf.extend(data);
}

fn write_ssh_mpint(buf: &mut Vec<u8>, data: &[u8]) {
    // SSH mpint format: if the first bit is set, prepend a zero byte
    if !data.is_empty() && (data[0] & 0x80) != 0 {
        let mut padded = vec![0];
        padded.extend(data);
        write_ssh_string(buf, &padded);
    } else {
        write_ssh_string(buf, data);
    }
}

pub fn generate_rsa_key_pair() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 4096)?;
    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key.to_pkcs1_pem(Default::default())?.to_string();

    // Prepare public key in OpenSSH format with proper mpint encoding
    let e_bytes = public_key.e().to_bytes_be();
    let n_bytes = public_key.n().to_bytes_be();

    let mut key_blob = Vec::new();
    write_ssh_string(&mut key_blob, b"ssh-rsa");
    write_ssh_mpint(&mut key_blob, &e_bytes);  // Use mpint format
    write_ssh_mpint(&mut key_blob, &n_bytes);  // Use mpint format

    let public_key_base64 = general_purpose::STANDARD.encode(&key_blob);
    
    let public_key_ssh = format!("ssh-rsa {}", public_key_base64);

    Ok((private_pem, public_key_ssh))
}

pub fn save_key(path: &str, key: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(key.as_bytes()).unwrap();
}