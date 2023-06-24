nginx 的默认配置文件 `nginx.conf` 位于 `/usr/local/nginx/conf`，`/etc/nginx` 豁 `/usr/local/etc/nginx`

[Beginner’s Guide (nginx.org)](http://nginx.org/en/docs/beginners_guide.html)

在 nginx 运行后，可以使用 `nginx -s xxx` 来控制它：

- `stop` 快速关闭
- `quit` 优雅地关闭
- `reload` 重新加载配置文件
- `reopen` 重新打开log文件

## 一、配置文件的结构

nginx由受配置文件中的指令控制的模块组成，指令分为简单指令和块指令。

每条指令由名称 + 空格隔开的参数组成，简单指令由分号结尾，块指令由大括号包裹

