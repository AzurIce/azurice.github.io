---
title: 『git』1. 仓库
date: 2023-03-06
tags:
    - git
---
T21:47:00+08:00
本节内容：

- `git init` 初始化空仓库
- `git clone` 从远端克隆下来一个仓库

**仓库** 是受 git 管理的项目根目录及其内部的所有内容，它包含被管理的文件以及一个 `.git` 目录（其中存放着 git 用于管理版本而产生的数据文件）。

通常获取 Git 仓库有两种方式：
1.  将尚未进行版本控制的本地目录转换为 Git 仓库
2.  从其它服务器 **克隆** 一个已存在的 Git 仓库。

## 一、在尚未进行版本控制的目录中初始化仓库

比如我们有这样一个目录：

```
 my_project/
 |- main.cpp
 |- main.exe
```

首先，你需要进入到想要管理的项目根目录中，然后执行初始化命令：

```shell
cd my_project/
git init
```

执行完毕后，你将看到多了一个名为 `.git` 的子目录，这个子目录含有你初始化的 Git 仓库中所有的必须文件，这些文件是 Git 仓库的骨干，不过通常你不必管它。

```diff
 my_project/
+|- .git/
 |- main.cpp
 |- main.exe
```

如果你所在的目录中已经存在一些文件，初始化仓库并不会自动让他们被 Git 跟踪，你还需要 [向 git 中添加文件]。

```shell
git add *
git commit -m "initial commit"
```

## 二、克隆现有的仓库

克隆仓库的命令是 `git clone <url>` 。 比如，要克隆 Git 的链接库 `libgit2`，可以用下面的命令：

```shell
$ git clone https://github.com/libgit2/libgit2
```

这会在当前目录下创建一个名为 “libgit2” 的目录，并在这个目录下初始化一个 `.git` 文件夹， 从远程仓库拉取下所有数据放入 `.git` 文件夹，然后从中读取最新版本的文件的拷贝放在目录下。

当然你也可以指定存储的目录名：

```shell
$ git clone https://github.com/libgit2/libgit2 mylibgit
```