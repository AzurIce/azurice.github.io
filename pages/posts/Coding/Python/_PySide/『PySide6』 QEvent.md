**Qt** 的主事件循环 `exec()` 从事件队列获取原生的窗口系统事件，翻译为 **QEvents** 并发送给 **QObject**。

**QObject** 以被调用 `event()` 函数的方式来接收事件，此函数可以被子类重新实现来自定义事件处理或添加事件类型。

默认，事件会分发给像是 `timerEvent()`，`mouseMoveEvnet()` 等的 event handler。