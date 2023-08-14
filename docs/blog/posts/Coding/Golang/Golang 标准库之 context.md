---
date: 2023-04-08
---

## 一、引入

在使用 goroutine 时会出现这样一个问题：

```go
package main

import "time"

func main() {
	go f()
	for range time.Tick(time.Second) {
		println("main tick")
	}
}

func f() {
	defer func() {
		println("exit f")
	}()
	go ff()
	println("f")
}

func ff() {
	for range time.Tick(time.Second) {
		println("ff tick")
	}
}

```

如果运行上面这段代码，会发现，创建运行 `ff` 的 goroutine 的 `f` 在退出之后 `ff` 仍在运行。

有时候我们希望这些 goroutine 具有类似主 goroutine 与其他 goroutine 的“父子”关系，即“父” goroutine 退出时终止“子” goroutine。

但是 golang 中的 goroutine 并不这样，但是我们可以通过 context 来是实现它。

---

再举一个具体点的例子：

```go
func main()  {
    http.HandleFunc("/", SayHello) // 设置访问的路由
    log.Fatalln(http.ListenAndServe(":8080",nil))
}

func SayHello(writer http.ResponseWriter, request *http.Request)  {
    fmt.Println(&request)

    go func() {
        for range time.Tick(time.Second) {
            fmt.Println("Current request is in progress")
        }
    }()

    time.Sleep(2 * time.Second)
    writer.Write([]byte("Hi, New Request Comes"))
}

```

每一个 Http 请求都会创建一个 goroutine 用于运行 Handler 函数，在这个例子中的 Handler 函数包含了一段使用 goroutine 运行一个无限循环的例子，这其实很常用（比如创建一个对当前 Handler 的监听器），我们会希望在 Handler 退出时，这个 goroutine 也被终止。但是实际上这段代码像先前的例子一样，这段循环会一直运行。

而 `request` 其中包含了一个方法让我们得以判断这个 Handler 是否处理完成：

```go
go func() {
    for range time.Tick(time.Second) {
        select {
        case <- request.Context().Done():
            fmt.Println("request is outgoing")
            return
        default:
            fmt.Println("Current request is in progress")
        }
    }
}()
```

而 context 便可以像这个例子一样解决我们遇到的问题。

## 二、什么是 context

官方对于 Context 的介绍是：在截止时间（deadline）、取消信号（cancellation signal）以及其他 request-scoped 的值

在 Golang 标准库的 context 包中，`Context` 是这样定义的：

```go
type Context interface {
	Deadline() (deadline time.Time, ok bool)
	Done() <-chan struct{}
	Err() error
	Value(key any) any
}
```

- `Deadline()`：返回当工作完成（context 被取消）的截止时间，当没有 deadline 的时候 `ok` 为 `false`。
- `Done()`：返回一个当工作完成（context 被取消）时关闭的 channel，当 context 永远不会被取消的时候返回 `nil`。
- `Err()`：如果 `Done` 还没有被关闭，则返回 `nil`；如果 `Done` 关闭了，则返回一个非 `nil` 的 `error` 解释关闭的原因。
- `Value(key any)`：返回通过 `key` 获取的与此 context 关联的键值对中的值。

---

有两种最基本的 context，他们都会返回 `emptyCtx`，即 `Deadline()` 直接返回而 `Done()`、`Err()`、`Value(key any)` 返回 `nil` 的 `Context`：

- `func Background() Context`

  常用于主函数、初始化、测试，以及作为请求的顶级 Context。

- `func TODO() Context`

  用于在不确定用何种 Context 或目前不可用时使用。

---

此外 Golang 在 context 库中提供了很多方便创建 `Context` 的工具函数：

- `WithCancel`

  有时我们希望通过关闭 `Done` channel 来向使用此 context 的 goroutine 传递 context 取消的信息（就像上面的例子），此时便可以使用此函数：

  ```go
  func WithCancel(parent Context) (ctx Context, cancel CancelFunc)
  ```

  这个函数会通过复制一个 `parent` context 并将其 `Done` 赋为一个新的 channel 的方式创建一个新的 context 并返回，

  同时还会返回一个用于关闭 `Done` 的函数 `cancel`。

  ---

  一个例子：

  ```go
  // gen 会在另一个 goroutine 中生成整数并传入返回的 channel。
  // 调用者应当在不再使用 gen 的时候立刻取消 context。
  gen := func(ctx context.Context) <-chan int {
      dst := make(chan int)
      n := 1
      go func() {
          for {
              select {
              case <-ctx.Done():
                  return // returning not to leak the goroutine
              case dst <- n:
                  n++
              }
          }
      }()
      return dst
  }
  
  ctx, cancel := context.WithCancel(context.Background())
  defer cancel() // 在下面对生成的整数的使用结束时取消 context
  
  for n := range gen(ctx) {
      fmt.Println(n)
      if n == 5 {
          break
      }
  }
  ```

- `WithCancelCause`

  与 `WithCancel` 很像，不过其返回的是一个 `CancelCauseFunc`，接受一个 `error` 类型的参数。

  使用一个非 `nil` 的 `error`（也就是所谓的 cause）会将它记录在 `ctx` 中，可以使用 `Cause(ctx)` 来获取它（在 context 被取消时会得到 `nil`）。

  也可以传入 `nil` 来使用 `ctx` 原本的 `Error`。

  ```go
  func WithCancelCause(parent Context) (ctx Context, cancel CancelCauseFunc)
  ```

  ---

  一个例子：

  ```go
  ctx, cancel := context.WithCancelCause(parent)
  cancel(myError)
  ctx.Err() // returns context.Canceled
  context.Cause(ctx) // returns myError
  ```

- `WithDeadline`

  会通过复制 `parent` 并使其 Deadline 返回 no later than d。如果 parent 的 Deadline 已经比 d 早了，就不变。

  在 deadline 到达时，将会关闭 Done channel。

  ---

  一个例子：

  ```go
  func main() {
  	d := time.Now().Add(1 * time.Millisecond)
  	ctx, cancel := context.WithDeadline(context.Background(), d)
  
  	// 即便 ctx 会由于 deadline 被取消，依旧使用 defer 将其取消是一个好习惯
  	defer cancel()
  
  	select {
  	case <-time.After(1 * time.Second):
  		fmt.Println("overslept")
  	case <-ctx.Done():
  		fmt.Println(ctx.Err())
  	}
  
  }
  ```

  输出：`context deadline exceeded`

- `WithTimeout`

  ```go
  func WithTimeout(parent Context, timeout time.Duration) (Context, CancelFunc)
  ```

  就是 `WithDeadline(parent, time.Now().Add(timeout))`。

- `WithValue`

  `func WithValue(parent Context, key, val any) Context`：其源码是用参数创建一个 `valueCtx` 并返回（要求 parent 非空，key 非空 key 可以比较）

  用于传递值。Use context Values only for request-scoped data that transits processes and APIs, not for passing optional parameters to functions.

  `valueCtx`：

  递归定义，以此可以保存多对 key val，其 Value 函数基于 key 是否相等的比较返回 val。

  ```go
  type valueCtx struct {
  	Context
  	key, val any
  }
  ```

## 参考

[GO语言基础进阶教程：Go语言的协程——Goroutine - 知乎 (zhihu.com)](https://zhuanlan.zhihu.com/p/77205289)

[深入理解Golang中的Context包_golang context_沈子恒的博客-CSDN博客](https://blog.csdn.net/shenziheng1/article/details/113924703)、

