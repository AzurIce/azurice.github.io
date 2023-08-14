---
date: 2023-03-14
---

## 一、Python解释器的交互模式

`python` 在命令行中有很多种使用方式，最基本的就是通过 `python 文件名.py` 来执行脚本，在 [起步](『Python教程』1. 起步) 一篇中我们还以仅仅一个 `python` 的方式使用过，结果是出现了这些输出：

```console
PS C:\Users\xiaob> python
Python 3.11.2 (tags/v3.11.2:878ead1, Feb  7 2023, 16:38:35) [MSC v.1934 64 bit (AMD64)] on win32
Type "help", "copyright", "credits" or "license" for more information.
>>>
```

这其实就是进入了 Python解释器的 **交互模式**。

> 退出方式为按 Ctrl + Z 或输入 `exit()` 后按回车。

你输入的每一行都会被作为 Python语句解析、执行，并将语句返回的结果显示出来。

比如我们输入 `2 + 2`，这个语句被执行后返回的结果便是 `4`：

```console
>>> 2 + 2
4
```

再比如我们输入 `print('yeah')`，这个语句没有返回值，但是在执行过程中会打印 `Yeah`：

```console
>>> print('Yeah')
Yeah
```

在 **交互模式中** 也可以进行变量的声明与访问，也就是说它是“有记忆性的”：

```console
>>> x = input("What's your name?")
What's your name?AzurIce
>>> x
'AzurIce'
>>> print("HelloWorld, " + x + "!")
HelloWorld, AzurIce!
```

> 第二行中 `What's your name?` 是 `input` 语句的输出，`AzurIce` 来自我键盘的输入。

这便使得在学习 Python 的过程中做一些尝试是十分简单的。

## 二、基础数据类型与运算

Python 种有如下4种基础数据类型：


| 类型    | 名称                                   | 描述                             |
| ------- | -------------------------------------- | -------------------------------- |
| int     | Integer 整型                           | 存储整数                         |
| float   | Floating Point Number 浮点型（双精度） | 存储小数                         |
| complex | Complex 复数型                         | 存储复数（表示方法例如`3 + 2j`） |
| str     | String 字符串                          | 存储字符串                       |
| bool    | Boolean 布尔型                         | 存储True或False                  |

Python 有一个内置函数 `type()`，它可以返回传入参数的类型：

```console
>>> type(9)
<class 'int'>
>>> type(9.99)
<class 'float'>
>>> type('??')
<class 'str'>
>>> type(True)
<class 'bool'>
>>> type(2.3+2j)
<class 'complex'>
```

下面详细介绍一下 int、float 和 str 以及相关运算，其他的自行查阅文档。

### 1. int、float

算数运算：

- 一元：`+`，`-`表示正负

- 二元：
  - `+`, `-`, `*`, `/` 加 减 乘 除
  - `//` 整除
  - `%` 取余（取模）
  - `**` 乘方

比如我们在交互模式中输入一些仅由数值运算组成的语句：

```console
>>> 2 + 2
4
>>> 50 - 5*6
20
>>> (50 - 5*6) / 7
2.857142857142857
>>> 8 // 5
1
>>> 8 % 5
3
>>> 2**10
1024
```

### 2. str

字符串使用 `'` 或 `"` 包裹，也可以使用 `"""` 或 `'''` 跨越多个行：

```console
>>> print('spam eggs')  # single quotes
spam eggs
>>> print('doesn\'t')  # use \' to escape the single quote...
doesn't
>>> print("doesn't")  # ...or use double quotes instead
doesn't
>>> print('"Yes," they said.')
"Yes," they said.
```

字符串之间可以使用 `+` 连接：

```console
>>> print('A' + 'B')
AB
```

也可以使用 `*` 来重复整个序列：

```console
>>> print('NB' * 6)
NBNBNBNBNBNB
```

> 其实这是序列的特性，对应的还有下标访问，将在后续文章中讲到 List 时讲解

#### 原始字符串

如果不希望前置 `\` 的字符转义成特殊字符，可以使用 **原始字符串**，在引号前添加 `r` 即可：

```console
>>> print('C:\some\name')  # here \n means newline!
C:\some
ame
>>> print(r'C:\some\name')  # note the r before the quote
C:\some\name
```

#### 多行字符串

字符串字面值可以包含多行。 一种实现方式是使用三重引号：`"""..."""` 或 `'''...'''`。 字符串中将自动包括行结束符，但也可以在换行的地方添加一个 `\` 来不包括此次换行：

```console
>>> print("""\
Usage: thingy [OPTIONS]
     -h                        Display this usage message
     -H hostname               Hostname to connect to
""")
Usage: thingy [OPTIONS]
     -h                        Display this usage message
     -H hostname               Hostname to connect to
```

### 3. bool

比较运算：

- `>`，`<`，`>=`，`<=`，`==`，`!=`  大于 小于 大于等于 小于等于 等于 不等于

布尔运算：

- `not` 非：若 `expression` 为真，则 `not expression` 为假，若为假，则为真。

- `and` 与：`expression1 and expression2` 只有都为真才是真，其他情况为假

- `or` 或：有一者为真则为真，都为假才是假。

> 计算机中常用 **布尔值** 表示 **满足** 与 **不满足**，满足为 True（真），不满足为 False（假）。

## 三、 类型转换

### 1. 隐式

```console
>>> type(9 + 9.99)
<class 'float'>
>>> type(9 * 9.99)
<class 'float'>
>>> type(9 / 8)
<class 'float'>
>>> type(9.9 // 8)
<class 'float'>
```

但是数字与字符串之间无法进行加法运算，不会进行隐式类型转换：

```console
>>> type('9' + 9)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: can only concatenate str (not "int") to str
```

### 2. 显式

```python
类型名(值) # 将 值 转换为 类型名 类型
```

转换为 int 类型：

```console
>>> int(True)
1
>>> int(False)
0
>>> int(9.8)
9
>>> int("98")
98
>>> int("9.8")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: invalid literal for int() with base 10: '9.8'
```

转换为 float 类型：

```console
>>> float(True)
1.0
>>> float(False)
0.0
>>> float(9.8)
9.8
>>> float("98")
98.0
>>> float("9.8")
9.8
```

转换为 str 类型：

```console
>>> str(True)
'True'
>>> str(False)
'False'
>>> str(9.8)
'9.8'
>>> str(9)
'9'
```

转换为 bool 类型：

```console
>>> bool(9.8)
True
>>> bool(0.0)
False
>>> bool(0)
False
>>> bool(-1)
True
>>> bool("9.8")
True
>>> bool("0")
True
>>> bool("")
False
```

> 对于字符串，非空即为 `True`
>
> 对于数字，非 `0` 即为 `True`



