---
title: 记 Archlinux 安装
date: 2021-09-19 19:53:57
categories: Archlinux
tags: 
  - Archlinux
  - log
---

## 记 Archlinux 安装

### 一、准备

#### 1.1 获取镜像

Archlinux 官方下载 [Arch Linux - Downloads](https://archlinux.org/download/)

清华大学开源软件镜像站 [Index of /archlinux/iso/ | 清华大学开源软件镜像站 | Tsinghua Open Source Mirror](https://mirrors.tuna.tsinghua.edu.cn/archlinux/iso/)

#### 1.2 制作启动U盘

我是使用的 Rufus 制作的启动U盘。

<img src="H:\__BlogHexo__\source\images\记Archlinux安装\image-20210919185540526.png" alt="image-20210919185540526" style="zoom:50%;" />

### 二、安装

启动进入启动U盘。

#### 2.1 连接到因特网

使用 **iwctl（iNet wireless daemon）** 来连接 **WLAN** 。

1. 进入 **iwctl** 的交互式提示符

   ```bash
   iwctl
   ```

   然后交互式提示就会以 `[iwd]#` 前缀显示出来了，随后可以使用 `<Tab>` 来进行自动补全。

2. 列出所有 WiFi 设备

   ```bash
   device list
   ```

3. 列出所有可用的网络

   ```bash
   station 1中设备名 get-networks
   ```

4. 连接到网络

   ```bash
   station 2中设备名 connect 3中SSID
   ```

   若有密码则会提示输入。

> `iwd` 会自动将网络密码存储在 `/var/lib/iwd` 目录中，以后就可以使用其自动连接记住的网络。

> 有关 `iwctl` 的更多，参阅我的另一篇文章 [没写呢]()

#### 2.2 更新系统时间

```bash
timedatectl set-ntp true
```

#### 2.3 建立硬盘分区

系统如果识别到磁盘，就会将其分配为一个 **块设备（见我的linux笔记 [没写呢]()）** ，如 `/dev/sda`、`/dev/nvme0n1` 或 `/dev/mmcblk0`。
可以使用 `fdisk` 查看

```bash
fdisk -l
```

结果中以 `rom`、`loop` 或者 `airoot` 结尾的设备可以被忽略。

使用 `fdisk` 来修改分区表：

```bash
fdisk /dev/the_disk_to_be_partitioned
```

之后在 `fdisk` 中修改（略）。

格式化建立好的分区：

将分区格式化为为 swap 格式

```bash
mkswap /dev/swap_parition
```

将分区格式化为 ext4 格式

```
mkfs.ext4 /dev/root_parition
```

启用 swap 分区

```bash
swapon /dev/swap_parition
```

挂载根目录

```bash
mount /dev/root_parition /mnt
```

挂载 EFI 分区

```bash
mkdir /mnt/efi
mount /dev/efi_parition /mnt/efi
```

#### 2.4 修改pacman源

1. 修改 `/etc/pacman.conf`

   有一行 `#Color` 可以去掉注释，会使得安装提示彩色显示

2. 修改 `/etc/pacman.d/mirrorlist`

   最上方空行添加清华源

   ```
   Server = https://mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os/$arch
   ```

#### 2.5 安装

```bash
pacstrap /mnt base linux linux-firmware
```

#### 2.6 生成fstab文件

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

#### 2.7 进入系统

```bash
arch-chroot /mnt
```

### 三、配置系统

#### 3.1 配置时区并同步时间

```bash
ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
```

```
hwclock --systohc
```

#### 3.2 本地化

见 [Installation guide (简体中文) - ArchWiki (archlinux.org)](https://wiki.archlinux.org/title/Installation_guide_(简体中文)#准备安装映像)

#### 3.3 网络配置

见 [Installation guide (简体中文) - ArchWiki (archlinux.org)](https://wiki.archlinux.org/title/Installation_guide_(简体中文)#准备安装映像)

#### 3.4 root密码

```bash
passwd
```

#### 3.5 安装引导程序

```bash
pacman -S grub efibootmgr intel-ucode os-prober
```

> 英特尔处理器则安装 intel-ucode，amd处理器则安装 amd-ucode

```bash
mkdir /boot/grub
```

编辑 `/etc/default/grub` ，添加

```
GRUB_DISABLE_OS_PROBER=false
```

生成配置

```bash
grub-mkconfig -o /boot/grub/grub.cfg
```

> 一定注意是 `/boot/grub/grub.cfg` 而非 `/efi/grub/grub.cfg`

安装

```bash
grub-install --target=x86_64-efi --efi-directory=/efi
```

> 可以使用 `uname -m` 来确认架构。

#### 3.5 安装一些必要软件

```bash
pacman -S networkmanager
```

#### 3.6 重启

退出 chroot 环境

```
exit
```

```
umount -R /mnt
```

```bash
reboot
```

 