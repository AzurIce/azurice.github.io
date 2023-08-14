---
date: 2023-05-01
---

文档注释（Doc Comments）是一些出现在顶层的包、常量、函数、类型、变量的定义前的文档。

原则上来讲，所有导出的名称都应该有文档注释（就是首字母大写的东西）

这些注释可以被 [go/doc](https://go.dev/pkg/go/doc) 和 [go/doc/comment](https://go.dev/pkg/go/doc/comment) 包从源代码中提取出来形成文档，你在 pkg.go.dev 中的每一个包中看到的文档内容其实都来自于源代码中的注释。

## 包的注释

每一个包都应该有一段 **包的注释** 用来介绍这个包的作用并提供一些相关信息。下面是一个例子

```go
// Package path implements utility routines for manipulating slash-separated
// paths.
//
// The path package should only be used for paths separated by forward
// slashes, such as the paths in URLs. This package does not deal with
// Windows paths with drive letters or backslashes; to manipulate
// operating system paths, use the [path/filepath] package.
package path
```

其中 `[path/filepath]` 会在文档中创建一个到 `filepath` 文件的链接。

Go 的文档注释使用完整的句子。对于包的注释来说，意味着第一个句子应当以 `Package` 开始。



如果某一个包由多个文件组成，那么包的注释应该只存在于其中一个源文件中。如果多个文件都有包的注释，那么他们最终会被合并。

## 命令的注释

（暂略

## 类型的注释

一个 **类型的注释** 应该解释这个类型表示或提供的示例是什么。

下面是几个示例：

```go
package zip

// A Reader serves content from a ZIP archive.
type Reader struct {
    ...
}
```

```go
package regexp

// Regexp is the representation of a compiled regular expression.
// A Regexp is safe for concurrent use by multiple goroutines,
// except for configuration methods, such as Longest.
type Regexp struct {
    ...
}
```

对于一个有导出属性（首字母大写）的结构体，所有的导出属性要么应该在文档注释中说明，要么应该在每一个属性的注释中说明。

比如这个文档注释就说明了属性的作用：

```go
package io

// A LimitedReader reads from R but limits the amount of
// data returned to just N bytes. Each call to Read
// updates N to reflect the new amount remaining.
// Read returns EOF when N <= 0.
type LimitedReader struct {
    R   Reader // underlying reader
    N   int64  // max bytes remaining
}
```

而下面这个就是讲这些说明移到了属性的注释中：

```go
package comment

// A Printer is a doc comment printer.
// The fields in the struct can be filled in before calling
// any of the printing methods
// in order to customize the details of the printing process.
type Printer struct {
    // HeadingLevel is the nesting level used for
    // HTML and Markdown headings.
    // If HeadingLevel is zero, it defaults to level 3,
    // meaning to use <h3> and ###.
    HeadingLevel int
    ...
}
```

## 函数的注释

一个函数的注释应该解释清楚函数的返回值或者函数被调用后会有什么作用以及其副作用。

命名了的参数或者返回值可以通过直接卸载注释中来引用（不用转义

下面是两个例子：

```go
package strconv

// Quote returns a double-quoted Go string literal representing s.
// The returned string uses Go escape sequences (\t, \n, \xFF, \u0100)
// for control characters and non-printable characters as defined by IsPrint.
func Quote(s string) string {
    ...
}
```

```go
package os

// Exit causes the current program to exit with the given status code.
// Conventionally, code zero indicates success, non-zero an error.
// The program terminates immediately; deferred functions are not run.
//
// For portability, the status code should be in the range [0, 125].
func Exit(code int) {
    ...
}
```

## 常量的注释

可以使用一个文档注释来介绍一整组常量：

```go
package scanner // import "text/scanner"

// The result of Scan is one of these tokens or a Unicode character.
const (
    EOF = -(iota + 1)
    Ident
    Int
    Float
    Char
    ...
)
```

当然有时候常量组并不需要文档注释，而是卸载每个常量后面：

```go
package unicode // import "unicode"

const (
    MaxRune         = '\U0010FFFF' // maximum valid Unicode code point.
    ReplacementChar = '\uFFFD'     // represents invalid code points.
    MaxASCII        = '\u007F'     // maximum ASCII value.
    MaxLatin1       = '\u00FF'     // maximum Latin-1 value.
)
```

没有分组的常量往往需要一个完整的文档注释（以变量名为开头）

```go
package unicode

// Version is the Unicode edition from which the tables are derived.
const Version = "13.0.0"
```

## 变量的注释

同常量。

## 其他的略了

参考：[Go Doc Comments - The Go Programming Language](https://go.dev/doc/comment)