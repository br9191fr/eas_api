#![allow(dead_code)]

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Credentials {
    #[serde(rename(serialize = "appId"))]
    app_id: String,
    #[serde(rename(serialize = "appToken"))]
    app_token: String,
    #[serde(rename(serialize = "accountName"))]
    account_name: String,
}

impl Credentials {
    pub fn new(id: String, token: String, name: String) -> Self {
        Credentials {
            app_id: id,
            app_token: token,
            account_name: name,
        }
    }
}

impl std::fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "credentials: {}\n{}\n{}",
                 self.app_id,
                 self.app_token,
                 self.account_name)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }
    pub fn get_token(&self) -> &String {
        &self.token
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "token: {}", self.token)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Ticket {
    #[serde(rename = "archiveTicket")]
    archive_ticket: String,
}

impl Ticket {
    pub fn new(ticket: String) -> Self {
        Ticket { archive_ticket: ticket }
    }
    pub fn get_ticket(&self) -> &String {
        &self.archive_ticket
    }
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ticket: {}", self.archive_ticket)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "errorCode")]
    error_code: String,
    #[serde(rename = "errorMessage")]
    error_message: String,
    status: String,
}

impl ErrorResponse {
    fn get_error_code(&self) -> &String {
        &self.error_code
    }
    fn get_error_message(&self) -> &String {
        &self.error_message
    }
    fn get_status(&self) -> &String {
        &self.status
    }
    fn new(error_code: String, error_message: String, status: String) -> Self {
        ErrorResponse { error_code, error_message, status }
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error Response: {} {} {} ", self.error_code, self.error_message, self.status)
    }
}

#[derive(Deserialize, Debug)]
pub struct EasError {
    message: String,
}

impl EasError {
    pub fn new(message: &str) -> Self { EasError { message: String::from(message) } }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl std::fmt::Display for EasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "message: {}", self.message)
    }
}

#[derive(Deserialize, Debug)]
pub struct SerdeError {
    message: String,
}

impl SerdeError {
    pub fn new(message: &str) -> Self { SerdeError { message: String::from(message) } }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct ReqWestError {
    message: String,
}

impl ReqWestError {
    pub fn new(message: &str) -> Self { ReqWestError { message: String::from(message) } }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Deserialize, Debug)]
pub enum EasResult {
    Token(Token),
    Ticket(Ticket),
    ErrorResponse(ErrorResponse),
    EasDocument(EasDocument),
    EasArchiveInfo(EasArchiveInfo),
    EasMetaData(EasMetaData),
    EasError(EasError),
    SerdeError(SerdeError),
    ReqWestError(ReqWestError),
    ApiOk,
    Unknown,
}

impl EasResult {
    fn get_ticket(&self) -> Option<&String> {
        if let EasResult::Ticket(at) = self {
            Some(at.get_ticket())
        } else {
            None
        }
    }
    fn get_token(&self) -> Option<&String> {
        if let EasResult::Token(t) = self {
            Some(t.get_token())
        } else {
            None
        }
    }
    pub fn show(&self, msg: &str) {
        match self {
            EasResult::Token(t) => println!("[{}] Token: {}", msg, t),
            EasResult::Ticket(at) => println!("[{}] Ticket: {}", msg, at),
            EasResult::EasDocument(d) => println!("[{}] Document: {}", msg, d),
            EasResult::EasArchiveInfo(ai) => println!("[{}] Archive Info: {}", msg, ai),
            EasResult::EasMetaData(m) => println!("[{}] MetaData: {}", msg, m),
            EasResult::EasError(m) => println!("Eas Error: {}", m.message),
            EasResult::SerdeError(m) => println!("Serde Error: {}", m.message),
            EasResult::ReqWestError(m) => println!("ReqWest Error: {}", m.message),
            EasResult::ApiOk => println!("[{}] API Called OK", msg),
            _ => println!("[{}] Unknown or Not implemented", msg)
        }
    }
}

pub fn get_result_status<T>(opt_t: Result<EasResult, T>) -> (EasResult, bool) {
    let (eas_r, status) = match opt_t {
        Ok(EasResult::ApiOk) => { (EasResult::ApiOk, true) }
        Ok(EasResult::Token(t)) => {(EasResult::Token(t), true)}
        Ok(EasResult::Ticket(a)) => {(EasResult::Ticket(a), true)}
        Ok(EasResult::EasDocument(d)) => {(EasResult::EasDocument(d), true)}
        Ok(EasResult::EasArchiveInfo(i)) => {(EasResult::EasArchiveInfo(i), true)}
        Ok(EasResult::EasMetaData(m)) => {(EasResult::EasMetaData(m), true)}
        Ok(EasResult::EasError(eas)) => {(EasResult::EasError(eas), false)}
        Ok(EasResult::SerdeError(s)) => {(EasResult::SerdeError(s), false)}
        Ok(EasResult::ReqWestError(r)) => {(EasResult::ReqWestError(r), false)}
        _ => {(EasResult::Unknown, false)
        }
    };
    (eas_r, status)
}

#[derive(Deserialize, Debug)]
pub struct EasInfo {
    token: String,
    filename: String,
    address: String,
    digest: String,
}

impl EasInfo {
    fn new(token: String, filename: String, address: String, digest: String) -> Self {
        EasInfo {
            token,
            filename,
            address,
            digest,
        }
    }
}

#[derive(Deserialize, Debug)]
struct EasNVPair {
    name: String,
    value: String,
}

impl std::fmt::Display for EasNVPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name: {}, value: {}", self.name, self.value)
    }
}

#[derive(Deserialize, Debug)]
pub struct EasMetaData {
    metadata: Vec<EasNVPair>,
}

impl std::fmt::Display for EasMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = self.metadata.iter().fold(String::new(), |acc, arg|
            acc + arg.name.as_str() + "->" + arg.value.as_str() + ", ");
        writeln!(f, "[{}]", res)
    }
}

#[derive(Deserialize, Debug)]
pub struct EasDocument {
    #[serde(rename = "mimeType")]
    mime_type: String,
    #[serde(rename = "base64Document")]
    base64_document: String,
}

impl EasDocument {
    fn new(mime_type: String, base64_document: String) -> Self {
        EasDocument { mime_type, base64_document,
        }
    }
    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }
    pub fn get_base64_document(&self) -> String {
        self.base64_document.clone()
    }
}

impl std::fmt::Display for EasDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "mimetype: {:#?}, data:{:#?}", self.mime_type, self.base64_document)
    }
}

#[derive(Deserialize, Debug)]
pub struct EasArchiveInfo {
    mime_type: String,
    length: usize,
}

impl EasArchiveInfo {
    pub fn new(mime_type: String, length: usize) -> Self {
        EasArchiveInfo {
            mime_type,
            length,
        }
    }
}

impl std::fmt::Display for EasArchiveInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "mime_type:{:#?}, length:{:#?}", self.mime_type, self.length)
    }
}
