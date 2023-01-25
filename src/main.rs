use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn proof_of_work(challenge: &str, difficulty: u8) -> String {
    let salt = rand::random::<u128>().to_string();
    let solution = challenge.to_owned() + &salt;
    let hash = Sha256::digest(solution.as_bytes());
    if hash.iter().take(difficulty as usize).all(|b| *b == 0) {
        return salt;
    }
    let mut leading_zeros = 0;
    for (i, byte) in hash.iter().rev().enumerate() {
        if *byte != 0 {
            break;
        }
        leading_zeros += i * 8;
    }
    if leading_zeros >= difficulty.into() {
        return salt;
    }
    return String::new();
}

fn handle_client(mut stream: TcpStream) {
    let challenge = "Please solve this POW to prove that you are not a bot: ";
    stream.write(challenge.as_bytes()).unwrap();

    let mut buf = [0; 256];
    let bytes_read = stream.read(&mut buf[..]).unwrap();
    let solution = std::str::from_utf8(&buf[..bytes_read]).unwrap();

    let difficulty = 8;
    let expected_solution = proof_of_work(challenge, difficulty);
    if solution != expected_solution {
        stream.write("Invalid solution".as_bytes()).unwrap();
        return;
    }

    let quotes = vec![
            "\"The glory of God is intelligence.\" - D&C 93:36",
            "\"And if your eye be single to my glory, your whole bodies shall be filled with light, and there shall be no darkness in you.\" - D&C 88:67",
            "\"And whatsoever ye shall ask the Father in my name, which is right, believing that ye shall receive, behold it shall be given unto you.\" - 3 Nephi 18:20",
        ];
    let quote = quotes[rand::random::<usize>() % quotes.len()];
    stream.write(quote.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut client_requests: HashMap<TcpStream, Instant> = HashMap::new();
    let request_timeout = Duration::from_secs(60);
    let mut connection_count = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if connection_count >= 10 {
                    stream  
                        .write(
                            "Too many connections at the moment, please try again later".as_bytes(),
                        )
                        .unwrap();
                    continue;
                }
                let current_time = Instant::now();
                if let Some(last_request_time) = client_requests.get(&stream) {
                    if current_time.duration_since(*last_request_time) < request_timeout {
                        stream
                            .write("Too many requests, please try again later".as_bytes())
                            .unwrap();
                        continue;
                    }
                }
                let client_request = client_requests.get(&stream).unwrap();
                if !client_request.is_valid() {
                    stream
                        .write("Invalid request, please try again".as_bytes())
                        .unwrap();
                    continue;
                }
                connection_count += 1;
                client_requests.insert(stream.into().try_clone().unwrap(), current_time);
                thread::spawn(move || {
                    handle_client(stream);
                    connection_count -= 1;
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}