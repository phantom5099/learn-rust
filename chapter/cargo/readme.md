# Cargo
## 什么是cargo
rust的构建系统和包管理器

## cargo的命令
- `cargo new <project_name>`：创建新的rust项目
- `cargo build`：构建项目
  - `cargo build --release`：构建项目的发布版本
- `cargo run`：编译并运行项目
- `cargo check`：检查代码，确保可编译且过程中不产生可执行文件
- `cargo test`：运行测试
- `cargo doc`：生成文档
  - `cargo doc --open`：生成文档并打开浏览器查看
  - `cargo doc --no-deps`：生成文档时不包含依赖的文档
  - `cargo doc --target-dir <directory>`：指定生成文档的输出目录
- `cargo publish`：发布包

## cargo.toml
- `edition`：指定rust版本
- `dependencies`：指定项目依赖的包
- `cargo.lock`：锁定依赖的版本
