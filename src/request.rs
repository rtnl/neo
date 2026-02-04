#[derive(Clone, Debug)]
pub struct Request {
    key: String,
    args: Vec<String>,
}

impl Request {
    pub fn new<S: Into<String>>(key: S, args: Vec<S>) -> Self {
        Self {
            key: key.into(),
            args: args.into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}
