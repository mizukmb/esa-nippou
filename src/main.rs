extern crate clap;
extern crate reqwest;

use clap::{App};
use reqwest::header::{Authorization, Bearer};

fn run() {
    let base_url = "https://api.esa.io/v1";
    let access_token = "XXXXXXXXXXXXXXXXXXXXXXX";
    let team = "XXXXXXXXXXXX";
    let path = format!("teams/{team}/posts", team=team);
    let url = format!("{base_url}/{path}", base_url=base_url, path=path);

    let client = reqwest::Client::new();
    let mut res = client.get(&url)
        .header(
            Authorization(
                Bearer{
                    token: access_token.to_string()
                }
            )
        )
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
