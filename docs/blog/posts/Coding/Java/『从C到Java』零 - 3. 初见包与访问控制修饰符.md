---
title: 『从C到Java』零 - 3. 初见包与访问控制修饰符
date: 2022-02-20
---
T18:12:00+08:00
## 一、包

> **包** 是一种为了更好地组织类与代码而产生的机制。

还记得我们用 **IntelliJ IDEA** 创建的项目初始的样子么？

在 **IDEA** 的 **Project 视图** 下，`src/` 目录下有一个看起来像是名为 `com.azurice` 的文件夹，而在其中的 `Main.java` 开头也有一句 `package com.azurice;`。
实际上，`com.azurice` 并不是 **一个** 文件夹，而是 `com/azurice/` 这样 **两个套着的** 文件夹，这个 `com.azurice` 就是一个包。

通过 **包**，你可以将相同功能或同一模块的类放在一个包；你可以在两个包中写两个名字相同的类，而这两个名字相同的类可以由包名区分开，进行导入与使用。

### 1.1 定义包

**包名** 是与 **路径** 一一对应的。

定义一个包的方法：

```java
package 包名
```

例如 `Main.java` 中的那句 `package com.azurice` 它规定这个文件属于 `com.azurice` 包，而它就在 `com/azruice/` 目录下，这个包被称作 **Base Package 基包**，一般其他的包都创建于这个包之下，来其名称来自于创建项目时的设定。

### 1.2 引入包

引入一个包的方法：

```java
import 包名.类名 // 引入包中的一个类
import 包名.*   // 引入包中的所有类 
```

java 中有一个 `java.util` 包，其中包含了很多实用工具，其中就包含一个方便日期操作的 `Date` **类**，我们可以借助它来编写一个打印当前日期的程序。

```java
package com.azurice

import java.util.Date; // 引入 java.util包 中的 Date类

public class HelloWorld {
    public static void main(String[] args) {
        System.out.println(new Date());
    }
}
```

在 `new Date()` 创建对象时，会以当前日期对其属性进行初始化。

### 1.3 我们来写一个包吧

比如创建一个 `com.azurice.util` **包**（创建文件夹）：

在这个包中创建一个 `TimeUtil.java` 文件。

在其中实现了一个 `TimeUtil` **类** 并实现了一个用来打印当前时间的方法 **printTime()**：

```java
package com.azurice.util;

import java.util.Date;

public class TimeUtil {
    void printTime() {
        System.out.println(new Date());
    }
}

```

然后我们就可以在 **主类** 中使用它了：

```java
package com.azurice;

import com.azurice.util.TimeUtil;

public class Main {
    public static void main(String[] args) {
        TimeUtil timeUtil = new TimeUtil();
        timeUtil.printTime();
    }
}

```

什么？报错了？

这里就要引入 **修饰符** 的概念了。

## 二、访问控制修饰符

Java中，可以使用 **访问控制修饰符** 来保护对类、变量、方法和构造方法的访问。Java 支持 4 种不同的访问权限。

- **default** (即默认，什么也不写）: 在同一包内可见，不使用任何修饰符。使用对象：类、接口（以后会讲）、变量、方法。
- **private** : 在同一类内可见。使用对象：变量、方法。 **注意：不能修饰类（外部类）**
- **public** : 对所有类可见。使用对象：类、接口、变量、方法
- **protected** : 对同一包内的类和所有子类可见。使用对象：变量、方法。 **注意：不能修饰类（外部类）**。


|   修饰符    | 当前类中 | 同一包内 | 其他包 |
| :---------: | :------: | :------: | :----: |
|  `public`   |    Y     |    Y     |   Y    |
| `protected` |    Y     |    Y     |   N    |
|  `default`  |    Y     |    Y     |   N    |
|  `private`  |    Y     |    N     |   N    |

### 2.1 `public`

我们在 `com.azurice` 包中能够使用 `com.azurice.util` 包中的类，就是因为这个类是 `public` 修饰的，因为 `printTime()` 函数没有加修饰符，所以其实它只能在同一包以及当前类中访问。我们为它加上 `public` 修饰的，顾名思义“公共的”。

- 一个文件中只能有一个 `public` 类，且其名字要与文件名相同

```java
package com.azurice.util;

import java.util.Date;

public class TimeUtil {
    public void printTime() {
        System.out.println(new Date());
    }
}

```

可以发现，报错消失了。

运行得到输出：

```
Wed Dec 22 22:46:56 CST 2021
```

### 2.2 `private`

如果加上 `private` 那么在就只有当前类中能够访问。

### 2.3 `protected`

这个，以后再讲~

## 三、`static` 修饰方法以及属性

**static** 意为 **静态**，何为 **静态**？就是不涉及 `this`（对象自身）。
一个 `static` 修饰的方法被称作 **静态方法**他被允许在对象没有被创建的情况下被调用。

我们刚刚写的函数其实并没有涉及到 TimeUtil 类型对象自身，所以我们可以把它写成静态方法：

```java
package com.azurice.util;

import java.util.Date;

public class TimeUtil {
    public static void printTime() {
        System.out.println(new Date());
    }
}
```

这样在主方法中可以不创建对象的情况下，直接使用 `类名.方法名()` 对其进行调用：

```java
package com.azurice;

import com.azurice.util.TimeUtil;

public class Main {
    public static void main(String[] args) {
        TimeUtil.printTime();
    }
}
```

同理，`static` 也可以修饰属性

---

差不多，有一定的理解就行啦，只是初步的形成认识，之后还要整体从头详细地讲一遍。