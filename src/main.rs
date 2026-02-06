use crate::program::Program;

mod executor;
mod mapping;
mod operation;
mod program;
mod request;
mod response;

#[tokio::main]
async fn main() {
    let program = Program::new().await;

    program.launch().await;
}
