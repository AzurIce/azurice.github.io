---
date: 2023-04-01
---

# 第6~7章 Derivation

# Chapter 6. Our First Derivation

从文件系统的角度来看，Derivation 是 Nix 系统的组成部分，而 Nix 语言就是用于描述它的。

## 一、创建 Derivation

`derivation` 内置函数[^1] 就是用于创建 Derivation 的。

Derivation 其实就是一个带有一些属性的集合。



`derivation` 函数接受一个至少包含以下三个属性的集合作为它的第一个参数：

- name：此 Derivation 的名字。在 nix store 格式就是 hash-name

- system：此 Derivation 可以被构建的系统

  可以使用 `builtins.currentSystem` 获取：

  ```console
  nix-repl> builtins.currentSystem
  "x86_64-linux"
  ```

- builder：构建 Derivation 的二进制程序



下面我们来创建一个 Derivation 并将其赋给 `d`：

```console
nix-repl> d = derivation { name = "myname"; builder = "mybuilder"; system = "mysystem"; }
nix-repl> d
«derivation /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv»
```

nix repl 并不会构建 Derivation，但是它会创建 `.drv` 文件。

---

那么什么是 `.drv` 文件呢？

做一个类比，如果将 `.nix` 文件比作 `.c` 文件，那么 `.drv` 文件就是像 `.o` 文件一样的中间文件，它包含描述如何构建一个 Derivation 的最少的信息，而最终我们的路径就是构建的结果。

我们可以使用 `nix show-derivation` 来查看一个 `.drv` 文件的内容：

```console
$ nix show-derivation /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv
{
  "/nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv": {
    "outputs": {
      "out": {
        "path": "/nix/store/40s0qmrfb45vlh6610rk29ym318dswdr-myname"
      }
    },
    "inputSrcs": [],
    "inputDrvs": {},
    "platform": "mysystem",
    "builder": "mybuilder",
    "args": [],
    "env": {
      "builder": "mybuilder",
      "name": "myname",
      "out": "/nix/store/40s0qmrfb45vlh6610rk29ym318dswdr-myname",
      "system": "mysystem"
    }
  }
}
```

其中包含：

- 输出路径（可以有多个），默认 Nix 会创建一个叫做 `out` 的输出路径

  out 路径的 hash 完全基于当前版本的 Nix 中的输入 Derivation，而非构建的内容（保留疑惑）

- 输入的 Derivation 的列表

- 系统和构建器的可执行文件

- 一系列传递给构建器的环境变量

## 二、构建 Derivation

在 nix repl 中可以使用 `:b` 来构建一个 Derivation，更多 nix repl 中的命令可以查看 `:?` 的输出。

```console
nix-repl> d = derivation { name = "myname"; builder = "mybuilder"; system = "mysystem"; }
nix-repl> :b d
[...]
these derivations will be built:
  /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv
building path(s) `/nix/store/40s0qmrfb45vlh6610rk29ym318dswdr-myname'
error: a `mysystem' is required to build `/nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv', but I am a `x86_64-linux'
```

在 nix repl 外可以使用 `nix-store -r` 来 realise 一个 `.drv` 文件（输出同上）：

```console
$ nix-store -r /nix/store/z3hhlxbckx4g3n9sw91nnvlkjvyw754p-myname.drv
```



可以看到提示需要在 `mysystem` 系统上才可以构建，而目前的系统为 `x86_64-linux`，我们稍作更改再尝试一次：

```console
nix-repl> d = derivation { name = "myname"; builder = "mybuilder"; system = builtins.currentSystem; }
nix-repl> :b d
[...]
build error: invalid file name `mybuilder'
```

这次提示 `mybuilder` 并不存在。

---

`derivation` 的返回值其实是一个属性集：

> `builtins.isAttrs` 判断传入参数是否为一个属性集
>
> `builtins.attrNames` 返回传入属性集的所有键的列表

```console
nix-repl> d = derivation { name = "myname"; builder = "mybuilder"; system = "mysystem"; }
nix-repl> builtins.isAttrs d
true
nix-repl> builtins.attrNames d
[ "all" "builder" "drvAttrs" "drvPath" "name" "out" "outPath" "outputName" "system" "type" ]
```

我们来看一看其中都有哪些属性：

- `drvAttrs`：这基本上其实就是 derivation 函数的传入参数

  ```console
  nix-repl> d.drvAttrs
  { builder = "mybuilder"; name = "myname"; system = "mysystem"; }
  ```

- `name`、`system`、`builder` 也是

- `out`：也就是 Derivation 自己

  ```console
  nix-repl> (d == d.out)
  true
  ```

- `drvPath`：就是 `.drv` 文件的路径

- `type`：值为 `derivation`

  如果你创建一个带有值为 `derivation` 的 `type` 属性的集合，那么它其实就是 Derivation 类型：

  ```console
  nix-repl> { type = "derivation"; }
  «derivation ???»
  ```

- `outPath`：构建的输出路径

## 三、引用其他 Derivation

首先介绍一个 Nix 的 Set 到 String 的转换：

```console
nix-repl> builtins.toString { outPath = "foo"; }
"foo"
nix-repl> builtins.toString { a = "b"; }
error: cannot coerce a set to a string, at (string):1:1
```

如果一个集合带有一个 `outPath` 属性，那么它就可以被转换为一个字符串。



比如我们想要使用来自 coreutils 的二进制文件（暂时忽略 nixpkgs 之类的东西）：

```console
nix-repl> :l <nixpkgs>
Added 3950 variables.
nix-repl> coreutils
«derivation /nix/store/1zcs1y4n27lqs0gw4v038i303pb89rw6-coreutils-8.21.drv»
nix-repl> builtins.toString coreutils
"/nix/store/8w4cbiy7wqvaqsnsnb3zvabq1cp2zhyz-coreutils-8.21"
```

在字符串中可以插入 Nix 表达式的值：

```console
nix-repl> "${d}"
"/nix/store/40s0qmrfb45vlh6610rk29ym318dswdr-myname"
nix-repl> "${coreutils}"
"/nix/store/8w4cbiy7wqvaqsnsnb3zvabq1cp2zhyz-coreutils-8.21"
```

比如我们想要使用 `bin/true` 二进制文件：

```console
nix-repl> "${coreutils}/bin/true"
"/nix/store/8w4cbiy7wqvaqsnsnb3zvabq1cp2zhyz-coreutils-8.21/bin/true"
```

如此就可以获取到其路径。

我们再次修改我们的 Derivation（每当 Derivation 被修改时都会创建一个新的哈希值）：

```console
nix-repl> :l <nixpkgs>
nix-repl> d = derivation { name = "myname"; builder = "${coreutils}/bin/true"; system = builtins.currentSystem; }
nix-repl> :b d
[...]
builder for `/nix/store/qyfrcd53wmc0v22ymhhd5r6sz5xmdc8a-myname.drv' failed to produce output path `/nix/store/ly2k1vswbfmswr33hw0kf0ccilrpisnk-myname'
```

现在它执行了构建起（`bin/true`）但是并没有创建输出路径，只是以返回值 0 退出了。

现在我们再来看看这个 `.drv` 文件：

```console
$ nix show-derivation /nix/store/qyfrcd53wmc0v22ymhhd5r6sz5xmdc8a-myname.drv
{
  "/nix/store/qyfrcd53wmc0v22ymhhd5r6sz5xmdc8a-myname.drv": {
    "outputs": {
      "out": {
        "path": "/nix/store/ly2k1vswbfmswr33hw0kf0ccilrpisnk-myname"
      }
    },
    "inputSrcs": [],
    "inputDrvs": {
      "/nix/store/hixdnzz2wp75x1jy65cysq06yl74vx7q-coreutils-8.29.drv": [
        "out"
      ]
    },
    "platform": "x86_64-linux",
    "builder": "/nix/store/qrxs7sabhqcr3j9ai0j0cp58zfnny0jz-coreutils-8.29/bin/true",
    "args": [],
    "env": {
      "builder": "/nix/store/qrxs7sabhqcr3j9ai0j0cp58zfnny0jz-coreutils-8.29/bin/true",
      "name": "myname",
      "out": "/nix/store/ly2k1vswbfmswr33hw0kf0ccilrpisnk-myname",
      "system": "x86_64-linux"
    }
  }
}
```

我们可以发现在 `inputDrvs` 中多了一个 `coreutils` 的 `.drv`。

## 6.6. When is the derivation built

Nix does not build derivations **during evaluation** of Nix expressions. In fact, that's why we have to do ":b drv" in `nix repl`, or use nix-store -r in the first place.

An important separation is made in Nix:

- **Instantiate/Evaluation time**: the Nix expression is parsed, interpreted and finally returns a derivation set. During evaluation, you can refer to other derivations because Nix will create .drv files and we will know out paths beforehand. This is achieved with [nix-instantiate](https://nixos.org/manual/nix/stable/command-ref/nix-instantiate.html).
- **Realise/Build time**: the .drv from the derivation set is built, first building .drv inputs (build dependencies). This is achieved with [nix-store -r](https://nixos.org/manual/nix/stable/command-ref/nix-store.html#operation---realise).

Think of it as of compile time and link time like with C/C++ projects. You first compile all source files to object files. Then link object files in a single executable.

In Nix, first the Nix expression (usually in a .nix file) is compiled to .drv, then each .drv is built and the product is installed in the relative out paths.

## 6.7. Conclusion

Is it that complicated to create a package for Nix? No, it's not.

We're walking through the fundamentals of Nix derivations, to understand how they work, how they are represented. Packaging in Nix is certainly easier than that, but we're not there yet in this post. More Nix pills are needed.

With the derivation function we provide a set of information on how to build a package, and we get back the information about where the package was built. Nix converts a set to a string when there's an `outPath`; that's very convenient. With that, it's easy to refer to other derivations.

When Nix builds a derivation, it first creates a .drv file from a derivation expression, and uses it to build the output. It does so recursively for all the dependencies (inputs). It "executes" the .drv files like a machine. Not much magic after all.

---

# Chapter 7. Working Derivation

这一章节，我们将尝试打包一个真实的程序：编译一个简单的 C 语言文件并为其创建一个 Derivation。

## 一、使用脚本作为一个构建器

我们写一个执行一系列命令的脚本 `builder.sh` 来构建我们的程序，并且我们希望我们的 Derivation 运行 `bash builder.sh`。

在 `builder.sh` 中我们不使用 hash bangs，因为在我们编写它的时候我们并不知道 bash 在 nix store 中的路径。



也就是说，在这个例子中 `bash` 就是我们的构建器，而我们要向它传递一个参数 `builder.sh`。

`derivation` 函数接受一个可选的参数 `args` 用于向构建器传递参数。



那么首先让我们在当前目录创建我们的 `builder.sh` 并输入以下内容：

```shell
declare -xp
echo foo > $out
```

这个脚本所做的事情如下：

- `declare -xp` 列出所有导出的变量（`declare` 是内置的 bash 函数）

  上一章中我们知道最终的 `.drv` 文件会包含一系列传递给构建器的环境变量，其中之一就是 `$out`

在 `$out` 中创建一个文件。



然后我们在构建的过程中使用 coreutils 中的 env 来打印环境变量，如此我们的依赖除了 bash 还有 coreutils。



```console
nix-repl> :l <nixpkgs>
Added 3950 variables.
nix-repl> "${bash}"
"/nix/store/ihmkc7z2wqk3bbipfnlh0yjrlfkkgnv6-bash-4.2-p45"
```

```console
nix-repl> d = derivation { name = "foo"; builder = "${bash}/bin/bash"; args = [ ./builder.sh ]; system = builtins.currentSystem; }
nix-repl> :b d
[1 built, 0.0 MiB DL]

this derivation produced the following outputs:
  out -> /nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo
```

We did it! The contents of `/nix/store/w024zci0x1hh1wj6gjq0jagkc1sgrf5r-foo` is really foo. We've built our first derivation.

Note that we used `./builder.sh` and not `"./builder.sh"`. This way, it is parsed as a path, and Nix performs some magic which we will cover later. Try using the string version and you will find that it cannot find `builder.sh`. This is because it tries to find it relative to the temporary build directory.

## 二、构建环境

我们可以使用 `nix-store --read-log` 来查看我们的构建器产生的输出：

```console
$ nix-store --read-log /nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo
declare -x HOME="/homeless-shelter"
declare -x NIX_BUILD_CORES="4"
declare -x NIX_BUILD_TOP="/tmp/nix-build-foo.drv-0"
declare -x NIX_LOG_FD="2"
declare -x NIX_STORE="/nix/store"
declare -x OLDPWD
declare -x PATH="/path-not-set"
declare -x PWD="/tmp/nix-build-foo.drv-0"
declare -x SHLVL="1"
declare -x TEMP="/tmp/nix-build-foo.drv-0"
declare -x TEMPDIR="/tmp/nix-build-foo.drv-0"
declare -x TMP="/tmp/nix-build-foo.drv-0"
declare -x TMPDIR="/tmp/nix-build-foo.drv-0"
declare -x builder="/nix/store/q1g0rl8zfmz7r371fp5p42p4acmv297d-bash-4.4-p19/bin/bash"
declare -x name="foo"
declare -x out="/nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo"
declare -x system="x86_64-linux"
```

Let's inspect those environment variables printed during the build process.

- `$HOME` is not your home directory, and `/homeless-shelter` doesn't exist at all. We force packages not to depend on `$HOME` during the build process.
- `$PATH` plays the same game as `$HOME`
- `$NIX_BUILD_CORES` and `$NIX_STORE` are [nix configuration options](https://nixos.org/manual/nix/stable/command-ref/conf-file.html)
- `$PWD` and `$TMP` clearly show that nix created a temporary build directory
- Then `$builder`, `$name`, `$out`, and `$system` are variables set due to the .drv file's contents.

And that's how we were able to use `$out` in our derivation and put stuff in it. It's like Nix reserved a slot in the nix store for us, and we must fill it.

In terms of autotools, `$out` will be the `--prefix` path. Yes, not the make `DESTDIR`, but the `--prefix`. That's the essence of stateless packaging. You don't install the package in a global common path under `/`, you install it in a local isolated path under your nix store slot.

## 三、`.drv` 的内容

We added something else to the derivation this time: the args attribute. Let's see how this changed the .drv compared to the previous pill:

```console
$ nix show-derivation /nix/store/i76pr1cz0za3i9r6xq518bqqvd2raspw-foo.drv
{
  "/nix/store/i76pr1cz0za3i9r6xq518bqqvd2raspw-foo.drv": {
    "outputs": {
      "out": {
        "path": "/nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo"
      }
    },
    "inputSrcs": [
      "/nix/store/lb0n38r2b20r8rl1k45a7s4pj6ny22f7-builder.sh"
    ],
    "inputDrvs": {
      "/nix/store/hcgwbx42mcxr7ksnv0i1fg7kw6jvxshb-bash-4.4-p19.drv": [
        "out"
      ]
    },
    "platform": "x86_64-linux",
    "builder": "/nix/store/q1g0rl8zfmz7r371fp5p42p4acmv297d-bash-4.4-p19/bin/bash",
    "args": [
      "/nix/store/lb0n38r2b20r8rl1k45a7s4pj6ny22f7-builder.sh"
    ],
    "env": {
      "builder": "/nix/store/q1g0rl8zfmz7r371fp5p42p4acmv297d-bash-4.4-p19/bin/bash",
      "name": "foo",
      "out": "/nix/store/gczb4qrag22harvv693wwnflqy7lx5pb-foo",
      "system": "x86_64-linux"
    }
  }
}
```

Much like the usual .drv, except that there's a list of arguments in there passed to the builder (bash) with `builder.sh`… In the nix store..? Nix automatically copies files or directories needed for the build into the store to ensure that they are not changed during the build process and that the deployment is stateless and independent of the building machine. `builder.sh` is not only in the arguments passed to the builder, it's also in the input derivations.

Given that `builder.sh` is a plain file, it has no .drv associated with it. The store path is computed based on the filename and on the hash of its contents. Store paths are covered in detail in [a later pill](https://nixos.org/guides/nix-pills/nix-store-paths.html).

## 四、打包一个简单的 C 语言程序

我们写一个简单的 `simple.c`：

```c
void main() {
  puts("Simple!");
}
```

以及它的构建脚本 `simple_builder.sh`：

```sh
export PATH="$coreutils/bin:$gcc/bin"
mkdir $out
gcc -o $out/simple $src
```

暂时先不用担心上面的变量都是从哪来的，我们先编写 Derivation 并构建它：

```console
nix-repl> :l <nixpkgs>
nix-repl> simple = derivation { name = "simple"; builder = "${bash}/bin/bash"; args = [ ./simple_builder.sh ]; gcc = gcc; coreutils = coreutils; src = ./simple.c; system = builtins.currentSystem; }
nix-repl> :b simple
this derivation produced the following outputs:

  out -> /nix/store/ni66p4jfqksbmsl616llx3fbs1d232d4-simple
```

Now you can run `/nix/store/ni66p4jfqksbmsl616llx3fbs1d232d4-simple/simple` in your shell.

---

我们在 `derivation` 添加了两个新的参数：`gcc` 和 `coreutils`。

In `gcc = gcc;`, the name on the left is the name in the derivation set, and the name on the right refers to the gcc derivation from nixpkgs. The same applies for coreutils.

We also added the `src` attribute, nothing magical — it's just a name, to which the path `./simple.c` is assigned. Like `simple-builder.sh`, `simple.c` will be added to the store.

> 所有传入 `derivation` 的参数都会被转换为字符串并作为环境变量传递给构建器，这就是那一堆环境变量是怎么来的。

在 `simple_builder.sh` 中我们设置了 `PATH` 环境变量，这样后面就可以使用 `mkdir` 和 `gcc` 了。

最终我们以 `$out` 作为目录，将输出的二进制文件置于其中。

## 五、够了！在 nix repl 外进行构建！

编写一个 `simple.nix`：

```nix
with (import <nixpkgs> {});
derivation {
  name = "simple";
  builder = "${bash}/bin/bash";
  args = [ ./simple_builder.sh ];
  inherit gcc coreutils;
  src = ./simple.c;
  system = builtins.currentSystem;
}
```

现在我们可以使用 `nix-build simple.nix` 来进行构建了。这会在当前的目录下创建一个链接 `result`，指向输出路径。

`nix-build` 会做两件事：

1. [nix-instantiate ](https://nixos.org/manual/nix/stable/command-ref/nix-instantiate.html)：解析并对 `simple.nix` 求值，在这个例子中返回对应解析的 Derivation 集合的 `.drv` 文件。
2. [nix-store -r ](https://nixos.org/manual/nix/stable/command-ref/nix-store.html#operation---realise): realise the .drv file, 也就是将 Derivation 构建.

Afterwards, we call the function with the empty set. We saw this already in [the fifth pill](https://nixos.org/guides/nix-pills/functions-and-imports.html). To reiterate: `import <nixpkgs> {}` is calling two functions, not one. Reading it as `(import <nixpkgs>) {}` makes this clearer.

The value returned by the nixpkgs function is a set. More specifically, it's a set of derivations. Using the `with` expression we bring them into scope. This is equivalent to the **:l \<nixpkgs\>** we used in nix repl; it allows us to easily access derivations such as `bash`, `gcc`, and `coreutils`.

## 参考

[^1]:[Derivations - Nix Reference Manual (nixos.org)](https://nixos.org/manual/nix/stable/language/derivations.html)
