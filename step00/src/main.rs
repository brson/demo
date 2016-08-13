extern crate hyper;

use std::io::Read;
use hyper::client::Client;

fn main() {
    let client = Client::new();

    let mut response =
        client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS")
              .send().unwrap();

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
