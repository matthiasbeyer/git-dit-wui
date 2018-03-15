pub mod issue {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueIdExtractor {
        pub id: String,
    }
}

pub mod message {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct MessageIdExtractor {
        id: String,
    }
}
