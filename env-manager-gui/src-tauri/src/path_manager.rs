use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::init::{EnvManager, get_home_directory, write_string_to_file};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEnvParam {
    pub uid: String,
    pub la: String,
    pub path: String,
    pub alias: String,
    pub usingss: bool,      // 表示路径是否正在使用
}


#[tauri::command]
pub fn load_data() -> Vec<AddEnvParam> {
    return read_config_json();
}
#[tauri::command]
pub fn delete_by_uid(uid:String,index:usize) -> bool {
    let mut vec = read_config_json();
    vec.remove(index);
    let json = serde_json::to_string(&vec).expect("Failed to serialize to JSON");
    write_config_json(json.as_str());
    return true;
    // return read_config_json();
}

fn generate_random_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}
#[tauri::command]
pub fn add_la(mut param: AddEnvParam) -> bool {
    println!("{:?}", param.path);
    let mut vec = read_config_json();
    param.uid = format!("{}", generate_random_uuid());
    vec.push(param);
    let json = serde_json::to_string(&vec).expect("Failed to serialize to JSON");


    write_config_json(json.as_str());
    return true;
}

#[tauri::command]
pub fn using_path(index: usize, bol: bool) -> bool {
    let mut vec = read_config_json();
    // 调整配置文件
    let mut envVec: Vec<EnvManager> = Vec::new();
    ///// todo: 检查一个语言只能有一个可以使用


    let mut curLa = String::new();
    let mut curUid = String::new();

    // // 使用额外的作用域块，以便在此块中获取可变引用
    if let Some(item) = vec.get_mut(index) {
        curLa = format!("{}", item.la);
        curUid = format!("{}", item.uid);
        item.usingss = bol;
    }

    for x in &mut vec {
        if x.la == curLa && x.uid != curUid {
            x.usingss = false;
        }
    }



    let json = serde_json::to_string(&vec).expect("Failed to serialize to JSON");
    write_config_json(json.as_str());
    for x in &vec {
        if x.usingss {
            let name = crate::init::EnvKeyName::from_la(x.la.as_str());
            envVec.push(EnvManager::new(name, x.path.as_str()));
        };
    }


    let mut all_results = String::new();

    for x in envVec {
        // 将 source_string 结果追加到字符串
        all_results.push_str(&x.source_string());
    }

    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        if let Err(err) = write_string_to_file(format!("{}/env_manager/customer", home_directory).as_str(), all_results.as_str()) {
            eprintln!("Error: {}", err);
        } else {
            println!("String written to file successfully.");
        }
    }
    println!("{}", all_results);
    return true;
}


/// 读取配置文件
pub fn read_config_json() -> Vec<AddEnvParam> {
    if let Ok(home_directory) = crate::init::get_home_directory().ok_or("无法获取 HOME 目录") {
        let config_path = format!("{}/env_manager/config.json", home_directory);
        if let Ok(file_contents) = crate::init::read_file_content(config_path.as_str()) {
            if let Ok(parsed_data) = serde_json::from_str::<Vec<AddEnvParam>>(&file_contents) {
                return parsed_data;
            } else {
                eprintln!("Error parsing JSON data from the config file");
            }
        } else {
            eprintln!("Error reading file content from the config file");
        }
    }
    Vec::new() // Return an empty vector if there's any error
}

/// 写配置文件
pub fn write_config_json(data: &str) {
    if let Ok(home_directory) = crate::init::get_home_directory().ok_or("无法获取 HOME 目录") {
        crate::init::write_string_to_file(format!("{}/env_manager/config.json", home_directory).as_str(), data).expect("写入配置文件失败");
    }
}




