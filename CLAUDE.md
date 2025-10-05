# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

### 配置说明

`src/main.rs` 中的 `ConfigContext` 包含:
- 博客元信息(标题、描述、作者等)
- GitHub/Bilibili/Steam 等社交链接
- Giscus 评论系统配置

## 开发注意事项

- 修改文章内容后需要重新运行构建命令,因为 `build.rs` 会重新生成 `docsgen.rs`
- `index.html` 中的注释标记(如 `AOIKE_SYCAMORE_SITE_ASSETS_START/END`)会在构建时被自动填充,不要手动编辑这些区域
- 样式文件使用 SCSS,通过 Trunk 编译
- Cargo.toml 指定了 edition = "2024",使用最新的 Rust 版本特性
