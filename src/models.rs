pub mod request {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Publish {
        pub script: Script,
    }

    #[derive(Serialize)]
    pub struct Script {
        pub contents: String,
        pub project: Project,
    }

    #[derive(Serialize)]
    pub struct Project {
        pub files: Vec<File>,
    }

    #[derive(Serialize)]
    pub struct File {
        pub path: String,
        pub content: String,
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Publish {
        pub id: String,
        pub bot_id: String,
        pub status: i64,
        pub name: String,
        pub config: String,
        pub revision: i64,
        pub workbench_url: String,
        pub script: Script,
        pub guild: Guild,
        pub errors: Vec<String>,
    }

    #[derive(Deserialize)]
    pub struct Script {
        pub id: String,
        pub project: String,
    }

    #[derive(Deserialize)]
    pub struct Guild {
        pub id: String,
        pub name: String,
        pub icon: String,
    }

    #[derive(Deserialize)]
    pub struct Error {
        pub msg: String,
    }

    #[derive(Deserialize)]
    pub struct RegistryEntry {
        pub name: String,
        #[serde(rename = "dist-tags")]
        pub dist_tags: DistTags,
    }

    #[derive(Deserialize)]
    pub struct DistTags {
        pub latest: String,
    }
}
