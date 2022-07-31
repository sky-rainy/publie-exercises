///  定义数据结构，message 类似golang中的struct
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionResult {
    ///  错误信息
    #[prost(string, optional, tag="1")]
    pub err: ::core::option::Option<::prost::alloc::string::String>,
    ///  是否成功
    #[prost(bool, tag="2")]
    pub success: bool,
    ///  查询结果
    #[prost(message, repeated, tag="3")]
    pub result: ::prost::alloc::vec::Vec<QueryResultItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResultItem {
    ///  文档id
    #[prost(uint64, tag="1")]
    pub id: u64,
    ///  文档标题
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    ///  文档关联的查询内容
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
    ///  得分
    #[prost(float, tag="4")]
    pub score: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contents {
    ///  文档id
    #[prost(uint64, tag="1")]
    pub id: u64,
    ///  文档标题
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    ///  内容
    #[prost(string, repeated, tag="3")]
    pub body: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContents {
    ///  内容
    #[prost(message, repeated, tag="1")]
    pub contents: ::prost::alloc::vec::Vec<Contents>,
}
