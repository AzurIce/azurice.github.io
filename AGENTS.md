# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## 项目概述

这是一个使用 Rust + Sycamore 框架构建的个人博客静态网站,基于 [aoike](https://github.com/azurice/aoike) 框架实现。

## 构建系统

### 核心构建流程

构建过程分为两个阶段:

1. **构建时 (build.rs)**:
   - 解析 `docs/` 目录中的 Markdown 文件
   - 使用 aoike 框架将文章转换为 Rust 代码
   - 生成 `src/docsgen.rs` 文件,包含所有文章数据
   - 将资源引用注入到 `index.html` 中的 `AOIKE_SYCAMORE_SITE_ASSETS` 标记处

2. **运行时**: Sycamore 框架使用生成的代码渲染 SPA

### 常用命令

- **构建项目**: `just build` (使用 trunk 构建工具)
- **Trunk 构建**: `trunk build` (手动构建)
- **Trunk 开发服务器**: `trunk serve` (启动开发服务器并实时重载)

## 项目架构

### 目录结构

- `docs/`: Markdown 文章源文件
  - `docs/index.md`: 首页内容
  - `docs/posts/`: 博客文章目录(如果存在)
- `src/main.rs`: 应用程序入口,配置 AoikeApp 组件
- `src/docsgen.rs`: 自动生成的文件,包含解析后的文章数据
- `static/`: 静态资源文件
  - `static/css/`: 样式文件(SCSS 和 UnoCSS)（由 build.rs 自动产生）
  - `static/avatar.jpg`: 头像图片
- `build.rs`: 构建脚本,负责解析 Markdown 并生成代码
- `index.html`: 主页面模板，其中由 build.rs 自动生成的资源引用会被注入

### 关键依赖

- **aoike**: 核心框架,提供文章解析和构建工具
- **aoike-sycamore**: Sycamore 集成,提供博客组件(如 Giscus 评论)
- **sycamore**: 响应式 UI 框架
- **trunk**: Rust WASM 应用构建工具

### 本地 aoike-ssg clone

- 本机有 aoike-ssg 源码 clone: `E:\aoike-ssg`
- 当前项目默认依赖 Git 版框架:
  - `aoike = { git = "https://github.com/azurice/aoike-ssg", branch = "main" }`
  - `aoike-sycamore = { git = "https://github.com/azurice/aoike-ssg", branch = "main" }`
- 需要联调框架改动时,可临时把 `Cargo.toml` 里的 workspace dependencies 切到本地 path:
  - `aoike = { path = "../aoike-ssg" }`
  - `aoike-sycamore = { path = "../aoike-ssg/packages/aoike-sycamore" }`
- **重要: 提交前必须恢复成 git 依赖,禁止把本地 path 依赖 commit 到 main。** CI 只使用 Git 依赖,本地 path 会导致部署失败。
- `Cargo.lock` 当前锁定的远端提交为 `8e3787aa8f757fca6096d849ce0ab676f2d71597`。
- `E:\aoike-ssg` 自身也是 Git 仓库;修改框架时先检查它的 `git status`,避免覆盖已有改动。

### aoike-ssg 结构速记

- 根 crate `aoike`: 框架无关的数据结构和 build-time 工具。
  - `src/lib.rs`: `PostData`、`Site`
  - `src/build.rs`: `Entity`、`parse_posts`、`get_assets_trunk_data`、`generate_code`
  - `src/build/post.rs`: Markdown/Typst -> HTML -> `Post`
  - `src/build/utils.rs`: HTML 摘要、引用资源扫描、`index.html` 注入、git 时间戳
- `packages/aoike-sycamore`: 当前博客实际使用的 Sycamore 集成。
  - `src/lib.rs`: `AoikeApp`、路由、首页/文章列表/文章页组件、`ConfigContext`
  - `src/layout/base.rs`: 顶部导航 `Header`
  - `src/components/giscus.rs`: Giscus 配置和 script 注入
  - `src/build.rs`: `init_aoike_sycamore()`,解压 CSS 到站点的 `static/css`,并注入 Trunk CSS 链接
  - `build.rs`: 将包内 `css/` 打包为 `css.zip`
- `packages/aoike-dioxus`: Dioxus 集成,目前不是本站主路径。
- `example/sycamore`: 与本站构建脚本和入口最接近的参考实现。

### 本站与 aoike 的衔接

- `build.rs` 调用 `aoike_sycamore::build::init_aoike_sycamore()` 生成/更新 `static/css` 并注入 `AOIKE_SYCAMORE` 区块。
- `build.rs` 调用 `aoike::build::parse_posts("docs/posts")` 和 `Post::try_from(Entity::new("docs/index.md"))` 解析内容。
- `aoike::build::generate_code(posts, index)` 生成 `src/docsgen.rs`;该文件在 `.gitignore` 中,属于构建产物。
- `aoike::build::get_assets_trunk_data(&posts, &index, "docs")` 会扫描 HTML 中的 `src="..."` 并向 `index.html` 的 `AOIKE_SYCAMORE_SITE_ASSETS` 区块注入 Trunk `copy-file` 链接。
- 文章支持 `.md` 和 `.typ`;Typst 路径依赖系统中可执行的 `typst compile ... -fhtml --features html`。
- 文章创建/更新时间来自 `git log`;未提交的新文章或历史缺失时可能显示 Unix epoch。

### 配置说明

`src/main.rs` 中的 `ConfigContext` 包含:
- 博客元信息(标题、描述、作者等)
- GitHub/Bilibili/Steam 等社交链接
- Giscus 评论系统配置

## 开发注意事项

- 修改文章内容后需要重新运行构建命令,因为 `build.rs` 会重新生成 `docsgen.rs`
- `index.html` 中的注释标记(如 `AOIKE_SYCAMORE_SITE_ASSETS_START/END`)会在构建时被自动填充,不要手动编辑这些区域
- 样式文件使用 SCSS,通过 Trunk 编译
- `static/css/` 由 `aoike-sycamore` 的构建初始化逻辑生成,已在 `.gitignore` 中;通常不要手动改本站里的这份输出,应改 `E:\aoike-ssg\packages\aoike-sycamore\css\` 源文件并重新构建。
- `index.html` 的 `AOIKE_SYCAMORE_START/END` 和 `AOIKE_SYCAMORE_SITE_ASSETS_START/END` 都是自动维护区块;如果需要改注入逻辑,优先看 `E:\aoike-ssg\packages\aoike-sycamore\src\build.rs` 和 `E:\aoike-ssg\src\build.rs`。
- Cargo.toml 指定了 edition = "2024",使用最新的 Rust 版本特性
