use std::{env, fmt};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

use log::{error, info, LevelFilter};
use serde::{Deserialize, Serialize};

// 定义枚举表示不同的 Shell
#[derive(Debug)]
enum ShellType {
    Bash,
    Zsh,
    Other(String),
}

impl ShellType {
    // 方法用于获取用户配置文件的路径
    pub fn config_file_path(&self) -> Result<String, String> {
        let home_directory = get_home_directory().ok_or("无法获取 HOME 目录")?;

        match self {
            ShellType::Bash => Ok(format!("{}/.bashrc", home_directory)),
            ShellType::Zsh => Ok(format!("{}/.zshrc", home_directory)),
            ShellType::Other(ref shell_str) => Err(format!("无法确定 Shell 类型: {}", shell_str)),
        }
    }
}

// 函数用于判断当前使用的 Shell
fn detect_shell() -> ShellType {
    // 获取 $SHELL 环境变量的值
    if let Some(shell_path) = env::var_os("SHELL") {
        let shell_str = shell_path.to_string_lossy().to_lowercase();

        // 判断当前 Shell 类型
        if shell_str.contains("bash") {
            ShellType::Bash
        } else if shell_str.contains("zsh") {
            ShellType::Zsh
        } else {
            ShellType::Other(shell_str.into())
        }
    } else {
        // 未能获取 $SHELL 环境变量
        ShellType::Other("unknown".into())
    }
}


pub fn read_env_config() -> Vec<EnvManager> {
    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        if let Ok(file_contents) = read_file_content(format!("{}/env_manager/customer", home_directory).as_str()) {
            let vec = parse_source_lines(file_contents.as_str());


            return vec;
        };
    };
    return Vec::new();
}

pub fn init_pj() {
    env_logger::Builder::from_default_env().filter_level(LevelFilter::Info).init();


    setup();

    // if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
    //     if let Ok(file_contents) = read_file_content(format!("{}/env_manager/customer", home_directory).as_str()) {
    //         let vec = parse_source_lines(file_contents.as_str());
    //
    //
    //         let mut all_results = String::new();
    //
    //         for x in vec {
    //             // 将 source_string 结果追加到字符串
    //             all_results.push_str(&x.source_string());
    //         }
    //
    //         // 打印合并后的字符串
    //         println!("{}", all_results);
    //
    //
    //         ;
    //         if let Err(err) = write_string_to_file(format!("{}/env_manager/customer", home_directory).as_str(), all_results.as_str()) {
    //             eprintln!("Error: {}", err);
    //         } else {
    //             println!("String written to file successfully.");
    //         }
    //     };
    // };
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EnvManager {
    pub name: EnvKeyName,
    pub value: String,
}


impl EnvManager {
    pub(crate) fn new(p0: EnvKeyName, p1: &str) -> EnvManager {
        return EnvManager {
            name: p0,
            value: p1.to_string(),
        };
    }
    pub fn source_string(&self) -> String {
        format!("export {}={}\nexport {}_BIN={}/bin", self.name, self.value, self.name, self.value)
    }
}

fn parse_source_lines(text: &str) -> Vec<EnvManager> {
    text.lines().filter_map(|line| {
        let parts: Vec<&str> = line.trim().splitn(2, '=').collect();
        if parts.len() == 2 {
            let name = parts[0].trim_start_matches("export").trim();

            Some(EnvManager {
                name: EnvKeyName::from_str(name).unwrap(),
                value: parts[1].trim().to_string(),
            })
        } else {
            None
        }
    }).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnvKeyName {
    TestHome,
    JavaHome,
    PythonHome,
    MavenHome,
    GradleHome,
    RustHome,
    NodeHome,
    GoHome,
    NONO,
}

impl EnvKeyName {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "TEST_HOME" => Some(EnvKeyName::TestHome),
            "JAVA_HOME" => Some(EnvKeyName::JavaHome),
            "PYTHON_HOME" => Some(EnvKeyName::PythonHome),
            "MAVEN_HOME" => Some(EnvKeyName::MavenHome),
            "GRADLE_HOME" => Some(EnvKeyName::GradleHome),
            "RUST_HOME" => Some(EnvKeyName::RustHome),
            "NODE_HOME" => Some(EnvKeyName::NodeHome),
            "GO_HOME" => Some(EnvKeyName::GoHome),
            _ => None,
        }
    }

    pub fn from_la(s: &str) -> EnvKeyName {
        match s {
            "Java" => EnvKeyName::JavaHome,
            "Python" => EnvKeyName::PythonHome,
            "Maven" => EnvKeyName::MavenHome,
            "Gradle" => EnvKeyName::GradleHome,
            "Rust" => EnvKeyName::GradleHome,
            "Node" => EnvKeyName::NodeHome,
            "Go" => EnvKeyName::GoHome,
            _ => EnvKeyName::NONO,
        }
    }
    pub fn to_lag(&self) -> &str {
        match self {
            EnvKeyName::TestHome => "test",
            EnvKeyName::JavaHome => "Java",
            EnvKeyName::PythonHome => "Python",
            EnvKeyName::MavenHome => "Maven",
            EnvKeyName::GradleHome => "Gradle",
            EnvKeyName::NodeHome => "Node",
            EnvKeyName::GoHome => "Go",
            EnvKeyName::NONO => "NONO",
            EnvKeyName::RustHome => { "Rust" }
        }
    }
}

impl fmt::Display for EnvKeyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            EnvKeyName::TestHome => "TEST_HOME",
            EnvKeyName::JavaHome => "JAVA_HOME",
            EnvKeyName::PythonHome => "PYTHON_HOME",
            EnvKeyName::MavenHome => "MAVEN_HOME",
            EnvKeyName::GradleHome => "GRADLE_HOME",
            EnvKeyName::NodeHome => "NODE_HOME",
            EnvKeyName::RustHome => "RUST_HOME",
            EnvKeyName::GoHome => "GO_HOME",
            EnvKeyName::NONO => { "NONO" }
        };
        write!(f, "{}", name)
    }
}


/// 初始化程序，创建一些所需文件
fn setup() {
// 调用函数检测当前 Shell
    let current_shell = detect_shell();
    let _sh = "source ~/env_manager/customer";


    if let Ok(home_directory) = get_home_directory().ok_or("无法获取 HOME 目录") {
        let directory_path = format!("{}/env_manager", home_directory);
        if let Err(err) = create_directory_if_not_exists(directory_path.as_str()) {
            eprintln!("Error: {}", err);
        } else {
            println!("程序执行完成。");
        }
        create_file_if_not_exists(format!("{}/env_manager/customer", home_directory).as_str(), "").expect("创建配置文件失败");
        create_file_if_not_exists(format!("{}/env_manager/config.json", home_directory).as_str(), "[]").expect("创建配置文件失败");
    }


    // 根据枚举值输出结果
    match current_shell {
        ShellType::Bash => info!("当前使用的是 Bash Shell"),
        ShellType::Zsh => info!("当前使用的是 Zsh Shell"),
        ShellType::Other(ref shell_str) => {
            info!("当前使用的是其他 Shell: {}", shell_str);
        }
    }


    // 调用函数获取用户主目录
    if let Ok(file_path) = current_shell.config_file_path() {
        // 读取文件内容
        if let Ok(file_contents) = read_file_content(file_path.as_str()) {
            // Log the file contents
            info!("文件内容: {}", file_contents);

            // Check if the file contents contain specific text
            let has_text = content_has_text(file_contents.as_str());
            if has_text {
                info!("文件内容包含指定文本");
            } else {
                info!("文件内容不包含指定文本");

                if let Err(err) = append_to_file(file_path.as_str(), _sh) {
                    eprintln!("Error: {}", err);
                } else {
                    println!("Text appended to file successfully.");
                }
            }
        } else {
            error!("无法读取文件内容");
        }
    } else {
        error!("无法获取用户配置文件路径");
    }
}

pub(crate) fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    info!("文件地址: {}", file_path);
    // 尝试打开文件
    let file_contents = fs::read_to_string(file_path)?;

    // 返回文件内容
    Ok(file_contents)
}

fn content_has_text(file_contents: &str) -> bool {
    let _sh = "source ~/env_manager/customer";
    return file_contents.contains(_sh);
}

pub(crate) fn get_home_directory() -> Option<String> {
    env::var_os("HOME").map(|path| path.to_string_lossy().into_owned())
}


fn append_to_file(file_path: &str, text: &str) -> Result<(), io::Error> {
    // 以追加模式打开文件
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(file_path)?;

    // 获取文件大小
    let file_size = file.metadata()?.len();

    // 如果文件不为空，则在写入前添加换行符
    if file_size > 0 {
        writeln!(file)?;
    }

    // 写入文本
    write!(file, "{}", text)?;

    Ok(())
}


fn create_directory_if_not_exists(dir_path: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(dir_path).is_ok() {
        // 如果文件夹不存在，创建它
        fs::create_dir(dir_path)?;
        println!("文件夹 '{}' 不存在，已创建。", dir_path);
    } else {
        println!("文件夹 '{}' 已经存在。", dir_path);
    }
    Ok(())
}

fn create_file_if_not_exists(file_path: &str, content: &str) -> Result<(), io::Error> {
    // 检查文件是否存在
    if fs::metadata(file_path).is_ok() {
        println!("文件 '{}' 已经存在。", file_path);
        return Ok(());
    }

    // 如果文件不存在，创建文件并写入内容
    let mut file = File::create(file_path)?;

    // 写入内容到文件
    file.write_all(content.as_bytes())?;

    println!("文件 '{}' 已创建并写入内容。", file_path);

    Ok(())
}

pub(crate) fn write_string_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    // 以写入模式打开文件
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;


    // 写入字符串到文件
    write!(file, "{}", content)?;

    Ok(())
}