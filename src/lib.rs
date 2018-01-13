#[macro_use] extern crate failure;
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod domain;

use failure::Error;
use reqwest::{Method, Url};
use reqwest::header::Headers;
use domain::AnnounceBoard;


#[derive(Debug, Fail)]
enum ScoreError{
    #[fail(display = "score tranlate error: {}", name)]
    ScoreTranslateError{
        name: String,
    }
}

pub fn run() -> Result<(), Error>{
    let url = "http://bifen4m.qiumibao.com/json/list.htm".parse().unwrap();
    let json_str = send_request(url)?;
    println!("{}", json_str);
    let board :AnnounceBoard = serde_json::from_str(&json_str)?;
    println!("board {:?}", board);
    Ok(())
}

fn send_request(url: Url) -> Result<String, Error> {

    let mut headers = Headers::new();
    // headers.set_raw("Content-Type", "text/html;charset=UTF-8");

    let client = reqwest::Client::new();
    let mut resp = client.request(Method::Get, url).headers(headers).send()?;
    Ok(resp.text()?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
