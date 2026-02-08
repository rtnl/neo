use crate::cli::{Cli, CliCommand};
use crate::executor::Executor;
use crate::request::Request;
use clap::Parser;
use owo_colors::OwoColorize;
use std::env;
use std::io::Result;
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

pub struct Program {
    executor: Arc<Executor>,
}

impl Program {
    pub async fn new() -> Arc<Self> {
        Arc::new(Self {
            executor: Executor::new().await,
        })
    }

    pub async fn launch(self: Arc<Self>) {
        let cli = Cli::parse();

        match cli.get_command().unwrap_or(CliCommand::Interactive) {
            CliCommand::Interactive => {
                self.launch_interactive().await;
            }
            CliCommand::Run { .. } => {}
            CliCommand::Version => {
                self.launch_version().await;
            }
        }
    }

    async fn launch_run(self: Arc<Self>) {}

    async fn launch_version(self: Arc<Self>) {
        eprintln!("version->#{}", &GIT_HASH.unwrap_or("???????")[0..7]);
    }

    async fn launch_interactive(self: Arc<Self>) {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);

        loop {
            self.prompt().await;

            let mut input = String::new();
            reader.read_line(&mut input).await.unwrap();
            if input.is_empty() {
                break;
            }

            let input = input.trim_end();

            let request = match self.parse_request(input).await {
                Ok(value) => value,
                Err(err) => {
                    println!("error parsing input: {}", err);
                    continue;
                }
            };

            let request = match request {
                Some(value) => value,
                None => continue,
            };

            self.executor.handle_request(request).await;
        }
    }

    async fn prompt(&self) {
        let mut writer = io::BufWriter::new(io::stderr());

        let user = whoami::username().unwrap_or("?".to_string());
        let host = whoami::hostname().unwrap_or("localhost".to_string());
        let path = format!("{}", env::current_dir().unwrap().display());

        let text = format!(
            "{} {} [{}@{}:{}]\n{} ",
            "###".bright_black(),
            "neo".green().bold(),
            user.purple(),
            host.bright_purple(),
            path.yellow(),
            "$".bright_black()
        );

        writer.write_all(text.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }

    async fn parse_request(&self, input: &str) -> Result<Option<Request>> {
        let parts: Vec<String> = input
            .split_ascii_whitespace()
            .map(ToString::to_string)
            .collect();

        let key = match parts.first() {
            Some(v) => v.to_string(),
            None => return Ok(None),
        };

        let args: Vec<String> = parts[1..].iter().map(ToString::to_string).collect();

        Ok(Some(Request::new(key, args)))
    }
}
