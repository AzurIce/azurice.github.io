

## Migrator

```go
// Migrator m struct
type Migrator struct {
	Config
}

// Config schema config
type Config struct {
	CreateIndexAfterCreateTable bool
	DB                          *gorm.DB
	gorm.Dialector
}
```

###  工具函数

#### 1. CurrentDatabase

```go
func (m Migrator) CurrentDatabase() (name string) {
	m.DB.Raw("SELECT DATABASE()").Row().Scan(&name)
	return
}
```

对于 PostgreSql 应为 `current_database()`

#### 2. HasTable

```go
// HasTable returns table exists or not for value, value could be a struct or string
func (m Migrator) HasTable(value interface{}) bool
```

基于如下 SQL 语句：

```sql
SELECT count(*) FROM information_schema.tables WHERE table_schema = ? AND table_name = ? AND table_type = ?
```

三个参数分别为 currentDatabase, stmt.Table, "BASE TABLE"

[PostgreSQL: Documentation: 15: Chapter 37. The Information Schema](https://www.postgresql.org/docs/15/information-schema.html)

[PostgreSQL: Documentation: 15: 37.54. tables](https://www.postgresql.org/docs/15/infoschema-tables.html)

- `table_schema`：Name of the schema that contains the table
- `table_name`：Name of the table
- `table_type`：Type of the table: 
  - `BASE TABLE` for a persistent base table (the normal table type)
  - `VIEW` for a view
  - `FOREIGN` for a foreign table
  - `LOCAL TEMPORARY` for a temporary table

#### 3. CreateTable

#### 4. ColumnTypes

```go
// ColumnTypes return columnTypes []gorm.ColumnType and execErr error
func (m Migrator) ColumnTypes(value interface{}) ([]gorm.ColumnType, error)
```

```go
// ColumnType column type interface
type ColumnType interface {
	Name() string
	DatabaseTypeName() string                 // varchar
	ColumnType() (columnType string, ok bool) // varchar(64)
	PrimaryKey() (isPrimaryKey bool, ok bool)
	AutoIncrement() (isAutoIncrement bool, ok bool)
	Length() (length int64, ok bool)
	DecimalSize() (precision int64, scale int64, ok bool)
	Nullable() (nullable bool, ok bool)
	Unique() (unique bool, ok bool)
	ScanType() reflect.Type
	Comment() (value string, ok bool)
	DefaultValue() (value string, ok bool)
}
```



### AutoMigrate

[](https://github.com/go-gorm/gorm/blob/f3874339efd829d9841ad8fb6b50d7c2059153d2/migrator/migrator.go#L103)

```go
func (m Migrator) AutoMigrate(values ...interface{}) error
```

首先，根据约束条件对 `values` 进行重新排序：`m.ReorderModels(values, true)`

然后遍历 `values`：

- 如果不存在该表则创建该表，出现错误则返回
- 如果存在则获取该表的列，并遍历 `value` 中的每一列进行比较
  - 如果存在对应列，则自动迁移（类型、约束等处理）
  - 如果不存在，则添加该列