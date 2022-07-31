use std::ffi::CStr;
mod core;
mod fts;
mod message;
mod mlog;

use anyhow::Result;
use message::{ActionResult, BatchContents, Contents, TantivyAction, TantivyActionType};
use prost::Message;
use tokio::sync::{mpsc, oneshot, RwLock};

// fts是否已经初始化
static mut INIT_FTS: bool = false;
lazy_static::lazy_static! {
   static ref ACTION_SENDER: RwLock<Option<mpsc::UnboundedSender<TantivyAction>>> = RwLock::new(None);
   static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: u64,
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
pub extern "C" fn delete_all() -> Buffer {
    let r = match task_action(TantivyActionType::DeleteAll) {
        Ok(v) => v,
        Err(e) => ActionResult {
            success: false,
            err: Some(e.to_string()),
            result: Vec::new(),
        },
    };
    let mut r_vec = r.encode_to_vec();
    let data = r_vec.as_mut_ptr();
    let len = r_vec.len() as u64;
    std::mem::forget(r_vec);
    Buffer { data, len  }
}
#[no_mangle]
pub  extern "C" fn query(query: *const libc::c_char) -> Buffer {
    let str_query = unsafe {
        CStr::from_ptr(query).to_str().unwrap().to_string()
    };
    let r = match task_action(TantivyActionType::Query(str_query)) {
        Ok(v) => v,
        Err(e) => ActionResult {
            success: false,
            err: Some(e.to_string()),
            result: Vec::new(),
        },
    };
    let mut r_vec = r.encode_to_vec();

    let data = r_vec.as_mut_ptr();
    let len = r_vec.len() as u64;
    std::mem::forget(r_vec);
    Buffer { data, len }
}



#[no_mangle]
pub extern "C" fn update(content_buf: Buffer) -> Buffer {
    let content_body = unsafe { std::slice::from_raw_parts_mut(content_buf.data, content_buf.len as usize) };
    let r = match Contents::decode(&*content_body) {
        Ok(v) => match task_action(TantivyActionType::Update(v)) {
            Ok(v) => v,
            Err(e) => ActionResult {
                success: false,
                err: Some(e.to_string()),
                result: Vec::new(),
            },
        },
        Err(e) => ActionResult {
            success: false,
            err: Some(e.to_string()),
            result: Vec::new(),
        },
    };
    let mut r_vec = r.encode_to_vec();
    let data = r_vec.as_mut_ptr();
    let len = r_vec.len() as u64;
    std::mem::forget(r_vec);
    Buffer { data, len }
}

#[no_mangle]
pub extern "C" fn delete_by_id(id: libc::c_ulonglong) -> Buffer {
    let r = match task_action(TantivyActionType::Delete(id)) {
        Ok(v) => v,
        Err(e) => ActionResult {
            success: false,
            err: Some(e.to_string()),
            result: Vec::new(),
        },
    };
    let mut r_vec = r.encode_to_vec();

    let data = r_vec.as_mut_ptr();
    let len = r_vec.len() as u64;
    std::mem::forget(r_vec);
    Buffer { data, len }
}
#[no_mangle]
pub extern "C" fn batch_add(content_buf: Buffer) -> Buffer {
    let content_body = unsafe { std::slice::from_raw_parts(content_buf.data, content_buf.len as usize) };
    let r = match BatchContents::decode(&*content_body) {
        Ok(v) => match task_action(TantivyActionType::BatchAdd(v.contents)) {
            Ok(v) => v,
            Err(e) => ActionResult {
                success: false,
                err: Some(e.to_string()),
                result: Vec::new(),
            },
        },
        Err(e) => ActionResult {
            success: false,
            err: Some(e.to_string()),
            result: Vec::new(),
        },
    };
    let mut r_vec = r.encode_to_vec();
    let data = r_vec.as_mut_ptr();
    let len = r_vec.len() as u64;
    std::mem::forget(r_vec);
    Buffer { data, len }
}

// 执行task
fn task_action(action_type: TantivyActionType) -> Result<ActionResult> {
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
pub extern "C" fn free_bytes(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}
