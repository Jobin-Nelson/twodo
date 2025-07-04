use crate::{
    app::{
        model::{App, AppState},
        update::message::Message,
    },
    controller::delegater::delegate_task_op,
    Result,
};

impl App {
    fn quit(&mut self) -> Result<Message> {
        self.app_state = AppState::CloseApp;
        Ok(Message::Noop)
    }

    pub async fn update(&mut self, action: Message) -> Result<Message> {
        match action {
            Message::Noop => unreachable!(),
            Message::Quit => self.quit(),
            // Task messages
            Message::TaskOp(op) => delegate_task_op(&self.db, op).await,
            Message::AddTaskBegin => return_noop(|| self.app_state = AppState::AddTask),
            Message::AddTaskCommit => self.add_task(),
            Message::AddTaskAbort => return_noop(|| self.app_state = AppState::NormalTask),
            Message::NextTask => return_noop(|| self.state.task_state.select_next()),
            Message::PrevTask => return_noop(|| self.state.task_state.select_previous()),
            Message::FocusTask => return_noop(|| self.app_state = AppState::NormalTask),

            // Project messages
            Message::NextProject => return_noop(|| self.state.project_state.select_next()),
            Message::PrevProject => return_noop(|| self.state.project_state.select_previous()),
            Message::FocusProject => return_noop(|| self.app_state = AppState::NormalProject),
        }
    }

    fn add_task(&mut self) -> Result<Message> {
        todo!();
        // Ok(Message::TaskOp(TaskOp::Add(TaskAddArg {
        //     title: todo!(),
        //     description: todo!(),
        //     project_id: todo!(),
        // })))
    }
}

fn return_noop<F: FnMut()>(mut f: F) -> Result<Message> {
    f();
    Ok(Message::Noop)
}
