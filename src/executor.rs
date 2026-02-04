use crate::operation::Operation;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Executor {
    operation_queue_tx: Sender<Operation>,
}

impl Executor {
    pub(crate) async fn new() -> Arc<Self> {
        let (operation_queue_tx, mut operation_queue_rx) = tokio::sync::mpsc::channel(1024);

        let this = Arc::new(Self { operation_queue_tx });

        let this_ref = this.clone();
        tokio::spawn(async move {
            loop {
                match operation_queue_rx.recv().await {
                    Some(operation) => {
                        this_ref.execute(operation).await;
                    }
                    None => break,
                };
            }
        });

        this
    }

    pub(crate) async fn push_operation(&self, operation: Operation) {
        let _ = self.operation_queue_tx.send(operation).await;
    }

    async fn execute(&self, operation: Operation) {
        eprintln!("executing op->{:?}", operation);

        match operation {
            Operation::FileRead => {}
            Operation::FileCopy => {}
        }
    }
}
