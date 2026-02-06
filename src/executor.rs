use crate::mapping::{Mapping, MappingDefault};
use crate::operation::Operation;
use crate::request::Request;
use crate::response::{Response, ResponseStatus};
use owo_colors::OwoColorize;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::process::Stdio;
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::process::Command;
use tokio::sync::Mutex;

pub struct Executor {
    mapping: Arc<Mutex<dyn Mapping + Send + Sync>>,
}

impl Executor {
    pub(crate) async fn new() -> Arc<Self> {
        Arc::new(Self {
            mapping: Arc::new(Mutex::new(MappingDefault {})),
        })
    }

    async fn get_operation_from_request(&self, request: &Request) -> Result<Operation> {
        for operation in Operation::iter() {
            if self
                .mapping
                .lock()
                .await
                .get_operation_aliases(operation)
                .iter()
                .any(|it| it == request.get_key())
            {
                return Ok(operation);
            }
        }

        Err(Error::new(ErrorKind::NotFound, "operation not found"))
    }

    pub(crate) async fn handle_request(&self, request: Request) -> Response {
        let operation = self
            .get_operation_from_request(&request)
            .await
            .unwrap_or_else(|_| {
                eprintln!(
                    "{} {}",
                    "?".bright_black(),
                    "using system command".bright_black()
                );
                Operation::CommandRun
            });

        match self.execute(operation, &request).await {
            Ok(_) => {
                eprintln!("{} {}", "*".bright_black(), "OK".bright_black());
                Response::new(ResponseStatus::Ok)
            }
            Err(_) => {
                eprintln!("{} {}", "*".bright_black(), "ERROR".red());
                Response::new(ResponseStatus::Error)
            }
        }
    }

    async fn execute(&self, operation: Operation, request: &Request) -> Result<()> {
        match operation {
            Operation::None => todo!(),
            Operation::FileRead => todo!(),
            Operation::FileCopy => todo!(),
            Operation::CommandRun => {
                eprintln!("{}", "```".bright_black());

                let command = Command::new(request.get_key())
                    .args(request.get_args())
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let output = command.wait_with_output().await?;

                eprintln!("{}", "```".bright_black());

                if output.status.success() {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "command failed"))
                }
            }
        }
    }
}
