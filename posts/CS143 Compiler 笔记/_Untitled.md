**编译器（compiler）**的前端部分的主要工作流程如下：

- **词法分析（lexical analysis）**

  将 **字符序列** 转换为 **记号（token）序列**

- **语法分析（syntactic analysis，也叫 parsing）**

  根据某种给定的 **文法** 由 **记号（token）序列** 得到其对应语法结构（语法分析树、抽象语法树等）

- **语义分析（semnatic analysis）**

  筛选出需要根据上下文来找出的错误，比如是否有在使用前未定义的变量、赋值语句类型是否匹配等等

经过这三步筛选与解析，只有合法的程序才能够通过并传递到代码生成器（后端部分）。

## 一、flex 概述

> flex 手册：[Lexical Analysis With Flex, for Flex 2.6.2: Top (westes.github.io)](http://westes.github.io/flex/manual/index.html)

**flex** 是一个用于生成基于 C/C++ 的词法分析器的程序。

它的输入是一个 `.flex` 文件，其中包含着一对对由 **正则表达式** 和 **C 语言代码** 组成的 **规则**。

它的输出是一个 C 语言源文件，其中会包含一个 `yylex()` 函数，该函数会对输入的字符序列进行扫描，根据 **规则** 中的一个个 **正则表达式** 进行匹配并执行其对应的 **C 语言代码**。

`.flex` 文件的结构大致如下：

```lex
definitions
%%
rules
%%
user code
```

下面是一个简单的例子：

```lex
%option noyywrap

DIGIT    [0-9]
%%
{DIGIT}             printf("Digit");
{DIGIT}+            printf("Integer");
{DIGIT}+"."{DIGIT}+ printf("Decimal");
%%
int main() {
    yylex();
}
```

使用 flex 生成 `lex.yy.c` 并编译运行：

```terminal
PS F:\__Syncthing__\Notes\03 啃\CS143 斯坦福大学编译原理\playground> flex hello.flex
PS F:\__Syncthing__\Notes\03 啃\CS143 斯坦福大学编译原理\playground> g++ lex.yy.c
PS F:\__Syncthing__\Notes\03 啃\CS143 斯坦福大学编译原理\playground> ./a
asdasd 123 213 2.0
asdasd Integer Integer Decimal Digit
```

`yylex()` 会自上而下匹配每一个规则（如果多个规则被匹配那么优先最后一个），如果匹配到了就执行对应的 C 语言代码，如果没有匹配到则直接复制到输出，在本例中使用了两个正则表达式来分辨整数与小数，而 `asdasd` 和所有的 空格 并不被匹配因此直接被复制到输出。

下面解释下这个 `.flex` 文件：

- **defination** 部分

  在这个部分可以使用 `%option <optionname>` 来设置一些选项

  > 有关 `noyywrap`：
  >
  > 当 `yylex()` 被调用的时候，它会从全局输入文件 `yyin`（默认是 `stdin`）中扫描词法单元，直到触及 EOF 或者其中某一个 Action 执行了 `return`。当扫描器从 YY_INPUT 接收到了一个 EOF 指示，它就会调用 `yywrap()` 函数，如果它返回 false，则认为 `yyin` 已经被设置为另一个输入文件并继续扫描，如果返回 true，则会终止，并向调用者返回 0。
  >
  > 而 `yywrap()` 这个函数需要我们进行编写，如果我们不提供该函数的话在编译 `lex.yy.c` 时会提示 `undefined reference to 'yywrap'` 。
  >
  > 当然不设置这个选项也可以写一个最简单的 `yywrap()`（注意返回值规定是 `int` 型）：
  >
  > ```c
  > int yywrap() { return true; }
  > ```

  在这个部分还可以定义一些 **名称**（即可以为常用的正则表达式定义一个“别名”），

  名称的定义形如：`name definition`，其中 `name` 为一个以字母或下划线开头的由字母、下划线、数字、短横线组成的词，`defination` 会从第一个非空格字符开始一直到此行结束。

  这些名称在之后可以使用 `{name}` 进行使用，会在处理时展开为 `(defination)`。

- **rules** 部分

  包含一系列规则，形如：`pattern action`，其中 `pattern` 必须没有缩进，并且 `action` 必须在同一行开始。

  详细的有关 Pattern 的内容见 [Lexical Analysis With Flex, for Flex 2.6.2: Patterns (westes.github.io)](http://westes.github.io/flex/manual/Patterns.html#Patterns)

  有一些特殊的 action，比如：`ECHO` 直接复制到输出，详细的有关 Action 的内容见 [Lexical Analysis With Flex, for Flex 2.6.2: Actions (westes.github.io)](http://westes.github.io/flex/manual/Actions.html#Actions)

- **user code** 部分

  会被直接复制到 `lex.yy.c` 中。
