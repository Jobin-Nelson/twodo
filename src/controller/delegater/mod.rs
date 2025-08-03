// region:    --- Modules
mod delegate;
mod item;
mod project;
mod task;

// -- Flatten
pub(crate) use task::{delegate_task_op, read_task};
pub(crate) use project::{delegate_project_op, read_project};
pub use delegate::delegate;
pub use item::delegate_item;

// endregion: --- Modules
