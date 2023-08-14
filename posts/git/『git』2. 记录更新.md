---
title: 『git』2. 记录更新
date: 2023-03-06
tags:
    - git
---
T21:48:00+08:00
本节内容：

 - `git status` 查看仓库状态
 - `git add` 开始跟踪文件
 - `git add` 暂存文件
 - `git rm` 移除文件（取消跟踪）
 - `git commit` 提交更新
 - `git log` 显示所有的提交日志

## 一、Git 中的工作流程

首先，在 Git 仓库中，任何文件都不外乎这两种状态：
- **已跟踪**：被纳入了版本管理，在上一次快照中有它们的记录
	对于这类文件有三种状态：已提交、已修改、已暂存
	- **已提交**：文件当前版本的快照已被保存
	- **已修改**：修改了文件，但没有被保存
	- **已暂存**：标记了文件的当前版本做了标记，会使之包含在下次提交的快照中
- **未跟踪**：没有被纳入版本管理

对应着这些文件被存储在不同的地方：
- **工作区**：也就是展现在外面的文件，这里是对项目的某个版本独立提取出来的内容，供你使用或更改。
- **暂存区**：`.git/` 目录中的一个文件，保存了下次将要提交的文件列表信息。
- **.git 目录**：Git 用来保存项目的元数据和对象数据库的地方。

基本的 Git 工作流程如下：
1.  在工作区中修改文件。
2.  将你想要下次提交的更改选择性地暂存，这样只会将更改的部分添加到暂存区。
3.  提交更新，找到暂存区的文件，将快照永久性存储到 Git 目录。

## 二、跟踪文件

还是上一个例子，比如有这样一个目录：

```diff
 my_project/
 |- main.cpp
 |- main.exe
```

我们来对仓库进行初始化：

```shell
git init
```

```diff
 my_project/
+|- .git/
 |- main.cpp
 |- main.exe
```

此时，我们可以通过下面的命令来查看当前仓库的状态：

```shell
git status
```

其输出如下：

```
On branch master

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        main.cpp
        main.exe

nothing added to commit but untracked files present (use "git add" to track)
```

它告诉我们 `main.cpp` 和 `main.exe` 没有被跟踪，要想跟踪一个文件，要执行如下命令：

```shell
git add main.cpp main.exe
```

> [!tip]
> 当然它支持通配符，所以也可以这么写来添加所有文件名为 `main` 后缀不限的文件：`git add main.*`，也可以这样子添加所有文件：`git add *` 当然也可以用 `git add .`。

我们再执行 `git status` 查看一下仓库状态：

```
On branch master

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
        new file:   main.cpp
        new file:   main.exe

```

现在文件就被跟踪了。

如果我们此时修改一下 `main.cpp`，再查看状态：

```
On branch master

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
        new file:   main.cpp
        new file:   main.exe

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   main.cpp

```

## 二、暂存与提交文件

暂存的目的是标记有哪些文件需要被提交，如果一个文件是刚被添加到 git 版本管理中的，那么它便是被暂存的。

也就是说刚才我们通过 `git add` 将 `main.cpp` 和 `main.exe` 跟踪后，他们便是暂存的，你也可以看到在 `git status` 中也显示他们是 `Changes to be committed`，而我们又对 `main.cpp` 做了修改，这时候这个修改便没有被暂存，`git status` 显示其为 `Changes not staged for commit`。

我们可以先提交一下更改：

```shell
git commit -m "Initial commit"
```

通过 `-m` 参数指定提交的信息。

输出：
```
 2 files changed, 8 insertions(+)
 create mode 100644 main.cpp
 create mode 100644 main.exe
```

再来看看状态：

```
On branch master
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   main.cpp

no changes added to commit (use "git add" and/or "git commit -a")
```

可以看到只有暂存的更改被提交了。

我们来将刚才修改过的 `main.cpp` 暂存一下，这个命令依旧是 `git add`：

```
git add main.cpp
```

再来看一下状态：

```
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        modified:   main.cpp

```

再进行提交，输出：

```
[master e0afb9c] updated main.cpp
 1 file changed, 1 insertion(+), 1 deletion(-)
```

最终的状态：

```
On branch master
nothing to commit, working tree clean
```

## 三、.gitignore文件 以及 文件的移除

有时候我们不希望一些文件被 git管理，比如本例子中的 `main.exe` 这类的构建文件，它是由源代码文件 `main.cpp` 编译出来的，因此我们常常不对这类文件进行管理。git 提供了一个方法来忽略一些特定的文件、目录，那就是 `.gitignore` 文件。

在项目根目录创建一个 `.gitignore` 文件：

```diff
 my_project/
 |- .git/
+|- .gitignore
 |- main.cpp
 |- main.exe
```

在里面写入以下内容：

```
*.exe
```

这时我们查看状态会显示：

```
On branch master
Untracked files:
  (use "git add <file>..." to include in what will be committed)
        .gitignore

nothing added to commit but untracked files present (use "git add" to track)
```

我们再重新编译刚才我们修改后的 `main.cpp` 为 `main.exe` 和 `a.exe`：

```
g++ main.cpp -o main
g++ main.cpp -o a
```

这时我们再查看状态我们会发现，`main.exe` 的修改，以及 `a.exe` 的新增都没有出现：

```
On branch master
Untracked files:
  (use "git add <file>..." to include in what will be committed)
        .gitignore

nothing added to commit but untracked files present (use "git add" to track)
```

我们对 `.gitignore` 加入跟踪，并且提交：

```shell
git add .
git commit -m "Added .gitignore"
```

但是其实，再文件的快照版本中，`main.exe` 依旧是存在的，只是这一次提交的快照中并没有包括对其的修改，而 `a.exe` 是不存在的，因为它还没有被跟踪。

从 git 中移除一个文件（取消跟踪的同时也会将其从工作区删除）的方式如下：

```shell
git rm main.exe
```

> [!info]
> 正常情况下删除了某个文件后的暂存操作也是使用 `git rm`。
> 也就是说 `git rm` 命令的作用是暂存对文件的移除，如果文件存在也会实际的将文件移除。

```diff
 my_project/
 |- .git/
 |- .gitignore
 |- main.cpp
-|- main.exe
```

我们再查看状态：

```
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        deleted:    main.exe

```

这时，`main.exe` 便被停止跟踪了，同时也被从本地目录删除。

再将其提交：

```shell
git commit -m "Removed main.exe"
```

到此为止我们做了很多的修改，我们可以通过 `git log` 来查看所有的提交（最新的提交会在最上面）：

```
commit 901aff72d7be178eb8626096bbab2a1cacb6ede6 (HEAD -> master)
Author: Azur冰弦 <973562770@qq.com>
Date:   Thu Dec 1 18:17:39 2022 +0800

    Removed main.exe

commit bff6c0e2f04628b372c715f6a04f07b34752a47f
Author: Azur冰弦 <973562770@qq.com>
Date:   Thu Dec 1 18:12:47 2022 +0800

    Added .gitignore

commit e0afb9cb05b556c0934ff059ea1aa234bac8e620
Author: Azur冰弦 <973562770@qq.com>
Date:   Thu Dec 1 10:13:45 2022 +0800

    updated main.cpp

commit 34ef417c28ab016015d36f70ceafc6cb9ad4f9a3
Author: Azur冰弦 <973562770@qq.com>
Date:   Thu Dec 1 10:12:27 2022 +0800

    Initial commit
```