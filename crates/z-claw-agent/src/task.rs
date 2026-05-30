use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub task_type: String,
    pub description: String,
    pub status: TaskStatus,
    pub progress: Option<u8>,
}

impl Task {
    pub fn new(task_type: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: task_type.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            progress: None,
        }
    }
}

/// Registry of tracked tasks with progress reporting.
pub struct TaskRegistry {
    tasks: Mutex<Vec<Task>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
        }
    }

    pub async fn start(&self, task_type: &str, description: &str) -> Task {
        let mut task = Task::new(task_type, description);
        task.status = TaskStatus::Running;
        self.tasks.lock().await.push(task.clone());
        task
    }

    pub async fn complete(&self, id: &str) {
        let mut tasks = self.tasks.lock().await;
        if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
            t.status = TaskStatus::Completed;
            t.progress = Some(100);
        }
    }

    pub async fn fail(&self, id: &str, error: &str) {
        let mut tasks = self.tasks.lock().await;
        if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
            t.status = TaskStatus::Failed(error.to_string());
        }
    }

    pub async fn update_progress(&self, id: &str, progress: u8, desc: &str) {
        let mut tasks = self.tasks.lock().await;
        if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
            t.progress = Some(progress.min(100));
            t.description = desc.to_string();
        }
    }

    pub async fn list(&self) -> Vec<Task> {
        self.tasks.lock().await.clone()
    }
}
