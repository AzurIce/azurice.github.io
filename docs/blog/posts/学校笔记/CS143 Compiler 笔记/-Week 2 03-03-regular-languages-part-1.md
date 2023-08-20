---
date: 2023-03-18
categories:
  - School/Compiler
---

# -Week 2 03-03-regular-languages-part-1

之前讲到 **词法单元的类型（Token Class）**就是若干字符串组成的集合。



常用 **正则语言（Regular Language）**来表达这些集合。



两个基本的正则表达式：

'c' = {"c"}

ɛ = {""}

复合正则表达式：

并（Union）

链接（Concatenation）

迭代（Iteration）

<img src="Week 2 03-03-regular-languages-part-1.assets/image-20230318165455880.png" alt="image-20230318165455880" style="zoom:67%;" />





某一个字母表 $\Sigma$ 上的正则表达式就是最小正则表达式的集合包含：

R = ɛ

  | 'c'    c ∈ $\Sigma$

  | R + R

  | RR

  | R*

上面这种用于描述正则表达式的方式称为 **文法（Grammar）**



比如对于字母表 $\Sigma = \{0, 1\}$，可以写出其上的正则表达式包含：

1*

(1 + 0)1

0* + 1*

(0 + 1)\*（由于这个正则表达式包含了所有字母表能组成的字符串，所以也称其为 $\Sigma^*$）



同一个正则语言表达方法也不唯一，比如 (1 + 0)1 也可以写作 11 + 01