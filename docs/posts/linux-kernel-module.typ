#set document(title: "编写 Linux 内核模块")

#title()

= 环境

华为云 ECS 服务器

```shell-unix-generic
root@ecs-3c5a ~# neofetch
            .-/+oossssoo+/-.               root@ecs-3c5a
        `:+ssssssssssssssssss+:`           -------------
      -+ssssssssssssssssssyyssss+-         OS: Ubuntu 24.04.2 LTS x8
    .ossssssssssssssssssdMMMNysssso.       Host: OpenStack Nova 13.2
   /ssssssssssshdmmNNmmyNMMMMhssssss/      Kernel: 6.8.0-59-generic
  +ssssssssshmydMMMMMMMNddddyssssssss+     Uptime: 1 hour, 55 mins
 /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/    Packages: 898 (dpkg)
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   Shell: fish 3.7.0
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   Resolution: 1024x768
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   Terminal: /dev/pts/1
ossyNMMMNyMMhsssssssssssssshmmmhssssssso   CPU: General Purpose (2)
+sssshhhyNMMNyssssssssssssyNMMMysssssss+   GPU: 00:02.0 Cirrus Logic
.ssssssssdMMMNhsssssssssshNMMMdssssssss.   Memory: 217MiB / 1775MiB
 /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/
  +sssssssssdmydMMMMMMMMddddyssssssss+
   /ssssssssssshdmNNNNmyNMMMMhssssss/
    .ossssssssssssssssssdMMMNysssso.
      -+sssssssssssssssssyyyssss+-
        `:+ssssssssssssssssss+:`
            .-/+oossssoo+/-.
```

= Linux 内核模块

Linux 内核模块是一个形如 `hello.ko` 的文件：

```shell-unix-generic
root@ecs-3c5a:~/_linux/module# file hello.ko
hello.ko: ELF 64-bit LSB relocatable, x86-64, version 1 (SYSV), BuildID[sha1]=7ebda760c804b79dd35c03a74ca02d727c904dbc, with debug_info, not stripped
```

通过 `insmod` 和 `rmmod` 可以加载和卸载模块：

```shell-unix-generic
root@ecs-3c5a:~/_linux/module# sudo insmod hello.ko # 加载 hello.ko 模块
root@ecs-3c5a:~/_linux/module# sudo rmmod hello     # 卸载 hello 模块
```

通过 `lsmod` 可以查看当前系统加载的全部模块：

```shell-unix-generic
root@ecs-3c5a:~/_linux/module# lsmod
Module                  Size  Used by
hello                  12288  0
tls                   155648  0
qrtr                   53248  2
intel_rapl_msr         20480  0
intel_rapl_common      40960  1 intel_rapl_msr
intel_uncore_frequency_common    16384  0
skx_edac_common        24576  0
nfit                   81920  1 skx_edac_common
rapl                   20480  0
i2c_piix4              32768  0
joydev                 32768  0
input_leds             12288  0
mac_hid                12288  0
serio_raw              20480  0
binfmt_misc            24576  1
sch_fq_codel           24576  3
dm_multipath           45056  0
msr                    12288  0
efi_pstore             12288  0
nfnetlink              20480  1
dmi_sysfs              24576  0
qemu_fw_cfg            24576  0
ip_tables              32768  0
x_tables               65536  1 ip_tables
autofs4                57344  2
btrfs                2039808  0
blake2b_generic        24576  0
raid10                 73728  0
raid456               196608  0
async_raid6_recov      20480  1 raid456
async_memcpy           16384  2 raid456,async_raid6_recov
async_pq               20480  2 raid456,async_raid6_recov
async_xor              16384  3 async_pq,raid456,async_raid6_recov
async_tx               16384  5 async_pq,async_memcpy,async_xor,raid456,async_raid6_recov
xor                    20480  2 async_xor,btrfs
raid6_pq              126976  4 async_pq,btrfs,raid456,async_raid6_recov
libcrc32c              12288  2 btrfs,raid456
raid1                  57344  0
raid0                  24576  0
hid_generic            12288  0
usbhid                 77824  0
hid                   180224  2 usbhid,hid_generic
crct10dif_pclmul       12288  1
crc32_pclmul           12288  0
polyval_clmulni        12288  0
polyval_generic        12288  1 polyval_clmulni
ghash_clmulni_intel    16384  0
sha256_ssse3           32768  1
sha1_ssse3             32768  0
psmouse               217088  0
pata_acpi              12288  0
cirrus                 20480  0
floppy                131072  0
aesni_intel           356352  0
crypto_simd            16384  1 aesni_intel
cryptd                 24576  2 crypto_simd,ghash_clmulni_intel
```

通过 `dmesg` 可以查看内核日志：

```shell-unix-generic
root@ecs-3c5a:~/_linux/module# dmesg | tail
[  444.503080] kauditd_printk_skb: 114 callbacks suppressed
[  444.503084] audit: type=1400 audit(1763024967.936:125): apparmor="STATUS" operation="profile_replace" info="same as current profile, skipping" profile="unconfined" name="/usr/lib/snapd/snap-confine" pid=2688 comm="apparmor_parser"
[  444.503853] audit: type=1400 audit(1763024967.937:126): apparmor="STATUS" operation="profile_replace" info="same as current profile, skipping" profile="unconfined" name="/usr/lib/snapd/snap-confine//mount-namespace-capture-helper" pid=2688 comm="apparmor_parser"
[  762.902684] hello: loading out-of-tree module taints kernel.
[  762.902691] hello: module license 'MIT' taints kernel.
[  762.902693] Disabling lock debugging due to kernel taint
[  762.902695] hello: module verification failed: signature and/or required key missing - tainting kernel
[  762.902696] hello: module license taints kernel.
[  762.902973] Hello, Ubuntu kernel module!
[  804.385241] Goodbye, Ubuntu kernel module!
```

= 使用 C 语言编写一个内核模块 | 单文件

创建一个 `hello.c` 文件：

```c
#include <linux/module.h>
#include <linux/init.h>

static int __init hello_init(void) {
    printk(KERN_INFO "Hello, Ubuntu kernel module!\n");
    return 0;
}

static void __exit hello_exit(void) {
    printk(KERN_INFO "Goodbye, Ubuntu kernel module!\n");
}

module_init(hello_init);
module_exit(hello_exit);

MODULE_LICENSE("MIT");
MODULE_DESCRIPTION("A simple Ubuntu kernel module");
MODULE_AUTHOR("AzurIce");
```
== Kbuild

Kbuild 的本质其实就是 `Makefile` 脚本，被 `Makefile` include。它基于一些约定的变量和规则使 Linux 的 Makefile 编写更加方便。

在 Kbuild 中 `obj-m` 表示要被编译为独立模块的目标文件列表。

创建一个 `Kbuild` 文件：

```make
obj-m += hello.o
```

== 构建与加载测试

然后使用 `make -C /lib/modules/$(uname -r)/build M=<dir> modules` 构建模块：

```shell-unix-generic
root@ecs-3c5a ~/_/module-c# make -C /lib/modules/6.8.0-59-generic/build M=/root/_linux/module-c modules
make[1]: Entering directory '/usr/src/linux-headers-6.8.0-59-generic'
warning: the compiler differs from the one used to build the kernel
  The kernel was built by: x86_64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0
  You are using:           gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0
  CC [M]  /root/_linux/module-c/hello.o
  MODPOST /root/_linux/module-c/Module.symvers
  CC [M]  /root/_linux/module-c/hello.mod.o
  LD [M]  /root/_linux/module-c/hello.ko
  BTF [M] /root/_linux/module-c/hello.ko
Skipping BTF generation for /root/_linux/module-c/hello.ko due to unavailability of vmlinux
make[1]: Leaving directory '/usr/src/linux-headers-6.8.0-59-generic'
```

构建完成后即得到一个 hello.ko 文件。

然后可以加载、卸载、查看日志：

```shell-unix-generic
root@ecs-3c5a ~/_/module-c# insmod hello_module.ko
root@ecs-3c5a ~/_/module-c# rmmod hello_module.ko
root@ecs-3c5a ~/_/module-c# dmesg | tail -2
[  592.588753] Hello, Ubuntu kernel module!
[  600.088256] Goodbye, Ubuntu kernel module!
```

= 使用 C 语言编写一个内核模块 | 多文件

```c
// hello.c
#include <linux/module.h>

MODULE_LICENSE("MIT");
MODULE_DESCRIPTION("A simple Ubuntu kernel module");
MODULE_AUTHOR("AzurIce");
```

```c
// hello_init.c
#include <linux/module.h>
#include <linux/init.h>

static int __init hello_init(void) {
    printk(KERN_INFO "Hello, Ubuntu kernel module!\n");
    return 0;
}

module_init(hello_init)
```

```c
// hello_exit.c
#include <linux/module.h>
#include <linux/init.h>

static void __exit hello_exit(void) {
    printk(KERN_INFO "Goodbye, Ubuntu kernel module!\n");
}

module_exit(hello_exit);
```

== Kbuild

使用 `<name>-objs` 来声明 `<name>.o` 的依赖。

```make
obj-m += module.o

module-objs := hello.o hello_init.o hello_exit.o
```

= 使用 Rust 编写一个内核模块

[Rust-for-Linux/rust-out-of-tree-module](https://github.com/Rust-for-Linux/rust-out-of-tree-module)。

用于 linux 6.8 的 commit hash：e36fc14c01e3293b5e4302b57ba18f2e90b6bd22

- 需要额外安装 `linux-lib-rust-$(uname -r)` 包
- 在 Ubuntu 上需要使用 apt install 来安装 rustc，不能使用 `rustup`（即便都是 1.75.0）
  ```
  error[E0514]: found crate `core` compiled by an incompatible version of rustc
  |
  = note: the following crate versions were found:
          crate `core` compiled by rustc 1.75.0 (82e1608df 2023-12-21) (built from a source tarball): /usr/src/linux-lib-rust-6.8.0-59-generic/rust/libcore.rmeta
  = help: please recompile that crate using this compiler (rustc 1.75.0 (82e1608df 2023-12-21)) (consider running `cargo clean` first)
  ```

== 构建与加载测试

```shell-unix-generic
root@ecs-3c5a ~/_/module-rust ((e36fc14c))# make
make -C /lib/modules/`uname -r`/build M=$PWD
make[1]: Entering directory '/usr/src/linux-headers-6.8.0-59-generic'
warning: the compiler differs from the one used to build the kernel
  The kernel was built by: x86_64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0
  You are using:           gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0
  RUSTC [M] /root/_linux/module-rust/rust_out_of_tree.o
  MODPOST /root/_linux/module-rust/Module.symvers
  CC [M]  /root/_linux/module-rust/rust_out_of_tree.mod.o
  LD [M]  /root/_linux/module-rust/rust_out_of_tree.ko
  BTF [M] /root/_linux/module-rust/rust_out_of_tree.ko
Skipping BTF generation for /root/_linux/module-rust/rust_out_of_tree.ko due to unavailability of vmlinux
make[1]: Leaving directory '/usr/src/linux-headers-6.8.0-59-generic'
```

```shell-unix-generic
root@ecs-3c5a ~/_/module-rust ((e36fc14c))# insmod rust_out_of_tree.ko
root@ecs-3c5a ~/_/module-rust ((e36fc14c))# rmmod rust_out_of_tree
```

```shell-unix-generic
root@ecs-3c5a ~/_/module-rust ((e36fc14c))# dmesg | tail -3
[ 6784.132227] rust_out_of_tree: Rust out-of-tree sample (init)
[ 6786.113010] rust_out_of_tree: My numbers are [72, 108, 200]
[ 6786.113019] rust_out_of_tree: Rust out-of-tree sample (exit)
```
