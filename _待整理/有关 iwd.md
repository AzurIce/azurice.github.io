---
title: 有关 iwd
date: 2021-09-19 20:23:27
categories: Archlinux
tags: 
  - Archlinux
---

## 有关 iwd

#### 断开网络连接

```bash
station device disconnect
```

#### 显示设备和连接信息

要显示 WiFi 设备详细情况，比如 MAC 地址：

```
[iwd]# device device show
```

要显示包括 WiFi 设备的连接网络在内的连接状态：

```
[iwd]# station device show
```

#### 管理已知网络

要列出以前连接过的网络：

```
[iwd]# known-networks list
```

要忘记已知的网络：

```
[iwd]# known-networks SSID forget
```
