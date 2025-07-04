use crate::{
    app::{
        model::{AddTaskMode, App, AppMode},
        update::message::Message,
    },
    cli::{TaskAddArg, TaskOp},
    controller::delegater::delegate_task_op,
    Error, Result,
};

use super::read_data::get_tasks_by_project;

impl App {
    fn quit(&mut self) -> Result<Message> {
        self.mode.app_mode = AppMode::Quit;
        Ok(Message::Noop)
    }

    pub async fn update(&mut self, action: Message) -> Result<Message> {
        match action {
            Message::Noop => unreachable!(),
            Message::Quit => self.quit(),
            // Task messages
            Message::ReloadTask => self.reload_task().await,
            Message::TaskOp(op) => delegate_task_op(&self.db, op).await,
            Message::AddTaskBegin => self.add_task_begin(),
            Message::AddTaskCommit => self.add_task_commit(),
            Message::AddTaskAbort => return_noop(|| self.mode.app_mode = AppMode::FocusTask),
            Message::FocusAddTaskTitle => {
                return_noop(|| self.mode.add_task_mode = AddTaskMode::AddTitle)
            }
            Message::FocusAddTaskDescription => {
                return_noop(|| self.mode.add_task_mode = AddTaskMode::AddDescription)
            }
            Message::NextTask => return_noop(|| self.state.task_state.select_next()),
            Message::PrevTask => return_noop(|| self.state.task_state.select_previous()),
            Message::FocusTask => return_noop(|| self.mode.app_mode = AppMode::FocusTask),

            // Project messages
            Message::NextProject => return_noop(|| self.state.project_state.select_next()),
            Message::PrevProject => return_noop(|| self.state.project_state.select_previous()),
            Message::FocusProject => return_noop(|| self.mode.app_mode = AppMode::FocusProject),
        }
    }

    fn add_task_begin(&mut self) -> Result<Message> {
        self.mode.app_mode = AppMode::AddTask;
        Ok(Message::FocusAddTaskTitle)
    }

    fn add_task_commit(&mut self) -> Result<Message> {
        self.mode.app_mode = AppMode::FocusTask;
        let project_id = self
            .state
            .project_state
            .selected()
            .ok_or(Error::MissingProjectId)? as i64;
        let title = self.popover.add_task.title.lines()[0].trim().to_string();
        let description = {
            let desc = self
                .popover
                .add_task
                .description
                .lines()
                .join("\n")
                .trim()
                .to_string();
            if desc.is_empty() {
                None
            } else {
                Some(desc)
            }
        };
        if title.is_empty() {
            return Ok(Message::Noop);
        }
        Ok(Message::TaskOp(TaskOp::Add(TaskAddArg {
            title,
            description,
            project_id,
        })))
    }

    async fn reload_task(&mut self) -> Result<Message> {
        let project_id = self
            .state
            .project_state
            .selected()
            .ok_or(Error::MissingProjectId)? as i64;

        self.twodo.tasks = get_tasks_by_project(&self.db, project_id).await?;

        Ok(Message::Noop)
    }
}

fn return_noop<F: FnMut()>(mut f: F) -> Result<Message> {
    f();
    Ok(Message::Noop)
}
