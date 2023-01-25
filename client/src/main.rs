use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    let challenge = "Please solve this POW to prove that you are not a bot: ";
    stream.write(challenge.as_bytes()).unwrap();

    let mut buf = [0; 256];
    let bytes_read = stream.read(&mut buf[..]).unwrap();
    let salt = std::str::from_utf8(&buf[..bytes_read]).unwrap().to_string();

    let solution = challenge.to_owned() + &salt;
    let hash = Sha256::digest(solution.as_bytes());

    stream.write(salt.as_bytes()).unwrap();

    let mut quote = [0; 256];
    let bytes_read = stream.read(&mut quote[..]).unwrap();
    let quote = std::str::from_utf8(&quote[..bytes_read]).unwrap();
    println!("{}", quote);
}