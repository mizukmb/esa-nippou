#[macro_use]
extern crate clap;
extern crate chrono;
extern crate reqwest;
extern crate rpassword;
extern crate serde_json;
extern crate serde_yaml;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::header::{Authorization, Bearer};
use serde_json::Value;
use std::env;
use std::io;
use std::io::Write;

mod esa_config;

struct Article {
    title: String,
    url: String,
    screen_name: String,
    wip: bool,
}

impl Article {
    pub fn new(p: (String, String, String, bool)) -> Article {
        Article {
            title: p.0,
            url: p.1,
            screen_name: p.2,
            wip: p.3,
        }
    }

    fn to_markdown_link(&self) -> String {
        let mut md: String;

        md = format!(
            "- [{title}]({url}) by @{screen_name}",
            title = self.title,
            url = self.url,
            screen_name = self.screen_name
        );

        if self.wip {
            md = format!("{md} **WIP**", md = &md);
        }

        md
    }
}

fn extract(value: &Value) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for i in value["posts"].as_array().unwrap() {
        // `XXX.as_str().unwrap().to_string()` convert from JSON to String without `""`
        // see: https://github.com/serde-rs/json/issues/367
        articles.push(Article::new((
            i["full_name"].as_str().unwrap().to_string(),
            i["url"].as_str().unwrap().to_string(),
            i["created_by"]["screen_name"].as_str().unwrap().to_string(),
            i["wip"].as_bool().unwrap(),
        )));
    }

    articles
}

fn build_query(created: String, username: String, wip: bool) -> String {
    format!(
        "created:>{created} user:{username} wip:{wip}",
        created = created,
        username = username,
        wip = wip
    )
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

    let today = Local::now().format("%Y-%m-%d");
    let created = today.to_string();
    let username = match env::var("ESA_NIPPOU_SCREEN_NAME") {
        Ok(val) => val,
        Err(_e) => config.screen_name,
    };
    let wip = value_t_or_exit!(app.value_of("wip"), bool);
    let query = build_query(created, username, wip);

    let posts_client = reqwest::Client::new();
    let mut posts_res = posts_client
        .get(&posts_url)
        .header(Authorization(Bearer {
            token: access_token.to_string(),
        }))
        .query(&[("q", query)])
        .send()
        .unwrap();

    let posts_value: Value = serde_json::from_str(&posts_res.text().unwrap()).unwrap();
    let articles = extract(&posts_value);

    println!("### {team}.esa.io", team = team);
    println!(""); // for new line

    for article in articles {
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
                .help("Print with WIP article (true|false)")
                .takes_value(true)
                .default_value("true"),
        )
        .subcommand(SubCommand::with_name("init").about("Initialize configurations interactively"))
        .get_matches();

    match app.subcommand_name() {
        Some("init") => init(),
        _ => run(app),
    }
}
