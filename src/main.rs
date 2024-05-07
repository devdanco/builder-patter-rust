#![allow(unused)]
use prelude::*;
use web::RequestBuilder;
mod error;
mod prelude;
mod web;

fn main() -> Result<()> {
    let request = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("Content-Type", "application/json")
        .build()?;
    println!("{:#?}", request);

    Ok(())
}
