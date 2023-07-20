use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Serialize, Deserialize};
use spinners::{Spinner,Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIChoices{
    text: String,
    index: u8,
    longprobs: Option<u8>,
    finish_reason: String
}
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<OAIChoices>
}

#[derive(Serialize, Debug)]
struct OAIRequest{
    prompt: String,
    max_tokens: u32
}
fn main() {}