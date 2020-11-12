<br>

<p align="center">
<img src="https://picb.zhimg.com/v2-cb1db68b184ed26bc6e2ff0b3108a827_1440w.jpg?source=172ae18b" alt="Rust">
</p>

<p align="center">高性能 • 轻量级 • 命令行 • Tokio异步IO</p>

# RUST
rust-cli

## 实现说明

为了学习 rust 基于 Tokio 异步IO系统之上构建的高级cli 

## 环境要求

需要 Rust1.39+


## 框架定位

绝对性能优先 基于Tokio异步IO的无栈协成

## 安装运行

```
cargo run test
```

## 使用文档

[文档地址](https://actix.rs/docs/)

##如果一切顺利，运行到最后你将看到如下的输出：
```
C:\Users\Administrator\Desktop\github\rust-cli>cargo run test
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target\debug\rust-cli.exe test`
spawned thread print 0
spawned thread print 3
spawned thread print 1
spawned thread print 2
spawned thread print 5
spawned thread print 8
spawned thread print 6
spawned thread print 7
spawned thread print 4
spawned thread print 9
Result  : 20
Result1  : 2000
success

```


## License

Apache License Version 2.0, http://www.apache.org/licenses/