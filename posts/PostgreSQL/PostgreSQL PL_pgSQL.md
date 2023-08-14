---
date: 2023-04-14
---

PL/pgSQL 是一个用于 PostgreSQL 的可加载的过程性语言

使用 PL/pgSQL 书写的函数可以接受服务器支持的任何标量或数组数据作为参数，同时它也可以返回任何这些类型。

## 创建 PL/SQL 函数

通过执行 `CREATE FUNCTION` 来在服务器创建 PL/pgSQL 函数：

```postgresql
CREATE FUNCTION somefunc(integer, text) RETURNS integer
AS 'function body text'
LANGUAGE plpgsql;
```

函数体就是个简单的字符串字面值

// TODO: https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING

pg/pgSQL 是一个块结构的语言，一个完整的函数体必须是一个块，块可以通过如下方式定义：

```postgresql
[ <<label>> ]
[ DECLARE
    declarations ]
BEGIN
    statements
END [ label ];
```

在块中的每一个定义或语句都要以分号结尾

`label` 只有在你希望制指定使用某一个块用于一个 `EXIT` 语句的时候需要，或者标识出块中定义的变量。
如果在 END 后写了 `label` 那么就要和开始的  `label` 相匹配。

下面是一个详细一些的例子：
```postgresql
CREATE FUNCTION somefunc() RETURNS integer AS $$
<< outerblock >>
DECLARE
    quantity integer := 30;
BEGIN
    RAISE NOTICE 'Quantity here is %', quantity;  -- Prints 30
    quantity := 50;
    --
    -- Create a subblock
    --
    DECLARE
        quantity integer := 80;
    BEGIN
        RAISE NOTICE 'Quantity here is %', quantity;  -- Prints 80
        RAISE NOTICE 'Outer quantity here is %', outerblock.quantity;  -- Prints 50
    END;

    RAISE NOTICE 'Quantity here is %', quantity;  -- Prints 50

    RETURN quantity;
END;
$$ LANGUAGE plpgsql;
```

> There is actually a hidden “outer block” surrounding the body of any PL/pgSQL function. This block provides the declarations of the function's parameters (if any), as well as some special variables such as FOUND (see Section 43.5.5). The outer block is labeled with the function's name, meaning that parameters and special variables can be qualified with the function's name.

## 表达式

所有在 PL/pgSQL 语句中使用的表达式都会使用服务器的主 SQL 执行器处理。
比如如果你写了一个像下面这样的 PL/pgSQL 语句：

```postgresql
IF expression THEN ...
```

那么 PL/pgSQL 就会像下面这样进行一次查询来对表达式求值：

```postgresql
SELECT expression
```

## 基本语句

### 1. 赋值

```
variable { := | = } expression;
```

### 2. 执行 SQL 命令

一般地，任何不返回行的 SQL 命令都可以通过直接写在 PL/pgSQL 中的方式来执行：

```postgresql
CREATE TABLE mytable (id int primary key, data text);
INSERT INTO mytable VALUES (1,'one'), (2,'two');
```

而如果一条命令会返回行（比如 `SELECT` 或带有 `RETURNING` 的 `INSERT/UPDATE/DELETE`），有两种方式来执行：
- 如果命令只返回一个行或者你只关心输出的第一行，可以通过添加一个 `INTO` 子句来捕获输出
- 如果要处理所有的输出行，可以将命令作为 `FOR` 循环的数据源

#### 2.1 执行单行结果的命令

如下：

```postgresql
SELECT select_expressions INTO [STRICT] target FROM ...;
INSERT ... RETURNING expressions INTO [STRICT] target;
UPDATE ... RETURNING expressions INTO [STRICT] target;
DELETE ... RETURNING expressions INTO [STRICT] target;
```

其中 `target` 可以是一个记录变量、行变量、逗号分割的简单变量和记录/行字段。


