extern crate hyper;


use std::env;
use std::sync::mpsc;
use std::time::Duration;
use std::io::Read;
use hyper::client::{Client, Request, Response};

//This is expected to fail to compile.

fn main() {
        let client = Client::new();

        let res = client.get("https://raw.githubusercontent.com/brson/demo/master/wishlist.html").send().unwrap();

        println!("Response: {}", res.body);
    }
}
