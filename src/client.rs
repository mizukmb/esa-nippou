#[cfg(test)]
extern crate mockito;

use super::query::Query;
use reqwest::Client as rc;

#[derive(Debug, PartialEq)]
pub struct Client {
    access_token: String,
}

impl Client {
    pub fn new(access_token: String) -> Client {
        Client {
            access_token: access_token,
        }
    }

    #[allow(unused_variables)]
    pub fn get_posts(&self, team: String, query: Query) -> Result<String, String> {
        #[cfg(test)]
        let posts_url = String::from(mockito::server_url());
        #[cfg(not(test))]
        let posts_url = format!("https://api.esa.io/v1/teams/{}/posts", team);

        let posts = rc::new()
            .get(&posts_url)
            .bearer_auth(&self.access_token)
            .query(&[("q", query.to_string())])
            .send();

        Ok(posts
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn build_new() {
        let access_token = String::from("dummy");
        let subject = Client::new(access_token);

        assert_eq!(
            subject,
            Client {
                access_token: String::from("dummy")
            }
        );
    }

    #[test]
    fn get_posts() {
        let access_token = String::from("dummy");
        let client = Client::new(access_token);
        let query = Query::new();

        mock("GET", "/posts").create();

        let subject = client.get_posts(String::from("dummy_team"), query);

        assert_eq!(subject.unwrap(), "");
    }
}
