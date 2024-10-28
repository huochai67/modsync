use modsync_core::mstask::{MSTask, MSTaskStatus};
use std::collections::{HashMap, VecDeque};
use tokio::sync::mpsc;

pub struct TaskPool {
    tx: mpsc::Sender<MSTaskStatus>,
    rx: mpsc::Receiver<MSTaskStatus>,

    tasks: VecDeque<Box<dyn MSTask + Send>>,
    task_status: HashMap<String, MSTaskStatus>,

    pub num_total: usize,
    pub num_finished: usize,
}

impl TaskPool {
    pub fn new() -> TaskPool {
        let (tx, rx) = mpsc::channel(198964);

        TaskPool {
            tx,
            rx,
            tasks: VecDeque::default(),
            task_status: HashMap::default(),
            num_finished: 0,
            num_total: 0,
        }
    }

    pub fn push(&mut self, task: Box<dyn MSTask + Send>) {
        self.tasks.push_back(task);
        self.num_total += 1;
    }

    pub async fn check(&mut self) -> Result<(), modsync_core::error::Error> {
        while let Ok(st) = self.rx.try_recv() {
            if st.finish {
                self.task_status.remove(&st.name);
                self.num_finished += 1;
            } else {
                let copy_ = st.clone();
                self.task_status.insert(st.name, copy_);
            }
        }

        while let Some(mut task) = self.tasks.pop_back() {
            let tx = self.tx.clone();
            tokio::spawn(async move { task.start(tx).await });
        }

        Ok(())
    }

    pub fn get_status(&mut self) -> Vec<MSTaskStatus> {
        self.task_status.iter().map(|(_k, v)| v.clone()).collect()
    }
}
