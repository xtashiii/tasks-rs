pub mod get;
pub mod post;
pub mod delete;
pub mod put;

pub use get::{index, get_tasks};
pub use post::create_task;
pub use delete::delete_task;
pub use put::update_task;