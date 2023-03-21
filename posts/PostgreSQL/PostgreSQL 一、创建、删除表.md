你可以通过指定表名和所有列名及其类型来创建一个新表，下面创建一个 weather 表：

```postgresql
CREATE TABLE weather (
    city       varchar(80),
    temp_lo    int,         -- low temperature
    temp_hi    int,         -- high temperature
    prcp       real,        -- precipitation
    date       date
);
```

> `psql` 会将 `;` 视为一个完整语句的结束。

> 在 SQL 命令中可以自由的使用空白（空格、制表符、换行），`--` 开头的为注释。

PostgreSQL 支持标准 SQL 类型 `int`、`smallint`、`real`、`double precision`、`char(N)`、`varchar(N)`、`date`、`time`、`timestamp`、`interval`，以及一些方便的工具类型和几何类型，同时也支持用户添加类型，详细的见数据类型一篇。

如果要删除 weather表：gh

```postgresql
DROP TABLE weather
```

## 参考

[PostgreSQL: Documentation: 15: 2.3. Creating a New Table](https://www.postgresql.org/docs/current/tutorial-table.html)