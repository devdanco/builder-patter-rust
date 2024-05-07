#![allow(unused)]
use web::RequestBuilder;
use prelude::*;
mod web;
mod error;
mod prelude;

fn main() -> Result<()> {



    let request = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("Content-Type", "application/json")
        .build()?;
    println!("{:#?}", request);

    Ok(())
}
