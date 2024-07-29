# ZKP DSL Language Design

## 目标
创建一种领域特定语言(DSL),用于简化零知识证明(ZKP)电路的编写和验证过程。

## 核心特性
1. 声明式: 允许用户声明约束而不是命令式地描述计算步骤
2. 类型安全: 提供强类型系统以捕获常见错误
3. 模块化: 支持可重用的电路组件
4. 可读性: 语法应该直观且易于理解
5. 效率: 生成的电路应该是高效的

## 基本类型
- Field: 表示有限域中的元素
- Boolean: 真或假值
- Array: 固定大小的数组
- Struct: 自定义复合类型

## 关键字
- `input`: 声明公共输入
- `private`: 声明私有输入
- `constraint`: 定义约束
- `assert`: 断言某个条件必须为真
- `for`: 用于迭代和生成重复的约束
- `function`: 定义可重用的电路组件

## 操作符
- 算术: `+`, `-`, `*`, `/`
- 比较: `==`, `!=`, `<`, `>`, `<=`, `>=`
- 逻辑: `&&`, `||`, `!`
- 位操作: `&`, `|`, `^`, `<<`, `>>`

## 示例语法
```
input public x: Field
input private y: Field

constraint c1 {
    x * y == 10
}

assert x < 100

for i in 0..5 {
    constraint c2[i] {
        x + i == y
    }
}

function hash(a: Field, b: Field) -> Field {
    // 定义哈希函数的约束
}

let z = hash(x, y)
assert z != 0
```