#[macro_use]
extern crate clap;
extern crate chrono;
extern crate reqwest;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;

mod api;
mod article;
mod client;
mod esa_config;
mod query;
mod subcommand;

fn extract(value: api::Posts, team: String) -> Vec<article::Article> {
    let mut articles: Vec<article::Article> = Vec::new();

    for i in value.posts {
        articles.push(article::Article::new((
            i.full_name,
            i.url,
            team.clone(),
            i.created_by.screen_name,
            i.updated_by.screen_name,
            i.created_at,
            i.updated_at,
            i.wip,
        )));
    }

    articles
}

fn run(app: ArgMatches) {
    let config = esa_config::load().unwrap();
    let team = if app.is_present("team") {
        app.value_of("team").unwrap().to_string()
    } else {
        match env::var("ESA_NIPPOU_TEAM") {
            Ok(val) => val,
            Err(_e) => config.team,
        }
    };

    let access_token = match env::var("ESA_NIPPOU_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_e) => config.parsonal_access_token,
    };

    let since_date = if app.is_present("since_date") {
        app.value_of("since_date").unwrap().to_string()
    } else {
        Local::now().format("%Y-%m-%d").to_string()
    };
    let screen_name = match env::var("ESA_NIPPOU_SCREEN_NAME") {
        Ok(val) => val,
        Err(_e) => config.screen_name,
    };

    let mut query = query::Query::new().user(screen_name).updated_gt(since_date);

    query = if app.is_present("until_date") {
        query.updated_lt(app.value_of("until_date").unwrap().to_string())
    } else {
        query
    };

    query = if app.is_present("wip") {
        query.wip(value_t_or_exit!(app.value_of("wip"), bool))
    } else {
        query
    };

    query = if app.is_present("not_in") {
        query.not_in(app.value_of("not_in").unwrap().to_string())
    } else {
        query
    };

    let esa_client = client::Client::new(access_token);
    let posts_value =
        serde_json::from_str(&esa_client.get_posts(team.clone(), query).unwrap()).unwrap();

    let updated_articles = extract(posts_value, team.clone());

    let format = match app.value_of("format") {
        Some(v) => v,
        None => "markdown",
    };

    match format {
        "scrapbox" => {
            if !updated_articles.is_empty() {
                println!("[*** {team}.esa.io]", team = team);

                for article in updated_articles {
                    println!("{}", article.to_scrapbox_link());
                }
            }
        }
        "markdown" | _ => {
            if !updated_articles.is_empty() {
                println!("### {team}.esa.io", team = team);
                println!(""); // for new line

                for article in updated_articles {
                    println!("{}", article.to_markdown_link());
                }
            }
        }
    }
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
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Select format type: markdown (default), scrapbox")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("wip")
                .short("w")
                .long("wip")
                .value_name("BOOL")
                .help("Print with WIP article only (true|false)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("since_date")
                .short("s")
                .long("since-date")
                .value_name("DATE")
                .help("Retrieves esa.io articles since the date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("until_date")
                .short("u")
                .long("until-date")
                .value_name("DATE")
                .help("Retrieves esa.io articles until the date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("not_in")
                .long("not-in")
                .value_name("KEYWORD")
                .help("Exclude to get not forward match keyword in category")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("team")
                .short("t")
                .long("team")
                .value_name("TEAM")
                .help("Team name")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("init").about("Initialize configurations interactively"))
        .get_matches();

    match app.subcommand_name() {
        Some("init") => subcommand::init(),
        _ => run(app),
    }
}
