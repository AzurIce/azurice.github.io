# 『Neovim』options
## 基础
- `se[t]` 显示出所有不同于默认值的 *options*。
- `se[t] all` 显示出所有的 *options*。

`all` 代表所有 *options*
在 *option* 后加 `?` 来显示出其值，
在 *option* 后加 `&` 来重置为默认值。
## 修改
### 1. 值为Boolean型的 *options*
| 命令                                     | 说明           |
| ---------------------------------------- | -------------- |
| `se[t] {option}`                         | 设置为 `true`  |
| `se[t] no{option}`                       | 设置为 `false` |
| `se[t] inv{option}` 或 `se[t] {option}!`   | 将值取反      |
|                                        |             |
### 2. 值为String或Number型的 *options*
`se[t] {option}={value}` （也可以用`:`，注意不要在 `=` 和 *value* 之间留空格）
> `+=` `-=` `^=` 分别代表 加上、减去、乘上。

*value* 中的 ` `，`\`，`|` 都需要用 `\` 转义。
## 选项
[Nvim documentation: quickref (neovim.io)](https://neovim.io/doc/user/quickref.html#Q_op)
