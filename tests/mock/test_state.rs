use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use chrono::Utc;
use error_stack::bail;
use todoapp::dtos::NewTask;
use todoapp::dtos::TaskFilter;
use todoapp::dtos::UpdateTask;
use todoapp::errors::AppError;
use todoapp::errors::AppResult;
use todoapp::models::StatusModel;
use todoapp::models::TaskModel;
use todoapp::states::AppState;
use todoapp::states::DbRepository;
use todoapp::states::StatusRepository;
use todoapp::states::TasksRepository;

pub fn create_test_state() -> AppResult<AppState<impl DbRepository>> {
    let status = HashMap::from([
        (
            1,
            StatusModel {
                id: 1,
                name: "S1".into(),
            },
        ),
        (
            2,
            StatusModel {
                id: 2,
                name: "S2".into(),
            },
        ),
        (
            3,
            StatusModel {
                id: 3,
                name: "S3".into(),
            },
        ),
    ]);
    let tasks = HashMap::from([
        (
            10,
            TaskModel {
                id: 10,
                summary: "T1".into(),
                description: Some("TD1".into()),
                status_id: 1,
                status_name: "S1".into(),
                ..Default::default()
            },
        ),
        (
            20,
            TaskModel {
                id: 20,
                summary: "T2".into(),
                description: None,
                status_id: 1,
                status_name: "S1".into(),
                ..Default::default()
            },
        ),
        (
            30,
            TaskModel {
                id: 30,
                summary: "T3".into(),
                description: Some("TD31".into()),
                status_id: 2,
                status_name: "S2".into(),
                ..Default::default()
            },
        ),
        (
            40,
            TaskModel {
                id: 40,
                summary: "T4".into(),
                description: Some("TD41".into()),
                status_id: 3,
                status_name: "S3".into(),
                ..Default::default()
            },
        ),
        (
            50,
            TaskModel {
                id: 50,
                summary: "T5".into(),
                description: Some("TD51".into()),
                status_id: 2,
                status_name: "S2".into(),
                ..Default::default()
            },
        ),
        (
            60,
            TaskModel {
                id: 60,
                summary: "T6".into(),
                description: None,
                status_id: 1,
                status_name: "S1".into(),
                ..Default::default()
            },
        ),
        (
            70,
            TaskModel {
                id: 70,
                summary: "T7".into(),
                description: None,
                status_id: 3,
                status_name: "S3".into(),
                ..Default::default()
            },
        ),
    ]);
    Ok(AppState {
        repo: TestState {
            tasks: RwLock::new(tasks),
            status: RwLock::new(status),
        },
    })
}

#[derive(Debug, Default)]
pub struct TestState {
    tasks: RwLock<HashMap<i64, TaskModel>>,
    status: RwLock<HashMap<i64, StatusModel>>,
}

#[async_trait]
impl TasksRepository for TestState {
    async fn get_filtered_tasks(&self, filter: &TaskFilter) -> AppResult<Vec<TaskModel>> {
        let tasks = self.tasks.read().unwrap();
        let tasks = tasks
            .values()
            .filter(|task| match filter.summary {
                Some(ref fsummary) => task.summary.contains(fsummary),
                None => true,
            })
            .filter(|task| {
                if filter.status_ids.is_empty() {
                    true
                } else {
                    filter
                        .status_ids
                        .iter()
                        .any(|&status_id| task.status_id == status_id)
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(tasks)
    }

    /// get task by id
    async fn get_task_by_id(&self, id: i64) -> AppResult<Option<TaskModel>> {
        let tasks = self.tasks.read().unwrap();
        let task = tasks.get(&id).cloned();
        Ok(task)
    }

    /// create new task
    async fn create_task(&self, new_task: &NewTask) -> AppResult<TaskModel> {
        let status_name = {
            let status = self.status.read().unwrap();
            let Some(status) = status.get(&new_task.status_id) else {
                bail!(AppError);
            };
            status.name.clone()
        };
        let mut tasks = self.tasks.write().unwrap();
        let id = (tasks.len() as i64 + 1) * 10;
        let now = Utc::now().naive_local();
        let new_task = TaskModel {
            id,
            summary: new_task.summary.clone(),
            description: if new_task.description.is_empty() {
                None
            } else {
                Some(new_task.description.clone())
            },
            status_id: new_task.status_id,
            status_name,
            created_at: now,
            updated_at: now,
        };
        tasks.insert(id, new_task.clone());
        Ok(new_task)
    }

    /// update the task with id
    async fn update_task(&self, id: i64, update_task: &UpdateTask) -> AppResult<Option<TaskModel>> {
        let mut tasks = self.tasks.write().unwrap();
        let Some(task) = tasks.get_mut(&id) else {
            return Ok(None);
        };

        task.summary = update_task.summary.clone();
        if update_task.description.is_empty() {
            task.description = None;
        } else {
            task.description = Some(update_task.description.clone());
        }
        if let Some(status) = {
            let st = self.status.read().unwrap();
            st.get(&update_task.status_id).cloned()
        } {
            task.status_id = status.id;
            task.status_name = status.name.clone();
        }
        task.updated_at = Utc::now().naive_local();

        Ok(Some(task.clone()))
    }

    /// delete the task with id
    async fn delete_task(&self, id: i64) -> AppResult<Option<TaskModel>> {
        let mut tasks = self.tasks.write().unwrap();
        let removed = tasks.remove(&id);
        Ok(removed)
    }
}

#[async_trait]
impl StatusRepository for TestState {
    /// get all statuses
    async fn get_all_statuses(&self) -> AppResult<Vec<StatusModel>> {
        let status = self.status.read().unwrap();
        let status = status.values().cloned().collect::<Vec<_>>();
        Ok(status)
    }

    /// get status by id
    async fn get_status_by_id(&self, status_id: i64) -> AppResult<Option<StatusModel>> {
        let status = self.status.read().unwrap();
        let status = status.get(&status_id).cloned();
        Ok(status)
    }
}

impl DbRepository for TestState {}
