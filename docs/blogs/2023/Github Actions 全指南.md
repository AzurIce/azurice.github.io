---
title: "Github Actions 全指南"
date: 2023-10-05
tags:
  - Github
---

# Github Actions 全指南

在 [Understanding GitHub Actions - GitHub Docs](https://docs.github.com/en/actions/learn-github-actions/understanding-github-actions) 中，Github Actions 是被这样描述的：

「Github Actions 是能够让你自动化构建、测试、部署流程的一个 CI/CD 平台」



其中有一个词 *CI/CD*，持续集成（Continuous Integration）/持续交付（Continuous Delivery），网络上对于他们的定义如下：

- CI：多名开发者开发不同功能代码的过程中，可以频繁地将代码合并到一起且不影响相互的工作

- CD：在对代码的自动化构建、测试、部署的基础上，将产品交付到线上生产环境。



简单地来讲，Github Actions 可以在仓库有新的事件发生（新的 Commit / 新的 Pull Request / 新的 Tag 等等）时按照配置文件自动化地执行一些工作，比如代码的测试、构建，甚至可以自动发表 Release。

## 一、Github Actions 的组成

<figure markdown>
![Diagram of an event triggering Runner 1 to run Job 1, which triggers Runner 2 to run Job 2. Each of the jobs is broken into multiple steps.](https://docs.github.com/assets/cb-25535/images/help/actions/overview-actions-simple.png)
<figcaption>workflow</figcaption>
</figure>


被事件触发的一系列操作被称作 *workflow*

一个 *workflow* 包含一个或多个 *job*（可以并行或串行执行）

每个 *job* 会在一个 *runner* 上运行，*runner* 本质类似于一个虚拟机

每个 *job* 包含一个或多个 *step*

在每个 *step* 中可以运行脚本/终端命令或其他的 *action*。

### events

#### pattern

- `*`：匹配零个或多个字符，但是不匹配 `/`
- `**`：匹配零个或多个任意字符
- `?`、`+`、`[]` 类似正则表达式

## 实例

### 1. 自动构建、发布 golang 程序

[GoReleaser：自动化你的软件发布 | 简体中文博客 (nsddd.top)](https://nsddd.top/zh/posts/go-release-tools/)
