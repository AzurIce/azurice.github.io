# 「Fabric」1. 从GUI开始

## 一、概述

**MinecraftClient类** 描述着一个逻辑上的Minecraft客户端，响应着渲染、声音播放、控制输入，同时管理着与逻辑上的服务器的连接（可以是远程服务器或 内置的服务器）。

在 `MinecraftClient.java` 中有一个可以返回当前 **MinecraftClient实例** 的静态方法 `getInstance()`：

```java
public static MinecraftClient getInstance() {
    return instance;
}
```

> `instance` 属性在构造函数中被初始化为 `this`

**Screen类** 描述着显示出来的一个界面。

**MinecraftClient** 中有这样一个属性描述着当前打开的 **Screen**，并提供了一个 set方法 来修改它。

```java
@Nullable
public Screen currentScreen;
```

```java
public void setScreen(@Nullable Screen screen)
```

如果提供的参数 *screen* 为 `null`：

- 如果客户端不在游戏里就会打开标题界面。

- 如果玩家死了，就会打开死亡界面。

## 二、挖掘挖掘这个传说中的 Screen

诸如 **TitleScreen**，**SelectWorldScreen**，**MultiplayerWarningScreen** 等类都有一个共同的基类 **Screen**。

一个 **Screen** 对象有以下重要的属性：

| 属性名            | 类型                             | 解释 |
| ----------------- | -------------------------------- | ---- |
| `client`          | `protected MinecraftClient`      |      |
| `width`，`height` | `public int`                     | 宽高 |
| `title`           | `protected final Text`           | 标题 |
| `children`        | `private final List<Element>`    |      |
| `selectables`     | `private final List<Selectable>` |      |
| `selected`        | `private Selectable`             |      |

有一些重要方法，通过对其重载可改变 **Screen** 的行为：

```java
// 这些看一眼应该就明白干什么的啦
public boolean shouldCloseOnEsc() { return true; }
public boolean shouldPause() { return true; }
public void onClose() { this.client.setScreen(null); }
```

```java
protected void init() {} // 当一个界面该被初始化时被调用，opened或resized

public void tick() {}

public void removed() {}
```

```java
public void renderBackground(MatrixStack matrices) {
    this.renderBackground(matrices, 0);
}
```

......

在Minecraft客户端中渲染的主入口是 `GameRenderer.render(float, long, boolean)`，客户端的 `currentScreen` 的 `render()` 方法就会在此处被调用，此方法又会在客户端的 `render()` 方法中被调用，而客户端的 `render()` 方法则被写在游戏的主循环中。

**Screen** 中的`render()` 是对 **Drawable接口** 的实现：

```java
public interface Drawable {
    public void render(MatrixStack var1, int var2, int var3, float var4);
}
```

这个 **MatrixStack** 又为何物？

### MatrixStack

一个栈，存矩阵，用来描述变换。每一个 **Entry** 都包含位置矩阵和法线矩阵，用栈结构来描述变换有独特的好处，例如在栈顶的矩阵基础上添加一个变换只需在这基础上压入一个新的矩阵，而弹出这个矩阵就可以回到变换之前的状态。一个 MaxtrixStack 在被创建的初始时刻包含一个单位矩阵。



我们来看一下最简单的 **MultiplayerWarningScreen类**：

![image-20220220164615212](V:\_Posts\__Obisidan附件__\image-20220220164615212.png)

在 `init()` 方法中初始化了一系列的控件：

```java
@Override
    protected void init() {
        super.init();
        this.lines = MultilineText.create(this.textRenderer, (StringVisitable)MESSAGE, this.width - 50);
        int i = (this.lines.count() + 1) * this.textRenderer.fontHeight * 2;
        this.addDrawableChild(new ButtonWidget(this.width / 2 - 155, 100 + i, 150, 20, ScreenTexts.PROCEED, button -> {
            if (this.checkbox.isChecked()) {
                this.client.options.skipMultiplayerWarning = true;
                this.client.options.write();
            }
            this.client.setScreen(new MultiplayerScreen(this.parent));
        }));
        this.addDrawableChild(new ButtonWidget(this.width / 2 - 155 + 160, 100 + i, 150, 20, ScreenTexts.BACK, button -> this.client.setScreen(this.parent)));
        this.checkbox = new CheckboxWidget(this.width / 2 - 155 + 80, 76 + i, 150, 20, CHECK_MESSAGE, false);
        this.addDrawableChild(this.checkbox);
    }
```

再在 `render()` 方法中命令渲染：

```java
@Override
    public void render(MatrixStack matrices, int mouseX, int mouseY, float delta) {
        this.renderBackgroundTexture(0);
        MultiplayerWarningScreen.drawTextWithShadow(matrices, this.textRenderer, HEADER, 25, 30, 0xFFFFFF);
        this.lines.drawWithShadow(matrices, 25, 70, this.textRenderer.fontHeight * 2, 0xFFFFFF);
        super.render(matrices, mouseX, mouseY, delta);
    }
```













不同的内容由不同的模块渲染：

| 渲染内容           | 模块                          |
| ------------------ | ----------------------------- |
| World              | `WorldRenderer`               |
| Blocks and Fluids  | `BlockRenderManager`          |
| Entities           | `EntityRenderDispatcher`      |
| Block entities     | `BlockEntityRenderDispatcher` |
| Items              | `ItemRenderer`                |
| Items held in hand | `HeldItemRenderer`            |
| Text               | `TextRenderer`                |
| Game hud           | `InGameHud`                   |






一些重要的属性：

| 属性                               | 解释                                                       |
| ---------------------------------- | ---------------------------------------------------------- |
| `public ClientWorld world;`        | 描述客户端在浏览的世界。（在游戏中非 `null`）              |
| `public ClientPlayerEntity player` | 描述当前客户端的玩家。（在游戏中非 `null`）                |