---
title: 记一次Typecho博客部署
date: 2022-02-15T13:15:00+08:00
---

# 记一次Typecho博客部署

## 〇、服务器

华为云ecs弹性云服务器
1vCPUs 1GiB s6.small.1

Debian 10.0.0 64bit

## 一、更新系统

查看系统版本：

```
cat /etc/os-release
```

> ```
> root@ecs-8bef:~# cat /etc/os-release 
> PRETTY_NAME="Debian GNU/Linux 10 (buster)"
> NAME="Debian GNU/Linux"
> VERSION_ID="10"
> VERSION="10 (buster)"
> VERSION_CODENAME=buster
> ID=debian
> HOME_URL="https://www.debian.org/"
> SUPPORT_URL="https://www.debian.org/support"
> BUG_REPORT_URL="https://bugs.debian.org/"
> ```

可以在此处查看不同版本代号对应包的版本：[Debian -- 软件包](https://www.debian.org/distrib/packages)

清华源Debian源：[debian | 镜像站使用帮助 | 清华大学开源软件镜像站 | Tsinghua Open Source Mirror](https://mirrors.tuna.tsinghua.edu.cn/help/debian/)

---

修改 `/etc/apt/sources.list`，使用清华源，版本代号 `testing`

> 这样才能安装到一些软件的较新版本（比如nvim0.6、php8.1等）

```
apt update
apt install aptitude
aptitude upgrade
```

更新后查看系统版本可以发现已经改变：

```bash
cat /etc/os-release
```

> ```
> PRETTY_NAME="Debian GNU/Linux bookworm/sid"
> NAME="Debian GNU/Linux"
> ID=debian
> HOME_URL="https://www.debian.org/"
> SUPPORT_URL="https://www.debian.org/support"
> BUG_REPORT_URL="https://bugs.debian.org/"
> ```

## 二、部署 typecho

### 1. 安装一些东西

```
aptitude install nginx
```

如果访问服务器IP出现 Welcome to nginx! 网页，则安装成功

```
aptitude install php php-curl php-sqlite3 sqlite3 php-mbstring php-fpm
```

### 2. 下载 Typecho

将 `.tar.gz` 中 `build` 内的内容放到 `/var/www/html` 目录下，并删除原本的 `.html` 文件

> 注意要下载开发板，1.1版本并不支持php8

修改 `www` 目录的权限

```
chmod -R 777 /var/www
```

### 3. 配置nginx

编辑 `/etc/nginx/sites-enabled/default`

默认内容：

```nginx
##
# You should look at the following URL's in order to grasp a solid understanding
# of Nginx configuration files in order to fully unleash the power of Nginx.
# https://www.nginx.com/resources/wiki/start/
# https://www.nginx.com/resources/wiki/start/topics/tutorials/config_pitfalls/
# https://wiki.debian.org/Nginx/DirectoryStructure
#
# In most cases, administrators will remove this file from sites-enabled/ and
# leave it as reference inside of sites-available where it will continue to be
# updated by the nginx packaging team.
#
# This file will automatically load configuration files provided by other
# applications, such as Drupal or Wordpress. These applications will be made
# available underneath a path with that package name, such as /drupal8.
#
# Please see /usr/share/doc/nginx-doc/examples/ for more detailed examples.
##

# Default server configuration
#
server {
	listen 80 default_server;
	listen [::]:80 default_server;

	root /var/www/html;

	index index.html index.htm index.nginx-debian.html;

	server_name _;

	location / {
		try_files $uri $uri/ =404;
	}

	# pass PHP scripts to FastCGI server
	#
	#location ~ \.php$ {
	#	include snippets/fastcgi-php.conf;
	#
	#	# With php-fpm (or other unix sockets):
	#	fastcgi_pass unix:/run/php/php7.4-fpm.sock;
	#	# With php-cgi (or other tcp sockets):
	#	fastcgi_pass 127.0.0.1:9000;
	#}

}

```

在 `index index.html index.htm index.nginx-debian.html;` 中加入 `index.php`

将 `location ~\.php$` 那部分取消注释，并稍做修改：

> 参考 [服务器环境设置 - Typecho Docs](http://docs.typecho.org/servers)

```nginx
#pass PHP scripts to FastCGI server

location ~ .*\.php(\/.*)*$ {
    include snippets/fastcgi-php.conf;

    set $path_info "";
    set $real_script_name $fastcgi_script_name;
    if ($fastcgi_script_name ~ "^(.+?\.php)(/.+)$") {
        set $real_script_name $1;
        set $path_info $2;
    }
    fastcgi_param SCRIPT_FILENAME $document_root$real_script_name;
    fastcgi_param SCRIPT_NAME $real_script_name;
    fastcgi_param PATH_INFO $path_info;

    # With php-fpm (or other unix sockets):
    fastcgi_pass unix:/run/php/php8.1-fpm.sock;
    # With php-cgi (or other tcp sockets):
    # fastcgi_pass 127.0.0.1:9000;
}
```

保存，并用 `nginx -s reload` 重新加载设定。

访问服务器地址，进行安装吧~
