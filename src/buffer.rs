use std::{collections::VecDeque, fmt::Debug, sync::Condvar};

#[derive(Debug)]
pub struct Buffer {
    pub buffer_size: Option<usize>,
    pub tasks: VecDeque<Task>,
}

pub struct Task {
    pub id: String,
    pub item: Box<dyn Fn() + Send + 'static>,
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task {{ id: {} }}", self.id)
    }
}

pub trait BufferTrait {
    fn new(buffer_size: Option<usize>) -> Self;
    fn add(
        &mut self,
        id: String,
        condvar: &Condvar,
        task: Box<dyn Fn() + Send + 'static>,
    ) -> Result<String, String>;
    fn list_tasks(&self) -> Vec<&Task>;
    fn remove(&mut self) -> Option<Task>;
}

impl BufferTrait for Buffer {
    fn new(buffer_size: Option<usize>) -> Self {
        match buffer_size {
            Some(size) => {
                return Buffer {
                    buffer_size: Some(size),
                    tasks: VecDeque::with_capacity(size),
                }
            }
            None => {
                return Buffer {
                    buffer_size: Some(10),
                    tasks: VecDeque::with_capacity(10),
                }
            }
        }
    }

    fn add(
        &mut self,
        id: String,
        condvar: &Condvar,
        task: Box<dyn Fn() + Send + 'static>,
    ) -> Result<String, String> {
        for item in self.list_tasks().iter() {
            if item.id == id {
                return Err(format!("Task with the same id already exists: {}", id));
            }
        }

        self.tasks.push_back(Task {
            id: id.clone(),
            item: task,
        });
        condvar.notify_one();
        return Ok(id);
    }

    fn list_tasks(&self) -> Vec<&Task> {
        return self.tasks.iter().collect();
    }

    fn remove(&mut self) -> Option<Task> {
        let removed_task = self.tasks.pop_front();
        return removed_task;
    }
}
