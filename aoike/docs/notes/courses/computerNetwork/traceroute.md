# traceroute

在类 Unix 系统中（Linux、MacOS），有 `traceroute` 命令，而在 Windows 中有 `tracert` 命令，他们都可以用于显示 IP 网络中的可能的路径并测量延迟。

## 原理

[IP 协议](./Internet Protocol.md) 一篇中讲到过有关 TTL 的内容。

数据包每经过一跳（经过一个路由器），其中的 TTL 就会减小 1，到达 0 时路由器就会直接丢弃这个包。

同时，路由器其实还会返回一个 ICMP Time Exceeded 信息，也因此我们得以获取到中间的路由的地址及相关信息（因为被 IP 协议包裹）。

`traceroute` 的原理就是，依次向目标地址发送 TTL 为 1、2、3、... 的包，在到达途中的第 1、2、3、... 跳时 TTL 降为 1，也就会返回 ICMP Time Exceeded 信息，以此实现追踪路由的效果。

## tracert 与 traceroute 区别

TODO

## 参考

[^1]: https://en.wikipedia.org/wiki/Traceroute
