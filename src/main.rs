mod cli;
mod executor;
mod mapping;
mod operation;
mod program;
mod request;
mod response;

use crate::program::Program;

#[tokio::main]
async fn main() {
    let program = Program::new().await;

    program.launch().await;
}
