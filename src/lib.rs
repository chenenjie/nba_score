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
use domain::{Competition, AnnounceBoard};


#[derive(Debug, Fail)]
enum ScoreError{
    #[fail(display = "score tranlate error: {}", name)]
    ScoreTranslateError{
        name: String,
    }
}

// {"id":"106886","sdate":"2018-01-15","time":"10:00","url":"\/zhibo\/nba\/2018\/0115senlinlangvskaituozhe.htm","type":"basketball","start":"2018-01-15 10:00","home_team":"\u68ee\u6797\u72fc","visit_team":"\u5f00\u62d3\u8005","home_score":"35","visit_score":"35","team1_scores":["31","4","0","0","0","0","0","0"],"team2_scores":["33","2","0","0","0","0","0","0"],"period_cn":"\u7b2c2\u8282 8:59","from":"dc.live","code":"202","update":"10:46:36","full_timeouts_1":"6","full_timeouts_2":"6","short_timeouts_1":"0","short_timeouts_2":"0","team_fouls_1":"","team_fouls_2":"","big_score_1":"","big_score_2":""}

// https://bifen4pc2.qiumibao.com/json/2018-01-15/106886.htm

pub fn run() -> Result<(), Error>{
    let url = "http://bifen4m.qiumibao.com/json/list.htm".parse().unwrap();
    let json_str = send_request(url)?;
    let board :AnnounceBoard = serde_json::from_str(&json_str)?;

    let nbagames: Vec<Competition> = board.list.into_iter().filter(|e| e.competition_type == "basketball" && e.url.contains("nba")).collect();
    println!("nba games {:?}", nbagames);
    Ok(())
}


fn send_request(url: Url) -> Result<String, Error> {

    let mut headers = Headers::new();

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
