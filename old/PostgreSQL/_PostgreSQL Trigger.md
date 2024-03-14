---
date: 2023-05-31
categories:
  - School/PostgreSQL
---

# PostgreSQL Trigger

Trigger 其实就是一个数据库在特定操作后自动执行的一种函数。

它可以被连接到表和视图。

在表上，**触发器** 可以被定义为在任何 `INSERT`，`UPDATE`，`DELETE` 操作 **之前** 或 **之后** 执行，且可以分为 **对每条语句执行一次** 还是 **对每一个被修改的行执行一次**。

对于 `UPDATE` 操作，还可以指定 **特定的行被更新后** 执行。



在 **视图** 上，**触发器** 可以被定义为 **代替** 任何 `INSERT`, `UPDATE`, `DELETE` 操作执行。

触发器可以分为 **语句级** 的和 **行级** 的：

- 语句级

  在 `BEFORE` 触发器中无法访问语句产生的改动

  在 `AFTER` 触发器中可以访问语句产生的所有改动

  

`BEFORE` 触发器无法访问语句所产生的改动（因为语句还未执行），而 `AFTER` 触发器可以访问语句所产生的所有改动。

## 语句

```postgresql
CREATE [ OR REPLACE ] [ CONSTRAINT ] TRIGGER /*name*/ { BEFORE | AFTER | INSTEAD OF } { /*event*/ [ OR ... ] }
    ON /*table_name*/
    [ FOR [ EACH ] { ROW | STATEMENT } ]
    [ WHEN ( /*condition*/ ) ]
    EXECUTE { FUNCTION | PROCEDURE } /*function_name*/ ( /*arguments*/ )
```

其中 `event` 可以是 `INSERT`，`UPDATE [ OF /*column_name*/ [, ...] ]`，`DELETE`。



## 数据变更触发器

data change trigger 是一个满足以下条件的函数：

- 无参数

- 返回类型为 `trigger`

在这样的触发器中有一些特殊的变量可以访问：

- `NEW`

  类型为 `RECORD`，在 **行级** 触发器中。

  其值为 `INSERT/UPDATE` 操作的 **新** 的行。

- `OLD`

  类型为 `RECORD`，在 **行级** 触发器中。

  其值为 `INSERT/UPDATE` 操作的 **旧** 的行。

- 