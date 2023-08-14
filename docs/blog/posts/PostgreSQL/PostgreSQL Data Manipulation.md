---
date: 2023-03-29
---

# PostgreSQL Data Manipulation

> 参考：[PostgreSQL: Documentation: 15: Chapter 6. Data Manipulation](https://www.postgresql.org/docs/15/dml.html)

## 一、插入数据

以这张表为例：

```postgresql
CREATE TABLE products (
    product_no    integer,
    name          text,
    price         numeric
);
```

可以通过下面的命令来插入一条记录：

```postgresql
INSERT INTO products VALUES (1, 'Cheese', 9.99);
```

但是上面的写法要求顺序与表中列得顺序对应，也可以采取下面的写法，与表名一一对应：

```postgresql
INSERT INTO products (product_no, name, price) VALUES (1, 'Cheese', 9.99);
INSERT INTO products (name, price, product_no) VALUES ('Cheese', 9.99, 1);
```

如果某一列没有值（为空）那么可以将其省略：

```postgresql
INSERT INTO products (product_no, name) VALUES (1, 'Cheese');
INSERT INTO products VALUES (1, 'Cheese');
```

> 上面第二行是 PostgreSQL 的扩展写法，会从左到右依次为列赋值，剩余为空。

也可以显式地指定使用某一列使用默认值或全部使用默认值：

```postgresql
INSERT INTO products (product_no, name, price) VALUES (1, 'Cheese', DEFAULT);
INSERT INTO products DEFAULT VALUES;
```

---

可以用一条命令插入多条数据：

```postgresql
INSERT INTO products (product_no, name, price) VALUES
    (1, 'Cheese', 9.99),
    (2, 'Bread', 1.99),
    (3, 'Milk', 2.99);
```

还可以插入查询的结果：

```postgresql
INSERT INTO products (product_no, name, price)
  SELECT product_no, name, price FROM new_products
    WHERE release_date = 'today';
```

