//! Async worker pool using tokio mpsc.

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::info;

pub struct AsyncWorkerPool<T: Send + 'static> {
    sender: mpsc::Sender<T>,
    _handles: Vec<JoinHandle<()>>,
}

impl<T: Send + 'static> AsyncWorkerPool<T> {
    pub fn new<F>(workers: usize, handler: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static + Clone,
    {
        let (tx, rx) = mpsc::channel::<T>(256);
        let mut handles = Vec::new();
        let handler = Arc::new(handler);

        // Single receiver task; in production use multiple with broadcast or work-stealing
        let h = handler.clone();
        let workers_label = workers;
        let handle = tokio::spawn(async move {
            info!("Worker pool started ({} logical workers)", workers_label);
            let mut rx = rx;
            while let Some(job) = rx.recv().await {
                h(job);
            }
            info!("Worker pool stopped");
        });
        handles.push(handle);

        Self {
            sender: tx,
            _handles: handles,
        }
    }

    pub async fn submit(&self, job: T) -> Result<(), mpsc::error::SendError<T>> {
        self.sender.send(job).await
    }
}
