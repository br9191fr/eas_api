#![allow(dead_code)]

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    appId: String,
    appToken: String,
    accountName: String,
}
impl Credentials {
    pub fn new(id: String, token: String, name: String) -> Self {
        Credentials {
            appId: id,
            appToken: token,
            accountName: name,
        }
    }
}
impl std::fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "credentials: {}\n{}\n{}",
                 self.appId,
                 self.appToken,
                 self.accountName)
    }
}
#[derive(Deserialize, Debug)]
pub struct Token {
    token: String,
}
impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }
    pub fn get_token(&self) -> &String {
        let string = &self.token;
        string
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "token: {}", self.token)
    }
}
#[derive(Deserialize, Debug)]
pub struct Ticket {
    ticket: String,
}
impl Ticket {
    pub fn new(ticket: String) -> Self {
        Ticket { ticket }
    }
    pub fn get_ticket(&self) -> &String {
        let string = &self.ticket;
        string
    }
}
impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ticket: {}", self.ticket)
    }
}