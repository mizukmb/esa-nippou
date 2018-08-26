#[macro_use]
extern crate clap;
extern crate chrono;
extern crate reqwest;
extern crate rpassword;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::header::{Authorization, Bearer};
use std::env;
use std::io;
use std::io::Write;

mod article;
mod esa;
mod esa_config;

fn extract(value: esa::api::Posts) -> Vec<article::Article> {
    let mut articles: Vec<article::Article> = Vec::new();

    for i in value.posts {
        // `XXX.as_str().unwrap().to_string()` convert from JSON to String without `""`
        // see: https://github.com/serde-rs/json/issues/367
        articles.push(article::Article::new((
            i.full_name,
            i.url,
            i.created_by.screen_name,
            i.wip,
        )));
    }

    articles
}

fn build_query_updated(date: String, screen_name: String) -> String {
    esa::query::Query::new()
        .user(screen_name)
        .updated(date)
        .to_string()
}

fn build_query_updated_with_wip(date: String, screen_name: String, wip: bool) -> String {
    esa::query::Query::new()
        .wip(wip)
        .user(screen_name)
        .updated(date)
        .to_string()
}

fn post(url: &String, query: &String, access_token: &String) -> Vec<article::Article> {
    let posts_client = reqwest::Client::new();
    let mut posts_res = posts_client
        .get(url)
        .header(Authorization(Bearer {
            token: access_token.to_string(),
        }))
        .query(&[("q", query)])
        .send()
        .unwrap();

    let posts_value: esa::api::Posts = serde_json::from_str(&posts_res.text().unwrap()).unwrap();

    extract(posts_value)
}

fn run(app: ArgMatches) {
    let config = esa_config::load();
    let base_url = "https://api.esa.io";
    let api_version = "v1";
    let team = match env::var("ESA_NIPPOU_TEAM") {
        Ok(val) => val,
        Err(_e) => config.team,
    };
    let posts_url = format!(
        "{base_url}/{api_version}/teams/{team}/posts",
        base_url = base_url,
        api_version = api_version,
        team = team
    );

    let access_token = match env::var("ESA_NIPPOU_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_e) => config.parsonal_access_token,
    };

    let today = Local::now().format("%Y-%m-%d").to_string();
    let username = match env::var("ESA_NIPPOU_SCREEN_NAME") {
        Ok(val) => val,
        Err(_e) => config.screen_name,
    };
    let query_for_updated = if app.is_present("wip") {
        let wip = value_t_or_exit!(app.value_of("wip"), bool);
        build_query_updated_with_wip(today, username, wip)
    } else {
        build_query_updated(today, username)
    };
    let updated_articles = post(&posts_url, &query_for_updated, &access_token);

    println!("### {team}.esa.io", team = team);
    println!(""); // for new line

    for article in updated_articles {
        println!("{}", article.to_markdown_link());
    }
}

fn init() {
    let mut team = String::new();
    let mut screen_name = String::new();

    let parsonal_access_token =
        rpassword::prompt_password_stdout("Personal access token (hidden): ").unwrap();
    print!("Team: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut team).unwrap();
    print!("Screen name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut screen_name).unwrap();

    let config = esa_config::new((
        team.trim().to_string(),
        screen_name.trim().to_string(),
        parsonal_access_token.trim().to_string(),
    ));

    config.write();
}

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let about = "Print today's your esa.io articles";
    let app = App::new(name)
        .version(version)
        .author(author)
        .about(about)
        .arg(
            Arg::with_name("wip")
                .short("w")
                .long("wip")
                .value_name("BOOL")
                .help("Print with WIP article only (true|false)")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("init").about("Initialize configurations interactively"))
        .get_matches();

    match app.subcommand_name() {
        Some("init") => init(),
        _ => run(app),
    }
}
