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
            Message::Quit => self.quit(),
            Message::TaskOp(op) => delegate_task_op(&self.db, op).await,
            Message::NextTask => return_noop(|| self.state.task_state.select_next()),
            Message::PrevTask => return_noop(|| self.state.task_state.select_previous()),
            Message::FocusProject => return_noop(|| self.app_state = AppState::NormalProject),
            Message::FocusTask => return_noop(|| self.app_state = AppState::NormalTask),
            Message::NextProject => return_noop(|| self.state.project_state.select_next()),
            Message::PrevProject => return_noop(|| self.state.project_state.select_previous()),
            Message::Noop => unreachable!(),
        }
    }
}

fn return_noop<F: FnMut()>(mut f: F) -> Result<Message> {
    f();
    Ok(Message::Noop)
}
