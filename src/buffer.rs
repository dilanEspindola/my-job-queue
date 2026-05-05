use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer_size: Option<usize>,
    tasks: VecDeque<Task>,
}

#[derive(Debug, Hash, Clone)]
pub struct Task {
    pub id: String,
    pub item: fn(),
}

pub trait BufferTrait {
    fn new(buffer_size: Option<usize>) -> Self;
    fn add(&mut self, id: String, task: fn()) -> Result<String, String>;
    fn list_tasks(&self) -> Vec<Task>;
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

    fn add(&mut self, id: String, task: fn()) -> Result<String, String> {
        for item in self.list_tasks().iter() {
            if item.id == id {
                return Err(format!("Task with the same id already exists: {}", id));
            }
        }
        self.tasks.push_back(Task {
            id: id.clone(),
            item: task,
        });
        return Ok(id);
    }

    fn list_tasks(&self) -> Vec<Task> {
        return self.tasks.iter().map(|el| el).cloned().collect();
    }
}
