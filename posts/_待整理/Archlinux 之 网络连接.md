---
title: Archlinux 之 网络连接
date: 2021-09-21 11:25:57
categories: Archlinux
tags: 
  - Archlinux
---



## Archlinux 之 网络连接

### iproute2

**iproute2** 是一个 `base` 包的依赖，提供 `ip` 命令行接口，用于管理网络接口、IP地址和路由表。

### Network interfaces

#### 前缀含义：

| Prefix | Desctiption      |
| ------ | ---------------- |
| `en`   | Wired / Ethernet |
| `wl`   | Wireless / WLAN  |
| `ww`   | WWAN             |

> `lo` 是 [virtual loopback interface](https://en.wikipedia.org/wiki/Loopback#Virtual_loopback_interface) 不用于建立网络连接。

#### 列出 Network interfaces

```bash
ip link
```

#### 启用/禁用网络接口

```bash
ip link set 接口名 up或down
```

### DHCP

安装 **dhcpcd** 后启动服务

```bash
systemctl start dhcpcd
```

### Network Manager

