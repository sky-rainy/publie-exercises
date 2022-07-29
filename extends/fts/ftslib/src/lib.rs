use std::ffi::{CStr, CString};
mod core;
mod message;
mod mlog;

use anyhow::Result;
use message::{TantivyAction, TantivyActionResult, TantivyActionType};
// use message::Contents;
// use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, RwLock};

// fts是否已经初始化
static mut INIT_FTS: bool = false;
lazy_static::lazy_static! {
   static ref ACTION_SENDER: RwLock<Option<mpsc::UnboundedSender<TantivyAction>>> = RwLock::new(None);
   static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}
// 初始化运行时 0 失败 1 成功
#[no_mangle]
pub extern "C" fn init_fts() -> libc::c_int {
    mlog::setup_logging(0).unwrap();
    let result = TOKIO_RUNTIME.block_on(async {
        let (sender, rx) = mpsc::unbounded_channel();
        ACTION_SENDER.write().await.replace(sender);
        core::run(rx).await
    });
    if result {
        unsafe { INIT_FTS = result }
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn delete_all() -> *const libc::c_char {
    let r = match task_action(TantivyActionType::DeleteAll) {
        Ok(v) => v,
        Err(e) => TantivyActionResult {
            success: false,
            err: Some(e.to_string()),
            result: None,
        },
    };
    let r_str = serde_json::to_string(&r).unwrap();
    CString::new(r_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn query(query: *const libc::c_char) -> *const libc::c_char {
    let cstr_query = unsafe { CStr::from_ptr(query) };
    let str_query = cstr_query.to_str().unwrap().to_string();
    let r = match task_action(TantivyActionType::Query(str_query)) {
        Ok(v) => v,
        Err(e) => TantivyActionResult {
            success: false,
            err: Some(e.to_string()),
            result: None,
        },
    };
    let r_str = serde_json::to_string(&r).unwrap();
    CString::new(r_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn update(content: *const libc::c_char) -> *const libc::c_char {
    let cstr_query = unsafe { CStr::from_ptr(content) };
    let str_content = cstr_query.to_str().unwrap();
    let contents = serde_json::from_str(str_content).unwrap();
    let r = match task_action(TantivyActionType::Update(contents)) {
        Ok(v) => v,
        Err(e) => TantivyActionResult {
            success: false,
            err: Some(e.to_string()),
            result: None,
        },
    };
    let r_str = serde_json::to_string(&r).unwrap();
    CString::new(r_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn delete_by_id(id: libc::c_ulonglong) -> *const libc::c_char {
    let r = match task_action(TantivyActionType::Delete(id)) {
        Ok(v) => v,
        Err(e) => TantivyActionResult {
            success: false,
            err: Some(e.to_string()),
            result: None,
        },
    };
    let r_str = serde_json::to_string(&r).unwrap();
    CString::new(r_str).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn batch_add(contents: *const libc::c_char) -> *const libc::c_char {
    let cstr_query = unsafe { CStr::from_ptr(contents) };
    let str_contents = cstr_query.to_str().unwrap();
    let v_contents = serde_json::from_str(str_contents).unwrap();
    let r = match task_action(TantivyActionType::BatchAdd(v_contents)) {
        Ok(v) => v,
        Err(e) => TantivyActionResult {
            success: false,
            err: Some(e.to_string()),
            result: None,
        },
    };
    let r_str = serde_json::to_string(&r).unwrap();
    CString::new(r_str).unwrap().into_raw()
}

// 执行task
fn task_action(action_type: TantivyActionType) -> Result<TantivyActionResult> {
    unsafe {
        if !INIT_FTS {
            return Err(anyhow::format_err!("未初始化runtime"));
        }
    }
    TOKIO_RUNTIME.block_on(async {
        log::info!("执行task");
        let (sender, receiver) = oneshot::channel();
        let action = TantivyAction {
            action_type,
            sender,
        };
        let r = ACTION_SENDER.read().await.as_ref().unwrap().send(action);
        if r.is_err() {
            let err_message = format!("发送action失败;失败原因是 {:#?}", r.err());
            log::error!("{err_message}");
            return Err(anyhow::format_err!("{err_message}"));
        }
        match receiver.await {
            Ok(value) => Ok(value),
            Err(_) => {
                log::error!("sender dropped");
                Err(anyhow::format_err!("sender dropped"))
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn free_cstring(s: *mut libc::c_char) {
    unsafe {
        let _ = CString::from_raw(s);
    }
}
