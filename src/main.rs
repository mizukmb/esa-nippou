extern crate clap;
use clap::{App};

fn run() {
    println!("{}", "Hello, esa-nippou!");
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
