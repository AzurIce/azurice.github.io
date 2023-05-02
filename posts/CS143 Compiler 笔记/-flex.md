## 一、有关 flex

flex 是一个用于生成 **扫描器** 的工具。**扫描器** 是一个能识别文本中词汇模式的程序。

flex 程序读取给定的输入文件，如果没有给定文件名，则读取其标准输入，以获取要生成的扫描器的描述。该描述以正则表达式-C语言代码对的形式出现，称为规则。flex生成一个C源文件作为输出，默认为lex.yy.c，其中定义了一个例程 yylex()。该文件可被编译并与flex运行库连接以产生一个可执行文件。当可执行文件被运行时，它分析其输入的正则表达式的出现情况。每当它发现一个，它就会执行相应的C代码。



当 `yylex()` 被调用的时候，它会从全局输入文件 `yyin`（默认是 `stdin`）中扫描词法单元，直到触及 EOF 或者其中某一个 Action 执行了 `return`。当扫描器从 YY_INPUT 接收到了一个 EOF 指示，它就会查看 `yywrap()` 函数，如果它返回 false，则认为 `yyin` 已经被设置为另一个输入文件并继续扫描，如果返回 true，则会终止，并向调用者返回 0。



如果不提供你的 `yywrap()` 的话就必须使用 `%option noyywrap` 或使用 `-lfl`（返回 1）

### 1. 文件格式

输入文件的格式：

```flex
definitions
%%
rules
%%
user code
```

#### 1.1 Definition 部分

用于定义一些名称或起始条件。

名称的定义形如：`name definition`，其中 `name` 为一个以字母或下划线开头的由字母、下划线、数字、短横线组成的词，`defination` 会从第一个非空格字符开始一直到此行结束。

在一个名称被定义后，之后可以通过 `{name}` 来使用，会在处理时展开为 `(defination)`。

比如为了方便进行如下定义：

```
	DIGIT    [0-9]
```

随后可以使用

```
{DIGIT}+"."{DIGIT}*
```

它与 `[0-9]+"."([0-9])*` 等价。



此外，没有缩进的注释会被复制到输出。

任何有缩进的内容或被包含在 `%{` 和 `%}` 之间的内容也会被复制到输出（这一对字符本身不能有缩进）。

#### 1.2. Rules 部分

包含一系列规则，形如：`pattern action`，其中 `pattern` 必须没有缩进，并且 `action` 必须在同一行开始。

详细的有关 Pattern 的内容见 [Lexical Analysis With Flex, for Flex 2.6.2: Patterns (westes.github.io)](http://westes.github.io/flex/manual/Patterns.html#Patterns)

详细的有关 Action 的内容见 [Lexical Analysis With Flex, for Flex 2.6.2: Actions (westes.github.io)](http://westes.github.io/flex/manual/Actions.html#Actions)

ECHO 直接复制到输出

#### 1.3. User Code 部分

这部分会直接 copy 到 `lex.yy.c` 里面，是可选的。

### 2. Start Conditions

Start conditions are declared in the definitions (first) section of the input using unindented lines beginning with either ‘%s’ or ‘%x’ followed by a list of names. The former declares *inclusive* start conditions, the latter *exclusive* start conditions. A start condition is activated using the `BEGIN` action. Until the next `BEGIN` action is executed, rules with the given start condition will be active and rules with other start conditions will be inactive. If the start condition is inclusive, then rules with no start conditions at all will also be active. If it is exclusive, then *only* rules qualified with the start condition will be active.

`<*>` 匹配所有 Start Condition。

BEGIN(0) 返回初始状态（即 INITIAL）

可以在 action 中使用 BEGIN xxx 来开始一个状态



比如一个识别 C 注释的扫描器：

```flex
    %x comment
    %%
            int line_num = 1;

    "/*"         BEGIN(comment);

    <comment>[^*\n]*        /* eat anything that's not a '*' */
    <comment>"*"+[^*/\n]*   /* eat up '*'s not followed by '/'s */
    <comment>\n             ++line_num;
    <comment>"*"+"/"        BEGIN(INITIAL);
```

## 二、一些例子

### 1. Hello World

下面编写一个简单的例子，当遇到字符串 `"test"` 的时候将其替换为 `"Hello world 2023"`。

编写 `hello_world.flex`：

```flex
%option noyywrap

%%
test printf("Hello world %d", 2023);
%%
int main() {
    yylex();
}
```

```
flex -o hello_world.yy.c .\hello_world.flex
g++ -o hello_world .\hello_world.yy.c
```

```
./hello_world
test asdhsadjhktest
Hello world 2023 asdhsadjhkHello world 2023
```

### 2. 字符数统计

编写 `char_count.flex`：

```flex
%option noyywrap

%{
int num_lines = 0, num_chars = 0;
%}

%%
\n      ++num_lines; ++num_chars;
.       ++num_chars;
%%
int main() {
    yylex();
    printf( "# of lines = %d, # of chars = %d\n", num_lines, num_chars );
}
```

```shell
flex -o char_count.yy.c .\char_count.flex
g++ -o char_count .\char_count.yy.c
```

```
./char_count
asdhfaskfasd
f sd
fa
sd
s
sad
c
sa

# of lines = 9, # of chars = 36
```

> 最后使用 `CTRL-C` 退出时得到输出。



## 参考

[词法分析器flex - 知乎 (zhihu.com)](https://zhuanlan.zhihu.com/p/52290783)
