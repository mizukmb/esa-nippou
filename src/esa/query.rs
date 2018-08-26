pub struct Query {
    string: String
}

impl Query {
    pub fn new() -> Query {
        Query{
            string: "".to_string()
        }
    }

    pub fn wip(&self, wip: bool) -> Query {
        Query{
            string: format!("{self} wip:{wip}", self=&self.string, wip=wip)
        }
    }

    pub fn user(&self, screen_name: String) -> Query {
        Query{
            string: format!("{self} user:{screen_name}", self=&self.string, screen_name=screen_name)
        }
    }

    pub fn updated(&self, updated: String) -> Query {
        Query{
            string: format!("{self} updated:{updated}", self=&self.string, updated=updated)
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
    fn build_query_updated() {
        let updated = "2018-08-26".to_string();
        let subject = Query::new().updated(updated).to_string();

        assert_eq!(subject, " updated:2018-08-26");
    }

    #[test]
    fn build_query_chain() {
        let wip = true;
        let screen_name = "mizukmb".to_string();
        let updated = "2018-08-26".to_string();
        let subject = Query::new().wip(wip).user(screen_name).updated(updated).to_string();    

        assert_eq!(subject, " wip:true user:mizukmb updated:2018-08-26");
    }
}
