**WSL（Windows Subsystem for Linux）** ，又叫做 **适用于 Linux 的 Windows 子系统**，可以在 Windows 中运行 GNU/Linux 环境，且不产生传统虚拟机或者双系统的启动开销。

它历经两个版本：

- WSL1：没有使用虚拟机，但 Linux 内核不完整，不过跨 OS 的文件系统性能高于 WSL2。
- WSL2：使用了虚拟机，有完整的 Linux 内核，以及完整的系统调用兼容性，甚至借助 WSLg 可以实现在 Windows 中使用 Linux 中的 GUI 应用。

本文将使用 WSL2。

## 一、安装

在终端中运行以下命令：

```powershell
wsl --install
```



