首先通过一个 Mixin注入 将 `clientTick` 方法注入到 客户端的 `tick()` 方法中

```java
@Mixin({minecraftClient.class})
/* loaded from: fabricmod_VoxelMap-1.10.15_for_1.17.0.jar:com/mamiyaotaru/voxelmap/fabricmod/mixins/APIMixinMinecraftClient.class */
public class APIMixinMinecraftClient {
    @Inject(method = {"tick()V"}, at = {@At("RETURN")})
    private void onTick(CallbackInfo ci) {
        FabricModVoxelMap.instance.clientTick((MinecraftClient) this);
    }
}
```

FabricVoxelMap

```java
public void clientTick(MinecraftClient client) {
    if (!this.initialized) {
        boolean OK = true;
        if (MinecraftClient.getInstance() == null || 
            client.getResourceManager() == null || 
            client.getTextureManager() == null) {
            OK = false;
        }
        if (OK) {
            lateInit();
        }
    }
    if (this.initialized) {
        this.master.onTick(client);
    }
}
```

VoxelMap

```java
public void onTick(MinecraftClient mc) {
        if (this.checkMOTD) {
            checkPermissionMessages(mc);
        }
        if ((GameVariableAccessShim.getWorld() != null && !GameVariableAccessShim.getWorld().equals(this.world)) || (this.world != null && !this.world.equals(GameVariableAccessShim.getWorld()))) {
            this.world = GameVariableAccessShim.getWorld();
            this.waypointManager.newWorld(this.world);
            this.persistentMap.newWorld(this.world);
            if (this.world != null) {
                MapUtils.reset();
                PacketByteBuf buffer = new class_2540(Unpooled.buffer());
                buffer.writeBytes(("worldinfo:world_id").getBytes(Charsets.UTF_8));
                mc.method_1562().method_2883(new class_2817(new class_2960("minecraft:register"), buffer));
                ByteBuf data1 = Unpooled.buffer(4);
                data1.writeInt(42);
                mc.field_1724.field_3944.method_2883(new class_2817(new class_2960("worldinfo:world_id"), new class_2540(data1)));
                ByteBuf data2 = Unpooled.buffer(4);
                data2.writeInt(43);
                new class_2817(new class_2960("journeymap:world_info"), new class_2540(data2));
                mc.field_1724.method_3117();
                Map<MinecraftProfileTexture.Type, MinecraftProfileTexture> skinMap = mc.method_1582().method_4654(mc.field_1724.method_7334());
                if (skinMap.containsKey(MinecraftProfileTexture.Type.SKIN)) {
                    mc.method_1582().method_4656(skinMap.get(MinecraftProfileTexture.Type.SKIN), MinecraftProfileTexture.Type.SKIN);
                }
                if (!this.worldName.equals(this.waypointManager.getCurrentWorldName())) {
                    this.worldName = this.waypointManager.getCurrentWorldName();
                    this.radarOptions.radarAllowed = true;
                    this.radarOptions.radarPlayersAllowed = this.radarOptions.radarAllowed;
                    this.radarOptions.radarMobsAllowed = this.radarOptions.radarAllowed;
                    this.mapOptions.cavesAllowed = true;
                    if (!mc.method_1496()) {
                        this.newServerTime = Long.valueOf(System.currentTimeMillis());
                        this.checkMOTD = true;
                    }
                }
                this.map.newWorld(this.world);
            }
        }
        TickCounter.onTick();
        this.persistentMap.onTick(mc);
    }
```











APIMixinInGameHud

```java
@Mixin({InG.class})
/* loaded from: fabricmod_VoxelMap-1.10.15_for_1.17.0.jar:com/mamiyaotaru/voxelmap/fabricmod/mixins/APIMixinInGameHud.class */
public class APIMixinInGameHud {
    @Inject(method = {"render(Lnet/minecraft/client/util/math/MatrixStack;F)V"}, at = {@At("RETURN")})
    private void onRenderGameOverlay(class_4587 matrixStack, float partialTicks, CallbackInfo ci) {
        FabricModVoxelMap.instance.renderOverlay(matrixStack);
    }
}
```

## GuiPersistentMap —— 地图页面

![image-20220221185235122](V:\_Posts\__Obisidan附件__\image-20220221185235122.png)

```java
this.passEvents = true;
this.oldNorth = this.mapOptions.oldNorth;
centerAt(this.options.mapX, this.options.mapZ);
this.f17mc.keyboard.method_1462(true);
if (getMinecraft().currentScreen == this) {
    this.closed = false;
}
this.screenTitle = I18nUtils.getString("worldmap.title", new Object[0]);
buildWorldName();
this.leftMouseButtonDown = false;
```

### 一、底部按钮部分

```java
this.sideMargin = 10;
this.buttonCount = 5;
this.buttonSeparation = 4;

// 计算按钮宽度
this.buttonWidth = ((this.width - (this.sideMargin * 2)) - (this.buttonSeparation * (this.buttonCount - 1))) / this.buttonCount;

// 添加 “导航点...” 按钮
addDrawableChild(new PopupGuiButton(this.sideMargin + (0 * (this.buttonWidth + this.buttonSeparation)), getHeight() - 28, this.buttonWidth, 20, new TranslatableText("options.minimap.waypoints"), buttonWidget_1 -> {
    getMinecraft().setScreen(new GuiWaypoints(this, this.master));
}, this));

// 添加 “导航点...” 按钮
this.multiworldButtonName = 
    new TranslatableText(getMinecraft().isConnectedToRealms() ? 
                         "menu.online" : "options.worldmap.multiworld");
this.multiworldButtonNameRed = 
    new TranslatableText(getMinecraft().isConnectedToRealms() ? 
                         "menu.online" : "options.worldmap.multiworld")
    .method_27692(class_124.field_1061);
if (!getMinecraft().isIntegratedServerRunning() &&
    !this.master.getWaypointManager().receivedAutoSubworldName()) {
    PopupGuiButton popupGuiButton = 
        new PopupGuiButton(this.sideMargin + (1 * (this.buttonWidth + this.buttonSeparation)),
                           getHeight() - 28, this.buttonWidth, 20, 
                           this.multiworldButtonName, 
                           buttonWidget_1 -> {
                               getMinecraft().setScreen(new GuiSubworldsSelect(this, this.master));
                           }, this);
    this.buttonMultiworld = popupGuiButton;
    addDrawableChild(popupGuiButton);
}

// 添加 “设置...” 按钮
addDrawableChild(new PopupGuiButton(this.sideMargin + (3 * (this.buttonWidth + this.buttonSeparation)), getHeight() - 28, this.buttonWidth, 20, new TranslatableText("menu.options"), null, this) { // from class: com.mamiyaotaru.voxelmap.persistent.GuiPersistentMap.1
    public void onPress() {
        GuiPersistentMap.this.getMinecraft().setScreen(new GuiMinimapOptions(GuiPersistentMap.this, GuiPersistentMap.this.master));
    }
});
// 添加 “玩成...” 按钮
addDrawableChild(new PopupGuiButton(this.sideMargin + (4 * (this.buttonWidth + this.buttonSeparation)), getHeight() - 28, this.buttonWidth, 20, new TranslatableText("gui.done"), null, this) { // from class: com.mamiyaotaru.voxelmap.persistent.GuiPersistentMap.2
    public void onPress() {
        GuiPersistentMap.this.getMinecraft().setScreen(GuiPersistentMap.this.parent);
    }
});
```

### 二、上方坐标文字部分

```java
this.coordinates = new TextFieldWidget(getFontRenderer(), this.sideMargin, 10, 140, 20, (class_2561) null);
```

### 三、乱七八糟

```java
this.passEvents = true;
this.oldNorth = this.mapOptions.oldNorth;
centerAt(this.options.mapX, this.options.mapZ);
this.f17mc.keyboard.method_1462(true);
if (getMinecraft().currentScreen == this) {
    this.closed = false;
}
this.screenTitle = I18nUtils.getString("worldmap.title", new Object[0]);
buildWorldName();
this.leftMouseButtonDown = false;

/* ... */

this.coordinates = new TextFieldWidget(getFontRenderer(), this.sideMargin, 10, 140, 20, (class_2561) null);
this.top = 32;
this.bottom = getHeight() - 32;
this.centerX = getWidth() / 2;
this.centerY = (this.bottom - this.top) / 2;
this.scScale = (float) this.f17mc.getWindow().getScaleFactor();
this.mapPixelsX = this.f17mc.getWindow().getFramebufferWidth();
this.mapPixelsY = this.f17mc.getWindow().getFramebufferHeight() - ((int) (64.0f * this.scScale));
this.lastStill = false;
this.timeAtLastTick = System.currentTimeMillis();
this.keyBindForward.setBoundKey(this.forwardCode);
this.keyBindLeft.setBoundKey(this.leftCode);
this.keyBindBack.setBoundKey(this.backCode);
this.keyBindRight.setBoundKey(this.rightCode);
this.keyBindSprint.setBoundKey(this.sprintCode);
this.f17mc.options.keyForward.setBoundKey(this.nullInput);
this.f17mc.options.field_1913.setBoundKey(this.nullInput);
this.f17mc.options.field_1881.setBoundKey(this.nullInput);
this.f17mc.options.field_1849.setBoundKey(this.nullInput);
this.f17mc.options.field_1867.setBoundKey(this.nullInput);
KeyBinding.updateKeysByCode();
```

