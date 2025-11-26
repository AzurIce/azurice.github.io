# 前端新技术

距离上一次写前端已经过去又一阵时间了，那时候趁着 svelte 和 solidjs 刚出来的时候各拿来尝试着用作业项目练了练手。最近因为在写京云杯#4的计算器，又开始碰前端的东西了，突然发现好像多了很多比较新的设计思想/编程模式/技术架构。

所以，开一篇文章简单记录一下。

## 一、无头 UI 库 | Headless UI Libraries

写京云杯#4的时候，决定用 solidjs 框架来写，然后发现之前用的 suid 依然有很多问题，且最关键的是，在使用 bun 的时候，suid 无法通过打包（https://github.com/swordev/suid/issues/297），于是开始找起了新的 UI 组件库。

于是找到了 Kobalte、Solid UI、chadcn-solid、ark-ui，以及一个概念 —— 无头 UI 库（Headless UI Libraries）

意思就是 —— 不带有任何样式预设，专注于逻辑与功能的组件库。

