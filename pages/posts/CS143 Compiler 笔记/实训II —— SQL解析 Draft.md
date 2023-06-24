## 使用 flex 生成 C++ 词法分析器

主要两种方法：

- 简单地直接用 C++ 编译器代替 C 编译器
- 也可以使用 `-+` 选项（或者添加 `%option c++`），这样就会生成 `lex.yy.cc` 而非 `lex.yy.c`



使用第二种方法：

在 `.l` 文件中的 **defination** 部分添加

```
%option c++
```

即可。



生成的 `lex.yy.cc` 文件中会引入一个 `FlexLexer.h` 头文件，其中定义了一个 `FlexLexer` 类，这是一个抽象的基类，定义了一般的扫描器类接口，在该类中提供了以下成员函数：

- `const char* YYText()` 等价于原来的 `yytext`

- `int YYLeng` 等价于原来的 `yyleng`

- `int lineno() const` 等价于原来的 `yylineno`（需要 `%option yylineno`）

- `void set_debug(int flag)`

  为扫描器设定 `debug` 开关，等价于赋值给 `yy_flex_debug`（需要 `%option debug`）

- `int debug()`

  返回当前 `debug` 开关的值

还有一个 `yyFlexLexer` 类，其中带有

- `virtual int yylex`

  等价于原来的 `yylex()`

  不过如果要自己写一个它的子类 `S` 就要通过 `%option yylclass="S"` 来告知 flex 使用该类而非 `yyFlexLexer`。

- `int yylex(istream* new_in, ostream* new_out = 0)`

  先切换 `yyin` 到 `new_in` 再返回 `yylex()` 的值

## 使用 bison 生成 C++ 文法解析器

```
%require "3.2"
%language "c++"
```

### 使用 C++ Variants

> [C++ Variants (Bison 3.8.1) (gnu.org)](https://www.gnu.org/software/bison/manual/html_node/C_002b_002b-Variants.html)

在 **prologue** 部分添加：

```
%define api.value.type variant
```



于是，原先我们需要这样写的代码：

```
%union
{
  int ival;
  std::string* sval;
}
%token <ival> NUMBER;
%token <sval> STRING;
```

就可以这样写了：

```
%token <int> NUMBER;
%token <std::string> STRING;
```

STRING 也就不再是个指针了。