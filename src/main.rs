mod cli;
mod component;
mod executor;
mod mapping;
mod node;
mod operation;
mod program;
mod renderer;
mod request;
mod response;

use crate::program::Program;

#[tokio::main]
async fn main() {
    let program = Program::new().await;

    program.launch().await;
}
