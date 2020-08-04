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
