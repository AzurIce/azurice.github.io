---
date: 2023-03-29
---

Indexes are a common way to enhance database performance. An index allows the database server to find and retrieve specific rows much faster than it could do without an index. But indexes also add overhead to the database system as a whole, so they should be used sensibly.

## 一、引入

比如对于这样一个表和查询：

```postgresql
CREATE TABLE test1 (
    id integer,
    content varchar
);
```

```postgresql
SELECT content FROM test1 WHERE id = xxx;
```

正常情况下数据库会一行一行地扫描整张表才能得到结果，显然这是个很低效的方法。

于是我们可以通过创建索引，来借助数据结构优化搜索的效率。

```postgresql
CREATE INDEX test1_id_index ON test1 (id);
```

> 在表 test1 中基于 id 列建立索引，名为 test1_id_index。

在索引被创建后，系统会随着数据的插入、更新、删除更新索引。

对一张很大的表创建索引可能会花费很长时间，默认情况下 PostgreSQL 允许在创建索引的同时支持读操作，但所有写操作都会阻塞直至索引创建完成，不过这一点可以修改，见 [Building Indexes Concurrently](https://www.postgresql.org/docs/15/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY)。

## 二、索引的类型

咕

## 三、多列索引

索引也可以基于多列来创建，比如对于下面的表和查询：

```postgresql
CREATE TABLE test2 (
  major int,
  minor int,
  name varchar
);
```

```postgresql
SELECT name FROM test2 WHERE major = constant AND minor = constant;
```

或许创建一个这样的索引是个不错的选择：

```postgresql
CREATE INDEX test2_mm_idx ON test2 (major, minor);
```

## 四、索引与 ORDER BY

索引也可以调整排序顺序，默认为升序。

使用 `DESC` 标明降序，使用 `NULLS FIRST` 和 `NULLS LAST` 来更改空值的位置。

```POSTGRESQL
CREATE INDEX test2_info_nulls_low ON test2 (info NULLS FIRST);
CREATE INDEX test3_desc_index ON test3 (id DESC NULLS LAST);
```

