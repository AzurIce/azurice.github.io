## 一、分支（if）

在程序中，经常会有做判断的需求，例如输入一个分数，来判断是否及格，这时候就需要使用 `if` 语句。

```python
if expression:
    something you want to do
    some other thing you want to do
```

当 `expression` 的值为 `True` 时，则会执行下方缩进一层级的代码。

当然还可以衔接 `else`，字面意思，值为 `False` 时执行。

```python
score = input("Please input youre score: ")
if score < 60:
    print("Fail")
else:
    print("Pass")
```

你会发现这段代码执行会有问题：

```
Traceback (most recent call last):
  File "C:/Users/xiaob/Desktop/test.py", line 2, in <module>
    if score < 60:
TypeError: '<' not supported between instances of 'str' and 'int'
```

这是因为 `input()` 获取到的内容是 str（字符串）类型的，python把它当作若干个字符的序列，而非一个数字。使用 `eval()` 可以将字符串的内容当作 python表达式 并得到这个表达式的结果，即 `eval('9 + 9 * 10')` 的结果是数字类型的 `99`。

```python
score = eval(input("Please input youre score: "))
if score < 60:
    print("Fail")
else:
    print("Pass")
```

如果你想要实现更多的分支可以使用 `elif`：

```python
score = eval(input("Please input youre score: "))
if 90 <= score:
    print("A")
elif 85 <= score <= 89:
    print("A-")
elif 81 <= score <= 84:
    print("B+")
elif 78 <= score <= 80:
    print("B")
elif 75 <= score <= 77:
    print("B-")
elif 72 <= score <= 74:
    print("C+")
elif 68 <= score <= 71:
    print("C")
elif 65 <= score <= 67:
    print("C-")
elif 63 <= score <= 64:
    print("D+")
elif 61 <= score <= 62:
    print("D")
elif score < 60:
    print("F")
```

其实分支条件还可以简化，因为下一个 `elif` 判断时注定不满足前一个条件：

```python
score = eval(input("Please input youre score: "))
if score >= 90:
    print("A")
elif score >= 85:
    print("A-")
elif score >= 81:
    print("B+")
elif score >= 78:
    print("B")
elif score >= 76:
    print("B-")
elif score >= 72:
    print("C+")
elif score >= 68:
    print("C")
elif score >= 65:
    print("C-")
elif score >= 63:
    print("D+")
elif score >= 61:
    print("D")
else:
    print("F")
```

## 二、循环 while

与 `if` 很像，不过每次执行完内部语句后都会回来检查条件是否为真，若为真则再执行一次内部语句，以此循环。

```
while expression:
    something you want to do
    some other thing you want to do
```

那么对于上面的程序，就可以补充一个“如果输入不在0~100之间则重新要求输入”的功能：

```python
score = eval(input("Please input youre score: "))
while score < 0 or score > 100:
    score = eval(input("Please input youre score: "))
```

