---
title: 「MC x Java」GUI
date: 2022-01-10 11:03
categories: Java
tags:
  - Java
  - MC
---

# 「MC x Java」GUI
## 一、一切的根基 —— Screen类
![[Pasted image 20220110111106.png]]
**Minecraft** 中一切GUI均继承自`Screen` 类，它是一个抽象类。
### 1.1 生命周期
`MinecraftClient` 类 中的 `setScreen()` 方法被用来设定当前打开的界面：

```java
public void setScreen(@Nullable Screen screen)
```

---

若当前有打开的 `Screen`，则调用该 `Screen` 的 `removed()` 方法。
若 `screen` 为 `null`：
- 不在世界中则打开 **标题界面TitleScreen**
- 玩家死亡则打开 **死亡界面DeathScreen**
设置 `MinecraftClient` 的 `currentScreen` 为 `screen`。
调用 `screen` 的 `init()` 方法。

每当 `MinecraftClient` 的 `tick()` 方法被调用时，均会调用 `currentScreen` 的 `tick()` 方法

---

那么我们可以总结出一个 `Screen` 的生命周期：
- `init()` 被使用 `setScreen` 打开时调用
- `tick()` 客户端 `tick()` 时调用
- `removed()` 被关闭时调用

---

### 2. 有关 `onClose()` 方法
默认的实现为调用 `client.setScreen(null)`
看一看在哪里被调用
![[Pasted image 20220110114411.png]]

`Screen` 中包含按键事件的响应函数 `KeyPresed()`，是对 `ParenrElement` 的实现：

```java
public boolean keyPressed(int keyCode, int scanCode, int modifiers)
```

默认的实现为：
如果按了 `ESC` 并且，`shouldCloseOnEsc()` 方法返回 `true`（默认实现就是），就调用 `onClose()` 方法。

可以得出总结：`onClose()` 方法负责实现对关闭当前界面方法的实现，只是为了方便控件操作、按键响应关闭界面。
要与生命周期区别开！！！

## 二、控件 —— 

## 二、游戏设置界面 —— GameOptionsScreen
