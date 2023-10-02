
use std::fs::File;
use std::error::Error;

use serde_derive::Deserialize;
use serde_json::from_str;

use std::{thread, time};
use std::io::{self, Write, Read};

use std::collections::HashMap;

use reqwest::Client;

use reqwest::Method;
use clap::Parser;

use reqwest::header::CONTENT_TYPE;


#[derive(Parser)]
struct Card {
    //// The ID of the card
    id: String
}


#[derive(Debug, Deserialize)]
struct Credentials {
    user: String,
    token: String,
    url: String
}


async fn add_worklog(credentials: Credentials, seconds: String, issue_id: String){


    println!("\rSending to {} seconds {}", &issue_id, &seconds);

    let mut body = HashMap::new();

    body.insert("comment", "");
    body.insert("timeSpentSeconds", &seconds);

    let client = Client::new();

    let url: String = format!("{}/rest/api/2/issue/{}/worklog", credentials.url, issue_id);

    let request_builder = client
        .request(Method::POST, &url)
        .basic_auth(credentials.user, Some(credentials.token))
        .header(CONTENT_TYPE, "application/json")
        .json(&body);

    let request = request_builder.build().unwrap();

    println!("Request Method: {:?}", request.method());
    println!("Request URL: {:?}", request.url());
    for (name, value) in request.headers().iter() {
        println!("Header: {:?}: {:?}", name, value);
    }

    println!("Request Body: {:?}", request.body());

    let response = client.execute(request).await.unwrap();
    let status = response.status();
    let response_body = response.text().await.unwrap();
    

    println!("Response Status {:?}\nBody: {:?}", status, response_body);

}


#[tokio::main]
async fn main() {

    let credentials: Result<Credentials, Box<dyn Error>> = async {
        let home_dir = dirs::home_dir().expect("Not found");
        let file_path = home_dir.join("credentials.json");
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let credentials: Credentials = from_str(&contents)?;

        Ok(credentials)
    }
    .await;
    

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let args = Card::parse();

    let mut seconds = 0;

    while running.load(std::sync::atomic::Ordering::SeqCst) {    
        print!("\rCard ID: {} Time: {} seconds", args.id, seconds);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
        seconds += 1;
    }
    
    match credentials {
        Ok(credentials) => {
            add_worklog(credentials, seconds.to_string(), args.id).await;
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

}
