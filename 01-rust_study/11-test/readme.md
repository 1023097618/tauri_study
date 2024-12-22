# rust中的测试函数
在函数上方加上`#[test]`以将函数标注为测试函数

使用`cargo new test --lib`来创建一个名为`test`的库项目

我们可以看到`lib.rs`中已经写好的文件如下
``` rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

使用`cargo test`来运行测试函数,每个测试函数会运行在一个独立的线程

如果`assert_eq!`左值和右值不相等，则为失败。或者如果触发了`panic`也为失败。
``` rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_failuers(){
        panic!("fail");
    }
}
```
使用`assert!`宏后面参数直接跟一个bool类型也能表示是否成功

我从看完11.1没有继续看下去了，如果未来哪一天我需要学这部分的内容请从这里开始看[11.2](https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.788.videopod.episodes&vd_source=a3e79c6044a74cfda15dd96a1d025f20&p=52)看到[11.10](https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.788.videopod.episodes&vd_source=a3e79c6044a74cfda15dd96a1d025f20&p=60)