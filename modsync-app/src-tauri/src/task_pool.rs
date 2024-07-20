use modsync_core::mstask::MSTask;

pub struct TaskPool {
    all_tasks: Vec<Box<dyn MSTask + Send + Sync>>,
    running_tasks: Vec<Option<Box<dyn MSTask + Send + Sync>>>,
    bounded: usize,

    pub num_total: usize,
    pub num_finished: usize,
}

impl TaskPool {
    pub fn new(bounded: usize) -> TaskPool {
        TaskPool {
            all_tasks: Default::default(),
            running_tasks: Default::default(),
            bounded,
            num_finished: 0,
            num_total: 0,
        }
    }

    pub fn push(&mut self, task: Box<dyn MSTask + Send + Sync>) {
        self.all_tasks.push(task);
        self.num_total += 1;
    }

    pub fn pop(
        vec: &mut Vec<Box<dyn MSTask + Send + Sync>>,
    ) -> Option<Box<dyn MSTask + Send + Sync>> {
        vec.pop()
    }

    pub async fn pop_and_spawn(
        vec: &mut Vec<Box<dyn MSTask + Send + Sync>>,
    ) -> Result<Option<Box<dyn MSTask + Send + Sync>>, modsync_core::error::Error> {
        match Self::pop(vec) {
            Some(mut task) => {
                task.spawn().await?;
                Ok(Some(task))
            }
            None => Ok(None),
        }
    }

    pub async fn check(
        &mut self,
    ) -> Result<&Vec<Option<Box<dyn MSTask + Send + Sync>>>, modsync_core::error::Error> {
        if self.running_tasks.len() < self.bounded {
            for _n in 0..(self.bounded - self.running_tasks.len()) {
                match Self::pop_and_spawn(&mut self.all_tasks).await? {
                    Some(task) => self.running_tasks.push(Some(task)),
                    None => break,
                }
            }
        }

        for task in self.running_tasks.iter_mut() {
            if let Some(sometask) = task {
                if !sometask.get_join_handle().is_finished() {
                    continue;
                }else {
                    self.num_finished += 1;
                }
            }
            match Self::pop_and_spawn(&mut self.all_tasks).await? {
                Some(replace) => *task = Some(replace),
                None => *task = None,
            }
        }
        Ok(&self.running_tasks)
    }
}
