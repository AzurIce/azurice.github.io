这里引入了一个 COOL 语言（Classroom Object Oriented Language）是教授自己写的，它被设计为可以在短时间内实现的语言。实际上它的编译器的数量要超过了用它写的程序的数量。

课程的内容将做一个 COOL 到 MIPS 汇编语言的编译器，将包含五个作业：
- 写一个 COOL 程序
- 词法分析器
- 解析器
- 语义分析器
- 代码生成

MIPS 是一个用于 20世纪80年代设计的机器的指令集，而现在有一个用于 MIPS 的可以运行在各种设备的模拟器 SPIN。

---

首先 COOL 语言的后缀名为 `.cl`。

创建一个 `1.cl`：

```cool
class Main {
	i : IO <- new IO;
	main() : Int { i.out_string("Hello World\n"); 1 };
};
```

使用  `coolc ` 编译：

```shell
coolc 1.cl
```

你会发现生成了一个 `1.s` 文件这就是对应的汇编语言文件，接下来可以使用 `spim` 来加载并运行它：

```shell
spim 1.s
```

建议直接看一遍视频：[Week 1 02-01-cool-overview-final_哔哩哔哩_bilibili](https://www.bilibili.com/video/BV1NE411376V?p=4)
