/*
ParsedJson
├── data: Data
│   └── post: Post
│       ├── interests: Vec<String>
│       └── images: Images
│           ├── image460sv: Option<Image460sv>?
│           │   └── url: String
│           └── image700: Image700
│               └── url: String
└── config: Config
    └── interests: HashMap<String, Interest>
        └── [key]: Interest
            └── name: String
*/

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct BrokenJson {
    pub data: Data,
    pub config: Config,
}

// "data" root key
#[derive(Deserialize, Debug)]
pub struct Data {
    pub post: Post,
}

// "data"->"post" key
#[derive(Deserialize, Debug)]
pub struct Post {
    pub interests: Vec<String>,
    pub images: Images,
    pub polling: Option<Polling>,
}

#[derive(Deserialize, Debug)]
pub struct Polling {
    pub metrics: PollingMetrics,
}

#[derive(Deserialize, Debug)]
pub struct PollingMetrics {
    pub options: Vec<Options>,
    pub counts: Vec<u64>,
    pub total_votes: u64,
    pub ends_at: u64,
    pub is_active: bool,
}

#[derive(Deserialize, Debug)]
pub struct Options {
    pub display: String,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub image460sv: Option<Image460sv>,
    pub image700: Image700,
}

#[derive(Deserialize, Debug)]
pub struct Image460sv {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Image700 {
    pub url: String,
}

// "config" root key
#[derive(Deserialize, Debug)]
pub struct Config {
    pub interests: HashMap<String, Interest>,
}

#[derive(Deserialize, Debug)]
pub struct Interest {
    pub name: String,
}
