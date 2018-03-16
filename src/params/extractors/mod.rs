pub mod issue {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueIdExtractor {
        pub id: String,
    }

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueListFilterExtractor {
        pub filter: IssueFilter,
    }

    #[derive(Deserialize, Clone, Copy, Debug)]
    #[serde(rename_all = "kebab-case")]
    pub enum IssueFilter {
        Open,
        Closed,
        All,
    }
}

pub mod message {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct MessageIdExtractor {
        pub id: String,
    }
}
