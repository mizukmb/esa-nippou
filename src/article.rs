use chrono::prelude::{DateTime, Local};

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

    pub fn to_scrapbox_link(&self) -> String {
        let members_url = format!("https://{team}.esa.io/members", team = self.team);
        let mut sb = format!(" [{title} {url}]", title = self.title, url = self.url);

        if self.created_at == self.updated_at {
            sb = format!(
                "{sb} created by [{created_by} {members_url}/{created_by}]",
                sb = &sb,
                created_by = self.created_by,
                members_url = members_url
            );
        } else {
            sb = format!(
                "{sb} updated by [{updated_by} {members_url}/{updated_by}]",
                sb = &sb,
                updated_by = self.updated_by,
                members_url = members_url
            );
        }

        if self.wip {
            sb = format!("{sb} [* WIP]", sb = &sb);
        }

        sb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::{TimeZone};

    #[test]
    fn build_to_markdown_link_when_creted_at_and_updated_at_are_same() {
        let article = Article::new((
            String::from("test-title"),
            String::from("test-url"),
            String::from("test-team"),
            String::from("test-created-by-user"),
            String::from("test-updated-by-user"),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            false,
        ));
        let expected_str = "- [test-title](test-url) created by :@test-created-by-user: [test-created-by-user](https://test-team.esa.io/members/test-created-by-user)";

        assert_eq!(article.to_markdown_link(), String::from(expected_str));
    }

    #[test]
    fn build_to_markdown_link_when_creted_at_and_updated_at_are_difirrent() {
        let article = Article::new((
            String::from("test-title"),
            String::from("test-url"),
            String::from("test-team"),
            String::from("test-created-by-user"),
            String::from("test-updated-by-user"),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            Local
                .datetime_from_str("2020/05/04 13:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            false,
        ));
        let expected_str = "- [test-title](test-url) updated by :@test-updated-by-user: [test-updated-by-user](https://test-team.esa.io/members/test-updated-by-user)";

        assert_eq!(article.to_markdown_link(), String::from(expected_str));
    }

    #[test]
    fn build_to_scrapbox_link_when_creted_at_and_updated_at_are_same() {
        let article = Article::new((
            String::from("test-title"),
            String::from("test-url"),
            String::from("test-team"),
            String::from("test-created-by-user"),
            String::from("test-updated-by-user"),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            false,
        ));
        let expected_str = " [test-title test-url] created by [test-created-by-user https://test-team.esa.io/members/test-created-by-user]";

        assert_eq!(article.to_scrapbox_link(), String::from(expected_str));
    }

    #[test]
    fn build_to_scrapbox_link_when_creted_at_and_updated_at_are_difirrent() {
        let article = Article::new((
            String::from("test-title"),
            String::from("test-url"),
            String::from("test-team"),
            String::from("test-created-by-user"),
            String::from("test-updated-by-user"),
            Local
                .datetime_from_str("2020/05/04 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            Local
                .datetime_from_str("2020/05/04 13:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            false,
        ));
        let expected_str = " [test-title test-url] updated by [test-updated-by-user https://test-team.esa.io/members/test-updated-by-user]";

        assert_eq!(article.to_scrapbox_link(), String::from(expected_str));
    }
}
