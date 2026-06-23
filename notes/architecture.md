## 功能扩展架构设计

- 入口：`ruyi build <file>`

- `ruyi` 检测 `<file>` 的类型

  类型包括：
  - `.k`, KCL 文件

    行为：二进制调用 `kcl run <file>`

  - `.ruyi`

    行为：通过 `ruyi` 内部模块解析 `<file>`, 来进行操作

### `.ruyi` 格式

简短的文本文档，设计为导入外部模块时使用。

被 build 会把 build 过后的 PackageStatic JSON 数据附加到末尾。

比如：

```
$ cat example.ruyi
import crate anyhow:0.0.1
```

```
$ ruyi build example.ruyi && cat example.ruyi
import crate anyhow:0.0.1
| {
|   PackageStatic
| }
```

#### 作用

通过 KCL 插件导入，拆建负责解析文件位置。

### 信息存放

**预期：**

| 类型 \ 位置      | 仓库（工作区） | 本地缓存 | 不存（仅 Debug） | 其它        |
| ---------------- | -------------- | -------- | ---------------- | ----------- |
| KCL              | Yes            |          |                  |             |
| Ruyi             | Yes            |          |                  |             |
| JSON (From KCL)  |                |          | Yes              |             |
| JSON (From Ruyi) |                |          |                  | 附加回 Ruyi |
| RPM Spec         |                |          | Yes              |             |

**开发前期（现在）：**

| 类型 \ 位置      | 仓库（工作区）  | 本地缓存 | 不存（仅 Debug） | 其它        |
| ---------------- | --------------- | -------- | ---------------- | ----------- |
| KCL              | Yes（独立仓库） |          |                  |             |
| Ruyi             | Yes（独立仓库） |          |                  |             |
| JSON (From KCL)  |                 |          | Yes              |             |
| JSON (From Ruyi) |                 |          |                  | 附加回 Ruyi |
| RPM Spec         | Yes（可用的）   |          |                  |             |

### 信息流向

```
KCL -> JSON -> RPM Spec
Ruyi + <IO> -> JSON -> RPM Spec
```
