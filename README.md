# Aether LSP Server

Language Server Protocol 实现 for Aether 编程语言。

## 特性

### 核心功能 ✅

- **完整的解析器**: 基于原生 Rust Parser + Lexer，支持 Aether 完整语法
- **符号表提取**: 自动提取变量、函数、生成器定义，支持跳转到定义
- **精确诊断**:
  - 语法错误检测（带精确行列信息）
  - 命名约定检查（强制 UPPER_SNAKE_CASE）
  - 错误代码分类（E001-E004, W001）
- **智能补全**:
  - 26 个关键字（带文档和示例）
  - 80+ 内置函数（自动生成，带完整签名和分类）
  - 用户定义符号（变量、函数）
- **丰富文档**: Markdown 格式的 hover 提示和补全文档

### 内置函数分类

- **I/O**: `Println`, `Print`, `ReadFile`, `WriteFile`, `ReadLine`
- **Array**: `Length`, `Append`, `First`, `Last`, `Map`, `Filter`, `Reduce`, `Sort`, `Reverse`, `Join`, `Slice`, `Contains`, `IndexOf`, `Flatten`, `Unique`
- **String**: `Split`, `Upper`, `Lower`, `Trim`, `Replace`, `StartsWith`, `EndsWith`, `Substring`, `Format`
- **Math**: `Sum`, `Mean`, `Median`, `Std`, `Variance`, `Max`, `Min`, `Abs`, `Floor`, `Ceil`, `Round`, `Sqrt`, `Pow`, `Log`, `Log10`, `Exp`, `Sin`, `Cos`, `Tan`, `Random`, `RandomInt`
- **Generator**: `Range`, `Repeat`, `Enumerate`, `Zip`
- **Type**: `Type`, `String`, `Number`, `Boolean`, `IsNumber`, `IsString`, `IsArray`, `IsDict`, `IsNull`
- **Dict**: `Keys`, `Values`, `Items`, `HasKey`, `Merge`
- **JSON**: `JsonParse`, `JsonStringify`
- **DateTime**: `Now`, `FormatDate`, `Sleep`

## 安装与构建

### 1. 构建 LSP 服务器

```bash
cd aether-lsp
cargo build --release

# 可执行文件位于: target/release/aether-lsp
```

### 2. 运行测试

```bash
cargo test --workspace
```

### 3. 安装 VSCode 扩展

```bash
cd vscode-extension
npm install
npm run compile
```

然后在 VSCode 中按 `F5` 启动扩展开发主机。

## 使用

1. 创建 `.aether` 文件
2. VSCode 会自动激活 Aether 语言支持
3. 享受 LSP 功能:
   - 输入时自动补全
   - 悬停查看函数文档
   - Cmd/Ctrl + Click 跳转定义

## Aether 语法示例

```aether
// 变量声明 (必须使用 UPPER_SNAKE_CASE)
Set MY_VAR 42
Set MY_NAME "Aether"

// 函数定义
Func CALCULATE_SUM(A, B) {
    Return (A + B)
}

// 调用函数
Set RESULT CALCULATE_SUM(10, 20)
Println(RESULT)

// 控制流
If (MY_VAR > 0) {
    Println("Positive")
} Else {
    Println("Non-positive")
}

// For 循环
For I In Range(0, 10) {
    Println(I)
}

// 数组操作
Set MY_ARRAY [1, 2, 3, 4, 5]
Set DOUBLED Map(MY_ARRAY, Lambda X -> (X * 2))
Println(DOUBLED)  // [2, 4, 6, 8, 10]

// 生成器
Generator FIBONACCI(N) {
    Set A 0
    Set B 1
    For I In Range(0, N) {
        Yield A
        Set TEMP (A + B)
        Set A B
        Set B TEMP
    }
}

// 惰性求值
Lazy EXPENSIVE_CALC(Func() {
    Return Sum(Range(1, 1000000))
})

Set RESULT Force(EXPENSIVE_CALC)
```

## 命名约定

Aether 强制使用 **UPPER_SNAKE_CASE** 命名:

✅ 正确:

```aether
Set MY_VARIABLE 10
Func CALCULATE_TOTAL(ITEMS) { ... }
```

❌ 错误:

```aether
Set myVariable 10      // 小写
Set My_Variable 10     // 混合大小写
Func calculateTotal() { ... }
```

## 内置函数

Aether 提供 200+ 内置函数,分类如下:

### 数学函数

- `Sum`, `Mean`, `Std`, `Max`, `Min`
- `Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`

### 数组函数

- `Map`, `Filter`, `Reduce`, `Sort`, `Reverse`
- `First`, `Last`, `Length`, `Append`, `Join`

### 字符串函数

- `Split`, `Upper`, `Lower`, `Trim`
- `Contains`, `Replace`, `Substring`

### IO 函数

- `Println`, `Print`, `ReadFile`, `WriteFile`

### 类型转换

- `String`, `Number`, `Boolean`, `Type`

## 开发

### 项目结构

```
aether-lsp/
├── src/
│   ├── main.rs          # LSP 服务器入口
│   ├── backend.rs       # LanguageServer 实现
│   ├── lexer.rs         # 词法分析器
│   ├── parser.rs        # 语法分析器
│   ├── token.rs         # Token 定义
│   ├── ast.rs           # AST 定义
│   ├── diagnostics.rs   # 诊断引擎
│   ├── completion.rs    # 自动补全
│   └── symbols.rs       # 符号表
├── vscode-extension/
│   ├── src/extension.ts # VSCode 扩展入口
│   ├── syntaxes/        # 语法高亮规则
│   └── package.json
├── Cargo.toml
└── README.md
```

### 运行测试

```bash
cargo test
```

### 调试 LSP

在 VSCode 设置中启用调试日志:

```json
{
  "aether.trace.server": "verbose"
}
```

## Roadmap

- [x] Phase 1: 基础语法高亮和命名检查
- [x] Phase 2: 完整解析器集成
- [ ] Phase 3: 语义分析和类型推断
- [ ] Phase 4: 调试器集成

## License

MIT

## 作者

xiaozuhui
