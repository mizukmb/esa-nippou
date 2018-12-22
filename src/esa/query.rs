pub struct Query {
    string: String,
}

impl Query {
    pub fn new() -> Query {
        Query {
            string: "".to_string(),
        }
    }

    pub fn wip(&self, wip: bool) -> Query {
        Query {
            string: format!("{self} wip:{wip}", self = &self.string, wip = wip),
        }
    }

    pub fn user(&self, screen_name: String) -> Query {
        Query {
            string: format!(
                "{self} user:{screen_name}",
                self = &self.string,
                screen_name = screen_name
            ),
        }
    }

    pub fn updated_gt(&self, updated: String) -> Query {
        Query {
            string: format!(
                "{self} updated:>{updated}",
                self = &self.string,
                updated = updated
            ),
        }
    }

    pub fn updated_lt(&self, updated: String) -> Query {
        Query {
            string: format!(
                "{self} updated:<{updated}",
                self = &self.string,
                updated = updated
            ),
        }
    }

    pub fn not_in(&self, keyword: String) -> Query {
        Query {
            string: format!(
                "{self} -in:{keyword}",
                self = &self.string,
                keyword = keyword
            ),
        }
    }

    pub fn to_string(self) -> String {
        self.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_query_wip() {
        let wip = true;
        let subject = Query::new().wip(wip).to_string();

        assert_eq!(subject, " wip:true");
    }

    #[test]
    fn build_query_user() {
        let screen_name = "mizukmb".to_string();
        let subject = Query::new().user(screen_name).to_string();

        assert_eq!(subject, " user:mizukmb");
    }

    #[test]
    fn build_query_updated_gt() {
        let updated = "2018-08-26".to_string();
        let subject = Query::new().updated_gt(updated).to_string();

        assert_eq!(subject, " updated:>2018-08-26");
    }

    #[test]
    fn build_query_updated_lt() {
        let updated = "2018-08-26".to_string();
        let subject = Query::new().updated_lt(updated).to_string();

        assert_eq!(subject, " updated:<2018-08-26");
    }

    #[test]
    fn build_query_not_in() {
        let keyword = "category".to_string();
        let subject = Query::new().not_in(keyword).to_string();

        assert_eq!(subject, " -in:category");
    }

    #[test]
    fn build_query_chain() {
        let wip = true;
        let screen_name = "mizukmb".to_string();
        let updated_gt = "2018-08-22".to_string();
        let updated_lt = "2018-08-26".to_string();
        let subject = Query::new()
            .wip(wip)
            .user(screen_name)
            .updated_gt(updated_gt)
            .updated_lt(updated_lt)
            .to_string();

        assert_eq!(
            subject,
            " wip:true user:mizukmb updated:>2018-08-22 updated:<2018-08-26"
        );
    }
}
