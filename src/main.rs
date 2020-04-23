use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;
use sodiumoxide::crypto::sealedbox;
use std::env;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Got {} arguments; expected 2.", args.len());
        eprintln!("Usage: octocat base64_encoded_pub_key < secretdata");
        ::std::process::exit(1);
    }

    let keybytes = match base64::decode(&args[1]) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to base64-decode the public key argument! {:?}", e);
            ::std::process::exit(1);
        },
    };
    let pubkey = match PublicKey::from_slice(&keybytes) {
        Some(k) => k,
        None => {
            eprintln!("Public key input was not valid public key data!");
            ::std::process::exit(1);
        },
    };

    // max length of a secret is 64K. Our average secret is much smaller.
    let mut input = Vec::with_capacity(1024);
    let mut stdin = std::io::stdin();
    stdin.read_to_end(&mut input).unwrap();

    // for the love of God, Montresor!
    let sealed = sealedbox::seal(&input, &pubkey);

    let mut encoded = Vec::new();
    {
        let mut enc = base64::write::EncoderWriter::new(&mut encoded, base64::STANDARD);
        enc.write_all(&sealed).unwrap();
        enc.finish().unwrap();
    }

    // we know b64 is safe so we unwrap with abandon
    println!("{}", std::str::from_utf8(&encoded).unwrap());
}
