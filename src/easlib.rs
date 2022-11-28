#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::str;

use data_encoding::{BASE64, HEXLOWER};
use reqwest::{Client, StatusCode};
use reqwest::multipart::Form;
use ring::digest::{Context, Digest, SHA256};
use serde_json::{Error, json};
use tokio::fs::File as Tokio_File;
use tokio::io::{AsyncReadExt};

use crate::models::{Credentials, Token, Ticket};
use crate::models::{ErrorResponse, EasError, SerdeError, ReqWestError, EasResult};
use crate::models::{EasDocument, EasArchiveInfo, EasMetaData};
use crate::utils::LOCATIONS;

// EasResponse is Error Result from authenticate, post document, get documents, download documents
// delete documents, get documents metadata, update document metadata, get content of archive

#[derive(Clone)]
pub struct EasAPI {
    credentials: Credentials,
    token: Option<Token>,
    digest: Option<String>,
    ticket: Option<Ticket>,
    error_response: Option<ErrorResponse>,
    doc_list: Option<Vec<String>>,
}

impl EasAPI {
    pub fn new(credentials: Credentials) -> Self {
        EasAPI { credentials, token: None, digest: None, ticket: None, error_response: None, doc_list: None }
    }
    pub fn set_credentials(&mut self, credentials: Credentials) {
        self.credentials = credentials;
    }
    pub fn set_token(&mut self, token: String) {
        self.token = Some(Token::new(token));
    }
    pub fn set_doc_list(&mut self, doc_list: Vec<String>) { self.doc_list = Some(doc_list); }
    pub fn show(&self) {
        println!("Summary\n---------------------");
        println!("credentials: {:?}", self.credentials);
        match &self.token {
            Some(t) => println!("token: {:?}", t),
            _ => println!("token: None")
        };
        match &self.digest {
            Some(d) => println!("digest: {:?}", d),
            _ => println!("digest: None")
        };
        match &self.ticket {
            Some(t) => println!("ticket: {:?}", t),
            _ => println!("ticket: None")
        };
        match &self.error_response {
            Some(e) => println!("error_response: {:?}", e),
            _ => println!("error_response: None")
        };
        match &self.doc_list {
            Some(dl) => {
                for d in dl { println!("doc: {:?}", d) }
            }
            _ => println!("doc_list: None")
        };
        println!("---------------------");
    }
    pub fn get_token_string(&self) -> &String {
        self.token.as_ref().unwrap().get_token()
    }
    pub fn get_ticket_string(&self) -> &String {
        self.ticket.as_ref().unwrap().get_ticket()
    }
    pub fn get_token(&self) -> &Option<Token> {
        match &self.token {
            Some(_) => &self.token,
            _ => &None,
        }
    }
    pub fn set_digest(&mut self, digest: String) {
        self.digest = Some(digest)
    }
    pub fn get_digest(&self) -> &Option<String> {
        match &self.digest {
            Some(_) => &self.digest,
            _ => &None,
        }
    }
    pub fn failure_info(&self, sc: StatusCode, body: &str) -> EasResult {
        return match sc {
            StatusCode::BAD_REQUEST => EasResult::ReqWestError(ReqWestError::new("Bad Request")),
            _ => {
                let er: Result<ErrorResponse, Error> = serde_json::from_str(body);
                let a_er: EasResult = match er {
                    Ok(res) => {
                        EasResult::EasError(EasError::new(res.to_string().as_str()))
                    }
                    Err(e) => {
                        EasResult::SerdeError(SerdeError::new(e.to_string().as_str()))
                    }
                };
                a_er
            }
        };
    }
    pub async fn eas_get_token(&mut self, display: bool) -> Result<EasResult, reqwest::Error> {
        let request_url = "https://apprec.cecurity.com/eas.integrator.api/service/authenticate";
        if display { println!("Start get token"); }
        let cred_value = serde_json::to_value(&self.credentials).unwrap();
        let response = Client::new()
            .post(request_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&cred_value)
            .send().await?;
        if display { println!("wait for answer"); }
        let sc = response.status();
        if display {
            let headers = response.headers();
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        }
        if display { println!("Decoding body"); }
        let body = response.text().await.unwrap();
        if !sc.is_success() {
            println!("Request failed => {}", sc);
            return Ok(EasResult::ReqWestError(ReqWestError::new(body.as_str())));
        }

        if display {
            println!("Status : {:#?}\n{:#?}", sc, body);
        }

        // deserialize to token

        let r: Result<Token, Error> = serde_json::from_str(&body);
        let t: EasResult = match r {
            Ok(res) => {
                self.token = Some(res);
                EasResult::ApiOk
            }
            Err(_e) => EasResult::Unknown
        };
        if display { println!("stop get token"); }
        Ok(t)
    }
    pub async fn file_as_part(&self, address: i32, mime_type: &str) -> Result<reqwest::multipart::Part, Box<dyn std::error::Error>> {
        println!("start unlock LOCATIONS");

        let my_ref1 = LOCATIONS.lock().unwrap();
        println!("Unlock LOCATIONS is OK");
        let address = my_ref1.get(&address);
        let fname = match address {
            Some(f) => {
                f
            }
            _ => {
                "/Users/bruno/dvlpt/rust/devdur-1.pdf"
            }
        };
        drop(my_ref1);
        let mut async_buffer = Vec::new();
        let path = Path::new(fname);
        let mut file = Tokio_File::open(path).await?;
        let _fcl = file.read_to_end(&mut async_buffer).await?;
        unsafe {
            let file_content = str::from_utf8_unchecked(&async_buffer).to_string();
            let file_part = reqwest::multipart::Part::text(file_content)
                .file_name(path.file_name().unwrap().to_string_lossy())
                .mime_str(mime_type).unwrap();
            Ok(file_part)
        }
    }
    pub async fn eas_post_document(&mut self, address: i32, display: bool) -> Result<EasResult, Box<dyn std::error::Error>> {
        let request_url = "https://apprec.cecurity.com/eas.integrator.api/eas/documents";
        if display { println!("Start post document"); }
        let auth_bearer = format!("Bearer {}", self.get_token_string());

        // compute digest of file 1
        let my_ref1 = LOCATIONS.lock().unwrap();
        let address_str = my_ref1.get(&address);
        let fname = match address_str {
            Some(f) => {
                if display { println!("ok nice use f == {}", f); }
                f
            }

            _ => {
                println!("ko use default value");
                "/Users/bruno/dvlpt/rust/devdur-1.pdf"
            }
        };
        drop(my_ref1);
        let (digest_string, status) = compute_digest(fname);
        if !status { return Ok(EasResult::EasError(EasError::new(digest_string.as_str()))); }
        let file1_name = Path::new(fname).file_name().unwrap().to_str().unwrap();

        self.set_digest(digest_string.clone());
        // build part for first file
        let file_part_async = self.file_as_part(address, "application/octet-stream").await?;
        if display {
            println!("SHA256 Digest for {} is {}", fname, self.digest.as_ref().unwrap().clone());
        }

        // compute digest of file 2
        let fname2 = "/users/bruno/dvlpt/rust/devdur-2.pdf";
        let (digest_string2, status2) = compute_digest(fname2);
        if !status2 { return Ok(EasResult::EasError(EasError::new(digest_string2.as_str()))); }
        if display {
            println!("SHA256 Digest for /users/bruno/dvlpt/rust/devdur-2.pdf is {}", digest_string2);
        }
        // build part for second file
        let mut sync_buffer = Vec::new();
        let path1 = Path::new("/users/bruno/dvlpt/rust/devdur-2.pdf");
        let file2_name = path1.file_name().unwrap().to_str().unwrap();
        let mut file1 = File::open(path1).unwrap();
        let _fcl = file1.read_to_end(&mut sync_buffer);
        let meta = json!([
            {"name": "ClientId", "value": "987654319"},
            {"name": "CustomerId", "value": "CLIENT-BRI2"},
            {"name": "Documenttype", "value": "Validated invoice"}
        ]);
        let upload_file_fingerprint = json!([
            {"fileName": file1_name, "fingerPrint" : digest_string.clone().to_lowercase(),"fingerPrintAlgorithm": "SHA-256"},
            {"fileName": file2_name, "fingerPrint" : digest_string2.clone().to_uppercase(),"fingerPrintAlgorithm" : "SHA-256"}
        ]);
        unsafe {
            let file_content = str::from_utf8_unchecked(&sync_buffer).to_string();
            let file_part_sync2 = reqwest::multipart::Part::text(file_content)
                .file_name(path1.file_name().unwrap().to_string_lossy())
                .mime_str("application/octet-stream").unwrap();

            // TODO Add additional file with metadata inside
            // was  fname instead of archive.txt


            let form = Form::new()
                .part("document", file_part_async)
                .part("document", file_part_sync2)
                .text("metadata", meta.to_string())
                .text("fingerPrints", upload_file_fingerprint.to_string());

            let response = Client::new()
                .post(request_url)
                .header("Authorization", auth_bearer)
                .header("Accept", "application/json")
                .multipart(form)
                .send()
                .await?;
            let sc = response.status();
            if display {
                let headers = response.headers();
                for (key, value) in headers.iter() {
                    println!("{:?}: {:?}", key, value);
                }
            }
            let body = response.text().await.unwrap();
            if !sc.is_success() {
                println!("Request failed => {} {}", sc, &body);
                return Ok(self.failure_info(sc, &body));
            }
            if display { println!("Status : {:#?}\n{:#?}", sc, body); }
            // Extract ticket
            let r: Result<Ticket, Error> = serde_json::from_str(&body);
            let eas_r: EasResult = match r {
                Ok(res) => {
                    self.ticket = Some(res);
                    if display { println!("Body contains ticket"); }
                    EasResult::ApiOk
                }
                Err(e) => {
                    if display {
                        println!("Unable to deserialize body => ticket\nError {}", e);
                    };
                    EasResult::SerdeError(SerdeError::new(e.to_string().as_str()))
                }
            };
            if display { println!("Stop post document"); }
            Ok(eas_r)
        }
    }
    pub async fn eas_get_content_list(&mut self, display: bool) -> Result<EasResult, reqwest::Error> {
        let request_url = format!("https://apprec.cecurity.com/eas.integrator.api/eas/documents/{}/contentList", self.get_ticket_string());
        if display { println!("Start get content list"); }
        let auth_bearer = format!("Bearer {}", self.get_token_string());

        let response = Client::new()
            .get(request_url)
            .header("Accept", "application/json")
            .header("Authorization", auth_bearer)
            .send().await?;
        let sc = response.status();
        if display {
            let headers = response.headers();
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        }
        let body = response.text().await.unwrap();
        if !sc.is_success() {
            println!("Request failed => {} {}", sc, &body);
            return Ok(self.failure_info(sc, &body));
        }
        if display {
            println!("Status : {:#?}\n{:#?}", sc, body);
        }
        let r: Result<Vec<String>, Error> = serde_json::from_str(&body);
        let mut doc_list: Vec<String> = Vec::new();
        let eas_r: EasResult = match r {
            Ok(res) => {
                println!("Found {} documents in contentList", res.len());
                for st in &res {
                    println!("Found {}", st);
                    doc_list.push(st.clone());
                }
                // TODO Save content list of documents in archive
                self.set_doc_list(doc_list);
                EasResult::ApiOk
            }
            Err(e) => EasResult::SerdeError(SerdeError::new(e.to_string().as_str()))
        };
        Ok(eas_r)
    }
    pub async fn eas_delete_archive(&self, ticket: &str, motivation: &str, display: bool) -> Result<EasResult, reqwest::Error> {
        // was self.get_ticket_string()
        let request_url = format!("{}/{}", "https://apprec.cecurity.com/eas.integrator.api/eas/documents", ticket);
        if display { println!("Start delete archive"); }
        let auth_bearer = format!("Bearer {}", self.get_token_string());

        let response = Client::new()
            .delete(request_url)
            .header("Accept", "application/json")
            .header("Authorization", auth_bearer)
            .query(&[("motivation", motivation)])
            .send().await?;
        let sc = response.status();
        if display {
            let headers = response.headers();
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        }
        let body = response.text().await.unwrap();
        if !sc.is_success() {
            println!("Request failed => {}", sc);
            return Ok(self.failure_info(sc, &body));
        }
        if display {
            println!("Status : {:#?}\n{:#?}", sc, body);
        }
        Ok(EasResult::ApiOk)
    }
    pub async fn eas_get_archive(&self, display: bool) -> Result<EasResult, reqwest::Error> {
        let request_url = format!("{}/{}", "https://apprec.cecurity.com/eas.integrator.api/eas/documents", self.get_ticket_string());
        if display { println!("Start get archive"); }
        let auth_bearer = format!("Bearer {}", self.get_token_string());

        let response = Client::new()
            .get(request_url)
            .header("Accept", "application/json")
            .header("Authorization", auth_bearer)
            .send().await?;
        let sc = response.status();
        if display {
            let headers = response.headers();
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        }
        let body = response.text().await.unwrap();
        if !sc.is_success() {
            println!("Request failed => {}", sc);
            return Ok(self.failure_info(sc, &body));
        }
        if display {
            println!("Status : {:#?}\n{:#?}", sc, body);
        }
        // deserialize doc from b64
        let r: Result<EasDocument, Error> = serde_json::from_str(&body);
        let (eas_r, status): (EasResult, bool) = match r {
            Ok(res) => (EasResult::EasDocument(res), true),
            Err(e) => (EasResult::SerdeError(SerdeError::new(e.to_string().as_str())), false)
        };
        if !status {
            println!("ERRRRRor");
            return Ok(eas_r);
        }
        // Transform base64 => [u8] and save
        if let EasResult::EasDocument(res) = &eas_r {
            let mime_type = &res.get_mime_type();
            let b64_document = &res.get_base64_document();
            let document = BASE64.decode(b64_document.as_bytes()).unwrap();
            let document_length = document.len();
            //let final_document = String::from_utf8(document).unwrap();
            if display { println!("Document (type:{}, length:{})", mime_type, document_length); }
            let mut file = File::create("/Users/bruno/my_arch.zip").unwrap();
            // Write a slice of bytes to the file
            let final_result = match file.write_all(document.as_slice()) {
                Ok(_r1) => true,
                Err(_e) => false
            };
            if final_result {
                if display { println!("stop get document"); }
                // Build result with info (length) from API result
                return Ok(EasResult::EasArchiveInfo(EasArchiveInfo::new((*mime_type).to_string(), document_length)));
            }
        }
        Ok(EasResult::EasError(EasError::new("Unable to save archive")))
    }
    pub async fn eas_get_document_metadata(&self, display: bool) -> Result<EasResult, reqwest::Error> {
        let request_url = format!("{}/{}/metadata", "https://apprec.cecurity.com/eas.integrator.api/eas/documents", self.get_ticket_string());
        if display { println!("Start retrieve document metadata"); }
        let auth_bearer = format!("Bearer {}", self.get_token_string());

        let response = Client::new()
            .get(request_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", auth_bearer)
            .send().await?;
        let sc = response.status();
        if display {
            let headers = response.headers();
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        }
        let body = response.text().await.unwrap();
        if !sc.is_success() {
            println!("Request failed => {}", sc);
            return Ok(self.failure_info(sc, &body));
        }
        if display {
            println!("Status : {:#?}\n{:#?}", sc, body);
        }
        // deserialize json to metadata

        let r: Result<EasMetaData, Error> = serde_json::from_str(&body);
        let eas_m: EasResult = match r {
            Ok(res) => {
                if display { println!("Deserializing OK."); }
                EasResult::EasMetaData(res)
            }
            Err(e) => {
                println!("Error while deserializing: {}", e);
                EasResult::Unknown
            }
        };
        if display { println!("MetaData: {:#?}", eas_m); }
        if display { println!("stop retrieve document metadata"); }
        Ok(eas_m)
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        if let Ok(count) = reader.read(&mut buffer) {
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }
    }
    Ok(context.finish())
}

fn compute_digest(path: &str) -> (String, bool) {
    let digest_string: String;
    if let Ok(input_file) = File::open(path) {
        let reader = BufReader::new(input_file);
        if let Ok(digest) = sha256_digest(reader) {
            digest_string = HEXLOWER.encode(digest.as_ref());
        } else {
            println!("Error while digest computation");
            return ("Digest Computation Error".to_string(), false);
        }
    } else {
        println!("Error opening file {}", path);
        return (format!("Error opening file {}", path), false);
    }
    (digest_string, true)
}

