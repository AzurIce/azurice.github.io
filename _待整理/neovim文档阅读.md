`nvim -d a b` 用 diff 显示区别

:diffsplit xxx

:piffpatch xxx.diff



`:edit` 打开文件

`:find` 打开 `path` 下的文件





## 有关窗口

窗口有关命令：`CTRL-W` + `xxx`

`+` `-` 高度加减

`>` `<` 宽度加减

`_` 设置高度为 `N`

`|` 设置宽度为 `N`（不提供N为最大）

`=` 所有窗口宽高相同（平分）

`H` `J` `K` `L` 移动窗口到最上下左右边

`P` 去上一个窗口

`R` 轮换位置（如果位于vsplit，则水平轮换位置）



`T` 移动到新的tab

### 分割窗口

`<C-w>s` 相当于 `:split` 上下分割，焦点在上方

`<C-w>v` 相当于 `:vsplit` 水平分割，焦点在左侧

可以在 `s` 和 `v` 前提供数字指定分割出来的新窗口大小

`<C-w>n` 新窗口（内部什么也没有）

## 有关标签页

[Nvim documentation: tabpage (neovim.io)](https://neovim.io/doc/user/tabpage.html)

`:[count]tabedit [++opt] [+cmd] {file}` 在新的标签页打开 `{file}`

会在 tabpage [count] 后打开新的标签页

`{count}gt` 或 `<C-kPageDown>`去标签页 {count}

`:tabn[ext] {count}` 或 `:{count}tabn[ext]`

`:[count]tab {cmd}` 执行 `{cmd}` 若打开新的窗口则在新的标签页打开。

### 切换标签页

| 命令                | 按键                                  | 作用           |
| ------------------- | ------------------------------------- | -------------- |
| `:tabn[ext]`        | `<C-kPageDown>` 或 `gt`               | 去下一个标签页 |
| `:{count}tabn[ext]` | `{count}<C-kPageDown>` 或 `{count}gt` | 去标签页count  |

### 移动标签页

| 命令            | 按键                    | 作用               |
| --------------- | ----------------------- | ------------------ |
| `:[N]tabm[ove]` | `<C-kPageDown>` 或 `gt` | 移动到标签页N或N后 |
|                 |                         |                    |



{count}\<C-kPageDown\>

{count}gt







变量：

v:count 最后一个 Normal mode 命令的计数Q
