//! Setup Client for interacting with GitHub
use github_rs::github::*;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// Make this more robust
pub fn gen_client() -> Client {
    let file = File::open("token").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    // error handle better
    let _ = reader.read_line(&mut buffer);
    Client::new(String::from(buffer.trim())).unwrap()
}

pub fn gen_collaborator_client() -> Client {
    let file = File::open("token").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    // error handle better
    let _ = reader.read_line(&mut buffer);
    Client::new_custom(String::from(buffer.trim()), vec![ MediaType::Collaborators, MediaType::Json], "thearesia").unwrap()
}

pub fn gen_review_client() -> Client {
    let file = File::open("token").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    // error handle better
    let _ = reader.read_line(&mut buffer);
    Client::new_custom(String::from(buffer.trim()), vec![MediaType::Reviews, MediaType::Json], "thearesia").unwrap()
}
