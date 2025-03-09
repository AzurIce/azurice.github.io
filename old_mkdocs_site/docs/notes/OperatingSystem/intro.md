# 引入及相关概述等知识

如果编写过一个 “裸机程序”，可以理解代码与硬件有着极大的关联性，要手动操作内存布局、手动操作寄存器调整 CPU 状态、手动与各种硬件交互，而且这些对于每一个不同的硬件都有所区别，操作系统的存在便是作为应用和硬件之间的一个桥梁：

- 将各种复杂繁琐的硬件操作接口加以包装，提供给应用程序统一的抽象。
- 管理内存、CPU 等资源，提高资源使用率。
- ......

---

### Shell 与 Terminal 有什么区别？

Shell 是芯，Terminal 是壳。

Shell 接收用户输入，传递给操作系统或其他程序（比如 bash、fish、zsh 等）。

Terminal 提供一个窗口来承载 Shell（比如 Windows Terminal、Alacritty、Kitty 等）。

---

Characteristics of an OS:

- Concurrency
- Sharing (Exclusive(抢占) sharing)
- Virtualization (like Virtual Memory)