extern crate clap;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use clap::{App};
use reqwest::header::{Authorization, Bearer};
use chrono::prelude::*;
use serde_json::Value;

struct Article {
    title: String,
    url: String,
    screen_name: String
}

impl Article {
    pub fn new(p:(String, String, String)) -> Article {
        Article{title: p.0, url: p.1, screen_name: p.2}
    }

    fn to_markdown_link(&self) -> String {
        format!("- [{title}]({url}) by @{screen_name}", title=self.title, url=self.url, screen_name=self.screen_name)
    }
}

fn extract(value: &Value) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for i in value["posts"].as_array().unwrap() {
        // `XXX.as_str().unwrap().to_string()` convert from JSON to String without `""`
        // see: https://github.com/serde-rs/json/issues/367
        articles.push( Article::new(( i["full_name"].as_str().unwrap().to_string(), i["url"].as_str().unwrap().to_string(), i["created_by"]["screen_name"].as_str().unwrap().to_string() )) );
    }

    articles
}

fn run() {
    let base_url = "https://api.esa.io";
    let api_version = "v1";
    let team = env!("ESA_NIPPOU_TEAMS");
    let posts_url = format!("{base_url}/{api_version}/teams/{team}/posts", base_url=base_url, api_version=api_version, team=team);

    let access_token = env!("ESA_NIPPOU_ACCESS_TOKEN");
    let today = Local::now().format("%Y-%m-%d");
    let created = today.to_string();
    let query = format!("created:>{created}", created=created);

    let client = reqwest::Client::new();
    let mut res = client.get(&posts_url)
        .header(
            Authorization(
                Bearer{
                    token: access_token.to_string()
                }
            )
        )
        .query(&[("q", "2018-01-01")])
        .send()
        .unwrap();

    let value: Value = serde_json::from_str(&res.text().unwrap()).unwrap();
    let articles = extract(&value);

    for article in articles {
        println!("{}", article.to_markdown_link());
    }
}

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let about = "Print today's your esa.io articles";
    App::new(name)
        .version(version)
        .author(author)
        .about(about)
        .get_matches();

    run();
}
