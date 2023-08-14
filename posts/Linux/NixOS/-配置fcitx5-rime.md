---
date: 2023-04-28
---

## Fcitx5 theme customization

> https://fcitx-im.org/wiki/Theme_Customization/zh-cn

配置文件位于`~/.config/fcitx5/classicui.conf`：

- `Vertical Candidate List`： 竖列选词列表
- `PerScreenDPI`：各屏幕使用独立DPI
- `WheelForPaging`：使用鼠标滚轮来前后翻页
- `UseInputMethodLanguageToDisplayText`：由输入法语言来渲染文字

```conf
# Vertical Candidate List
Vertical Candidate List=False
# Use Per Screen DPI
PerScreenDPI=True
# Use mouse wheel to go to prev or next page
WheelForPaging=True
# Use input method language to display text
UseInputMethodLangaugeToDisplayText=True

# Font
Font="JetBrainsMono Nerd Font Medium 14"
# Menu Font
MenuFont="JetBrainsMono Nerd Font Medium 14"
# Tray Font
TrayFont="JetBrainsMono Nerd Font Medium 14"
# Tray Label Outline Color
TrayOutlineColor=#000000
# Tray Label Text Color
TrayTextColor=#ffffff
# Prefer Text Icon
PreferTextIcon=False
# Show Layout Name In Icon
ShowLayoutNameInIcon=True
# Theme
Theme=Catppuccin
# Force font DPI on Wayland
ForceWaylandDPI=0
```

### Theme

> https://fcitx-im.org/wiki/Fcitx_5_Theme

`~/.local/share/fcitx5/themes/<ThemeName>/theme.conf`

```
''
# vim: ft=dosini
[Metadata]
Name=Catppuccin-light
Version=0.2
Author=justTOBBI and Isabelinc
Description=Catppuccin Latte Color Theme (Dark)
ScaleWithDPI=True

[InputPanel]
# 字体
Font=JetBrainsMono Nerd Font Medium 14
# 非选中候选字颜色
#Blue
NormalColor=#1E66F5
# 选中候选字颜色
#Peach
HighlightCandidateColor=#FE640B
# 高亮前景颜色(输入字符颜色)
#Peach
HighlightColor=#FE640B
# 输入字符背景颜色
# Black3/surface0
HighlightBackgroundColor=#CCD0DA
#
Spacing=3

[InputPanel/TextMargin]
# 候选字对左边距
Left=10
# 候选字对右边距
Right=10
# 候选字向上边距
Top=6
# 候选字向下边距
Bottom=6

[InputPanel/Background]
#Black3/surface0
Color=#CCD0DA
#Black3/surface0
BorderColor=#CCD0DA
BorderWidth=2

[InputPanel/Background/Margin]
Left=2
Right=2
Top=2
Bottom=2

[InputPanel/Highlight]
#Black3/surface0
Color=#CCD0DA

[InputPanel/Highlight/Margin]
# 高亮区域左边距
Left=10
# 高亮区域右边距
Right=10
# 高亮区域上边距
Top=7
# 高亮区域下边距
Bottom=7

[Menu]
Font=Sans 10
#White/Text
NormalColor=#4C4F69
#HighlightColor=#4c566a
Spacing=3

[Menu/Background]
#Black3/surface0
Color=#CCD0DA

[Menu/Background/Margin]
Left=2
Right=2
Top=2
Bottom=2

[Menu/ContentMargin]
Left=2
Right=2
Top=2
Bottom=2

[Menu/Highlight]
#Pink
Color=#EA76CB 

[Menu/Highlight/Margin]
Left=10
Right=10
Top=5
Bottom=5

[Menu/Separator]
#Black2/base
Color=#EFF1F5

[Menu/CheckBox]
Image="${./radio.png}"

[Menu/SubMenu]  
Image="${./arrow.png}"

[Menu/TextMargin]
Left=5
Right=5
Top=5
Bottom=5
''
```