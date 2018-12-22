use chrono::prelude::DateTime;
use chrono::prelude::Local;

pub struct Article {
    title: String,
    url: String,
    team: String,
    created_by: String,
    updated_by: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    wip: bool,
}

impl Article {
    pub fn new(
        p: (
            String,
            String,
            String,
            String,
            String,
            chrono::DateTime<chrono::Local>,
            chrono::DateTime<chrono::Local>,
            bool,
        ),
    ) -> Article {
        Article {
            title: p.0,
            url: p.1,
            team: p.2,
            created_by: p.3,
            updated_by: p.4,
            created_at: p.5,
            updated_at: p.6,
            wip: p.7,
        }
    }

    pub fn to_markdown_link(&self) -> String {
        let members_url = format!("https://{team}.esa.io/members", team = self.team);
        let mut md = format!("- [{title}]({url})", title = self.title, url = self.url);

        if self.created_at == self.updated_at {
            md = format!(
                "{md} created by :@{created_by}: [{created_by}]({members_url}/{created_by})",
                md = &md,
                created_by = self.created_by,
                members_url = members_url
            );
        } else {
            md = format!(
                "{md} updated by :@{updated_by}: [{updated_by}]({members_url}/{updated_by})",
                md = &md,
                updated_by = self.updated_by,
                members_url = members_url
            );
        }

        if self.wip {
            md = format!("{md} **WIP**", md = &md);
        }

        md
    }
}
