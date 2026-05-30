use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

#[derive(Debug, Clone)]
pub struct CronTask {
    pub id: String,
    pub name: String,
    pub expression: String,
    pub last_run: Option<DateTime<Utc>>,
}

/// A simple cron-based task scheduler. Checks every 60 seconds.
pub struct CronScheduler {
    tasks: Arc<Mutex<Vec<CronTask>>>,
    on_tick: Option<Arc<dyn Fn(&CronTask) + Send + Sync>>,
}

impl CronScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            on_tick: None,
        }
    }

    pub fn on_tick(mut self, f: impl Fn(&CronTask) + Send + Sync + 'static) -> Self {
        self.on_tick = Some(Arc::new(f));
        self
    }

    pub async fn add_task(&self, name: &str, expression: &str) -> Result<String, String> {
        Schedule::from_str(expression).map_err(|e| format!("Invalid cron: {e}"))?;
        let task = CronTask {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            expression: expression.to_string(),
            last_run: None,
        };
        self.tasks.lock().await.push(task.clone());
        Ok(task.id)
    }

    pub async fn list_tasks(&self) -> Vec<CronTask> {
        self.tasks.lock().await.clone()
    }

    pub async fn remove_task(&self, id: &str) -> bool {
        let mut tasks = self.tasks.lock().await;
        let len = tasks.len();
        tasks.retain(|t| t.id != id);
        tasks.len() < len
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(60));
            loop {
                tick.tick().await;
                let now = Utc::now();
                let mut tasks = self.tasks.lock().await;
                for task in tasks.iter_mut() {
                    if let Ok(schedule) = Schedule::from_str(&task.expression) {
                        let should_run = match task.last_run {
                            Some(last) => schedule.after(&last).next().map_or(false, |n| n <= now),
                            None => true,
                        };
                        if should_run {
                            task.last_run = Some(now);
                            if let Some(ref cb) = self.on_tick {
                                cb(task);
                            }
                        }
                    }
                }
            }
        });
    }
}
