---
date: 2023-04-09
---

Golang 标准库中的 `database/sql` 包提供了访问 SQL（或类 SQL）数据库的通用接口，需要与数据库驱动[^1]结合使用。

本文以 PostgreSQL 数据库为例，使用 [jackc/pgx: PostgreSQL driver and toolkit for Go (github.com)](https://github.com/jackc/pgx) 并假设已在本机安装了 PostgreSQL并监听本机的 5432 端口。



`database/sql`：[sql package - database/sql - Go Packages](https://pkg.go.dev/database/sql)

`pgx`：[pgx package - github.com/jackc/pgx/v5 - Go Packages](https://pkg.go.dev/github.com/jackc/pgx/v5)

## 一、连接数据库

[`Open`](https://pkg.go.dev/database/sql/#Open) 用于创建一个数据库 handle（根据驱动的不同也许只会验证参数并不会真的创建与数据库的连接）:

```go
db, err := sql.Open(driver, dataSourceName)
```

这里的两个参数都是 string 类型的：

- `driver`：指定使用的数据库驱动
- `dataSourceName`：指定了数据库连接信息，比如数据库名、验证信息等，也就是数据库 URL。

比如，使用 `pgx` 数据库驱动可以这么写：

```go
// urlExample := "postgres://username:password@localhost:5432/database_name"
db, err := sql.Open("pgx", os.Getenv("DATABASE_URL"))
if err != nil {
    fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
    os.Exit(1)
}
defer db.Close()
```

Open 函数会返回一个 *DB 类型的值，这个类型有很多方法，很多数据库的操作诸如查询、SQL语句执行等都会用到它。

> 一些数据库驱动库也会实现自己的相关方法，不过这可能会使得后续的一些操作可能会与其他 SQL 数据库不兼容：
>
> ```go
> // urlExample := "postgres://username:password@localhost:5432/database_name"
> conn, err := pgx.Connect(context.Background(), os.Getenv("DATABASE_URL"))
> if err != nil {
>     fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
>     os.Exit(1)
> }
> defer conn.Close(context.Background())
> ```
>
> Connect 函数会返回一个 Conn 类型的指针，其实可以发现这个类型与 DB 类型很像。
>
> 这里还使用了 context 库，具体见 [Golang 标准库之 context](../Golang 标准库 之 context)。

## 二、执行 SQL 语句

### 1. 使用 `Exec` 执行非查询语句（返回 Result）

DB 类型有这样一个方法用于执行任何 SQL 语句，但是 **不会返回任何行**：

```go
func (db *DB) Exec(query string, args ...any) (Result, error)
```

以下是 Result 类型的定义：

```go
type Result interface {
	// LastInsertId 返回数据库为一个命令生成的 ID
    // 一般在向包含 auto increment 列的表插入新行时会用到，
    // （不一定所有的数据库都支持，且不同的数据库的支持也不尽相同）
	LastInsertId() (int64, error)

	// RowsAffected 返回一次 update, insert, 或 delete 影响到的列的数量
    // （不一定所有的数据库都支持）
	RowsAffected() (int64, error)
}
```

例子：

```go
id := 47
result, err := db.Exec(ctx, "UPDATE balances SET balance = balance + 10 WHERE user_id = ?", id)
if err != nil {
    log.Fatal(err)
}
rows, err := result.RowsAffected()
if err != nil {
    log.Fatal(err)
}
if rows != 1 {
    log.Fatalf("expected to affect 1 row, affected %d", rows)
}
```

### 2. 使用 `Query` 执行查询命令（返回 Rows）

DB 类型有另一个方法 **可以返回行**（一般用于 `SELECT`）：

```go
func (db *DB) Query(query string, args ...any) (*Rows, error)
```

Rows 类型是查询的结果。它的指针从第一行之前开始，可以使用 `Next` 方法来移动到下一行：

`func (rs *Rows) Next() bool`

此外还有 `NextResultSet` 用于移动到下一个结果集：

`func (rs *Rows) NextResultSet() bool`

它还有一些其他方法：

- `func (rs *Rows) Close() error`：用于关闭 Rows 防止 `Next` 的枚举，在 `Next` 遍历完所有行后会自动关闭。
- `func (rs *Rows) Columns() ([]string, error)`：返回列名。
- `func (rs *Rows) ColumnTypes() ([]*ColumnType, error)`：返回列的类型，有关 ColumnType 先咕了，或者看 [sql package - database/sql - Go Packages](https://pkg.go.dev/database/sql#ColumnType)。
- `func (rs *Rows) Scan(dest ...any) error`：从当前行赋值所有列到 dest 指向位置（参数数量要与列数量相等）。
- `func (rs *Rows) Err() error`：返回在迭代过程中遇到的错误

一个多结果集查询的例子：

```go
age := 27
q := `
create temp table uid (id bigint); -- Create temp table for queries.
insert into uid
select id from users where age < ?; -- Populate temp table.

-- First result set.
select
users.id, name
from
users
join uid on users.id = uid.id
;

-- Second result set.
select 
ur.user, ur.role
from
user_roles as ur
join uid on uid.id = ur.user
;
`
rows, err := db.Query(q, age)
if err != nil {
    log.Fatal(err)
}
defer rows.Close()

for rows.Next() {
    var (
        id   int64
        name string
    )
    if err := rows.Scan(&id, &name); err != nil {
        log.Fatal(err)
    }
    log.Printf("id %d name is %s\n", id, name)
}
if !rows.NextResultSet() {
    log.Fatalf("expected more result sets: %v", rows.Err())
}
var roleMap = map[int64]string{
    1: "user",
    2: "admin",
    3: "gopher",
}
for rows.Next() {
    var (
        id   int64
        role int64
    )
    if err := rows.Scan(&id, &role); err != nil {
        log.Fatal(err)
    }
    log.Printf("id %d has role %s\n", id, roleMap[role])
}
if err := rows.Err(); err != nil {
    log.Fatal(err)
}
```

### 3. 使用 `QueryRow` 执行查询命令（返回 Row）

如果结果没有包含任何一行，就返回 ErrNoRows，否则就返回第一行并忽略其他行。

例子：

```go
id := 123
var username string
var created time.Time
err := db.QueryRowContext(ctx, "SELECT username, created_at FROM users WHERE id=?", id).Scan(&username, &created)
switch {
case err == sql.ErrNoRows:
    log.Printf("no user with id %d\n", id)
case err != nil:
    log.Fatalf("query error: %v\n", err)
default:
    log.Printf("username is %q, account created on %s\n", username, created)
}
```

---

更多的方法见 文档 [^2]

## 参考

[^1]:[https://golang.org/s/sqldrivers](https://golang.org/s/sqldrivers)

[^2]: [sql package - database/sql - Go Packages](https://pkg.go.dev/database/sql)

## 废文案

- 