> 参考：[PostgreSQL: Documentation: 15: 5.4. Constraints](https://www.postgresql.org/docs/15/ddl-constraints.html)

## 二、默认值

```postgresql
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric DEFAULT 9.99
);
```

默认值也可以被设置为一个表达式，表达式会在记录被插入时求值。

一个例子就是时间戳 `DEFAULT CURRENT_TIMESTAMP`，还有就是自增的序列号：

```postgresql
CREATE TABLE products (
    product_no integer DEFAULT nextval('products_product_no_seq'),
    ...
);
```

这个 `nextval()` 函数见 [PostgreSQL: Documentation: 15: 9.17. Sequence Manipulation Functions](https://www.postgresql.org/docs/15/functions-sequence.html)

## 四、Constraints 约束

### 一、Check 约束

check 约束是最通用的约束类型，规定某一列必须满足一个布尔表达式。

比如要求产品价格 `price` 必须为正数：

```postgresql
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric CHECK (price > 0)
);
```

可以使用 `CONSTRAINT` 关键字来指定约束的名字：

```postgresql
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric CONSTRAINT positive_price CHECK (price > 0)
);
```

上面的是 **对于某一列的约束**，还可以添加 **对整张表的约束**：

```{.postgresql hl_lines="6"}
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric CHECK (price > 0),
    discounted_price numeric CHECK (discounted_price > 0),
    CHECK (price > discounted_price)
);
```

额外的约束并没有紧接着写在某一列后面，而是单独出现在列的列表中。

对某一列的约束应当只引用当前列，而对整张表的约束可以引用多个列（虽然 PostgreSQL 并不强制，但是其他 SQL 可能会强制要求）。

下面是等价的一些其他写法：

```{.postgresql hl_lines="3 5"}
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric,
    CHECK (price > 0),
    discounted_price numeric,
    CHECK (discounted_price > 0),
    CHECK (price > discounted_price)
);
```

```{.postgresql hl_lines="6"}
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric CHECK (price > 0),
    discounted_price numeric,
    CHECK (discounted_price > 0 AND price > discounted_price)
);
```

同样对于表的约束也可以使用 `CONSTRAINT` 关键字指定约束名。

### 二、Not-Null 约束

要求某一列不能为空值。

```postgresql
CREATE TABLE products (
    product_no integer NOT NULL,
    name text NOT NULL,
    price numeric
);
```

其实等价于：

```postgresql
CREATE TABLE products (
    product_no integer CHECK (product_no is NOT NULL),
    name text CHECK (name is NOT NULL),
    price numeric
);
```

多个约束条件可以用空格隔开这么写：

```{.postgresql hl_lines="4"}
CREATE TABLE products (
    product_no integer NOT NULL,
    name text NOT NULL,
    price numeric NOT NULL CHECK (price > 0)
);
```

### 3. Unique 约束

要求某一列的值不重复。

```postgresql
CREATE TABLE products (
    product_no integer UNIQUE,
    name text UNIQUE,
    price numeric
);
```

写作对表的约束可以这么写：

```postgresql
CREATE TABLE products (
    product_no integer,
    name text,
    price numeric,
    UNIQUE (product_no, name)
);
```

---

要注意的是 NULL 被视为不相同，也就是说如果两条记录的 Unique 约束内的某一列都为 NULL，是不违反约束的。可以通过添加 `NULLS NOT DISTINCT` 来规定将 NULL 值视为相等：

```postgresql
CREATE TABLE products (
    product_no integer UNIQUE NULLS NOT DISTINCT,
    name text,
    price numeric
);
```

### 4. Primary Keys

一个主键唯一确定一条记录，也就是 UNIQUE 且 NOT NULL。

所以

```postgresql
CREATE TABLE products (
    product_no integer PRIMARY KEY,
    name text,
    price numeric
);
```

其实等价于

```postgresql
CREATE TABLE products (
    product_no integer UNIQUE NOT NULL,
    name text,
    price numeric
);
```

可以以一组列作为主键：

```postgresql
CREATE TABLE example (
    a integer,
    b integer,
    c integer,
    PRIMARY KEY (a, c)
);
```

### 5. Foreign Keys

外键必须在其他表中存在，即参照完整性。

比如对于这样一张产品表 products：

```postgresql
CREATE TABLE products (
    product_no integer PRIMARY KEY,
    name text,
    price numeric
);
```

其中的 `product_no` 可能要被其他表引用，比如订单表 orders：

```postgresql
CREATE TABLE orders (
    order_id integer PRIMARY KEY,
    product_no integer REFERENCES products (product_no),
    quantity integer
);
```

这时，如果新的记录的 `product_no` 在 products 表中不存在则会违反约束。

上面的命令也可以简写：

```postgresql
CREATE TABLE orders (
    order_id integer PRIMARY KEY,
    product_no integer REFERENCES products,
    quantity integer
);
```

因为对于另一个表的引用其实默认就是以那个表的主键为引用列的。

也可以引用多个列：

```postgresql
CREATE TABLE t1 (
  a integer PRIMARY KEY,
  b integer,
  c integer,
  FOREIGN KEY (b, c) REFERENCES other_table (c1, c2)
);
```

当然数量和类型必须对应。

---

有时候外键会是自己同一张表的主键：

```postgresql
CREATE TABLE tree (
    node_id integer PRIMARY KEY,
    parent_id integer REFERENCES tree,
    name text,
    ...
);
```

这叫做 **自引用外键**。

---

还会有一个问题，就是如果引用的外键在其他表中对应记录被删除呢？此时这个记录就不合法了。

这是有几个选择：

- 不允许删除作为外键被引用的记录 `ON DELETE RESTRICT`
- 将引用了外键的记录的记录也删除掉 `ON DELETE CASACDE`

```postgresql
CREATE TABLE products (
    product_no integer PRIMARY KEY,
    name text,
    price numeric
);

CREATE TABLE orders (
    order_id integer PRIMARY KEY,
    shipping_address text,
    ...
);

CREATE TABLE order_items (
    product_no integer REFERENCES products ON DELETE RESTRICT,
    order_id integer REFERENCES orders ON DELETE CASCADE,
    quantity integer,
    PRIMARY KEY (product_no, order_id)
);
```

如果 products 中的某条记录被引用，那么不允许删除 products 中的该条记录。

如果 orders 中的某条记录被删除，那么 order_items 引用了对应记录的键的记录就会被删除。

还有一些其他的：

如果什么都不写就是 `NO ACTION`，会抛出错误。

还有 `SET NULL` 和 `SET DEFAULT xxx`，顾名思义。

<font color=red>这块还有点复杂，先咕一下，后面用到了再详细整理。</font>

### 6. Exclusion 约束

咕
