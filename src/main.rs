use rand::Rng;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

const DIFFICULTY: u32 = 4;
const MAX_NONCE: u64 = 1_000_000;
const MAX_REQUESTS_PER_IP: u32 = 100;
const BAN_TIME: u64 = 60;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let mut banned_ips: HashMap<String, u64> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                let ip = addr.ip().to_string();
                let ban_end = banned_ips.get(&ip);

                if ban_end.is_some() && ban_end.unwrap() > &Instant::now().elapsed().as_secs() {
                    println!(
                        "Client {} is banned for {} sec",
                        ip,
                        ban_end.unwrap() - Instant::now().elapsed().as_secs()
                    );
                } else if !handle_rate_limit(ip.clone(), &mut banned_ips) {
                    println!(
                        "Client {} has exceeded the maximum number of requests per IP",
                        ip
                    );
                } else if !handle_pow(ip.clone()) {
                    println!("Client {} failed to complete PoW challenge", ip.clone());
                } else {
                    handle_client(stream);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_rate_limit(ip: String, banned_ips: &mut HashMap<String, u64>) -> bool {
    let ban_end = banned_ips.get(&ip);

    if ban_end.is_some() {
        return ban_end.unwrap() < &Instant::now().elapsed().as_secs();
    }

    let mut requests = 0;

    if !banned_ips.contains_key(&ip) {
        banned_ips.insert(ip.clone(), 0);
    } else {
        requests = *banned_ips.get(&ip).unwrap();
    }

    if requests >= MAX_REQUESTS_PER_IP.into() {
        banned_ips.insert(ip, Instant::now().elapsed().as_secs() + BAN_TIME);
        return false;
    } else {
        banned_ips.insert(ip, requests + 1);
        return true;
    }
}

fn handle_pow(ip: String) -> bool {
    let nonce = rand::random::<u64>() % MAX_NONCE;
    let challenge = format!("{}{}", ip, nonce);
    let digest = Sha256::digest(challenge.as_bytes());
    let digest_str = format!("{:x}", digest);
    for i in 0..DIFFICULTY {
        if digest_str.chars().nth(i as usize) != Some('0') {
            return false;
        }
    }
    return true;
}

fn handle_client(mut stream: TcpStream) {
    let wisdom_book = "../wisdom_book.txt";
    let wisdom = generate_wisdom(wisdom_book);
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => println!("Received: {}", String::from_utf8_lossy(&buffer)),
        Err(e) => println!("Error reading stream: {}", e),
    }

    match stream.write(wisdom.as_bytes()) {
        Ok(_) => println!("Sent: {}", wisdom),
        Err(e) => println!("Error writing to stream: {}", e),
    }
}

fn generate_wisdom(file_path: &str) -> String {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open wisdom book: {}", e);
            return "Error: Failed to open wisdom book.".to_string();
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if lines.is_empty() {
        return "Error: Wisdom book is empty.".to_string();
    } else {
        let random_index = rand::thread_rng().gen_range(0..lines.len());
        return lines[random_index].to_string();
    }
}
