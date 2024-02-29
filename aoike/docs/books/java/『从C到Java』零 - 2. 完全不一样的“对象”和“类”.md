---
title: 『从C到Java』零 - 2. 完全不一样的“对象”和“类”
date: 2022-01-13
categories:
  - Coding/Java
---

# 『从C到Java』零 - 2. 完全不一样的“对象”和“类”

T23:13:00+08:00
**对象 Object**，有自己的属性，自己还能够做出一些行为。

**类 Class**，是用来描述对象的样子的。

可以说 **Class** 很像结构体，而 **Object** 很像一个结构体变量，区别就是，类和对象其中可以有函数，在这里被称作 **方法 Method**。

这一篇，用以初步理解 **对象** 和 **类** 的概念。

## 一、写个像结构体的类

### 1.1 声明一个 Good 类

```java
package com.azurice;

// 声明一个 Good 类
class Good {
    int id;
    int stock;
    float price;
}

public class Main {
    public static void main(String[] args) {
        System.out.println("HelloWorld!");
    }
}
```

是不是很像结构体。

### 1.2 声明一个 Good 类的对象

```java
package com.azurice;

// 声明一个 Good 类
class Good {
    int id;
    int stock;
    float price;
}

public class Main {
    public static void main(String[] args) {
        // 声明了一个 Good 类的变量
        Good g;
  
        System.out.println(g); // 编译器报错，变量未初始化
    }
}
```

这里声明出来的 g 并没有关联到一个具体的对象身上。

如果在这个时候访问这个对象，编译器会产生错误不允许编译，因为这个变量并没有初始化（对象并没有实例化）。

### 1.3 实例化对象

引入一个语句：`new` 来创建一个具体的对象（实例化对象）。

```java
package com.azurice;

// 声明一个 Good 类
class Good {
    int id;
    int stock;
    float price;
}

public class Main {
    public static void main(String[] args) {
        // 声明了一个 Good 类的对象
        Good g;
  
        g = new Good();
  
        System.out.println(g);
    }
}
```

得到输出：

```
com.azurice.Good@4eec7777
```

`@` 前是对象所属的类，`@` 后是地址。

看看它的值：

```java
System.out.println(g.id);
System.out.println(g.stock);
System.out.println(g.price);
```

得到输出：

```
0
0
0.0
```

你可以像对结构体成员操作一样来对这个对象的属性进行操作：

```java
g.id = 1;
g.stock = 7
g.price = 9.96f;
System.out.println(g.id);
System.out.println(g.stock);
System.out.println(g.price);
```

得到输出：

```
1
7
9.96
```

### 1.4 整理 & 与C语言对比

**Java语言**：

```java
package com.azurice;

// 声明一个 Good 类
class Good {
    int id;
    int stock;
    float price;
}

public class Main {
    public static void main(String[] args) {
        // 声明了一个 Good 类的对象
        Good g;
  
        g = new Good();
  
        g.id = 1;
        g.stock = 7;
        g.price = 9.96f;
  
        System.out.println(g.id);
        System.out.println(g.stock);
        System.out.println(g.price);
    }
}
```

**C语言**

```c
#include <stdio.h>
#include <stdlib.h>

// 声明一个 Good 结构体
struct Good {
    int id;
    int stock;
    float price;
}；

int main() {
    // 声明了一个 Good 结构体的指针
    struct Good *g;

    g = (Good *)malloc(sizeof(struct Good));

    g->id = 1;
    g->stock = 7
    g->price = 9.96;

    printf("%d\n", g->id);
    printf("%d\n", g->stock);
    printf("%f\n", g->price);
    return 0;
}
```

## 二、给类写点不像结构体的东西

### 2.1 方法

刚刚提到，类之中可以包含函数，叫做 **方法**。

程序开始运行的 "Main 函数"，在 "class Main"（**主类**）中，因此我们称其为 "**Main 方法**"。

那么我们在类里写一个打印信息的函数试试：

```java
struct Good {
    int id;
    int stock;
    float price;
  
    void printInfo() {
        System.out.println("id: " + id);
        System.out.println("stock: " + g.stock);
        System.out.println("price: " + price);
    }
}；
```

在主方法中可以调用：

```java
public static void main(String[] args) {
    // 声明了一个 Good 类的对象
    Good g;

    g = new Good();

    g.id = 1;
    g.stock = 7;
    g.price = 9.96f;

    g.printInfo(); // 调用 g 的 printInfo 方法
}
```

得到输出：

```
id: 1
stock: 7
price: 9.96
```

### 2.2 另一个方法

当然，方法也可以有参数，有返回值，还可以修改自身的属性：

```java
struct Good {
    int id;
    int stock;
    float price;
  
    void printInfo() {
        System.out.println("id: " + id);
        System.out.println("stock: " + stock);
        System.out.println("price: " + price);
    }
  
    // 成功卖出返回 true，否则返回 false
    boolean sell(int num) {
        if (num > stock) {
            return false;
        } else {
            stock -= num;
            return true;
        }
    }
}；
```

主方法：

```java
public static void main(String[] args) {
    Good g;

    g = new Good();

    g.id = 1;
    g.stock = 7;
    g.price = 9.96f;

    g.printInfo();
    if (!g.sell(8)) {
    	System.out.println("Don't have enough stock");
    } else {
    	System.out.println("Sell success");
    }
    g.printInfo();
    if (!g.sell(2)) {
    	System.out.println("Don't have enough stock");
    } else {
    	System.out.println("Sell success");
    }
    g.printInfo();
}
```

得到输出：

```
id: 1
stock: 7
price: 9.96
Don't have enough stock
id: 1
stock: 7
price: 9.96
Sell success
id: 1
stock: 5
price: 9.96
```

### 2.3 更特殊的东西 —— 构造函数

相信注意到了，实例化对象的语句中使用的是类名加()，很像是在调用函数：

```java
g = new Good();
```

其实确实是这样，这个函数叫做构造函数，默认的构造函数参数为空，而我们可以为类亲自写一个构造函数：

```java
struct Good {
    int id;
    int stock;
    float price;
  
    Good() {
  
    }
  
    // ...
}；
```

构造函数就是一个名字与类名相同的函数，参数随意，其作用就是”构造”，比如可以传进去 `id`，`stock`，`price` 直接为此对象完成初始化。

按照局部优先，如果参数名与属性名相同的话，用 `id` 访问到的其实是参数的 `id` 而非这个对象内部的属性 `id` 。

所以需要让参数名改一改才行：

```java
Good(int _id, int _stock, float _price) {
	id = _id;
    stock = _stock;
    price = _price;
}
```

或者有一个新的东西叫做 `this` 它代表的是当前的对象：

```java
Good(int id, int stock, float price) {
	this.id = id;
    this.stock = stock;
    this.price = price;
}
```

都可以实现在实例化的时候就为对象初始化属性值的功能。

主方法：

```java
public static void main(String[] args) {
    Good g;

    g = new Good(1, 7, 9.96f);

    g.printInfo();
}
```

得到输出：

```
id: 1
stock: 7
price: 9.96
```

## 三、所谓的 “面向对象” 到底是什么

现在大概已经对 **类** 和 **对象** 有了初步的理解。

我们一直在说 **C语言** 是面向过程的，而 **Java语言** 是面向对象的，那这两个词到底是什么意思呢？

考虑一个图书管理系统，用 **C语言** 来写的话，是考虑一个个的功能，去实现一个个的函数，再把这些函数串起来，像是你学会了做一件件事情的方法，然后去使用这一个个方法达成目标；而用 **Java语言** 来写的话，是一个个的类，每一个类有自己的属性和方法，像是你教会了一个个小人他们的职责，然后指挥一个个小人去做事情。

例如上面的购买商品的例子：

**面向过程** ，算法 + 数据结构，是考虑到 **商品购买**，实现购买商品的函数。当整个项目体量很大的时候，用这种思考方式要考虑的内容实在太多。

**面向对象** ，是考虑 **商品本身**，商品本身的属性以及它所能有的行为（例如：被购买，数量减少），还有商品与其他类型的对象之间的关系。就算当整个项目体量很大，再具体实现某一个类的某一个方法的时候，需要考虑的内容要没有那么复杂，同时这种思维方式也更接近人一些。当你将项目中的对象们都完善完成，你会发现，整个项目十分清晰。

之后还要接触到 **Java类** 的 **继承**，**多态** 等一系列性质，对 **面向对象** 的理解也会逐步加深。

---

以上。
