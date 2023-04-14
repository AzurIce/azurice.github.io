## 一、词法结构

> [PostgreSQL: Documentation: 15: 4.1. Lexical Structure](https://www.postgresql.org/docs/15/sql-syntax-lexical.html)

关键字列表：[PostgreSQL: Documentation: 15: Appendix C. SQL Key Words](https://www.postgresql.org/docs/15/sql-keywords-appendix.html)

### 常量

#### 1. 字符串常量

A string constant in SQL is an arbitrary sequence of characters bounded by single quotes (`'`)

可以使用 C-风格 的转义：

| Backslash Escape Sequence                         | Interpretation                                   |
| ------------------------------------------------- | ------------------------------------------------ |
| `\b`                                              | backspace                                        |
| `\f`                                              | form feed                                        |
| `\n`                                              | newline                                          |
| `\r`                                              | carriage return                                  |
| `\t`                                              | tab                                              |
| `\*`o`*`, `\*`oo`*`, `\*`ooo`*` (*`o`* = 0–7)     | octal byte value                                 |
| `\x*`h`*`, `\x*`hh`*` (*`h`* = 0–9, A–F)          | hexadecimal byte value                           |
| `\u*`xxxx`*`, `\U*`xxxxxxxx`*` (*`x`* = 0–9, A–F) | 16 or 32-bit hexadecimal Unicode character value |

### 注释

单行：

```sql
-- This is a standard SQL comment
```

多行：

```sql
/* multiline comment
 * with nesting: /* nested block comment */
 */
```



## 参考

[PostgreSQL: Documentation: 15: 4.1. Lexical Structure](https://www.postgresql.org/docs/15/sql-syntax-lexical.html)