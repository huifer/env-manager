// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod path_manager;
mod init;

use std::process::Command;
use crate::path_manager::{load_data, using_path,delete_by_uid};
use crate::path_manager::add_la;
use crate::init::init_pj;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    source("/Users/zhangsan/.bashrc.d/custom");
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn source(path: &str) {
    // 在新的 shell 进程中执行 source 命令
    let result = Command::new("sh")
        .arg("-c")
        .arg(format!("source {}",  path))
        .status();

    match result {
        Ok(status) => {
            if status.success() {
                println!("source 命令执行成功");
            } else {
                println!("source 命令执行失败");
            }
        }
        Err(err) => {
            println!("错误: {}", err);
        }
    }
}
fn main() {
    init_pj();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,load_data,add_la,using_path,delete_by_uid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
