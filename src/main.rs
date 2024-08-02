#![allow(non_snake_case)]
#![allow(unused_assignments)]

use anyhow::{anyhow, Result};
// use std::fs::File;
use std::fs;
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
    let text = read_content("log.txt").unwrap();

    ui.on_show_text({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut i = ui.get_page_num();
            if i < 0 {
                i = 0;
            }
            let i = i as usize;
            println!("page_num: {}", i);
            let txt: String = text[i * 1024..(i + 1) * 1024].join("\n");
            ui.set_content(txt.into());
        }
    });
    // ui.on_prev_page({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let i = ui.get_page_num() as usize;
    //         println!("page_num: {}", i);
    //         let txt: String = &text[i * 1024..(i + 1) * 1024].join("\n").clone_into();
    //         ui.set_content(txt.into());
    //     }
    // });
    // ui.on_next_page({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let i = ui.get_page_num() as usize;
    //         println!("page_num: {}", i);
    //         let txt: &String = &text[i * 1024..(i + 1) * 1024].join("\n");
    //         ui.set_content(txt.into());
    //     }
    // });

    ui.run()
}

// 读取文件内容
fn read_content(s: &str) -> Result<Vec::<String>> {
    // 打开文件并读取内容
    let content: String = fs::read_to_string(s)?;
    // 按行分割内容
    let v: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    Ok(v)
}