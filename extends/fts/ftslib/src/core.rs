use std::sync::Arc;
use tantivy::{IndexReader, IndexWriter};

use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, TermQuery};
use tantivy::{schema::*, SnippetGenerator};
use tantivy::{Index, ReloadPolicy};
use tokio::sync::{mpsc, RwLock};

use crate::message::{ActionResult, QueryResultItem, TantivyAction, TantivyActionType};
// 根据id查询返回doc
fn extract_doc_given_id(reader: &IndexReader, id_term: &Term) -> tantivy::Result<Option<Document>> {
    let searcher = reader.searcher();
    let term_query = TermQuery::new(id_term.clone(), IndexRecordOption::Basic);
    let top_docs = searcher.search(&term_query, &TopDocs::with_limit(1))?;

    if let Some((_score, doc_address)) = top_docs.first() {
        let doc = searcher.doc(*doc_address)?;
        Ok(Some(doc))
    } else {
        Ok(None)
    }
}
pub async fn run(mut rx: mpsc::UnboundedReceiver<TantivyAction>) -> bool {
    let init_result = tantivy_init();
    if init_result.is_err() {
        log::error!("tantivy_init fail {:?} ", init_result.err());
        false
    } else {
        tokio::spawn(async move {
            let config = init_result.unwrap();
            let o_id_filed = Arc::new(config.schema.get_field("index").unwrap());
            let o_title_filed = Arc::new(config.schema.get_field("title").unwrap());
            let o_body_filed = Arc::new(config.schema.get_field("body").unwrap());
            loop {
                let id_filed = o_id_filed.clone();
                let title_filed = o_title_filed.clone();
                let body_filed = o_body_filed.clone();
                let config_index = config.index.clone();
                let config_reader = config.reader.clone();
                let config_index_writer = config.index_writer.clone();
                if let Some(action) = rx.recv().await {
                    match action.action_type {
                        TantivyActionType::Query(q) => {
                            tokio::task::spawn(async move {
                                if q.trim().is_empty() {
                                    if let Err(err) = action.sender.send(ActionResult {
                                        success: false,
                                        err: Some("构建查询条件失败 空字符".to_string()),
                                        result: Vec::new(),
                                    }) {
                                        log::error!("信息发送失败{err:?}")
                                    }
                                } else {
                                    let searcher = config_reader.read().await.searcher();
                                    let query_parser = QueryParser::for_index(
                                        &*config_index,
                                        vec![*title_filed, *body_filed],
                                    );
                                    let query = match query_parser.parse_query(&q) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            if let Err(err) = action.sender.send(ActionResult {
                                                success: false,
                                                err: Some(format!("构建查询条件失败{e:?}")),
                                                result: Vec::new(),
                                            }) {
                                                log::error!("信息发送失败{err:?}")
                                            }
                                            return;
                                        }
                                    };
                                    let top_docs =
                                        match searcher.search(&query, &TopDocs::with_limit(10)) {
                                            Ok(v) => v,
                                            Err(e) => {
                                                if let Err(err) = action.sender.send(ActionResult {
                                                    success: false,
                                                    err: Some(format!("查询失败{e:?}")),
                                                    result: Vec::new(),
                                                }) {
                                                    log::error!("信息发送失败{err:?}")
                                                }
                                                return;
                                            }
                                        };
                                    // 不考虑转换失败 上面已经判断了空字符
                                    let snippet_generator =
                                        SnippetGenerator::create(&searcher, &*query, *body_filed)
                                            .unwrap();
                                    let r: Vec<QueryResultItem> = top_docs
                                        .into_iter()
                                        .map(|(score, doc_address)| {
                                            let doc = searcher.doc(doc_address).unwrap();
                                            let snippet = snippet_generator.snippet_from_doc(&doc);
                                            QueryResultItem {
                                                // unwarp可用
                                                id: doc
                                                    .get_first(*id_filed)
                                                    .unwrap()
                                                    .as_u64()
                                                    .unwrap(),
                                                title: doc
                                                    .get_first(*title_filed)
                                                    .unwrap()
                                                    .as_text()
                                                    .unwrap()
                                                    .to_string(),
                                                data: snippet.to_html(),
                                                score,
                                            }
                                        })
                                        .collect();
                                    if let Err(err) = action.sender.send(ActionResult {
                                        success: true,
                                        err: None,
                                        result: r,
                                    }) {
                                        log::error!("信息发送失败{err:?}")
                                    }
                                }
                            });
                        }
                        // 由于有文档索引，所以不需要Update函数，直接batch_add就行了，会自动删除文档索引对应得文档的
                        TantivyActionType::Update(c) => {
                            tokio::task::spawn(async move {
                                // 新增文档
                                let mut doc_new = Document::default();
                                doc_new.add_text(*title_filed, c.title);
                                doc_new.add_u64(*id_filed, c.id);
                                c.body.iter().for_each(|i| {
                                    doc_new.add_text(*body_filed, i);
                                });
                                let frankenstein_id = Term::from_field_u64(*id_filed, c.id);
                                let frankenstein_doc_misspelled = extract_doc_given_id(
                                    &*config_reader.read().await,
                                    &frankenstein_id,
                                );
                                // 清除旧文档
                                let r = match frankenstein_doc_misspelled {
                                    Ok(doc) => {
                                        // 获取读写锁
                                        let mut lock_config_index_writer =
                                            config_index_writer.write().await;
                                        if doc.is_some() {
                                            lock_config_index_writer.delete_term(frankenstein_id);
                                        }
                                        let r = match (
                                            lock_config_index_writer.add_document(doc_new),
                                            lock_config_index_writer.commit(),
                                        ) {
                                            (Ok(_), Ok(_)) => ActionResult {
                                                success: true,
                                                ..Default::default()
                                            },
                                            (Ok(_), Err(e)) | (Err(e), Ok(_)) => {
                                                let rollback_r =
                                                    lock_config_index_writer.rollback();
                                                if rollback_r.is_err() {
                                                    log::error!(
                                                        "数据回滚失败{:#?}",
                                                        rollback_r.err()
                                                    );
                                                }
                                                ActionResult {
                                                    success: false,
                                                    err: Some(format!("{e:?}")),
                                                    ..Default::default()
                                                }
                                            }
                                            (Err(e), Err(r)) => {
                                                let rollback_r =
                                                    lock_config_index_writer.rollback();
                                                if rollback_r.is_err() {
                                                    log::error!(
                                                        "数据回滚失败{:#?}",
                                                        rollback_r.err()
                                                    );
                                                };
                                                ActionResult {
                                                    success: false,
                                                    err: Some(format!("{e:?}; {r:?}")),
                                                    ..Default::default()
                                                }
                                            }
                                        };
                                        r
                                        // 释放读写锁
                                    }
                                    Err(e) => {
                                        log::error!("Tantivy查询失败{e:?}");
                                        ActionResult {
                                            success: false,
                                            err: Some(format!("{e:?}")),
                                            result: Vec::new(),
                                        }
                                    }
                                };

                                if let Err(err) = action.sender.send(r) {
                                    log::error!("信息发送失败{err:?}")
                                }
                            });
                        }
                        TantivyActionType::Delete(id) => {
                            tokio::task::spawn(async move {
                                let frankenstein_id = Term::from_field_u64(*id_filed, id);
                                let frankenstein_doc_misspelled = extract_doc_given_id(
                                    &*config_reader.read().await,
                                    &frankenstein_id,
                                );
                                let r = match frankenstein_doc_misspelled {
                                    Ok(doc) => {
                                        let mut r = ActionResult {
                                            success: true,
                                            ..Default::default()
                                        };
                                        if doc.is_some() {
                                            // 获取文档写锁
                                            let mut lock_config_index_writer =
                                                config_index_writer.write().await;
                                            lock_config_index_writer.delete_term(frankenstein_id);
                                            if let Err(err) = lock_config_index_writer.commit() {
                                                r.success = false;
                                                r.err = Some(format!("{err:?}"));
                                                let rollback_r =
                                                    lock_config_index_writer.rollback();
                                                if rollback_r.is_err() {
                                                    log::error!(
                                                        "数据回滚失败{:#?}",
                                                        rollback_r.err()
                                                    );
                                                };
                                            }
                                        }
                                        r
                                    }
                                    Err(e) => {
                                        log::error!("Tantivy查询失败{e:?}");
                                        ActionResult {
                                            success: false,
                                            err: Some(format!("{e:?}")),
                                            result: Vec::new(),
                                        }
                                    }
                                };
                                if let Err(err) = action.sender.send(r) {
                                    log::error!("信息发送失败{err:?}")
                                }
                            });
                        }
                        // TantivyActionType::Stop => break,
                        TantivyActionType::BatchAdd(list) => {
                            // deleteAll
                            tokio::task::spawn(async move {
                                // 获取文档写锁
                                let mut lock_config_index_writer =
                                    config_index_writer.write().await;
                                // 获取reader读锁
                                let lock_config_reader = config_reader.read().await;
                                // lock_config_index_writer.rollback();
                                let mut r = ActionResult {
                                    success: true,
                                    ..Default::default()
                                };
                                for c in list {
                                    // 新增文档
                                    let mut doc_new = Document::default();
                                    doc_new.add_text(*title_filed, c.title);
                                    doc_new.add_u64(*id_filed, c.id);
                                    c.body.iter().for_each(|i| {
                                        doc_new.add_text(*body_filed, i);
                                    });
                                    let frankenstein_id = Term::from_field_u64(*id_filed, c.id);
                                    let frankenstein_doc_misspelled =
                                        extract_doc_given_id(&lock_config_reader, &frankenstein_id);
                                    // 清除旧文档
                                    match frankenstein_doc_misspelled {
                                        Ok(doc) => {
                                            // 获取读写锁
                                            if doc.is_some() {
                                                lock_config_index_writer
                                                    .delete_term(frankenstein_id);
                                            }

                                            match lock_config_index_writer.add_document(doc_new) {
                                                Ok(_) => continue,
                                                Err(e) => {
                                                    r = ActionResult {
                                                        success: false,
                                                        err: Some(format!("{e:?}")),
                                                        ..Default::default()
                                                    };
                                                    break;
                                                }
                                            };
                                            // 释放读写锁
                                        }
                                        Err(e) => {
                                            log::error!("Tantivy查询失败{e:?}");
                                            r = ActionResult {
                                                success: false,
                                                err: Some(format!("{e:?}")),
                                                result: Vec::new(),
                                            };
                                            break;
                                        }
                                    };
                                }
                                if let Err(err) = lock_config_index_writer.commit() {
                                    let rollback_r = lock_config_index_writer.rollback();
                                    if rollback_r.is_err() {
                                        log::error!("数据回滚失败{:#?}", rollback_r.err());
                                    };
                                    r.success = false;
                                    r.err = Some(format!("{err:?}"));
                                    log::error!("添加文档提交失败{err:?}");
                                }
                                if let Err(err) = action.sender.send(r) {
                                    log::error!("信息发送失败{err:?}")
                                }
                            });
                        }
                        TantivyActionType::DeleteAll => {
                            let mut r = ActionResult {
                                success: true,
                                err: None,
                                result: Vec::new(),
                            };
                            // 获取读写锁
                            {
                                let mut lock_config_index_writer =
                                    config_index_writer.write().await;
                                if let Err(err) = lock_config_index_writer.delete_all_documents() {
                                    r.success = false;
                                    r.err = Some(format!("{err:?}"));
                                    log::error!("删除全部文档失败{err:?}");
                                } else if let Err(err) = lock_config_index_writer.commit() {
                                    let rollback_r = lock_config_index_writer.rollback();
                                    if rollback_r.is_err() {
                                        log::error!("数据回滚失败{:#?}", rollback_r.err());
                                    };
                                    r.success = false;
                                    r.err = Some(format!("{err:?}"));
                                    log::error!("删除全部文档提交失败{err:?}");
                                }
                            }
                            // 释放读写锁
                            if let Err(err) = action.sender.send(r) {
                                log::error!("信息发送失败{err:?}")
                            }
                        }
                    }
                }
            }
        });
        true
    }
}

pub struct TantivyConfig {
    pub(crate) index_writer: Arc<RwLock<IndexWriter>>,
    pub(crate) schema: Schema,
    pub(crate) reader: Arc<RwLock<IndexReader>>,
    pub(crate) index: Arc<Index>,
}

pub fn tantivy_init() -> tantivy::Result<TantivyConfig> {
    let index_path = std::path::Path::new("index");
    if !index_path.exists() {
        std::fs::create_dir_all(index_path).unwrap();
    }
    // 创建表
    let mut schema_builder = Schema::builder();
    // 创建分词器
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("jieba")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    // add_text_field
    schema_builder.add_text_field("title", text_options.clone());
    schema_builder.add_text_field("body", text_options);
    schema_builder.add_u64_field("index", INDEXED | STORED);
    // 创建
    let schema = schema_builder.build();
    // 初始化索引器 创建或者重用
    let directory = tantivy::directory::MmapDirectory::open(index_path)?;
    let index = tantivy::Index::open_or_create(directory, schema.clone())?;
    // 初始化分词器
    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    index.tokenizers().register("jieba", tokenizer);
    // To insert a document we will need an index writer.
    // There must be only one writer at a time.
    // This single `IndexWriter` is already
    // multithreaded.
    //
    // Here we give tantivy a budget of `50MB`.
    // Using a bigger memory_arena for the indexer may increase
    // throughput, but 50 MB is already plenty.
    // 给他500m
    let index_writer = index.writer(500_000_000)?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    Ok(TantivyConfig {
        index_writer: Arc::new(RwLock::new(index_writer)),
        schema,
        reader: Arc::new(RwLock::new(reader)),
        index: Arc::new(index),
    })
}

// format方式
// fn highlight(snippet: Snippet) -> String {
//     let mut result = String::new();
//     let mut start_from = 0;

//     for fragment_range in snippet.highlighted() {
//         result.push_str(&snippet.fragment()[start_from..fragment_range.start]);
//         result.push_str(" --> ");
//         result.push_str(&snippet.fragment()[fragment_range.clone()]);
//         result.push_str(" <-- ");
//         start_from = fragment_range.end;
//     }

//     result.push_str(&snippet.fragment()[start_from..]);
//     result
// }
