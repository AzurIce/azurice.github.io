配置文件以 `init.vim`（使用 *Vimscript* 编写）或 `init.lua`（以 *lua* 编写）形式存在。
Unix 下位于 `~/.config/nvim/` 目录下
Windows 下位于 `~/AppData/Local/nvim/` 目录下

## standard-path
config
- Unix：`~/.config/nvim`
- Windows：`~/AppData/Local/nvim`

data
- Unix：`~/.local/share/nvim`
- Windows：`~/AppData/Local/nvim-data`

## 



nvim 自动调整了 `package.path` 在 `runtimepath` 中的每个目录后添加 `/lua/?.lua` 和 `/lua/?/init.lua`。

