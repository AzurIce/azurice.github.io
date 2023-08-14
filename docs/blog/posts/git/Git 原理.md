---
date: 2023-03-24
---
[Book (git-scm.com)](https://git-scm.com/book) 的笔记。

一个 Git仓库 中的 `.git/` 目录下一般有四个关键的文件/目录：
- `HEAD` 文件：用于指向当前被检出的分支
- `index` 文件（还未被创建）：用于保存暂存区信息
- `objects` 目录：存储所有的 Git Objects
- `refs` 目录：存储指向数据的提交对象的指针

## 一、Git Objects

Git 是一个 content-addressable 文件系统，其核心其实是一个简单的 **键值对数据库**。
因此，你可以向 Git 仓库插入任何类型的内容，对应的 Git 会借助哈希返回一个唯一的键值，通过键值可以在任意时刻再次取回内容。

前面提到 Git 的核心其实是一个 **键值对数据库**，而键通过哈希得到。

如果你随便找一个仓库，逛一逛它的 `.git/objects` 目录，可以发现里面有很多两个字符的命名的文件夹在其内有一个或多个38个字符命名的文件。将它们连接在一起可以得到一个40个字符的完整的 SHA-1值，这便是数据的 **键值**，而数据本身就被存储在文件中。

为了存储不同的信息，Git 对象主要有三种：`blob`、`tree` 和 `commit`

- 
- `tree` 对象：存储目录结构
- `commit` 对象：存储提交信息

下面将依次进行讲解。

### 1. blob 对象

`blob` 对象用于存储文件内容数据。

Git 的一个底层命令 `git hash-object` 可以计算并返回传入的数据，也可以将其写入 `.gits/objects` 目录（Git Objects 数据库），下面我们将使用这个命令来进行一些尝试。

---

首先初始化一个新的仓库：

```console
$ mkdir GitPlayground
$ cd GitPlayground
$ git init
Initialized empty Git repository in /mnt/d/_Dev/GitPlayground/.git/
```

在仓库刚被创建的时候 `.git/objects` 目录会被初始化，其中有两个子目录 `info` 和 `pack`，不过目前 `.git/objcets` 目录中没有任何一个文件：

```console
$ find .git/objects
.git/objects
.git/objects/info
.git/objects/pack
$ find .git/objects -type f
```

---

使用 `git hash-object` 创建一个新的数据对象并使用 `-w` 指示 Git 将其存储到数据库中：

```console
$ echo 'test content' | git hash-object -w --stdin
d670460b4b4aece5915caf5c68d12f560a9fe3e4
```

它返回了一个 40 个字符长度的字符串，这是数据 `test content` 的 SHA-1 哈希值。

现在再查看一下 `.git/objects` 中的内容：

```console
$ find .git/objects -type f
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
```

Git 将上面的 SHA-1 哈希值的前 2 个字符作为子目录名，后面 38个字符作为文件名将数据存储为文件。

---

下面介绍另一个命令 `git cat-file`，这个命令可以用来很方便地查看 Git Objects 的内容：

```console
$ git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
test content
```

---

下面创建一个新文件并将其写入数据库：

```console
$ echo 'version 1' > test.txt
$ git hash-object -w test.txt
83baae61804e65cc73a7201a7252750c76066a30
```

然后修改其内容，再写入数据库：

```console
$ echo 'version 2' > test.txt
$ git hash-object -w test.txt
1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
```

现在 `.git/objects` 中就会包含三个文件，分别存储了先前的字符串以及 `test.txt` 的两个版本：

```console
$ find .git/objects -type f
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
```

现在就算我们将 `test.txt` 删除，也可以通过唯一的键值获取到对应版本的内容：

```console
$ git cat-file -p 83baae61804e65cc73a7201a7252750c76066a30 > test.txt
$ cat test.txt
version 1
```

```console
$ git cat-file -p 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a > test.txt
$ cat test.txt
version 2
```

这就是 `blob` 对象：

```console
$ git cat-file -t 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
blob
```

---

不过你其实可以注意到，`blob` 对象只能够存储文件的数据内容，而不能存储目录结构以及文件名等信息。

### 2. tree 对象

Tree 对象用于存储存储目录结构（文件路径、文件名等）。
快照其实就是存储根目录信息的 tree 对象。

---

这里先以一个假设的仓库为例解释一下 tree 对象的概念：

假设有一个仓库，其最新的 `tree` 如下：

```console
$ git cat-file -p master^{tree}
100644 blob a906cb2a4a904a152e80877d4088654daad0c859      README
100644 blob 8f94139338f9404f26296befa88755fc2598c289      Rakefile
040000 tree 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0      lib
```

> `master^{tree}` 指定了 `master` 分支最新的提交所指向的 tree 对象。

可以看到 tree 对象的内容包含一系列 Git 对象的关联模式、类型、哈希值以及文件名。
这与 Unix 的文件系统很相似，不过是经过简化的。

如果进一步查看 `lib` 对象的内容可以得到：

```console
$ git cat-file -p 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0
100644 blob 47c6340d6459e05787f644c2447d2595f5d3a54b      simplegit.rb
```

其结构可以用下面这张图来表示：

<img src="Git 原理.assets/image-20230324205004043.png" alt="image-20230324205004043" style="zoom:67%;" />

---

接下来进行一些尝试：

Git 创建 tree 时会使用 暂存区 或 索引 的状态来创建，所以我们要想创建一个 tree 对象，也需要通过暂存一些文件来创建索引。

以一个单入口 `test.txt` 文件为例：

```console
$ git update-index --add --cacheinfo \
  100644 83baae61804e65cc73a7201a7252750c76066a30 test.txt
```

通过 `git update-index` 命令来更新索引，使用 `--add` 是因为 `test.txt` 目前并不在暂存区内（甚至暂存区都还未创建），使用 `--cacheinfo` 是因为 `test.txt` 目前不在目录中而是在数据库中。
之后指定模式、哈希值、文件名。

`100644` 表示是一个普通文件，其他更多的模式比如 `100755` 表示可执行文件，`120000` 表示一个符号链接。

现在索引创建完毕，可以使用 `git write-tree` 来将暂存区写入到 tree 对象中并保存进数据库。

```console
$ git write-tree
d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git cat-file -p d8329fc1cc938780ffdd9f94e0d364e0ea74f579
100644 blob 83baae61804e65cc73a7201a7252750c76066a30      test.txt
```

接下来再创建一个由第二个版本的 `test.txt` 以及一个新文件 `new.txt` 组成的 tree 对象：

```console
$ echo 'new file' > new.txt
$ git update-index --cacheinfo 100644 \
  1f7a7a472abf3dd9643fd615f6da379c4acb3e3a test.txt
$ git update-index --add new.txt
```

```console
$ git write-tree
0155eb4229851634a0f03eb265b69f5a2d56f341
$ git cat-file -p 0155eb4229851634a0f03eb265b69f5a2d56f341
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt
```

接下来可以通过 `git read-tree` 来读取 tree 对象的内容并放到暂存区内，我们取出第一个 tree 的内容置于 `bak` 目录（使用 `--prefix` 可以指定存储 tree 对象的目录）然后再创建一个 tree 对象：

```console
$ git read-tree --prefix=bak d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git write-tree
3c4e9cd789d88d8d89c1073707c3585e41b0e614
$ git cat-file -p 3c4e9cd789d88d8d89c1073707c3585e41b0e614
040000 tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579      bak
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt
```

现在整个仓库的状态可以用下图表示：

<img src="Git 原理.assets/image-20230324205016509.png" alt="image-20230324205016509" style="zoom:67%;" />

这便是 tree 对象。

### 3. commit 对象

到目前为止，`blob` 和 `tree` 对象虽然可以存储所有文件及目录的信息，但是仍旧没有保存下来有关谁在何时为何保存了快照的信息，而这些信息就由 commit 对象保存。

可以通过 `git commit-tree` 并指定一个 tree 对象来创建 commit 对象：

```console
$ echo 'First commit' | git commit-tree d8329f
fdf4fc3344e67ab068f836878b6c4951e3b15f3d
```

```console
$ git cat-file -p fdf4fc3
tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579
author Scott Chacon <schacon@gmail.com> 1243040974 -0700
committer Scott Chacon <schacon@gmail.com> 1243040974 -0700

First commit
```

一个 commit 对象包含以下内容：
- 用于表示当前快照的顶级的 tree 对象
- 前一个 commit 对象（如果有）
- 作者和提交者的相关信息（用户名称以及邮箱还有时间戳）
- 提交信息

下面再创建两个 commit 对象，并使用 `-p` 来指定前一个提交：

```console
$ echo 'Second commit' | git commit-tree 0155eb -p fdf4fc3
cac0cab538b970a37ea1e769cbbde608743bc96d
$ echo 'Third commit'  | git commit-tree 3c4e9c -p cac0cab
1a410efbd13591db07496601ebc7a059dd55cfe9
```

其实目前，我们几乎通过手动操作得到了一个实际的 Git 仓库，可以使用 `git log` 来查看历史记录：

```console
$ git log --stat 1a410e
commit 1a410efbd13591db07496601ebc7a059dd55cfe9
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:15:24 2009 -0700

	Third commit

 bak/test.txt | 1 +
 1 file changed, 1 insertion(+)

commit cac0cab538b970a37ea1e769cbbde608743bc96d
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:14:29 2009 -0700

	Second commit

 new.txt  | 1 +
 test.txt | 2 +-
 2 files changed, 2 insertions(+), 1 deletion(-)

commit fdf4fc3344e67ab068f836878b6c4951e3b15f3d
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:09:34 2009 -0700

    First commit

 test.txt | 1 +
 1 file changed, 1 insertion(+)
```

现在再查看一下 `.git/objects`（注释表示存储的内容）：

```console
$ find .git/objects -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341 # tree 2
.git/objects/1a/410efbd13591db07496601ebc7a059dd55cfe9 # commit 3
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a # test.txt v2
.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614 # tree 3
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30 # test.txt v1
.git/objects/ca/c0cab538b970a37ea1e769cbbde608743bc96d # commit 2
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4 # 'test content'
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579 # tree 1
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92 # new.txt
.git/objects/fd/f4fc3344e67ab068f836878b6c4951e3b15f3d # commit 1
```

整个仓库的内容可以表示为下图：

<img src="Git 原理.assets/image-20230324205031736.png" alt="image-20230324205031736" style="zoom:67%;" />

## 二、Git References

到目前为止，我们从 Git仓库 取东西都需要一个对应对象的哈希值，**Git引用** 就是一个特殊的文件，通过保存不同的哈希值来动态地指向不同的 Git对象，他们被存储在 `.git/refs` 目录下。

对于我们刚才手动创建的“仓库”，目前并没有任何引用：

```console
$ find .git/refs
.git/refs
.git/refs/heads
.git/refs/tags
$ find .git/refs -type f
```

若要创建一个新引用来帮助记忆最新提交所在的位置，从技术上讲我们只需简单地做如下操作：

```console
$ echo 1a410efbd13591db07496601ebc7a059dd55cfe9 > .git/refs/heads/master
```

现在，你就可以在 Git 命令中使用这个刚创建的新引用来代替 SHA-1 值了：

```console
$ git log --pretty=oneline master
1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
cac0cab538b970a37ea1e769cbbde608743bc96d second commit
fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit
```

不过并不建议直接手动修改文件， 如果想更新某个引用，Git 提供了一个更加安全的命令 `update-ref` 来完成此事：

```console
$ git update-ref refs/heads/master 1a410efbd13591db07496601ebc7a059dd55cfe9
```

这基本就是 Git 分支的本质：一个指向某一系列提交之首的指针或引用。 若想在第二个提交上创建一个分支，可以这么做：

```console
$ git update-ref refs/heads/test cac0ca
```

这个分支将只包含从第二个提交开始往前追溯的记录：

```console
$ git log --pretty=oneline test
cac0cab538b970a37ea1e769cbbde608743bc96d second commit
fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit
```

现在，仓库看起来会像是这样：

<img src="Git 原理.assets/image-20230324205042844.png" alt="image-20230324205042844" style="zoom:67%;" />

### 1. HEAD 引用

HEAD 文件通常是一个符号引用（symbolic reference），指向目前所在的分支。 所谓符号引用，表示它是一个指向其他引用的指针。

然而在某些罕见的情况下，HEAD 文件可能会包含一个 git 对象的 SHA-1 值。 当你在检出一个标签、提交或远程分支，让你的仓库变成 [“分离 HEAD”](https://git-scm.com/docs/git-checkout#_detached_head)状态时，就会出现这种情况。

如果查看 HEAD 文件的内容，通常我们看到类似这样的内容：

```console
$ cat .git/HEAD
ref: refs/heads/master
```

如果执行 `git checkout test`，Git 会像这样更新 HEAD 文件：

```console
$ cat .git/HEAD
ref: refs/heads/test
```

当我们执行 `git commit` 时，该命令会创建一个提交对象，并用 HEAD 文件中那个引用所指向的 SHA-1 值设置其父提交字段。

你也可以手动编辑该文件，然而同样存在一个更安全的命令来完成此事：`git symbolic-ref`。 可以借助此命令来查看 HEAD 引用对应的值：

```console
$ git symbolic-ref HEAD
refs/heads/master
```

同样可以设置 HEAD 引用的值：

```console
$ git symbolic-ref HEAD refs/heads/test
$ cat .git/HEAD
ref: refs/heads/test
```

不能把符号引用设置为一个不符合引用规范的值：

```console
$ git symbolic-ref HEAD test
fatal: Refusing to point HEAD outside of refs/
```

### 2. Tags 引用

前面我们刚讨论过 Git 的三种主要的对象类型（**数据对象**、**树对象** 和 **提交对象** ），然而实际上还有第四种。 **标签对象（tag object）** 非常类似于一个提交对象——它包含一个标签创建者信息、一个日期、一段注释信息，以及一个指针。 主要的区别在于，标签对象通常指向一个提交对象，而不是一个树对象。 它像是一个永不移动的分支引用——永远指向同一个提交对象，只不过给这个提交对象加上一个更友好的名字罢了。

咕