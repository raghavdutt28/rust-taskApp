use serde::Serialize;
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Task{
    pub user_id: String,
    pub task_id: String,
    pub task_type: String,
    pub state: TaskState,
    pub source_file: String,
    pub result_file: Option<String>
}

#[derive(Serialize, PartialEq, Eq, EnumString, Display)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed
}

impl Task {
    pub fn new(user_id: String, task_type: String, source_file: String) -> Task{
        Task{
            user_id,
            task_id: Uuid::new_v4().to_string(),
            task_type,
            state: TaskState::NotStarted,
            source_file,
            result_file: None
        }
    }
    pub fn get_global_id(&self) -> String {
        format!("{}-{}", self.user_id, self.task_id)
    }
    pub fn can_transition_to(&self, state: &TaskState) -> bool {
        self.state !=*state
    }
    
}