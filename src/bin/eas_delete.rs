extern crate easlib;

use std::env;
use easlib::easlib::{EasAPI};
use easlib::bri_cred::{get_credentials};

use easlib::models::{get_result_status};

async fn eas_process(ticket: &str, motivation: &str, display: bool) -> Result<bool, reqwest::Error> {
    //let credentials = Credentials::new("xxxxx".to_owned(),"tttt".to_owned(),"myAccount".to_owned());
    let credentials_ok = get_credentials();
    let mut api = EasAPI::new(credentials_ok);

    if display { println!("Step authenticate"); }
    // authenticate and get token
    let opt_t = api.eas_get_token(false).await;
    if display { println!("Step get status"); }
    let (eas_r, status) = get_result_status(opt_t);
    if !status {
        println!("Failed to get token. End eas process !");
        return Ok(false);
    }
    if display { println!("token found {}", api.get_token_string()); }
    eas_r.show("Get Token");

    // delete document now
    let opt_da = api.eas_delete_archive(
        ticket,
        motivation,
        display).await;
    let (eas_r, status) = get_result_status(opt_da);
    if !status {
        println!("Failed to delete archive. End eas process !");
        return Ok(false);
    }
    eas_r.show("Delete Archive");

    return Ok(true);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Missing arguments\nUsage: pgm ticket_to_delete");
        return;
    }

    let final_result = eas_process(
        args[1].as_str(), "IChoseToDelete",false).await;
    match final_result {
        Ok(true) => println!("eas delete is ok"),
        Ok(false) => println!("eas delete failed"),
        Err(e) => println!("Reqwest error {:#}", e)
    }

    println!("end");
}