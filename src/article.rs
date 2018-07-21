pub struct Article {
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

    pub fn to_markdown_link(&self) -> String {
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
