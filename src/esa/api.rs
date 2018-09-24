use chrono::prelude::DateTime;
use chrono::prelude::Local;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatedBy {
    name: String,
    pub screen_name: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedBy {
    name: String,
    pub screen_name: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    number: u32,
    pub name: String,
    tags: Vec<String>,
    category: String,
    pub full_name: String,
    pub wip: bool,
    body_md: String,
    body_html: String,
    #[serde(with = "esa_api_date_format")]
    pub created_at: DateTime<Local>,
    #[serde(with = "esa_api_date_format")]
    pub updated_at: DateTime<Local>,
    message: String,
    pub url: String,
    revision_number: u32,
    pub created_by: CreatedBy,
    pub updated_by: UpdatedBy,
    kind: String,
    comments_count: u8,
    tasks_count: u8,
    done_tasks_count: u8,
    stargazers_count: u8,
    watchers_count: u8,
    star: bool,
    watch: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Posts {
    pub posts: Vec<Post>,
    prev_page: Option<u8>, // accept u8 | null
    next_page: Option<u8>, // accept u8 | null
    total_count: u8,
    page: u8,
    per_page: u8,
    max_per_page: u8,
}

mod esa_api_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{Deserialize, Deserializer, Serializer};

    // Format documents ( https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html#specifiers )
    const FORMAT: &str = "%+";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(::serde::de::Error::custom)
    }
}
