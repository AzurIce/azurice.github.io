## C++ RTTI（Run-Time Type Identification）

环境：

```
Ubuntu clang version 14.0.0-1ubuntu1.1
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
Found candidate GCC installation: /usr/bin/../lib/gcc/x86_64-linux-gnu/11
Selected GCC installation: /usr/bin/../lib/gcc/x86_64-linux-gnu/11
Candidate multilib: .;@m64
Selected multilib: .;@m64
```

### `typeid` 运算符

可以获取对应 **类型** / **表达式** 的 `std::type_info`：

```cpp
class type_info {
public:
    virtual ~type_info();
    constexpr bool operator==(const type_info& rhs) const noexcept;
    bool before(const type_info& rhs) const noexcept; // ordering
    size_t hash_code() const noexcept; // hash of type
    const char* name() const noexcept; // name of type

    type_info(const type_info&) = delete;            // cannot be copied
    type_info& operator=(const type_info&) = delete; // cannot be copied
};
```

> `operator!=` 在 c++20 中被移除（因为定义了 `==` 后会自动生成 `!=` 的默认实现）

### 例 1 | 非多态类（无虚函数）RTTI

```cpp
#include <iostream>

class Parent {};

class Children : public Parent {};

void test_typeid() {
    Children children;
    Parent* p_parent = &children;
    Parent& ref_parent = children;

    // 6Parent
    std::cout << typeid(Parent).name() << std::endl;
    // 8Children
    std::cout << typeid(Children).name() << std::endl;
    // 8Children
    std::cout << typeid(children).name() << std::endl;
    // P6Parent
    std::cout << typeid(p_parent).name() << std::endl;
    
    // 6Parent
    std::cout << typeid(*p_parent).name() << std::endl;
    // 6Parent
    std::cout << typeid(ref_parent).name() << std::endl;
}

int main() { test_typeid(); }
```

### 例 2 | 多态类（有虚函数）RTTI

```cpp
#include <iostream>

class Parent {
    virtual void foo() {}
};

class Children : public Parent {
    virtual void foo() {}
};

void test_typeid() {
    Children children;
    Parent* p_parent = &children;
    Parent& ref_parent = children;

    // 6Parent
    std::cout << typeid(Parent).name() << std::endl;
    // 8Children
    std::cout << typeid(Children).name() << std::endl;
    // 8Children
    std::cout << typeid(children).name() << std::endl;
    // P6Parent
    std::cout << typeid(p_parent).name() << std::endl;
    
    // !!! 8Children !!! 这里变为了子类类型
    std::cout << typeid(*p_parent).name() << std::endl;
    // !!! 8Children !!! 这里变为了子类类型
    std::cout << typeid(ref_parent).name() << std::endl;
}

int main() { test_typeid(); }
```

使用 `dynamic_cast` 进行 *downcast* 的原理也就是依靠 `std::type_info` 的信息进行转换。

### 虚函数表与 RTTI 信息

对于多态类，其 RTTI 信息存储在虚函数表中（对于 gcc/clang 存储于 `-1` 索引处）：

```cpp
// ...

void test_vtable_rtti() {
  /*
               +------------+
               | vtable[-1] | --> std::type_info
  +------+     +------------+
  | vptr | --> | vtable[0]  | --> vfunc
  +------+     +------------+
  */
  Children children;
  void *p = (void *)&children;
  std::cout << p << std::endl; // 0x7ffd2316dd18
  void ***vptr = (void ***)p;

  void **vtable = *vptr;
  std::cout << vtable << std::endl; // 0x55dc9ebb6d90
  
  std::type_info *type_info_ptr = (std::type_info *)vtable[-1];
  // 0x55dc9ebb6d48 8Children
  std::cout << type_info_ptr << " " << type_info_ptr->name() << std::endl;
}

int main() { test_vtable_rtti(); }
```

> 在 windows 上测试时，`type_info_ptr->name()` 似乎并不能正确调用（会 freeze），可能与不同平台 ABI 有关：
>
> ```
> clang version 19.1.5
> Target: x86_64-pc-windows-msvc
> Thread model: posix
> InstalledDir: C:\Users\xiaob\scoop\apps\llvm\19.1.5\bin
> ```

## 参考

- https://nirvana1997.github.io/RTTI%E7%9A%84%E5%8E%9F%E7%90%86/
- https://cplusplus.com/doc/tutorial/typecasting/
- https://zhuanlan.zhihu.com/p/267794224