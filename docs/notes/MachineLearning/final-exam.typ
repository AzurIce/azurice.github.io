#set page(margin: 1cm, flipped: true)

#set text(font: "LXGW Bright")

= 机器学习期末

#columns(3)[
== 符号定义

/ $m$: 训练样本数量
/ $n$: 特征数量
/ $x$: 特征/输入变量
/ $y$: 目标/输出变量
/ $h$: 学习算法的解决方案/函数（假设）
/ $hat(y) = h(x)$: 预测结果
/ $(x, y)$: 训练样本
/ $(x^((i)), y^((i)))$: 第 $i$ 个观察样本

== 第二章 线性模型
=== 2.1 线性回归
==== 一元线性回归

$w = (sum_(i=1)^m (x^((i)) - macron(x)) (y^((i)) - macron(y))) / (sum_(i=1)^m (x^((i)) - macron(x))^2)$

$b = macron(y) - w macron(x)$

== 第三章 感知机与神经网络
=== 3.1. 感知机


]

