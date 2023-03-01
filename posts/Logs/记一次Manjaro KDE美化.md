---
title: "记一次Manjaro KDE美化"
date: 2020-10-13T22:47:00+08:00
draft: false
categories:
    - Logs
tags:
    - Linux
    - Manjaro
    - KDE
---

<img src="http://www.azurice.com:7777/api/v3/file/get/83/Screenshot_20201108_093301.png?sign=7V2NFjmfQCyWzYBRDkev9yTkUDlCOFZmUt2U1pya_MQ%3D%3A0" alt="Screenshot_20201108_093301" style="zoom:20%;" /><img src="http://www.azurice.com:7777/api/v3/file/get/87/%E6%B7%B1%E5%BA%A6%E6%88%AA%E5%9B%BE_plasmashell_20201108123817.png?sign=O8Fr26-sNMjzfW_WypXO8dASPEjF7qRuQVzJKhKMwyY%3D%3A0" alt="深度截图_plasmashell_20201108123817" style="zoom:20%;" />

## 〇、配置国内源

1. 切换镜像源至中国

	`sudo pacman-mirrors -i -c China -m rank` 

	在弹出的窗口中选一个最快的源

2. 更新

	`sudo pacman -Syyu`

## 一、好东西们

> **pacman** `-S` 安装， `-R` 卸载。

### 0. OvO

#### 总是最先要安装的东西们

```shell
sudo pacman -S git    # git版本管理
sudo pacman -S neovim # 编辑器
sudo pacman -S yay    # 一些pacman无法安装的包可以用这个来安装
```

#### 总是最先要搞定的东西们

- 把 `/home` 下的各种目录名改成英文

	先重命名。

	再修改目录映射文件 `~/.config/user-dirs.dirs`

	```
	XDG_DESKTOP_DIR="$HOME/Desktop"
	XDG_DOCUMENTS_DIR="$HOME/Documents"
	XDG_DOWNLOAD_DIR="$HOME/Downloads"
	XDG_MUSIC_DIR="$HOME/Music"
	XDG_PICTURES_DIR="$HOME/Pictures"
	XDG_PUBLICSHARE_DIR="$HOME/Public"
	XDG_TEMPLATES_DIR="$HOME/Templates"
	XDG_VIDEOS_DIR="$HOME/Videos"
	```

	重启

### 1. 输入法

基于 `fcitx5` 配置 `rime-cloverpinyin`

1. 安装 `fcitx5` 输入法框架

	`pacman -S fcitx5 fcitx5-qt fcitx5-gtk fcitx5-configtool`

	配置环境变量，编辑 `~/.xprofile` ，写入：

	```
	export GTK_IM_MODULE=fcitx5
	export QT_IM_MODULE=fcitx5
	export XMODIFIERS="@im=fcitx5"
	
	export LANG="zh_CN.UTF-8"
	export LC_CTYPE="zh_CN.UTF-8"
	```

	> 配置开机启动，系统设置->工作区->开机和关机->自动启动->添加程序，添加 `fcitx5`

2. 安装 `rime`

	`pacman -S fcitx-rime`

3. 配置 `cloverpinyin` 输入方案

	到 https://github.com/fkxxyz/rime-cloverpinyin/releases 下载最新的release，并直接解压到 `~/.local/share/fcitx5/rime` 目录中。

	> rime初次初始化完成后才会有这个目录

	在相同目录创建 `default.custom.yaml` ，写入：

	```yaml
	patch:
	  "menu/page_size": 9
	  schema_list:
	    - schema: clover
	```

	> `"manu/page_size"` 为每页候选词数目，可根据自己习惯设为1~9。

现在，找到一个可以打字的地方，切换到 `rime` 输入法，右键系统托盘的图标，点击重新部署，待加载完成可以看到出现 `🍀四叶草简体拼音` 。

### 2. Shell

配置 `oh-my-zsh`

1. 修改默认 shell 为 zsh

  `chsh -s /usr/bin/zsh`

  

2. 安装 `oh-my-zsh`

	`wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | sh`

    > 如果多次提示被拒绝，就修改 `/etc/hosts` ：
    >
    > ```
    > # GitHub Start
    > 151.101.76.133 raw.githubusercontent.com
    > # GitHub End
    > ```

	> 重启后即会生效
	
3. 优化体验

	- 命令高亮 `zsh-syntax-highlighting`

		```
		git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
		```

	- 记住之前使用过的命令 `autosuggestions`

		```text
		git clone git://github.com/zsh-users/zsh-autosuggestions ~/.oh-my-zsh/custom/plugins/zsh-autosuggestions
		```

	修改 `~/.zshrc` 启用插件

	```
	plugins=(git zsh-syntax-highlighting zsh-autosuggestions sudo extract)
	```

	> **sudo** 是自带插件，双击Esc在输入命令开头添加 `sudo`
	>
	> **extract** 也是，不同文件可以直接使用 `extract xxx` 来解压

### 3. 录视频、截图、写文章相关

- 录屏 `sudo pacman -S simplescreenrecorder`

- 键盘按键显示 `sudo pacman -S screenkey`

- 截图 `yay -S deepin-screenshot`

	系统设置 -> 快捷键 -> 自定义快捷键 ->编辑 -> 新建 -> 全局快捷键 -> 命令/URL

	> 名称、注释、触发器按自己喜好配置，习惯了Win10的 `<Windows>` + `<Shift>` + `<s>` ，就设置成一样的了。
	>
	> 命令/URL：`deepin-screenshot`

	> KDE有一个自带的截图工具 `Spectacle` ，不过没有 `deepin-screenshot` 好用，如果有快捷键冲突可以在全局快捷键里将其禁用。

### 4. 编辑器

- neovim `sudo pacman -S neovim`

## 二、美化

### 面板部分

1. 删除默认面板

	右键下方面板 -> 编辑面板 -> More Options... -> 删除面板

2. 创建空面板

	右键桌面 -> 添加面板 -> 空面板

	右键面板 -> 编辑面板 -> 点击并按住屏幕边缘，拖至上方。

	>  自行调整高度

3. 向面板添加部件，我的布局为：

	程序启动器 - 数字时钟 - 全局菜单 - **间距** （在面板编辑中点击添加间距） - *Panon* - **间距** - *Netspeed Widget* - 系统托盘 - 调度器

	这里会用到两个需要安装的很棒的部件：

	>  在添加部件界面最下方点击 获取新部件 -> 下载新 Plasma 部件 ，搜索安装即可

	- **Panon** 一个可视化不同频率声音的 ~装B（划掉）~ 部件

	  > 需要前置 `sudo pacman -S qt5-websockets python-docopt python-numpy python-pyaudio python-cffi python-websockets`

	  我的配置是宽度770，中立（反）方向北，视觉特效bar1ch,width7，width2，decay0.007，音频数据源PulseAudio，输入设备混合所有麦克风和扬声器，减小低音权重，音域0~9000Hz

	- **Netspeed Widget** 一个显示实时上/下载速度的部件
	
		我的配置：kayoutSidebyside，FontSize80%

### Dock部分

1. 安装 **Latte-Dock**

	```
	sudo pacman -S latte-dock
	```

2. 配置

	右键，布局 -> 我的布局（或者新建一个）

	右键，面板设置，先选择为面板

	打开高级

	行为：底部，两端对齐，避开活动窗口，延迟显示100隐藏50，悬停时显示标题提示

	外观：绝对大小48鼠标悬停时放大0最大46%边距5%高度20%，颜色Plasma，背景大小100%不透明度45%

	特效：阴影大小30%不透明度70%，任务指示装饰Latte，Latte指示装饰选项中样式为直线，光晕关掉，为最小化窗口使用不同颜色，窗口活动室显示额外的点。任务：动作悬停预览和高亮窗口

3. 我的布局

	应用程序面板 - *Latte Separator* - 对齐分割器 - 固定的应用 - 对其分割器 - *Latte Separator* - 回收站

	这里又会用到一个小部件：

	- **Latte Separator** 安装方式同上面的两个。

### KDE主题部分

系统设置 -> 全局主题：微风

Plasma样式：Breeze Transparent

应用程式风格 -> 窗口装饰：Glassy

颜色：亮色微风

图标：Tela

光标：亮色微风

> 没有的可以在右下的获取主题处获得

系统设置 -> 工作区 -> 工作空间行为

桌面特效：摆动窗口、破碎

焦点：黯淡未激活窗口

系统设置 -> 工作区 -> 窗口管理 -> 窗口行为

焦点：焦点跟随鼠标（鼠标优先）聚焦延迟77



好啦，注销/重启后就好啦！