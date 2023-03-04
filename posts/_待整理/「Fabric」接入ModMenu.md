---
title: 「Fabric」ModMenuAPI
date: 2022-01-09 22:20
categories: Java 
tags:
  - Fabric
---

# 「Fabric」接入ModMenu

## 〇、如何在开发中运行时使用其他mod
了解Gradle：[[../Gradle/笔记]]  
添加 `modRuntime` 依赖

```groovy
repositories {
    maven {
        url 'https://maven.terraformersmc.com/releases'
    }
}
```

```
dependencies {
	modRuntimeOnly "com.terraformersmc:modmenu:${project.modmenu_version}"
}
```

## 一、基本内容 JSON API
## 二、更进一步 Java API
修改 `fabric.mod.json` 文件中的 `entrypoionts` 字段，添加 `modmenu` 类别：
```json
"entrypoints": [
	"modmenu": [
      "com.azurice.azurmap.client.ModMenuImpl"
    ]
]
```
填入实现类的包名。
创建 `ModMenuImpl.java` 文件：

```java
package com.azurice.azurmap.client;

import com.terraformersmc.modmenu.api.ModMenuApi;

public class ModMenuImpl implements ModMenuApi {

}

```

### 2.1 Config界面
实现 `getModConfigScreenFactory` 方法：
```java
public class ModMenuImpl implements ModMenuApi {  
	@Override  
	public ConfigScreenFactory<?> getModConfigScreenFactory() {  
		return ModMenuApi.super.getModConfigScreenFactory();
	}  
}
```

读一读 Minecraft 自己的 KeyBindsScreen 是怎么写的：
继承图↓
![[Pasted image 20220110000452.png]]

