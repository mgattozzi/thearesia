//! Thearesia a bot to maintain GitHub Repos and Organizations
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate futures;
extern crate github_rs;
extern crate hyper;
extern crate hyper_rustls;
extern crate tokio_core;
extern crate serde;

mod airtable;
mod error;
mod json;
mod sync;

use airtable::AirtableClient;
use dotenv::dotenv;
use error::*;
use github_rs::client::Github;
use sync::*;

use std::env;

lazy_static! {
    static ref GITHUB_KEY: String = {
        dotenv().ok();
        env::var("GITHUB_KEY").expect("Expected GITHUB_KEY in the .env file")
    };
    static ref AIRTABLE_KEY: String = {
        dotenv().ok();
        env::var("AIRTABLE_KEY").expect("Expected AIRTABLE_KEY in the .env file")
    };
    static ref ASSIGNED_ISSUES_URL: String = {
        dotenv().ok();
        env::var("ASSIGNED_ISSUES_URL").expect("Expected ASSIGNED_ISSUES_URL in the .env file")
    };
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<()> {
    let gclient = Github::new(GITHUB_KEY.to_string())?;
    let mut aclient = AirtableClient::new()?;
    sync_new_assigned_issues(&gclient, &mut aclient)?;
    Ok(())
}

