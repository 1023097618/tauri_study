# rust的安装
1去rust[官网](https://www.rust-lang.org/zh-TW/tools/install)进行安装，在下载下来`rustup-init.exe`文件之后点击运行，输入1，等待安装

>**tip**:可以使用以下命令来加速
``` bat
set http_proxy=http://localhost:7891
set https_proxy=http://localhost:7891
rustup-init.exe
```
这边的http_proxy和https_proxy设成你的代理服务器地址

---
# rust基本命令行操作
### 查看rust版本
``` bat
rustc --version
```
### 查看帮助文档
``` bat
rustup doc
```
### 编译相关文件
``` bat
rustc file
```

---
# Cargo
通过cargo我们可以编译多文件
通过以下命令验证cargo的安装
``` bat
cargo --version
```

通过以下命令使用cargo创建项目
``` bat
cargo new project_name
```

其中表示配置的文件是`Cargo.toml`
通过以下命令使用cargo构建项目
``` bat
cargo build
```
通过以下命令使用cargo构建项目（发行版本）
``` bat
cargo build --release
```

通过以下命令构建并且运行项目(如果有构建过且源代码没改变就执行target中的文件)
``` bat
cargo run
```

使用这个命令保证此项目能通过编译
``` bat
cargo check
```