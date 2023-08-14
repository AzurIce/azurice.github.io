---
date: 2023-05-26
---

[nix-community/NixOS-WSL: NixOS on WSL(2) maintainer=@nzbr (github.com)](https://github.com/nix-community/NixOS-WSL)

首先下载最新的 `nixos-wsl-installer.tar.gz`。‘

（如果后面卡在 Starting Systemd... 就换 nixos-wsl-x86_64-linux.tar.gz

然后找一个地方创建根目录对应的文件夹 `NixOS/` 执行如下命令：

```terminal
PS C:\> wsl --import NixOS .\NixOS\ "C:\Users\xiaob\Downloads\nixos-wsl-installer.tar.gz" --version 2
正在导入，这可能需要几分钟时间。
操作成功完成。
```

然后可以通过

```terminal
wsl -d NixOS
```

来运行 NixOS。

The installer will unpack the file system and subsequently start NixOS. A few warnings about file systems and locales will pop up. You can safely ignore them. After systemd has started, you should be greeted with a bash prompt inside your fresh NixOS installation.

可以使用

```terminal
wsl -s NixOS
```

来将 NixOS 设置为默认的发行版

[NixOS on WSL – Forrest Jacobs](https://forrestjacobs.com/nixos-on-wsl/)
