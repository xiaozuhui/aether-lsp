# 三个关键问题的修复

## 📋 问题总结与解决方案

### 问题 1: 运行时错误难以发觉 🔴

**问题描述**:

- LSP 服务器运行时没有日志输出
- 错误发生时用户不知道发生了什么
- 难以调试和排查问题

**解决方案**:
在 `src/backend.rs` 中添加详细日志：

```rust
async fn parse_and_diagnose(&self, uri: Url, text: String) {
    // ✅ 添加：解析开始日志
    self.client
        .log_message(MessageType::INFO, format!("Parsing document: {}", uri))
        .await;

    let mut parser = Parser::new(&text);
    let parsed = parser.parse();
    let diagnostics = DiagnosticEngine::analyze(&parsed, &text);

    // ✅ 添加：诊断结果日志
    self.client
        .log_message(
            MessageType::INFO,
            format!("Found {} diagnostics for {}", diagnostics.len(), uri),
        )
        .await;
    
    // ...
}

async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    // ✅ 添加：Hover 请求日志
    self.client
        .log_message(
            MessageType::INFO,
            format!("Hover requested at {}:{}", position.line, position.character),
        )
        .await;
    
    // ✅ 添加：查找内置函数日志
    if let Some(word) = extract_word_at_position(&doc.text, position) {
        self.client
            .log_message(MessageType::INFO, format!("Looking for builtin: {}", word))
            .await;
        // ...
    }
}
```

**如何查看日志**:

1. 打开 VSCode 输出面板（Cmd+Shift+U / Ctrl+Shift+U）
2. 在下拉菜单选择 "Aether Language Server"
3. 现在可以看到实时日志：

   ```
   [INFO] Parsing document: file:///path/to/file.aether
   [INFO] Found 2 diagnostics for file:///path/to/file.aether
   [INFO] Hover requested at 5:10
   [INFO] Looking for builtin: PRINTLN
   ```

---

### 问题 2: 函数解释和跳转不能用 🔴

**问题描述**:

- 鼠标悬停在内置函数（如 `PRINTLN`, `MAP`）上没有任何提示
- 无法查看函数签名和说明
- 跳转功能只对用户定义的函数有效

**解决方案**:

#### 2.1 添加内置函数查找功能

在 `src/builtins.rs` 中添加：

```rust
/// Find a builtin function by name (case-insensitive)
pub fn find_builtin(name: &str) -> Option<BuiltinFunction> {
    let name_upper = name.to_uppercase();
    get_builtin_functions()
        .into_iter()
        .find(|f| f.name.to_uppercase() == name_upper)
}

/// Create hover content for a builtin function
pub fn builtin_to_hover(func: &BuiltinFunction) -> Hover {
    let content = format!(
        "## {} (内置函数)\n\n**签名**: `{}`\n\n**描述**: {}\n\n**分类**: {}\n\n**示例**:\n```aether\n{}\n```",
        func.name,
        func.signature,
        func.description,
        func.category,
        func.examples.join("\n")
    );

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    }
}
```

#### 2.2 实现文本提取功能

在 `src/backend.rs` 中添加辅助函数：

```rust
/// Extract the word (identifier) at the given position
fn extract_word_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }

    let line = lines[position.line as usize];
    let char_pos = position.character as usize;
    
    // 找到单词的起始和结束位置
    let mut start = char_pos;
    let mut end = char_pos;

    // 向左查找单词开始（支持字母、数字、下划线）
    while start > 0 {
        let ch = line.chars().nth(start - 1)?;
        if ch.is_alphanumeric() || ch == '_' {
            start -= 1;
        } else {
            break;
        }
    }

    // 向右查找单词结束
    while end < line.len() {
        let ch = line.chars().nth(end)?;
        if ch.is_alphanumeric() || ch == '_' {
            end += 1;
        } else {
            break;
        }
    }

    if start < end {
        Some(line[start..end].to_string())
    } else {
        None
    }
}
```

#### 2.3 增强 Hover 功能

更新 `hover()` 方法：

```rust
async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
    if let Some(doc) = self.documents.get(&uri) {
        // 1. 先查找用户定义的符号
        if let Some(symbol_info) = doc.symbols.find_at_position(position) {
            return Ok(Some(Hover { /* 用户符号 */ }));
        }

        // 2. ✅ 新增：查找内置函数
        if let Some(word) = extract_word_at_position(&doc.text, position) {
            if let Some(builtin) = crate::builtins::find_builtin(&word) {
                return Ok(Some(crate::builtins::builtin_to_hover(&builtin)));
            }
        }
    }

    Ok(None)
}
```

**效果演示**:

鼠标悬停在 `PRINTLN` 上：

```
┌────────────────────────────────────┐
│ ## PRINTLN (内置函数)              │
│                                    │
│ **签名**: `PRINTLN(value...)`     │
│                                    │
│ **描述**: 打印值到控制台并换行     │
│                                    │
│ **分类**: IO                       │
│                                    │
│ **示例**:                          │
│ ```aether                          │
│ PRINTLN("Hello World")             │
│ PRINTLN(MY_VAR, MY_VAR2)           │
│ ```                                │
└────────────────────────────────────┘
```

---

### 问题 3: 补全只有函数和关键字，没有已存在变量 🟡

**问题描述**:

- 输入变量名时没有补全提示
- 已定义的变量需要手动输入
- 降低编码效率

**解决方案**:

#### 3.1 添加变量补全函数

在 `src/completion.rs` 中添加：

```rust
/// Get variable completions from symbol table
fn get_variable_completions(symbols: &SymbolTable) -> Vec<CompletionItem> {
    symbols
        .variables
        .iter()
        .map(|var| CompletionItem {
            label: var.name.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: var.detail.clone().or_else(|| Some(format!("Variable: {}", var.name))),
            documentation: if !var.documentation.is_empty() {
                Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: var.documentation.clone(),
                }))
            } else {
                None
            },
            insert_text: Some(var.name.clone()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect()
}

/// Get function completions from symbol table
fn get_function_completions(symbols: &SymbolTable) -> Vec<CompletionItem> {
    symbols
        .functions
        .iter()
        .map(|func| CompletionItem {
            label: func.name.clone(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: func.detail.clone().or_else(|| Some(format!("Function: {}", func.name))),
            documentation: if !func.documentation.is_empty() {
                Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: func.documentation.clone(),
                }))
            } else {
                None
            },
            insert_text: Some(format!("{}($1)", func.name)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}
```

#### 3.2 更新主补全函数

```rust
pub fn get_completions(doc: &ParsedDocument, _position: Position) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // 1. 关键字补全
    completions.extend(get_keyword_completions());

    // 2. 内置函数补全
    completions.extend(builtins::builtin_to_completion_items());

    // 3. ✅ 新增：用户定义的变量补全
    completions.extend(get_variable_completions(&doc.symbols));

    // 4. ✅ 新增：用户定义的函数补全
    completions.extend(get_function_completions(&doc.symbols));

    completions
}
```

**效果演示**:

```aether
Set MY_VARIABLE 42
Set MY_ARRAY [1, 2, 3]

Func MY_FUNCTION(X) {
    Return (X * 2)
}

# 输入 "MY" 后按 Ctrl+Space，现在会看到：
MY_VARIABLE    📦 Variable: MY_VARIABLE
MY_ARRAY       📦 Variable: MY_ARRAY
MY_FUNCTION    🔧 Function: MY_FUNCTION(X)
```

---

## 🎯 测试验证

### 测试 1: 日志功能

```bash
# 1. 构建项目
cargo build

# 2. 在 VSCode 中按 F5 启动扩展

# 3. 打开输出面板（Cmd+Shift+U）

# 4. 选择 "Aether Language Server"

# 5. 打开 .aether 文件，应该看到：
[INFO] Parsing document: file:///...
[INFO] Found 0 diagnostics for ...
```

### 测试 2: 内置函数 Hover

```aether
# 创建测试文件
PRINTLN("Test")
MAP([1,2,3], Lambda X -> X * 2)

# 鼠标悬停在 PRINTLN 上
# ✅ 应该显示：签名、描述、分类、示例

# 鼠标悬停在 MAP 上
# ✅ 应该显示完整文档
```

### 测试 3: 变量补全

```aether
# 定义变量
Set MY_VAR 42
Set MY_NAME "Alice"
Set MY_ARRAY [1, 2, 3]

# 在新行输入 "MY" 然后 Ctrl+Space
# ✅ 应该看到：
#    - MY_VAR (Variable)
#    - MY_NAME (Variable)
#    - MY_ARRAY (Variable)

# 选择 MY_VAR
# ✅ 应该自动插入 MY_VAR
```

### 测试 4: 函数补全

```aether
Func CALCULATE_SUM(A, B) {
    Return (A + B)
}

Func GET_DOUBLE(X) {
    Return (X * 2)
}

# 输入 "CALC" 然后 Ctrl+Space
# ✅ 应该看到：CALCULATE_SUM (Function)

# 选择后自动插入：CALCULATE_SUM($1)
#                              ^^^ 光标在这里
```

---

## 📊 改进对比

| 功能 | 修复前 | 修复后 |
|------|--------|--------|
| **日志输出** | ❌ 没有 | ✅ 详细日志（解析、诊断、Hover） |
| **内置函数 Hover** | ❌ 不工作 | ✅ 显示完整文档 |
| **变量补全** | ❌ 没有 | ✅ 自动补全所有变量 |
| **用户函数补全** | ❌ 没有 | ✅ 自动补全所有函数 |
| **调试能力** | 🔴 困难 | 🟢 容易 |

---

## 🔧 技术实现细节

### 修改的文件

1. **`src/backend.rs`** (核心修改)
   - 添加日志记录
   - 实现 `extract_word_at_position()` 辅助函数
   - 增强 `hover()` 方法支持内置函数

2. **`src/builtins.rs`**
   - 添加 `find_builtin()` 查找函数
   - 添加 `builtin_to_hover()` 生成 Hover 内容

3. **`src/completion.rs`**
   - 添加 `get_variable_completions()`
   - 添加 `get_function_completions()`
   - 更新 `get_completions()` 集成用户符号

### 编译结果

```bash
$ cargo build
   Compiling aether-lsp v0.1.0
warning: 7 warnings (non-blocking)
    Finished `dev` profile in 1.63s

$ cargo test
running 7 tests
test result: ok. 7 passed; 0 failed
```

---

## 🚀 立即使用

```bash
# 1. 重新编译
cd /Users/xuzh/codes/aether-lsp
cargo build --release

# 2. 在 VSCode 中按 F5

# 3. 测试新功能：
#    - 查看日志：Cmd+Shift+U → "Aether Language Server"
#    - 悬停函数：鼠标悬停在 PRINTLN 上
#    - 补全变量：输入 MY 然后 Ctrl+Space
```

---

## 📝 下一步优化建议

1. **精确符号位置** - 当前位置都是 line 0，需要添加 Span 跟踪
2. **上下文感知补全** - 根据当前位置只显示相关补全
3. **补全排序** - 按相关性和使用频率排序
4. **snippets 补全** - 添加常用代码模板
