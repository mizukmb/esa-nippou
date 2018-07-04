extern crate clap;
extern crate chrono;
extern crate reqwest;

use clap::{App};
use reqwest::header::{Authorization, Bearer};
use chrono::prelude::*;

fn run() {
    let base_url = "https://api.esa.io/v1";
    let access_token = env!("ESA_NIPPOU_ACCESS_TOKEN");
    let team = env!("ESA_NIPPOU_TEAMS");
    let path = format!("teams/{team}/posts", team=team);
    let url = format!("{base_url}/{path}", base_url=base_url, path=path);
    let today = Local::now().format("%Y-%m-%d");
    let created = today.to_string();
    let query = format!("created:>{created}", created=created);

    let client = reqwest::Client::new();
    let mut res = client.get(&url)
        .header(
            Authorization(
                Bearer{
                    token: access_token.to_string()
                }
            )
        )
        .query(&[("q", &query)])
        .send()
        .unwrap();
    println!("{}", res.text().unwrap());

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
