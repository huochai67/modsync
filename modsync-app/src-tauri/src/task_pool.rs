use modsync_core::mstask::MSTask;
use tokio::sync::{Mutex, MutexGuard};

type Tasks = Vec<Box<dyn MSTask + Send + Sync>>;
pub struct TaskPool {
    all_tasks: Mutex<Tasks>,
    running_tasks: Mutex<Tasks>,
    bounded: usize,
}

impl TaskPool {
    pub fn new(bounded: usize) -> TaskPool {
        TaskPool {
            all_tasks: Default::default(),
            running_tasks: Default::default(),
            bounded,
        }
    }

    pub async fn push(&self, task: Box<dyn MSTask + Send + Sync>) {
        self.all_tasks.lock().await.push(task);
    }

    pub async fn check(&self) -> Result<MutexGuard<Tasks>, modsync_core::error::Error> {
        let mut running_task = self.running_tasks.lock().await;
        if running_task.len() < self.bounded {
            let mut all_task = self.all_tasks.lock().await;
            for _n in 0..(self.bounded - running_task.len()) {
                match all_task.pop() {
                    Some(mut task) => {
                        task.spawn().await?;
                        running_task.push(task)
                    }
                    None => break,
                }
            }
        }
        for task in running_task.iter_mut() {
            let mut all_task = self.all_tasks.lock().await;
            if task.get_join_handle().is_finished() {
                if let Some(mut replace) = all_task.pop() {
                    replace.spawn().await?;
                    *task = replace;
                }
            }
        }
        Ok(running_task)
    }
}
