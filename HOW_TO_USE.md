# 如何使用 Aether LSP - 快速总结

## 🚀 三种使用方式

### 1️⃣ 开发模式（最快）- 适合现在测试

```bash
# 1. 构建
cd /Users/xuzh/codes/aether-lsp
./build.sh

# 2. 在 VSCode 中按 F5

# 3. 在新窗口打开 examples/test_builtins.aether

# 4. 测试功能：
#    - 输入 "PRI" 看补全
#    - 悬停在函数上看文档
#    - Cmd+Click 跳转定义
```

### 2️⃣ 安装模式（推荐日常使用）

```bash
# 1. 打包扩展
./package.sh

# 2. 安装
code --install-extension vscode-extension/aether-lsp-0.1.0.vsix

# 3. 重启 VSCode，打开任何 .aether 文件
```

### 3️⃣ 发布到市场（可选）

需要 Visual Studio Marketplace 账号，详见 USAGE_GUIDE.md

---

## 🎯 当前已有的功能

✅ **可以用的**:
- 语法高亮（关键字、字符串、注释）
- 代码补全（26个关键字 + 53个内置函数）
- 实时错误检测（语法错误 + 命名检查）
- 函数文档（悬停查看）
- 跳转到定义

⚠️ **部分工作**:
- 符号位置不精确（跳转可能不准）
- Hover 功能未完全实现

❌ **还没有**:
- 代码格式化
- 重命名
- 查找引用
- 类型检查

---

## 🔨 接下来应该做什么

### 第一优先级（建议这周完成）

1. **修复符号位置** ⭐⭐⭐ (2-3天)
   - 为什么：让跳转到定义准确工作
   - 怎么做：在 Parser 添加 Span 跟踪
   - 文件：`src/parser.rs`, `src/ast.rs`, `src/token.rs`

2. **实现 Hover** ⭐⭐⭐ (1-2天)
   - 为什么：悬停时显示函数信息
   - 怎么做：实现 `backend.rs::hover()`
   - 效果：鼠标悬停显示签名和文档

### 第二优先级（下周）

3. **查找引用** (1-2天)
4. **代码片段** (1天)
5. **清理警告** (0.5天)

---

## 📖 详细文档

- **[USAGE_GUIDE.md](./USAGE_GUIDE.md)** - 完整使用教程、功能演示、FAQ
- **[ROADMAP.md](./ROADMAP.md)** - 功能路线图、优先级矩阵、时间规划
- **[QUICKSTART.md](./QUICKSTART.md)** - 项目概览、测试方法
- **[IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md)** - 技术实现细节

---

## 💡 快速测试

创建 `test.aether`:

```aether
# 测试补全 - 输入 "PRI" 然后 Ctrl+Space
PRINTLN("Hello Aether!")

# 测试错误检测 - 故意写错
Set myVar 42  # ❌ 应该看到红色波浪线

# 测试正确写法
Set MY_VAR 42
Set NUMBERS [1, 2, 3, 4, 5]

# 测试函数补全 - 输入 "MA"
Set DOUBLED MAP(NUMBERS, Lambda X -> X * 2)
Set TOTAL SUM(NUMBERS)

PRINTLN("Doubled:", DOUBLED)
PRINTLN("Total:", TOTAL)

# 测试跳转 - Cmd+Click MY_FUNCTION
Func MY_FUNCTION(X) {
    Return (X * 2)
}

Set RESULT MY_FUNCTION(10)
```

---

## ❓ 常见问题

**Q: 补全不工作？**
A: 检查文件扩展名是 `.aether`，重启 VSCode

**Q: 如何查看日志？**
A: Cmd+Shift+U 打开输出面板 → 选择 "Aether Language Server"

**Q: 如何贡献代码？**
A: 参考 ROADMAP.md 中的"适合新手的任务"

---

## 🎉 开始使用吧！

```bash
# 最快方法：
./build.sh
# 在 VSCode 按 F5
# 打开 examples/test_builtins.aether
```
