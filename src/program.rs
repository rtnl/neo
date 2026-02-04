use crate::executor::Executor;
use crate::mapping::{Mapping, MappingDefault};
use crate::operation::Operation;
use crate::request::Request;
use owo_colors::OwoColorize;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

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

            let operation = match self.get_operation_from_request(&request).await {
                Ok(value) => value,
                Err(err) => {
                    println!("error finding operation: {}", err);
                    continue;
                }
            };

            self.executor.push_operation(operation).await;
        }
    }

    async fn prompt(&self) {
        let stdout = io::stdout();
        let mut writer = io::BufWriter::new(stdout);

        let text = format!("{} {} ", "neo".green().bold(), "|".bright_black());

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

    async fn get_operation_from_request(&self, request: &Request) -> Result<Operation> {
        for operation in Operation::iter() {
            let mapping = MappingDefault {};

            if mapping
                .get_operation_aliases(operation)
                .iter()
                .any(|it| it == request.get_key())
            {
                return Ok(operation);
            }
        }

        Err(Error::new(ErrorKind::NotFound, "operation not found"))
    }
}
