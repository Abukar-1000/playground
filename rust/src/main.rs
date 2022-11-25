extern crate reqwest;
extern crate easy_http_request;
extern crate autopilot;
use std::collections::HashMap;
use easy_http_request::DefaultHttpRequest;
use std::process::Command;
extern crate sysinfo;
use sysinfo::{ Pid, ProcessExt, System, SystemExt };
use regex::RegexSet;
use std::time::Duration;
use  serde::{Deserialize};
use std::{thread};
use random_number::random;


#[derive(Deserialize, Debug)]
struct jsonRes {
    vitality: String,
    response: String
}

async fn makeCall(url:String) -> Result<jsonRes, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send()
        .await?
        .json::<jsonRes>()
        .await?;

    Ok(body)
}

fn trollMove(timeout: i32) {
    let mut x:i32 = random!();
    let mut y:i32 = random!();
    x %= 1920;
    y %= 1080;

    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }
    autopilot::mouse::move_to(autopilot::geometry::Point::new(
        x as f64,
        y as f64
    )).expect("Unable to move mouse");

    thread::sleep(Duration::from_millis(timeout as u64 * 1000));
}

#[tokio::main]
async fn main(){

    let mut test:jsonRes = jsonRes { vitality: (String::from("Alive")), response: (String::from("Firing with V cylinders")) };
    
    let precessesToTerminate = RegexSet::new(&[
        r"Spotify",
        r"Discord"
    ]).unwrap();
    let mut targetIds: Vec<i32> = vec![];
    // processes 
    let mut system = System::new_all();
    let mut running: bool = true;

    let url:String = String::from("https://stormy-jumper.cyclic.app/");

    let mut running:bool = true;
    // let twoSecs = time::Duration::from_millis(2000);
    while (running) {
        let response = makeCall(url.clone()).await;
        match response {
            Ok(res) => {
                if res.vitality == String::from("Alive") {
                    trollMove(2);
                }
                else {
                    running = false;
                }
            },
            Err(e) => {
                print!("could not get results for request");
            }
        }
    }
    
}