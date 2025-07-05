use crate::{
    app::{
        model::{AddTaskMode, App, AppMode},
        update::message::Message,
    },
    cli::{TaskAddArg, TaskDeleteArg, TaskDoneArg, TaskOp},
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
            Message::SelectNextTask => return_noop(|| self.state.task_state.select_next()),
            Message::SelectPrevTask => return_noop(|| self.state.task_state.select_previous()),
            Message::FocusTask => return_noop(|| self.mode.app_mode = AppMode::FocusTask),
            Message::DeleteTask => self.delete_task(),
            Message::ToggleTaskStatus => self.toggle_task_status(),
            Message::SelectFirstTask => return_noop(|| self.state.task_state.select_first()),
            Message::SelectLastTask => return_noop(|| self.state.task_state.select_last()),

            // Project messages
            Message::SelectNextProject => self.select_next_project(),
            Message::SelectPrevProject => self.select_previous_project(),
            Message::SelectFirstProject => self.select_first_project(),
            Message::SelectLastProject => self.select_last_project(),
            Message::FocusProject => return_noop(|| self.mode.app_mode = AppMode::FocusProject),
        }
    }

    fn select_first_project(&mut self) -> Result<Message> {
        self.state.project_state.select_first();
        Ok(Message::ReloadTask)
    }
    fn select_last_project(&mut self) -> Result<Message> {
        self.state.project_state.select_last();
        Ok(Message::ReloadTask)
    }
    fn select_previous_project(&mut self) -> Result<Message> {
        self.state.project_state.select_previous();
        Ok(Message::ReloadTask)
    }
    fn select_next_project(&mut self) -> Result<Message> {
        // Need to check if the selected index is within bounds
        // because select_next() will always increment the index
        // and not check bounds till the next render
        if self
            .state
            .project_state
            .selected()
            .is_some_and(|i| i < self.twodo.projects.len() - 1)
        {
            self.state.project_state.select_next();
            return Ok(Message::ReloadTask);
        }
        Ok(Message::Noop)
    }

    fn add_task_begin(&mut self) -> Result<Message> {
        self.mode.app_mode = AppMode::AddTask;
        Ok(Message::FocusAddTaskTitle)
    }

    fn toggle_task_status(&mut self) -> Result<Message> {
        let task = self
            .state
            .task_state
            .selected()
            .map(|i| &self.twodo.tasks[i])
            .ok_or(Error::MissingTaskId)?;
        let task_op = if task.done {
            TaskOp::UnDone(TaskDoneArg { id: task.id })
        } else {
            TaskOp::Done(TaskDoneArg { id: task.id })
        };

        Ok(Message::TaskOp(task_op))
    }

    fn delete_task(&mut self) -> Result<Message> {
        let id = self
            .state
            .task_state
            .selected()
            .map(|i| self.twodo.tasks[i].id)
            .ok_or(Error::MissingTaskId)?;

        Ok(Message::TaskOp(TaskOp::Delete(TaskDeleteArg { id })))
    }

    fn add_task_commit(&mut self) -> Result<Message> {
        self.mode.app_mode = AppMode::FocusTask;
        let project_id = self
            .state
            .project_state
            .selected()
            .map(|i| self.twodo.projects[i].id)
            .ok_or(Error::MissingProjectId)?;
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
            .and_then(|i| self.twodo.projects.get(i))
            .map(|p| p.id)
            .ok_or(Error::MissingProjectId)? as i64;

        self.twodo.tasks = get_tasks_by_project(&self.db, project_id).await?;

        Ok(Message::Noop)
    }
}

fn return_noop<F: FnMut()>(mut f: F) -> Result<Message> {
    f();
    Ok(Message::Noop)
}
