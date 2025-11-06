#set document(title: "x86_64 最小 Linux 内核")

#title()

= 编译内核

首先 Clone 下来内核源码：

```shell-unix-generic
git clone https://github.com/torvalds/linux.git -b v6.17 --depth=1
```

然后调整 make 配置：

```shell-unix-generic
make tinyconfig # 应用最小配置
make menuconfig # 进入配置菜单 可能需要 ncurses 库
```

```
[*] 64-bit kernel
General setup -> [*] Initial RAM filesystem and RAM disk (initramfs/initrd) support
General setup -> [*] Configure standard kernel features (expert users) -> [*] Enable support for printk
Executable file formats -> [*] Kernel support for ELF binaries
Device Drivers -> Character devices -> [*] Enable TTY support
```

编译：

```shell-unix-generic
make -j<x> # 使用 x 个 CPU 核心编译
```

编译完成后，内核镜像文件位于 `arch/x86/boot/bzImage`。此时如果尝试直接用 qemu 启动会得到 Kernel panic，因为没有 initramfs。

= 做个极简的 init 作为 initramfs

```c
// ~/fun/init.c
#include <unistd.h>
#include <sys/wait.h>

int main()
{
  char prompt[] = "# ";
	char command[256];
	for (;;) {
		write(1, prompt, sizeof(prompt) - 1);
		int count = read(0, command, sizeof(command));
		command[count - 1] = 0;
		pid_t pid = fork();
		if (pid == 0) {
			execve(command, 0, 0);
			break;
		} else {
			siginfo_t info;
			waitid(P_ALL, 0, &info, WEXITED);
		}
	}

	_exit(0);
}
```

```shell-unix-generic
# At ~/fun/
gcc init.c -static -o init
echo init | cpio -H newc -o > initramfs.cpio
```

然后可以用 `make isoimage` 来将 kernel 和 initramfs 打包成一个 ISO 镜像，然后用 qemu 启动：

```shell-unix-generic
cd linux
make isoimage FDARGS="initrd=/initramfs.cpio"  FDINITRD=~/fun/initramfs.cpio
qemu-system-x86_64 -cdrom arch/x86/boot/image.iso
```

然后就可以看到你自己的简单 shell 了。然而现在并没有什么用，因为 initramfs 里没有任何其他程序（）。

= Busybox

Clone 下来 Busybox 然后：

```shell-unix-generic
make defconfig
make && make install
```

会在 `busybox/` 下产生一个 `_install` 目录：

```shell-unix-generic
root@ecs-c89d ~# ls busybox/_install/
bin/  dev/  linuxrc@  sbin/   usr/
```

用类似的办法可以将这整个目录做成 initramfs：

```shell-unix-generic
cd busybox/_install/
find . | cpio -H newc -o | gzip > ../../initramfs.cpio.gz
```

然后除了做成 image.iso 也可以在启动的时候通过参数来制定加载它：

```shell-unix-generic
qemu-system-x86_64 -kernel linux/arch/x86_64/boot/bzImage -initrd initramfs.cpio.gz \
  -append "init=/linuxrc" -display vnc=:1
```

然后启动时运行的 linuxrc 其实就是 busybox 的 sh。

= 杂

== qemu 小技巧

如果在有图形系统的 Linux 本机上，运行 qemu 后会用 gtk 创建一个窗口，并显示虚拟机的画面。但是如果是 ssh 或者无图形系统，就会比较麻烦，有些教程会建议用诸如 `-nodisplay -serial mon:stdio` 之类的参数，但是很麻烦，有时候也并不管用。一个简单的方法是使用 VNC 显示虚拟机画面：

```
qemu ... -display vnc=:1
```

然后通过 VNC 客户端连接到 `:5901` 就可以看到虚拟机画面了。






