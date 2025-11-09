# Aether LSP 快速开始

## 项目状态

✅ **Phase 1 完成** - 基础 LSP 功能

- [x] Lexer/Parser 集成（完整原生实现）
- [x] 符号表提取（变量、函数、生成器）
- [x] 精确诊断系统（语法错误 + 命名约定）
- [x] 智能补全（关键字 + 80+ 内置函数）
- [x] 所有单元测试通过

## 如何测试

### 1. 编译 LSP 服务器

```bash
cargo build --release
# 或运行测试
cargo test --workspace
```

### 2. 测试符号提取

```bash
# 已创建测试文件
cat examples/test_symbols.aether
```

### 3. 在 VSCode 中测试

1. 打开 VSCode
2. 按 `F5` 启动扩展开发
3. 在新窗口创建 `.aether` 文件
4. 测试功能：
   - 输入 `Set` 触发补全
   - 输入 `Pr` 看到 `Println`, `Print` 等
   - 输入错误语法查看诊断

## 已实现的功能

### A. 符号表提取 ✅

- 从 AST 提取变量定义（`Set`）
- 从 AST 提取函数定义（`Func`, `Generator`）
- 从 AST 提取惰性定义（`Lazy`）
- 递归提取嵌套作用域中的符号
- 支持 `find_at_position()` 和 `find_definition()`

**代码位置**: `src/symbols.rs` - `SymbolTable::from_ast()`

### B. 改进诊断系统 ✅

- 解析错误精确映射（行列位置）
- 错误代码分类：
  - `E000`: 通用错误
  - `E001`: 命名约定违规
  - `E002`: 意外 token
  - `E003`: 期望 token
  - `E004`: 无效表达式
  - `W001`: 命名约定警告
- 智能错误长度估算
- 命名建议（自动转 UPPER_SNAKE_CASE）

**代码位置**: `src/diagnostics.rs`

### C. 内置函数补全 ✅

80+ 内置函数，分 9 大类：

- IO (5 个)
- Array (15 个)
- String (9 个)
- Math (22 个)
- Generator (4 个)
- Type (9 个)
- Dict (5 个)
- JSON (2 个)
- DateTime (3 个)

每个函数包含：

- 完整签名
- 中文描述
- 代码示例
- Markdown 文档
- Snippet 插入

**代码位置**: `src/builtins.rs`

## 下一步计划

### Phase 2 (短期)

1. **Hover 提示增强**
   - 显示函数完整签名
   - 显示变量类型推断
   - 链接到文档

2. **格式化支持**
   - 自动缩进
   - 代码美化

3. **Workspace 符号**
   - 全局符号搜索
   - 文件间跳转

### Phase 3 (中期)

1. **重构支持**
   - 重命名 (rename) 完善
   - 提取函数
   - 内联变量

2. **代码操作**
   - 快速修复命名约定
   - 自动导入

3. **语义分析**
   - 类型推断
   - 未使用变量检测

### Phase 4 (长期)

1. **调试器集成**
2. **REPL 支持**
3. **性能优化**
   - 增量解析
   - 缓存优化

## 性能测试

```bash
# 测试解析性能
time target/release/aether-lsp < examples/test_symbols.aether

# 测试补全性能
# (在 VSCode 中输入测试)
```

## 贡献指南

### 添加新的内置函数

编辑 `src/builtins.rs`，添加到 `get_builtin_functions()`:

```rust
BuiltinFunction {
    name: "MyFunc",
    signature: "MyFunc(param1, param2)",
    description: "函数描述",
    category: "Category",
    examples: &["Set RESULT MyFunc(1, 2)"],
},
```

### 改进诊断

编辑 `src/diagnostics.rs`，在 `DiagnosticEngine::analyze()` 中添加检查。

### 扩展符号提取

编辑 `src/symbols.rs`，修改 `extract_symbols_from_stmt()` 逻辑。

## 已知问题

1. ⚠️ 符号位置信息为占位符（当前为 line 0）
   - 需要在 Parser 中记录每个节点的 Span
   - 计划在 Phase 2 修复

2. ⚠️ 部分 warnings（unused imports/fields）
   - 不影响功能
   - 可通过 `cargo fix` 清理

3. ⚠️ `SymbolInformation::deprecated` 已弃用
   - 应使用 `tags` 字段
   - 计划在下次更新修复

## 联系方式

- 作者: xiaozuhui
- 项目: <https://github.com/xiaozuhui/aether-lang>
