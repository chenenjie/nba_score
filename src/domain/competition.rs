use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceBoard{
    pub code: String,
    pub second: String,
    pub list: Vec<Competition>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Competition{
    pub id : String,
    pub sdate: String,
    pub time: String,
    pub url: String,
    #[serde(rename = "type")]
    pub competition_type: String,
    pub start: String,
    pub home_team: String,
    pub visit_team: String,
    pub home_score: Value,
    pub visit_score: Value,
    pub period_cn: String,
    pub from: String,
    pub code: String,
    pub update: String,
    pub big_score_1: Option<String>,
    pub big_score_2: Option<String>,
}


