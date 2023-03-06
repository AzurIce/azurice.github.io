---
title: 『Python教程』3. 基础数据类型与变量
date: 2022-03-06T21:41:00+08:00
---

## 〇、算术运算

### 1. 一元

`+`，`-`表示正负

### 2. 二元

`+`, `-`, `*`, `/` 加 减 乘 除

`//` 整除

`%` 取余（取模）

`**` 乘方

## 一、概述

变量，顾名思义，可以改变的量。像是挖了个槽，给它起个名，然后可以把各种值塞到这个槽里。

把值塞到这个槽里的行为叫做 **赋值**，使用 `=` 来执行，它会把右侧的表达式的结果赋给左侧的变量。

例：

```python
name = 'AzurIce' # 将 'AzurIce' 这个值赋给 name变量
print(f'My mame is {name}')
```

> 输出：
>
> `My Name is AzurIce`

```python
name = 'Asurx' # 将 'Asurx' 这个值赋给 name变量
print(f'My mame is {name}')
name = 'AzurIce' # 将 'AzurIce' 这个值赋给 name变量
print(f'Now my mame is {name}')
```

> 输出：
>
> `My name is Asurx`
>
> `Now my name is AzurIce`

## 二、存储数据的类型

Python 种有如下4种基础数据类型：


| 类型    | 名称                                   | 描述                             |
| ------- | -------------------------------------- | -------------------------------- |
| int     | Integer 整型                           | 存储整数                         |
| float   | Floating Point Number 浮点型（双精度） | 存储小数                         |
| complex | Complex 复数型                         | 存储复数（表示方法例如`3 + 2j`） |
| str     | String 字符串                          | 存储字符串                       |
| bool    | Boolean 布尔型                         | 存储True或False                  |

> 还有4种高级数据类型：List列表，Tuple元组，Set集合，Dictionary字典。

```python
type(值) # 获取 值 的数据类型
```

> 例：
>
> ```python
> print(type(9))      # 输出 <class 'int'>
> print(type(9.99))   # 输出 <class 'float'>
> print(type('??'))   # 输出 <class 'str'>
> print(type(True))   # 输出 <class 'bool'>
> print(type(2.3+2j)) # 输出 <class 'complex'>
> ```

## 三、 类型转换

### 1. 隐式

```python
# int 与 float 做运算的结果为 float
# 除法运算的结果为 float
# 整除运算的结果为 int
print(type(9 + 9.99)) # 输出 <class 'float'>
print(type(9 * 9.99)) # 输出 <class 'float'>
print(type(9 / 8))    # 输出 <class 'float'>
print(type(9.9 // 8))    # 输出 <class 'float'>

# 不能自动转换
print(type('??' + 9)) # 报错
```

### 2. 显式

```python
类型名(值) # 将 值 转换为 类型名 类型
```

```python
print(int(True))  # 1
print(int(False)) # 0
print(int(9.8))   # 9
print(int("98"))  # 98
print(int("9.8")) # 报错
```

```python
print(float(True))  # 1.0
print(float(False)) # 0.0
print(float(9.8))   # 9.8
print(float("98"))  # 98.0
print(float("9.8")) # 9.8
```

```python
print(str(True))  # True
print(str(False)) # False
print(str(9.8))   # 9.8
```

> 其实直接用 print 打印时，就隐式的转换为了 str:
>
> ```python
> print(True) # 相当于 print(str(True))
> print(999)  # 相当于 print(str(999))
> ```

注意 bool：

```python
print(bool(9.8))   # True
print(bool(0.0))   # False
print(bool(0))     # False
print(bool(-1))     # True
print(bool("9.8")) # True
print(bool("0"))   # True
print(bool(""))    # False
```

> 对于字符串，非空即为 True
>
> 对于数字，非0即为 True

## 四、一些例子

### 1. 计算x的阶乘

```python
x = int(input())
ans = 1
while x:
    ans = ans * x
    x = x - 1;
print(ans)
```

### 2. 计算1到n的自然数平方和

```python
x = int(input())
ans = 0
while x:
    ans = ans + (x ** 2)
    x = x - 1;
print(ans)
```
