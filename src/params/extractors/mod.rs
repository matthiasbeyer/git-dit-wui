pub mod issue {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueIdExtractor {
        pub id: String,
    }

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueListFilterExtractor {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter: Option<IssueFilter>,
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

pub mod update {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct UpdateFlagExtractor {
        pub update: Option<bool>,
    }
}
