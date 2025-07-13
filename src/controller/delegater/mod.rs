// region:    --- Modules
mod delegate;
mod item;
mod project;
mod task;

// -- Flatten
pub(crate) use task::delegate_task_op;
pub(crate) use task::read_task;
pub use delegate::delegate;
pub use item::delegate_item;

// endregion: --- Modules
