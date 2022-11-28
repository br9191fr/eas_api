extern crate easlib;

use std::env;
//use std::thread;
//use std::thread::yield_now;
use easlib::easlib::{EasAPI};
use easlib::bri_cred::{get_credentials};
use easlib::utils::{build_static_locations};
use easlib::models::{get_result_status};

async fn eas_process(address: i32, display: bool, delete_finally: bool) -> Result<bool, reqwest::Error> {
    //let credentials = Credentials::new("xxxxx".to_owned(),"tttt".to_owned(),"myAccount".to_owned());
    let credentials_ok = get_credentials();
    let mut api = EasAPI::new(credentials_ok);

    if display { println!("Step authenticate"); }
    // authenticate and get token
    let opt_t = api.eas_get_token(display).await;
    if display { println!("Step get status"); }
    let (eas_r, status) = get_result_status(opt_t);
    if !status {
        println!("Failed to get token. End eas process !");
        return Ok(false);
    }
    if display { println!("token found {}", api.get_token_string()); }
    eas_r.show("Get Token");
    println!("Start upload now !");
    // upload document now
    let opt_at = api.eas_post_document(
        address,
        display).await;
    let (eas_r, status) = get_result_status(opt_at);
    if !status {
        println!("Failed to get archive ticket. End eas process !");
        return Ok(false);
    }
    eas_r.show("Upload Doc");
    let opt_cl = api.eas_get_content_list(false).await;
    let (eas_r, status) = get_result_status(opt_cl);

    if !status {
        println!("Failed to get content list. End eas process !");
        return Ok(false);
    }
    eas_r.show("Content list");

    let opt_ar = api.eas_get_archive(false).await;
    let (eas_r, status) = get_result_status(opt_ar);

    if !status {
        println!("Failed to get full archive. End eas process !");
        return Ok(false);
    }
    eas_r.show("Archive Info");
    api.show();
    // TODO play with metadata with /eas/documents/{ticket}/metadata
    let opt_dm = api.eas_get_document_metadata(display).await;
    let (eas_r, status) = get_result_status(opt_dm);

    if !status {
        println!("Failed to get document metadata. End eas process !");
        return Ok(false);
    }
    eas_r.show("MetaData");
    if delete_finally {
        println!("Try to delete archive {}", api.get_ticket_string().clone());
        // delete document now
        let opt_da = api.eas_delete_archive(
            api.get_ticket_string(),
            "IdecideToDelete",
            display).await;
        let (eas_r, status) = get_result_status(opt_da);
        if !status {
            println!("Failed to delete archive. End eas process !");
            return Ok(false);
        }
        eas_r.show("Delete Archive");
    }

    // TODO Upload in threads
    /*
    let num_threads = 10;
    let mut thread_list = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let mut api_clone = api.clone();
        thread_list.push(tokio::spawn(async move  {
            //post_document(api_clone, address, display);
            let opt_at = api_clone.eas_post_document(
                address,
                false).await;
            let (eas_r, status) = get_result_status(opt_at);
            if !status {
                println!("Failed to get archive ticket. End eas process !");
            } else {
                eas_r.show("Upload Doc");
            }
            yield_now();
        }));
    }

    for jh in thread_list {
        println!("{:?}", jh.join().unwrap());
    }
    */
    // TODO download individual file with GET to /eas/documents/{ticket}/fileName
    // TODO filename in requestBody (schema downloadItemRequest)

    // TODO use get/post/patch http commands

    // TODO get matching documents
    //api.show();
    Ok(true)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("Missing arguments\nUsage: pgm file_to_archive");
        return;
    }
    let file_to_archive = &args[1];

    let address = build_static_locations(1, file_to_archive);
    let test = true;
    let delete_finally = false;
    if test {
        let final_result = eas_process(
            address, true,delete_finally ).await;
        match final_result {
            Ok(true) => println!("eas test is ok"),
            Ok(false) => println!("eas test failed"),
            Err(e) => println!("Reqwest error {:#}", e)
        }
    } else {
        println!("infos file: {}\n, address: {}",
                 file_to_archive, address);
    }

    println!("end");
}