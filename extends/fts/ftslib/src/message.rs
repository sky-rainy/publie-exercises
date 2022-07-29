use tokio::sync::oneshot;
#[derive(Debug, serde::Deserialize)]
pub struct Contents {
    pub id: u64, // id,
    pub title: String,
    pub contents: Vec<String>,
}
#[derive(Debug)]
pub enum TantivyActionType {
    Query(String),
    Update(Contents),
    BatchAdd(Vec<Contents>),
    Delete(u64),
    // Stop,
    DeleteAll,
}

#[derive(Debug, serde::Serialize)]
pub struct QueryResult {
    pub id: u64, // id,
    pub title: String,
    pub contents: String,
    pub score: f32,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct TantivyActionResult {
    pub success: bool,                    // 执行是否成功
    pub err: Option<String>,              // 运行错误
    pub result: Option<Vec<QueryResult>>, // 查询返回结果
}
#[derive(Debug)]
pub struct TantivyAction {
    pub action_type: TantivyActionType,
    pub sender: oneshot::Sender<TantivyActionResult>,
}
