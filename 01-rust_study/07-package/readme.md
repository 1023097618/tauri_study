# package
`package`指的是你用`cargo new`出来的整个项目

# crate
约定俗成得

`binary crate`的`crate root`的路径是`src/main.rs`

`library crate`的`crate root`的路径是`src/lib.rs`

> 他讲得这块有点抽象，没听懂

# 绝对路径和相对路径调用(super、use、as)
在这段代码中

`crate::front_service::service::add_room();`是绝对路径调用

`front_service::service::add_room()`是相对路径调用
``` rust
mod front_service{
    mod service{
        fn add_room(){

        }
    }
}
fn order_room(){
    //absolute path calling
    crate::front_service::service::add_room();
    //relative path calling
    front_service::service::add_room();
}
```

进一步的，还可以使用`super`来调用父级，相当于操作系统中的`../`
``` rust
mod front_service{
    pub mod service{
        pub fn add_room(){
            super::super::order_room()
        }
    }
}
fn order_room(){
    
    //absolute path calling
    crate::front_service::service::add_room();
    //relative path calling
    front_service::service::add_room();
}
```

使用`use`关键字来引入，可以不用重复书写前缀，和java很像
``` rust
mod front_service{
    pub mod service{
        pub fn add_room(){
            super::super::order_room()
        }
    }
}
use crate::front_service::service;
fn order_room(){
    service::add_room();
}

```
使用`as`可以为导入的模块取别名，像python

使用`{}`来防止同一模块下的不同包的重复书写，比如以下代码，上方和下方是等价的
``` rust
use std::cmp::Ordering;
use std::io;
```
``` rust
use std::{cmp::Ordering,io};
```
使用`self`来进行自引用，比如下方的代码，上方和下方是等价的
``` rust
use std::io;
use std::io::Write;
```
``` rust
use std::io::{self,Write};
```

# pub关键字
文件根级的mod默认是公共可调用的，但是如果mod下面的mod就可能是私有的

以下代码加上了`pub`关键字，暴露了模块，从而使其可以被调用
``` rust
mod front_service{
    pub mod service{
        pub fn add_room(){

        }
    }
}
fn order_room(){
    
    //absolute path calling
    crate::front_service::service::add_room();
    //relative path calling
    front_service::service::add_room();
}
```
除此之外，`struct`中的字段也默认是私有的，如果希望外部访问的话，需要在字段前加上`pub`关键字，这和java很像

# 模块的导出
如果想导出一些模块，让其他文件也可以访问到这个模块，可以使用`pub use`关键字

通过这种方式，就导出了`crate::front_service::service`这个模块，也将`crate::front_service::service`导入到了当前的作用域中
``` rust
mod front_service{
    pub mod service{
        pub fn add_room(){
            super::super::order_room()
        }
    }
}
pub use crate::front_service::service;
```

# 模块依赖的导入
按照以下步骤进行
1. 在`Cargo.toml`的`[dependencies]`下可以指定导入的包，指定完之后，会从`https://crates.io/`对包进行下载、保存
2. (如果卡了)在vscode顶端的输入框输入`>rust-analyzer:Stop Server`，找到那个选项回车，没找到就是没装这个vscode插件。
3. (上接2)此时尝试一下`cargo build`命令，如果没动静，那就输入`where cargo`，比如说我这边输出的是`C:\Users\10230\.cargo\bin\cargo.exe`，那我进入`C:\Users\10230\.cargo`目录下，删掉其中的隐藏文件`.package-cache`
4. 如果需要cargo的镜像，可以参考[这个视频](https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.788.videopod.episodes&vd_source=a3e79c6044a74cfda15dd96a1d025f20&p=31)的`03:34`秒进行相关的配置
5. 使用`use rand::Rng;`进行模块的相关导入

# 模块文件的定义
在`lib.rs`中使用这种方法进行模块文件导入
``` rust
mod front_service;
```
然后在`src`目录下创建`front_service.rs`，里面代码如下
``` rust
pub mod service;
```
然后创建`front_service`文件夹，下方创建`service.rs`文件，里面代码如下
``` rust
pub fn add_room(){
    super::super::order_room()
}
```