---
title: 记 Archlinux 安装（二）
date: 2022-01-30 12:50
categories: Archlinux
tags:
  - Archlinux
  - log
---

## 一、连接WIFI
1. 启动 `NetworkManager` 服务

	```
	$ systemctl enable NetworkManager
	$ systemctl start NetworkManager
	```
	
1. 使用 `nmcli`，连接网络

	```
	$ nmcli device wifi list
	$ nmcli device wifi connect _SSID_ password _password_
	```

## 安装各种东西
### 基础开发用
```
pacman -S base-devel
```
### 添加用户
```
useradd -m -G wheel azurice
passwd azurice
```

编辑sudo文件

```
ln -s /usr/bin/nvim /usr/bin/vi
visudo
```
取消注释
```
## Uncomment to allow members of group wheel to execute any command
%wheel ALL=(ALL) ALL
```


wayland
plasma plasma-applications plasma-wayland-session sddm sddm-kcm
sudo systemctl enable sddm

ntfs-3g
