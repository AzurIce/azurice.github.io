#set document(title: "Unreal C++ 基类")

#title()

= UObject 与 UClass

在 Unreal 中，有一个极为基础的基类：UObject，它提供了垃圾回收、反射、序列化、生命周期等一系列的最基础的支持。

在 Python 中，有“一切皆对象”的概念，所有对象都继承自一个基类 `object`，同时，有一个特殊的 `type` 类用于表示类型，通过 `type(obj)` 或 `obj.__class__` 可以获取到它。

而在 Unreal 中，也有类似的概念，所有对象都继承自一个基类 `UObject`，同时，有一个特殊的 `UClass` 类用于表示类型。