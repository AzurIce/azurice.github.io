---
date_update: 2023-04-02
date: 2023-04-01
---

# _第12章 Inputs 设计模式

## 一、单一仓库模式

像 Debian 一样的系统将软件包分散在一些小仓库中，这会使得跟踪相互依赖的改变以及贡献新的软件包变得困难。而像 Gentoo 的系统将所有的软件包描述放在一个单一的仓库中。

对于 Nix 来说，Nixpkgs 就是一个包含所有软件包的描述的单一的仓库。

在 Nix 中的实现就是对每一个包创建对应的表达式，并创建一个顶级的 Nix 表达式，导入并合并所有的表达式为一个巨大的 名称 -> 软件包属性集。

这看上去会比较笨重比较庞大，但是 Nix 是一个懒惰的语言，所以它只在需要的时候重新求值。

## 二、打包 Graphviz

```nix
let
  pkgs = import <nixpkgs> {};
  mkDerivation = import ./autotools.nix pkgs;
in mkDerivation {
  name = "graphviz";
  src = ./graphviz-2.49.3.tar.gz;
}
```

咕咕咕咕咕

啃是啃了，有空再写。
