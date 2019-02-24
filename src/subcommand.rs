use super::esa_config;
use std::io;
use std::io::Write;

pub fn init() {
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
