# Env Manager
可视化环境切换工具。

## 原理说明

核心操作逻辑是给Linux设置系统环境变量。

本软件在启动后会创建目录 `$HOME/env_mananger` 目录，创建 `customer` 文件和 `config.json` 文件

其中`$HOME/env_manager/customer`文件会写入到 `~/.zshrc`  或者 `~/.bashrc` 。

>   注意：写入到哪个文件和你当前的shell环境有关。
>


## 操作手册

1. 通过点击增加环境按钮弹出窗口进行环境录入

![image-20240118133108045](images/image-20240118133108045.png)

2.   点击启用环境完成启用。

>   注意：**每个语言只能有一个环境进行使用，系统会自动处理。**

![image-20240118133144371](images/image-20240118133144371.png)

3.   验证

>   注意：每次切换环境后需要重新打开终端。修改环境后需要重启。

![image-20240118133235819](images/image-20240118133235819.png)

已上图为例现在环境是JDK8，打开终端执行`java -version`

```
java -version
openjdk version "1.8.0_312"
OpenJDK Runtime Environment (Zulu 8.58.0.13-CA-macosx) (build 1.8.0_312-b07)
OpenJDK 64-Bit Server VM (Zulu 8.58.0.13-CA-macosx) (build 25.312-b07, mixed mode)
```

将JDK17设置为启用，并且打开新的终端 执行`java -version`

```
java -version
java version "17.0.1" 2021-10-19 LTS
Java(TM) SE Runtime Environment (build 17.0.1+12-LTS-39)
Java HotSpot(TM) 64-Bit Server VM (build 17.0.1+12-LTS-39, mixed mode, sharing)
```

