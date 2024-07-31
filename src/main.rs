#![allow(non_snake_case)]
#![allow(unused_assignments)]

use anyhow::{anyhow, Result};
// use std::path::{Path, PathBuf};
// use std::rc::Rc;

// use std::fs::{self, File};
// use std::io::Read;
// use toml::Value;
// use tracing::{debug, error, info, warn};

// use chrono::{Local, Timelike, Duration};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    //日志初始化
    // let file_appender = tracing_appender::rolling::daily("./log", "tracing.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // logging(non_blocking);
    // 根据输入的文档路径，加载文档内容，并写入到编辑器中
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    // let app = App::new().unwrap();
    let text = "Hello, world!".to_string();
    // app.set_text(text);

    ui.run()
}

// 读取文件内容

// fn read_content(s: &str) -> Result<Vec<String>> {
//     let mut v = Vec::new();
//     v.push("Hello, world!".to_string());
//     Ok(v)
// }
