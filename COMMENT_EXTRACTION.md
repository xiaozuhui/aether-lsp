# 注释提取功能文档

## 📝 功能说明

现在 Aether LSP 支持在代码补全时显示变量的注释说明！

当你在变量声明语句前（或上方）添加注释时，这些注释会自动出现在代码补全的文档中。

## 🎯 支持的注释格式

### 1. 单行注释 `//`

```aether
// 这是用户的名字
Set MY_NAME "Alice"

// 用户的年龄
Set MY_AGE 25
```

**效果**：输入 `MY` 然后 Ctrl+Space，会看到：

```
MY_NAME     📦 Variable: MY_NAME
            📖 这是用户的名字

MY_AGE      📦 Variable: MY_AGE
            📖 用户的年龄
```

### 2. 多行注释 `//`

```aether
// 这是一个重要的配置变量
// 它用来控制调试模式
// 默认值为 False
Set DEBUG_MODE True
```

**效果**：

```
DEBUG_MODE  📦 Variable: DEBUG_MODE
            📖 这是一个重要的配置变量
               它用来控制调试模式
               默认值为 False
```

### 3. 块注释 `/* */`（单行）

```aether
/* 这是一个重要的常量 */
Set MAX_SIZE 100
```

**效果**：

```
MAX_SIZE    📦 Variable: MAX_SIZE
            📖 这是一个重要的常量
```

### 4. 块注释 `/* */`（多行）

```aether
/*
多行注释示例
这个变量存储了一个数组
包含了前5个偶数
*/
Set EVEN_NUMBERS [2, 4, 6, 8, 10]
```

**效果**：

```
EVEN_NUMBERS  📦 Variable: EVEN_NUMBERS
              📖 多行注释示例
                 这个变量存储了一个数组
                 包含了前5个偶数
```

## 🔧 实现细节

### 注释提取规则

1. **向上查找**：从变量声明语句开始，向上查找注释
2. **连续注释**：提取所有连续的注释行（中间可以有空行）
3. **停止条件**：遇到非注释、非空行时停止
4. **块注释**：遇到块注释后停止向上查找

### 示例代码

```rust
// src/symbols.rs

/// Find comment for a variable by searching for "Set VARIABLE_NAME" pattern
fn find_comment_for_variable(text: &str, var_name: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    
    // Search for the line containing "Set VARIABLE_NAME"
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("Set ") || trimmed.starts_with("set ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == var_name {
                return extract_preceding_comment(text, idx);
            }
        }
    }
    
    String::new()
}
```

## 📊 测试案例

测试文件：`examples/test_comments.aether`

```aether
// 这是用户的名字
Set MY_NAME "Alice"

// 用户的年龄
// 注意：这是整数类型
Set MY_AGE 25

/* 这是一个重要的配置变量
   它用来控制调试模式 */
Set DEBUG_MODE True

// 计算两个数的和
Set SUM (MY_AGE + 5)

/*
多行注释示例
这个变量存储了一个数组
包含了前5个偶数
*/
Set EVEN_NUMBERS [2, 4, 6, 8, 10]

// 无注释的变量
Set NO_COMMENT_VAR 42
```

## 🎬 使用演示

### 步骤 1：在 VSCode 中打开测试文件

```bash
code examples/test_comments.aether
```

### 步骤 2：按 F5 启动 LSP 调试

在 VSCode 中按 `F5`，会打开一个新窗口运行 LSP。

### 步骤 3：测试代码补全

在新窗口中，打开 `examples/test_comments.aether`：

1. 在文件末尾输入 `MY`
2. 按 `Ctrl+Space`（Windows/Linux）或 `Cmd+Space`（macOS）
3. 查看补全列表中的文档

**预期结果**：

```
MY_NAME
├─ Kind: Variable
├─ Detail: Variable: MY_NAME
└─ Documentation: 这是用户的名字

MY_AGE
├─ Kind: Variable
├─ Detail: Variable: MY_AGE
└─ Documentation: 用户的年龄
                  注意：这是整数类型
```

### 步骤 4：测试 Hover

1. 将鼠标悬停在 `MY_NAME` 上
2. 查看 Hover 提示

**预期结果**：

```
┌────────────────────────────┐
│ Variable: MY_NAME          │
│                            │
│ 这是用户的名字              │
└────────────────────────────┘
```

## ⚙️ 配置和扩展

### 支持函数注释（未来）

目前只支持变量注释。未来可以扩展到函数注释：

```aether
// 计算两个数的平方和
// 参数：
//   - X: 第一个数
//   - Y: 第二个数
// 返回：X² + Y²
Func SQUARE_SUM(X, Y) {
    Return ((X * X) + (Y * Y))
}
```

### 自定义注释格式（未来）

支持特殊标记：

```aether
// @description: 用户的年龄
// @type: Integer
// @range: 0-150
Set USER_AGE 25
```

## 🐛 已知限制

1. **仅支持 Set 语句**：目前只提取 `Set` 语句前的注释，不支持 `Lazy` 和函数
2. **位置信息**：目前符号位置使用占位符（line 0），需要改进
3. **注释格式**：不支持行内注释（如 `Set X 42 // 注释`）

## 📈 改进建议

1. **扩展到函数**：为函数定义提取注释
2. **参数文档**：解析 `@param` 等标记
3. **返回值文档**：解析 `@return` 标记
4. **示例代码**：解析 `@example` 标记
5. **类型提示**：解析 `@type` 标记

## 🚀 快速开始

```bash
# 1. 构建项目
cargo build --release

# 2. 编译 VSCode 扩展
cd vscode-extension
npm run compile

# 3. 在 VSCode 中按 F5

# 4. 打开测试文件
# File → Open File → examples/test_comments.aether

# 5. 测试补全
# 输入 "MY" 然后按 Ctrl+Space
```

## 📝 编码建议

为了获得最佳的补全体验，建议：

1. **在变量声明前添加注释**
2. **使用清晰简洁的描述**
3. **多行注释说明复杂变量**
4. **使用一致的注释风格**

### 好的例子 ✅

```aether
// 应用程序的配置目录路径
Set CONFIG_DIR "/etc/myapp"

// 最大重试次数（默认3次）
Set MAX_RETRIES 3

/* 
数据库连接配置
包含主机、端口、用户名等信息
*/
Set DB_CONFIG {...}
```

### 不好的例子 ❌

```aether
Set CONFIG_DIR "/etc/myapp"  // 配置目录（行内注释不会被提取）

Set MAX_RETRIES 3
// 这个注释在声明后面，不会被提取

/*

空注释

*/
Set DB_CONFIG {...}
```

---

## 🎉 总结

现在你可以为 Aether 代码添加丰富的文档注释，LSP 会自动在补全时显示这些注释，让代码更易读、更易维护！

享受编码的乐趣吧！🚀
