# Aether LSP 使用指南

## 🚀 快速开始

### 方法 1: VSCode 扩展开发模式（推荐用于测试）

这是最快的测试方式，适合开发调试。

#### 步骤：

1. **构建项目**
   ```bash
   cd /Users/xuzh/codes/aether-lsp
   ./build.sh
   ```

2. **启动扩展开发主机**
   - 在 VSCode 中打开此项目
   - 按 `F5` 或点击"运行" → "启动调试"
   - 会打开一个新的 VSCode 窗口（标题显示 `[Extension Development Host]`）

3. **测试 LSP 功能**
   - 在新窗口中打开 `examples/test_builtins.aether`
   - 尝试以下功能：
     - 输入 `PRI` → 应该看到 `PRINT` 和 `PRINTLN` 补全
     - 输入 `MAP` → 查看函数文档
     - 鼠标悬停在 `PRINTLN` 上 → 查看函数签名
     - 故意写错命名 `Set myVar 42` → 应该看到红色波浪线错误

---

### 方法 2: 打包并安装 VSCode 扩展（推荐用于日常使用）

将扩展打包成 `.vsix` 文件，可以安装到任何 VSCode 中。

#### 步骤：

1. **安装 vsce 打包工具**
   ```bash
   npm install -g @vscode/vsce
   ```

2. **打包扩展**
   ```bash
   cd vscode-extension
   vsce package
   # 会生成 aether-lsp-0.1.0.vsix
   ```

3. **安装扩展**
   
   **方法 A - 命令行安装：**
   ```bash
   code --install-extension aether-lsp-0.1.0.vsix
   ```

   **方法 B - VSCode UI 安装：**
   - 打开 VSCode
   - 点击扩展图标（或 Cmd+Shift+X）
   - 点击右上角 `...` → "从 VSIX 安装..."
   - 选择 `aether-lsp-0.1.0.vsix`

4. **重启 VSCode** 并打开任何 `.aether` 文件

---

### 方法 3: 直接运行 LSP 服务器（用于调试）

如果你想单独测试 LSP 服务器，可以直接运行它。

```bash
# 构建
cargo build --release

# 运行（标准输入/输出模式）
./target/release/aether-lsp

# LSP 服务器会等待 JSON-RPC 消息输入
```

---

## 📖 功能演示

### 1. 代码补全

在 `.aether` 文件中输入：

```aether
# 输入 "PRI" 然后按 Ctrl+Space
PRI█
```

应该看到：
- `PRINT` - 打印值(不换行)
- `PRINTLN` - 打印值到控制台并换行

继续输入：

```aether
# 输入 "MA" 
MA█
```

应该看到：
- `MAP` - 对数组每个元素应用函数
- `MAX` - 返回最大值

### 2. 语法错误诊断

```aether
# ❌ 语法错误 - 缺少右括号
Set MY_VAR [1, 2, 3
            ^^^^^^^^ 红色波浪线: E002: Unexpected token

# ❌ 命名约定错误
Set myVar 42
    ^^^^^ 红色波浪线: E001: 应该使用 UPPER_SNAKE_CASE (建议: MY_VAR)

# ✅ 正确
Set MY_VAR [1, 2, 3]
```

### 3. 悬停文档

将鼠标悬停在任何内置函数上：

```aether
Set NUMBERS [1, 2, 3, 4, 5]
Set DOUBLED MAP(NUMBERS, Lambda X -> X * 2)
           ^^^
           悬停在这里会显示:
           MAP(array, function)
           对数组每个元素应用函数
           
           示例:
           Set DOUBLED MAP(NUMBERS, Lambda X -> (X * 2))
```

### 4. 跳转到定义

```aether
Func MY_FUNCTION(X) {
    Return (X * 2)
}

Set RESULT MY_FUNCTION(5)
           ^^^^^^^^^^^
           Cmd/Ctrl + Click 跳转到函数定义
```

---

## 🎯 当前支持的功能

### ✅ 已实现

- [x] **语法高亮** - 关键字、字符串、注释、数字等
- [x] **代码补全**
  - [x] 26 个关键字（Set, Func, If, For, While, Return, etc.）
  - [x] 53 个内置函数（全大写命名）
  - [x] 用户定义的变量和函数
- [x] **实时诊断**
  - [x] 语法错误检测
  - [x] 命名约定检查（强制 UPPER_SNAKE_CASE）
  - [x] 错误代码（E001-E004, W001）
- [x] **悬停提示** - 显示函数签名和文档
- [x] **跳转到定义** - 跳转到变量/函数定义处
- [x] **符号表** - 提取所有符号供查找使用

### 🚧 部分实现

- [ ] **符号位置精确度** - 当前使用占位符（line 0），需要添加 Span 跟踪
- [ ] **Hover 信息** - 基础框架已有，需要完善显示内容

---

## 🛠️ 需要添加的功能

### 优先级 1: 核心改进（1-2 周）

#### 1.1 精确符号位置 ⭐⭐⭐
**问题**: 当前符号位置都是 `line: 0`，导致跳转不准确

**解决方案**:
```rust
// 在 Parser 中为每个 AST 节点添加 Span
pub struct Span {
    pub start: usize,  // 字符偏移
    pub end: usize,
}

// 修改 AST 定义
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,  // 新增
}
```

**工作量**: 2-3 天

#### 1.2 完善 Hover 功能 ⭐⭐⭐
**当前状态**: `hover()` 返回 `None`

**需要实现**:
- 变量悬停 → 显示类型（如果有类型推断）
- 函数悬停 → 显示完整签名和文档
- 内置函数 → 显示分类、签名、示例

**示例**:
```rust
impl Backend {
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let doc = self.documents.get(&params.text_document_position_params.text_document.uri)?;
        let position = params.text_document_position_params.position;
        
        // 查找符号
        if let Some(symbol) = doc.symbols.find_at_position(position) {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**{}**\n\n{}", symbol.name, symbol.documentation),
                }),
                range: Some(symbol.range),
            }));
        }
        
        Ok(None)
    }
}
```

**工作量**: 1-2 天

#### 1.3 代码格式化 ⭐⭐
**功能**: 自动格式化 Aether 代码

**需要实现**:
- 统一缩进（4 空格）
- 运算符周围空格
- 括号对齐
- 多行数组/字典格式化

**示例**:
```aether
# 格式化前
Set MY_VAR[1,2,3]
Func MY_FUNC(X){Return X*2}

# 格式化后
Set MY_VAR [1, 2, 3]
Func MY_FUNC(X) {
    Return (X * 2)
}
```

**工作量**: 3-4 天

---

### 优先级 2: 高级功能（2-4 周）

#### 2.1 重命名功能 ⭐⭐⭐
**功能**: 智能重命名变量/函数，更新所有引用

**实现步骤**:
1. 找到所有引用位置
2. 验证新名称符合命名约定
3. 批量更新所有位置

**工作量**: 2-3 天

#### 2.2 查找所有引用 ⭐⭐
**功能**: 找到变量/函数的所有使用位置

**工作量**: 1-2 天

#### 2.3 代码片段（Snippets）⭐⭐
**功能**: 快速插入常用代码模板

**示例**:
```json
{
  "for": {
    "prefix": "for",
    "body": [
      "For ${1:I} In ${2:RANGE(0, 10)} {",
      "    ${3:PRINTLN(I)}",
      "}"
    ]
  },
  "func": {
    "prefix": "func",
    "body": [
      "Func ${1:FUNCTION_NAME}(${2:PARAM}) {",
      "    ${3:Return PARAM}",
      "}"
    ]
  }
}
```

**工作量**: 1 天

#### 2.4 工作区符号搜索 ⭐
**功能**: `Cmd+T` 快速搜索项目中的所有符号

**工作量**: 2 天

---

### 优先级 3: 语义分析（1-2 月）

#### 3.1 类型推断 ⭐⭐⭐
**功能**: 静态分析变量类型

**示例**:
```aether
Set X 42          # 推断: Number
Set Y "hello"     # 推断: String
Set Z [1, 2, 3]   # 推断: Array<Number>

# 悬停在 X 上显示: X: Number
```

**工作量**: 1-2 周

#### 3.2 未使用变量检测 ⭐⭐
**功能**: 检测定义但从未使用的变量

```aether
Set UNUSED 42     # ⚠️ W002: Variable UNUSED is never used
Set USED 10
PRINTLN(USED)     # ✅ USED 被使用了
```

**工作量**: 3-5 天

#### 3.3 类型错误检测 ⭐⭐
**功能**: 检测类型不匹配

```aether
Set X 42
Set Y "hello"
Set Z (X + Y)     # ❌ E005: Cannot add Number and String
```

**工作量**: 1-2 周

---

### 优先级 4: 增强功能（可选）

#### 4.1 代码折叠
- 折叠函数体、循环体、注释块

#### 4.2 面包屑导航
- 显示当前所在函数/作用域

#### 4.3 大纲视图
- 在侧边栏显示文件结构

#### 4.4 调试支持（DAP）
- 断点、单步执行、变量查看

#### 4.5 测试框架集成
- 识别测试函数，一键运行测试

---

## 🐛 已知问题

1. **符号位置不精确** - 所有符号位置都是 `line: 0`
   - 影响: 跳转到定义不准确
   - 修复: 需要在 Parser 中添加 Span 跟踪

2. **Hover 未实现** - 悬停时不显示信息
   - 影响: 无法快速查看函数文档
   - 修复: 实现 `hover()` 方法

3. **编译 warnings** - 7 个未使用字段/导入警告
   - 影响: 无，仅影响编译输出
   - 修复: 运行 `cargo fix`

---

## 📊 开发路线图

### 第 1 阶段: 稳定核心功能（当前）✅
- [x] 完整解析器集成
- [x] 符号表提取
- [x] 基础补全
- [x] 语法诊断

### 第 2 阶段: 用户体验优化（接下来 2 周）
- [ ] 精确符号位置
- [ ] Hover 功能完善
- [ ] 代码格式化
- [ ] 代码片段

### 第 3 阶段: 高级 IDE 功能（1 个月）
- [ ] 重命名
- [ ] 查找引用
- [ ] 工作区符号
- [ ] 类型推断基础

### 第 4 阶段: 语义分析（2-3 个月）
- [ ] 完整类型系统
- [ ] 未使用变量检测
- [ ] 类型错误检测
- [ ] 智能建议

---

## 💡 贡献指南

如果你想添加新功能：

1. **阅读代码结构**
   - `src/parser.rs` - 语法分析
   - `src/symbols.rs` - 符号表管理
   - `src/diagnostics.rs` - 错误检测
   - `src/completion.rs` - 代码补全
   - `src/backend.rs` - LSP 服务器主逻辑

2. **运行测试**
   ```bash
   cargo test
   ```

3. **实现功能** 参考现有模块

4. **更新文档** 在此文件中记录新功能

---

## 🔧 配置选项

在 VSCode 设置中可以配置：

```json
{
  "aether.trace.server": "verbose"  // LSP 调试日志: off | messages | verbose
}
```

查看 LSP 日志：
- `Cmd+Shift+P` → "Developer: Show Logs" → "Extension Host"
- 或点击 VSCode 输出面板，选择 "Aether Language Server"

---

## 📚 相关资源

- **Aether 官方文档**: https://docs.rs/aether-azathoth/0.3.0/aether/
- **LSP 规范**: https://microsoft.github.io/language-server-protocol/
- **tower-lsp 文档**: https://docs.rs/tower-lsp/

---

## ❓ 常见问题

### Q: 为什么补全不工作？
**A**: 检查：
1. 文件扩展名是 `.aether`
2. LSP 服务器是否启动（查看 VSCode 输出面板）
3. 重启 VSCode

### Q: 如何查看 LSP 日志？
**A**: 
1. 打开 VSCode 输出面板（Cmd+Shift+U）
2. 在下拉菜单中选择 "Aether Language Server"
3. 或在设置中启用 `"aether.trace.server": "verbose"`

### Q: 如何更新扩展？
**A**:
```bash
cd aether-lsp
./build.sh
# 在 VSCode 中按 F5 重启扩展开发主机
```

### Q: 扩展可以发布到市场吗？
**A**: 可以！需要：
1. 注册 Visual Studio Marketplace 账号
2. 获取 Personal Access Token
3. 运行 `vsce publish`

详见: https://code.visualstudio.com/api/working-with-extensions/publishing-extension
