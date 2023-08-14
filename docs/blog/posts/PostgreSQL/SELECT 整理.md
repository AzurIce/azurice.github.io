---
date: 2023-04-28
---

# SELECT 整理

> https://www.postgresql.org/docs/15/sql-select.html

`SELECT`可以从一个或多个表获取行。

```
[ WITH [ RECURSIVE ] with_query [, ...] ]
SELECT [ ALL | DISTINCT [ ON ( expression [, ...] ) ] ]
    [ * | expression [ [ AS ] output_name ] [, ...] ]
    [ FROM from_item [, ...] ]
    [ WHERE condition ]
    [ GROUP BY [ ALL | DISTINCT ] grouping_element [, ...] ]
    [ HAVING condition ]
    [ WINDOW window_name AS ( window_definition ) [, ...] ]
    [ { UNION | INTERSECT | EXCEPT } [ ALL | DISTINCT ] select ]
    [ ORDER BY expression [ ASC | DESC | USING operator ] [ NULLS { FIRST | LAST } ] [, ...] ]
    [ LIMIT { count | ALL } ]
    [ OFFSET start [ ROW | ROWS ] ]
    [ FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } { ONLY | WITH TIES } ]
    [ FOR { UPDATE | NO KEY UPDATE | SHARE | KEY SHARE } [ OF table_name [, ...] ] [ NOWAIT | SKIP LOCKED ] [...] ]
```


如果提供了 `WHERE` clause，所有的不满足要求的行都会被从输出中移除。
如果提供了 `GROUP BY` clause，或者调用了任何聚组函数，输出就会被分为一个个匹配一个或多个值的组，如果还提供了 `HAVING` clause，所有不满足要求的组都会被从输出中移除。
`SELECT DISTINCT` 会从结果中移除重复的行。
`SELECT DISTINCT ON ( expression [, ...] )` 会基于提供的表达式来删除


## `WITH` Clause

```
[ WITH [ RECURSIVE ] with_query [, ...] ]
```

使用 `WITH` clause 可以指定一个或多个子查询，可以在主查询中以名字来引用。

## `FROM` Clause

使用 `FROM` Clause 可以为 `SELECT` 指定一个或多个源表。
如果制定了多个源表，那么结果就是所有源表的 **笛卡尔积（交叉链接）**

```
[ FROM from_item [, ...] ]
```

其中 `from_item` 可以是：

```
where from_item can be one of:

    [ ONLY ] table_name [ * ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
                [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
    [ LATERAL ] ( select ) [ AS ] alias [ ( column_alias [, ...] ) ]
    with_query_name [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    [ LATERAL ] function_name ( [ argument [, ...] ] )
                [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    [ LATERAL ] function_name ( [ argument [, ...] ] ) [ AS ] alias ( column_definition [, ...] )
    [ LATERAL ] function_name ( [ argument [, ...] ] ) AS ( column_definition [, ...] )
    [ LATERAL ] ROWS FROM( function_name ( [ argument [, ...] ] ) [ AS ( column_definition [, ...] ) ] [, ...] )
                [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    from_item join_type from_item { ON join_condition | USING ( join_column [, ...] ) [ AS join_using_alias ] }
    from_item NATURAL join_type from_item
    from_item CROSS JOIN from_item
```


`table_name` 表或视图名，如果在其前面有 `ONLY`，那么只有这个表会被扫描，否则这个表和所有它级联的表都会被扫描。Optionally, * can be specified after the table name to explicitly indicate that descendant tables are included.

简化来看：

```
table_name [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
```

`alias` 可以为选择的 `table_name` 指定一个别名
