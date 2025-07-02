use crate::{
    app::{
        model::{App, RunningState},
        update::message::Message,
    },
    controller::delegate_task_op,
    Result,
};

impl App {
    fn quit(&mut self) -> Result<Message> {
        self.running = RunningState::Done;
        Ok(Message::Noop)
    }

    pub async fn update(&mut self, action: Message) -> Result<Message> {
        match action {
            Message::Quit => self.quit(),
            Message::TaskOp(op) => delegate_task_op(&self.db, op).await,
            Message::NextTask => {
                self.state.task_state.select_next();
                Ok(Message::Noop)
            }
            Message::PrevTask => {
                self.state.task_state.select_previous();
                Ok(Message::Noop)
            }

            // Update will never be called with Noop
            Message::Noop => unreachable!(),
        }
    }
}
