# 1.3 Hello Cargo

cargo 是 rust 自带的**构建系统**和**包管理器**（比 C++ 好到不知道哪里去了，基本相当于 cmake + vcpkg）。

- 检查版本：`cargo --version`

- 创建式初始化：`cargo new hello_cargo`

  > `Cargo.toml`：包名 + 依赖
  >
  > git 仓库

- cargo build：编译

- cargo run：编译 + 运行

- cargo check：仅语法检查，不编译，比 build 快

