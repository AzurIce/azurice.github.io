用户表设计：

```postgresql
CREATE TABLE user (
    ID          integer     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username    varchar(10) NOT NULL CHECK (username <> '')
    nickname    varchar(10) NOT NULL CHECK (username <> '')
    password    varchar(20) NOT NULL CHECK (CHAR_LENGTH(password) BETWEEN 6 AND 20)
    group       integer     CHECK (group BETWEEN 0 AND 3)
)
```



用户权限：

0. <font color="blue">默认管理员</font>

1. 管理员：管理系统，最高权限。 

   管理员进行系统维护、权限分配和合同流程配置之类的权限。

   对业务操作只能查询不能对自己授予业务继续新增、修改、删除等操作。

   可以根据需要进行管理员权限的细化，如设计系统管理员专门进行系统权限的管理，设计合同管理员进行合同流程配置和跟踪等。 

2. 操作员：管理合同操作流程，并可以跟踪合同的不同阶段。

3. 新用户：**没有任何权限**，等待合同管理员分配权限。