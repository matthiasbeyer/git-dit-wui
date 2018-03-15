pub mod issue {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct IssueIdExtractor {
        id: String,
    }
}

pub mod message {
    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct MessageIdExtractor {
        id: String,
    }
}
