---
date: 2023-03-28
---

# Debian服务器配置Postgresql远程访问

以华为云 HECS 云耀云服务器 Debian 11.1.0 64-bit 为例。

首先进行安装：

```shell
apt-get install postgresql
```

在安装完成后默认用户名为 `postgres`，密码为空。

但是目前并不能直接通过 `psql -U postgres` 来访问数据库，因为 PostgreSQL 在本地连接时的默认认证方式为 peer，会从操作系统内核中获取当前用户名作为允许连接的用户名进行认证。

这可以通过修改 `/etc/postgresql/13/main/pg_hba.conf` 来改变：

```diff
#
# Database administrative login by Unix domain socket
-local   all             postgres                                peer
+local   all             postgres                                trust

# TYPE  DATABASE        USER            ADDRESS                 METHOD

# "local" is for Unix domain socket connections only
local   all             all                                     peer
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
# Allow replication connections from localhost, by a user with the
# replication privilege.
local   replication     all                                     peer
host    replication     all             127.0.0.1/32            md5
host    replication     all             ::1/128                 md5

+host    all             all             0.0.0.0/0               trust
```

顺便在最后添加了一个允许所有IP访问。

然后修改 `/etc/postgresql/13/main/postgresql.conf`

```diff
-#listen_addresses = 'localhost'          # what IP address(es) to listen on;
+listen_addresses = '*'          # what IP address(es) to listen on;
```

之后可以使用 `systemctl restart postgresql` 来重启服务。

可以使用 `systemctl enable postgresql` 启用开机启动服务。

现在就可以在其他的机器上使用 `psql -h xxx.xxx.xxx.xxx -U postgres` 来访问。

> 注：这里的配置只用于学习用途，不保证生产环境下的安全问题。