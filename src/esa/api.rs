#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatedBy {
    name: String,
    screen_name: String,
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
    name: String,
    tags: Vec<String>,
    category: String,
    pub full_name: String,
    pub wip: bool,
    body_md: String,
    body_html: String,
    created_at: String,
    updated_at: String,
    message: String,
    pub url: String,
    revision_number: u32,
    pub created_by: CreatedBy,
    updated_by: UpdatedBy,
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
