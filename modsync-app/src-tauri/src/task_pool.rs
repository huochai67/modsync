use modsync_core::{
    error::Error,
    mstask::{MSTask, MSTaskStatus},
};
use std::collections::HashMap;
use tokio::{sync::mpsc, task::JoinSet};

type BoxTask = Box<dyn MSTask + Send>;
type BoxTasks = Vec<BoxTask>;

pub struct Task {
    pub tx: mpsc::Sender<MSTaskStatus>,
    rx: mpsc::Receiver<MSTaskStatus>,

    task_status: HashMap<String, MSTaskStatus>,

    pub num_total: usize,
    pub num_finished: usize,
    pub num_running: usize,

    pub num_piece_size: usize,
}

impl Task {
    pub fn new() -> Task {
        let (tx, rx) = mpsc::channel(198964);

        Task {
            tx,
            rx,
            task_status: HashMap::default(),
            num_finished: 0,
            num_total: 0,
            num_running: 0,
            num_piece_size: 0,
        }
    }

    pub async fn run(&self, mut tasks: Box<BoxTasks>) -> Result<(), Error> {
        //self.num_total = tasks.len();

        while tasks.len() > 0 {
            let piece = tasks.split_off(self.num_piece_size);

            let mut set = JoinSet::new();
            for task in piece {
                let tx = self.tx.clone();
                set.spawn(async move { task.start(tx).await });
            }
            /*
                        let fetches = tokio_stream::iter(piece.into_iter())
                            .map(|mut ta| {
                                let tx = self.tx.clone();
                                async move { ta.start(tx).await }
                            })
                            .buffer_unordered(8)
                            .collect::<Vec<Result<(), Error>>>();
            */
            for result in set.join_all().await {
                if let Err(err) = result {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    pub fn get_status(&mut self) -> Vec<MSTaskStatus> {
        while let Ok(st) = self.rx.try_recv() {
            if st.finish {
                self.task_status.remove(&st.name);
                self.num_finished += 1;
                self.num_running -= 1;
            } else {
                let copy_ = st.clone();
                self.task_status.insert(st.name, copy_);
            }
        }

        self.task_status.iter().map(|(_k, v)| v.clone()).collect()
    }
}

pub async fn ddrun(tx: mpsc::Sender<MSTaskStatus>, mut tasks: Box<BoxTasks>) -> Result<(), Error> {
    while tasks.len() > 0 {
        let piece = tasks.split_off(match tasks.len() > 100 {
            true => 100,
            false => tasks.len(),
        });
        let mut set = JoinSet::new();
        for task in piece {
            let tx = tx.clone();
            set.spawn(async move { task.start(tx).await });
        }

        for result in set.join_all().await {
            if let Err(err) = result {
                return Err(err);
            }
        }
    }
    Ok(())
}
