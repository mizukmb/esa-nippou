use chrono::prelude::DateTime;
use chrono::prelude::Local;

pub struct Article {
    title: String,
    url: String,
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
            DateTime<Local>,
            DateTime<Local>,
            bool,
        ),
    ) -> Article {
        Article {
            title: p.0,
            url: p.1,
            created_by: p.2,
            updated_by: p.3,
            created_at: p.4,
            updated_at: p.5,
            wip: p.6,
        }
    }

    pub fn to_markdown_link(&self) -> String {
        let mut md = format!("- [{title}]({url})", title = self.title, url = self.url);

        if self.created_at == self.updated_at {
            md = format!(
                "{md} created by @{created_by}",
                md = &md,
                created_by = self.created_by
            );
        } else {
            md = format!(
                "{md} updated by @{updated_by}",
                md = &md,
                updated_by = self.updated_by
            );
        }

        if self.wip {
            md = format!("{md} **WIP**", md = &md);
        }

        md
    }
}
