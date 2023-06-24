> 参考：[PostgreSQL: Documentation: 15: 41.2. Views and the Rule System](https://www.postgresql.org/docs/15/rules-views.html)

视图是从其他表中导出的表，是一个虚表。数据库只保存视图的定义，而不保存视图的数据（因为视图其实可以理解为对子查询的一个别名）。



而在 PostgreSQL 中的 **视图** 其实是使用 rule system 实现的，所以下面两个命令其实在本质上是一样的：

```postgresql
CREATE VIEW myview AS SELECT * FROM mytab;
```

```postgresql
CREATE TABLE myview (/*same column list as mytab*/);
CREATE RULE "_RETURN" AS ON SELECT TO myview DO INSTEAD
    SELECT * FROM mytab;
```

不过这会带来一些副作用，其中之一就是在 system catalog 中，一个 **视图** 的信息适合一个 **表** 完全一样的，所以对于解析器，它们之间没有任何区别，他们都是一个东西 —— 关系。

---

```postgresql
CREATE [ OR REPLACE ] [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW /*name*/ [ ( /*column_name*/ [, ...] ) ]
    [ WITH ( /*view_option_name*/ [= /*view_option_value*/] [, ... ] ) ]
    AS /*query*/
    [ WITH [ CASCADED | LOCAL ] CHECK OPTION ]
```

最基本的创建视图的命令如下：

```postgresql
CREATE VIEW /*name*/ AS /*query*/
```

如果天加上 `OR REPLACE` 则会在视图存在的时候将其替换。

其他的一些参数：

- `TEMP`：临时的视图会在当前 session 结束时自动 drop 掉

- `RECURSIVE`：创建一个递归的视图

  ```postgresql
  CREATE RECURSIVE VIEW [ schema . ] /*view_name*/ (/*column_names*/) AS SELECT ...;
  ```

  其实等价于

  ```postgresql
  CREATE VIEW [ schema . ] /*view_name*/ AS WITH RECURSIVE /*view_name*/ (/*column_names*/) AS (SELECT ...) SELECT /*column_names*/ FROM /*view_name*/;
  ```

- `CHEK OPTION`：控制自动更新的视图的行为

  开启后，`INSERT` 和 `UPDATE` 命令会被检查确保新的行满足视图定义的条件，能够在视图中显示。

  如果有视图依赖于视图的情况：

  - `LOCAL`：会仅检查当前视图
  - `CASCADE`（默认）：会递归地检查每个视图