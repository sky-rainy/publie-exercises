use tokio::sync::oneshot;

pub use crate::fts::*;

#[derive(Debug)]
pub enum TantivyActionType {
    Query(String),
    Update(Contents),
    BatchAdd(Vec<Contents>),
    Delete(u64),
    // Stop,
    DeleteAll,
}

#[derive(Debug)]
pub struct TantivyAction {
    pub action_type: TantivyActionType,
    pub sender: oneshot::Sender<ActionResult>,
}
