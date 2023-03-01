---
title: "Golang 接口"
date: 2023-02-14T10:24:39+08:00
draft: false
categories:
  - 笔记
tags:
  - Golang
---

在 go1.18 以前，接口的定义是 **方法的集合**。
而在 go1.18 及之后，其定义就变更为了 **类型的集合**。

## 一、基础接口 与 嵌套接口（go1.18 前）

这部分其实就是 go1.18 前接口的样子，每一个接口规定了一个方法的集合，只要一个类型的方法集是接口类型所规定的方法集的超集，就视其实现了方法。

### 1.1 基础接口

比如对于这一个接口，其规定了一个包含 `Read`、`Write`、`Close` 的方法集，那么只要一个类型的方法集中包含这三个方法就视其实现了这个接口：

```go
// A simple File interface.
type File interface {
	Read([]byte) (int, error)
	Write([]byte) (int, error)
	Close() error
}
```

### 1.2 嵌套接口

嵌套接口，其实就是对「要求」进行交集的操作，比如对于下面的 `ReadWriter` 接口，它要求实现它的类型即满足 `Reader` 又 满足 `Writer`，也就是说当一个类型的方法集中包含 `Read`、`Write`、`Close` 这三个方法才说它实现了这个接口。

```go
type Reader interface {
	Read(p []byte) (n int, err error)
	Close() error
}

type Writer interface {
	Write(p []byte) (n int, err error)
	Close() error
}

// ReadWriter's methods are Read, Write, and Close.
type ReadWriter interface {
	Reader  // includes methods of Reader in ReadWriter's method set
	Writer  // includes methods of Writer in ReadWriter's method set
}
```

这里提到的「要求」其实就是所谓的「类型集」的概念，一个要求可以确定出对应的一系列满足要求的类型，在 go1.18 及之后，接口的功能被进行了拓展，可以将这个「要求」不局限于方法的要求，进一步可以规定类型的要求。
或者换句话说，原本的方法集其实也是规定了一个个类型集，在 go1.18 及之后使其更加通用了。

## 二、通用接口（go1.18 及之后）

首先，上面的 `ReadWriter` 其实就是 `Reader` 所表示的类型集和 `Writer` 所表示的类型集的交集，当一个接口中包含多个非空类型集的时候，它所表示的类型集就是他们的交集：

```go
// ReadWriter's methods are Read, Write, and Close.
type ReadWriter interface {
	Reader  // includes methods of Reader in ReadWriter's method set
	Writer  // includes methods of Writer in ReadWriter's method set
}
```

相应的也可以表示并集（其实这里就是泛型的类型约束），比如这样一个接口，就表示是 `float32` 或 `float64` 的类型的集合：

```go
type Float interface {
	float32 | float64
}
```

## 三、再看一看官方文档的定义

go1.18 前：

> **Interface types**
> An interface type specifies a ***method set*** called its _interface_. A variable of interface type can store a value of any type with a method set that is any superset of the interface. Such a type is said to _implement the interface_.The value of an uninitialized variable of interface type is `nil`.

一个接口类型定义一个 **方法集**。
一个属于某一个接口类型的变量能够存储任何类型的值，而它的方法集要求是接口的方法集的超集。

其实就是讲一个类型的方法集包含接口中的所有方法时，就称其实现了接口。

go1.18 及之后：

> **Interface types**
> An interface type defines a ***type set***.
> A variable of interface type can store a value of any type that is in the type set of the interface. Such a type is said to [implement the interface](https://go.dev/ref/spec#Implementing_an_interface).The value of an uninitialized variable of interface type is `nil`.

一个接口类型定义一个 **类型集**。
一个属于某一个接口类型的变量能够存储属于这个接口所规定的类型集中的任意类型的值。

第二句话可能有点绕，不过绕绕也能绕出来，这里为了绕一绕所以翻译得原汁原味一些，其实表达的就是实现接口的意思，对应着下面的实现接口的定义：

> **Implementing an interface**
> A type `T` implements an interface `I` if
> 
> -   `T` is not an interface and is an element of the type set of `I`; or
> -   `T` is an interface and the type set of `T` is a subset of the type set of `I`.
> 
> A value of type `T` implements an interface if `T` implements the interface.

对于一个类型 `T` 来说，有两种情况我们称其实现了接口 `I`：
- `T` 本身不是个接口类型，但是其属于 `I` 所定义的类型集。
- `T` 是一个接口，且 `T` 所规定的类型集是 `I` 所定义的类型集的子集。

而对于一个 `T` 类型的值来说，它的类型 `T` 实现了接口，就称它实现了接口。