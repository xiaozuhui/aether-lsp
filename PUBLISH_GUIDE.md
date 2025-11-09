# VSCode 扩展发布指南

## 📋 发布前准备清单

### 1. 完善 package.json

在发布前，需要完善 `vscode-extension/package.json`：

```json
{
    "name": "aether-lsp",
    "displayName": "Aether Language Support",
    "description": "Language Server Protocol support for Aether programming language with syntax highlighting, auto-completion, and diagnostics",
    "version": "0.1.0",
    "publisher": "xiaozuhui",
    "author": {
        "name": "xiaozuhui"
    },
    "license": "MIT",
    "repository": {
        "type": "git",
        "url": "https://github.com/xiaozuhui/aether-lsp"
    },
    "bugs": {
        "url": "https://github.com/xiaozuhui/aether-lsp/issues"
    },
    "homepage": "https://github.com/xiaozuhui/aether-lsp#readme",
    "keywords": [
        "aether",
        "language-server",
        "lsp",
        "syntax-highlighting",
        "completion",
        "diagnostics"
    ],
    "icon": "icon.png",
    "galleryBanner": {
        "color": "#1e1e1e",
        "theme": "dark"
    },
    "engines": {
        "vscode": "^1.75.0"
    },
    "categories": [
        "Programming Languages",
        "Linters"
    ]
}
```

### 2. 添加必需文件

#### 2.1 README.md（必需）

在 `vscode-extension/README.md` 创建扩展说明：

```markdown
# Aether Language Support

为 Aether 编程语言提供完整的 LSP 支持。

## 特性

- ✅ **语法高亮** - 完整的 TextMate 语法支持
- ✅ **代码补全** - 智能补全关键字、内置函数和用户定义符号
- ✅ **注释提取** - 自动显示变量注释文档
- ✅ **诊断** - 实时语法错误检测和命名约定检查
- ✅ **Hover 提示** - 查看符号和内置函数文档
- ✅ **跳转到定义** - 快速导航到符号定义

## 快速开始

1. 安装扩展
2. 打开 `.aether` 文件
3. 开始编码！

## 示例

```aether
// 这是用户的名字
Set MY_NAME "Alice"

// 计算平方
Func SQUARE(X) {
    Return (X * X)
}

PRINTLN("Hello, Aether!")
```

## 内置函数

支持 53 个内置函数，包括：

- **I/O**: PRINTLN, PRINT, READ_FILE, WRITE_FILE
- **Array**: MAP, FILTER, REDUCE, SORT, LENGTH
- **String**: SPLIT, UPPER, LOWER, TRIM, REPLACE
- **Math**: SUM, MEAN, MAX, MIN, SQRT, POW
- 更多...

## 要求

无特殊要求。扩展已包含 LSP 服务器二进制文件。

## 反馈

遇到问题？[提交 Issue](https://github.com/xiaozuhui/aether-lsp/issues)

## 许可证

MIT License

```

#### 2.2 CHANGELOG.md（推荐）

在 `vscode-extension/CHANGELOG.md` 创建更新日志：

```markdown
# 更新日志

## [0.1.0] - 2025-11-09

### 新增
- 🎉 首次发布
- ✨ 完整的语法高亮支持
- ✨ 智能代码补全（关键字、内置函数、用户符号）
- ✨ 注释提取功能 - 在补全中显示变量注释
- ✨ 实时语法诊断
- ✨ 命名约定检查（UPPER_SNAKE_CASE）
- ✨ Hover 提示（用户符号 + 内置函数）
- ✨ 跳转到定义
- ✨ 53 个内置函数支持

### 功能
- 支持单行注释 `//` 和块注释 `/* */`
- 错误代码分类（E001-E004, W001）
- Markdown 格式文档
- 自动文件监视和诊断更新
```

#### 2.3 LICENSE（必需）

在 `vscode-extension/LICENSE` 创建许可证文件：

```text
MIT License

Copyright (c) 2025 xiaozuhui

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

#### 2.4 图标（可选但推荐）

创建一个 128x128 的 PNG 图标：`vscode-extension/icon.png`

你可以使用在线工具创建：

- <https://www.canva.com/>
- <https://www.figma.com/>
- 或使用 DALL-E、Midjourney 等 AI 工具

建议：

- 简洁的设计
- 清晰可辨识
- 与 Aether 语言相关的元素

### 3. 包含 LSP 二进制文件

确保在 `.vscodeignore` 中**不要**排除二进制文件：

创建/修改 `vscode-extension/.vscodeignore`：

```
.vscode/**
.vscode-test/**
src/**
.gitignore
.yarnrc
vsc-extension-quickstart.md
**/tsconfig.json
**/.eslintrc.json
**/*.map
**/*.ts
!out/**/*.js
node_modules/
*.vsix

# 确保包含 LSP 二进制文件
!../target/release/aether-lsp
!../target/debug/aether-lsp
```

或者，在打包时手动复制二进制：

```bash
# 在 vscode-extension/ 目录创建 bin 文件夹
mkdir -p bin

# 复制二进制文件
cp ../target/release/aether-lsp bin/
cp ../target/release/aether-lsp.exe bin/ 2>/dev/null || true  # Windows
```

然后修改 `extension.ts` 中的路径：

```typescript
const serverPath = context.asAbsolutePath(
    path.join('bin', 'aether-lsp' + (process.platform === 'win32' ? '.exe' : ''))
);
```

## 🚀 发布步骤

### 方法 1: 使用 vsce（推荐）

#### 1. 安装 vsce

```bash
npm install -g @vscode/vsce
```

#### 2. 创建 Personal Access Token (PAT)

1. 访问 <https://dev.azure.com/>
2. 登录你的 Microsoft 账号
3. 点击右上角用户图标 → **User settings** → **Personal access tokens**
4. 点击 **New Token**
5. 配置：
   - **Name**: VSCode Extension Publishing
   - **Organization**: All accessible organizations
   - **Expiration**: 自定义（建议 90 天或更长）
   - **Scopes**:
     - ✅ **Marketplace** → **Manage** (必须勾选)
6. 点击 **Create**
7. **重要**: 复制生成的 token（只显示一次！）

#### 3. 创建发布者账号

如果还没有发布者账号：

1. 访问 <https://marketplace.visualstudio.com/manage>
2. 点击 **Create publisher**
3. 填写信息：
   - **Publisher ID**: xiaozuhui（必须与 package.json 中的 publisher 一致）
   - **Display name**: Xiaozuhui 或你的名字
   - **Email**: 你的邮箱
4. 点击 **Create**

#### 4. 登录 vsce

```bash
cd vscode-extension
vsce login xiaozuhui
```

输入刚才创建的 PAT。

#### 5. 打包测试

```bash
# 确保已编译
npm run compile

# 打包成 .vsix 文件
vsce package
```

会生成 `aether-lsp-0.1.0.vsix` 文件。

#### 6. 本地测试

```bash
# 在 VSCode 中安装测试
code --install-extension aether-lsp-0.1.0.vsix

# 测试功能是否正常
```

#### 7. 发布到市场

```bash
vsce publish
```

或者指定版本号：

```bash
vsce publish 0.1.0
```

或者发布已打包的 .vsix：

```bash
vsce publish --packagePath aether-lsp-0.1.0.vsix
```

### 方法 2: 手动上传

1. 访问 <https://marketplace.visualstudio.com/manage/publishers/xiaozuhui>
2. 点击 **New extension** → **Visual Studio Code**
3. 上传 `.vsix` 文件
4. 填写扩展信息
5. 点击 **Upload**

## 📦 打包脚本

创建 `vscode-extension/publish.sh`：

```bash
#!/bin/bash
set -e

echo "🔨 构建 LSP 服务器..."
cd ..
cargo build --release

echo "📦 准备扩展..."
cd vscode-extension

# 创建 bin 目录
mkdir -p bin

# 复制二进制文件
echo "📋 复制二进制文件..."
cp ../target/release/aether-lsp bin/

echo "🔧 编译 TypeScript..."
npm run compile

echo "📦 打包扩展..."
vsce package

echo "✅ 完成！"
echo "生成的文件："
ls -lh *.vsix
```

使用：

```bash
chmod +x publish.sh
./publish.sh
```

## 🔄 更新发布

### 更新版本号

```bash
# 补丁版本（0.1.0 → 0.1.1）
vsce publish patch

# 小版本（0.1.0 → 0.2.0）
vsce publish minor

# 大版本（0.1.0 → 1.0.0）
vsce publish major
```

### 手动更新

1. 修改 `package.json` 中的 `version`
2. 更新 `CHANGELOG.md`
3. 运行 `vsce publish`

## ⚠️ 常见问题

### Q1: "ERROR Missing publisher name"

**A**: 确保 `package.json` 中有 `"publisher": "xiaozuhui"`

### Q2: "ERROR Make sure to edit the README.md file"

**A**: 必须创建有实际内容的 `README.md`（不能只是模板）

### Q3: "ERROR Missing license"

**A**: 添加 `"license": "MIT"` 到 `package.json` 并创建 `LICENSE` 文件

### Q4: 二进制文件太大

**A**:

- 使用 `cargo build --release` 构建
- 使用 `strip` 移除调试符号：`strip target/release/aether-lsp`
- 考虑使用 UPX 压缩：`upx --best target/release/aether-lsp`

### Q5: 找不到 LSP 二进制文件

**A**:

- 检查 `.vscodeignore` 没有排除二进制文件
- 使用 `vsce ls` 查看打包的文件列表
- 考虑使用绝对路径或相对路径

### Q6: 跨平台支持

**A**: 需要为每个平台构建：

```bash
# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

然后在 `extension.ts` 中检测平台并使用相应的二进制。

## 📊 发布后

### 1. 监控统计

访问 <https://marketplace.visualstudio.com/manage/publishers/xiaozuhui> 查看：

- 安装量
- 评分
- 反馈

### 2. 推广

- 在 GitHub README 添加市场链接
- 在社交媒体分享
- 在相关论坛发布

### 3. 维护

- 及时回复 Issues
- 定期更新
- 收集用户反馈

## 🔗 有用的链接

- **VSCode 扩展市场**: <https://marketplace.visualstudio.com/>
- **发布者管理**: <https://marketplace.visualstudio.com/manage>
- **官方文档**: <https://code.visualstudio.com/api/working-with-extensions/publishing-extension>
- **vsce 文档**: <https://github.com/microsoft/vscode-vsce>
- **扩展指南**: <https://code.visualstudio.com/api>

## 📝 检查清单

发布前确认：

- [ ] `package.json` 已完善（publisher, license, repository, keywords）
- [ ] 创建了 `README.md`（内容丰富，有示例）
- [ ] 创建了 `CHANGELOG.md`
- [ ] 创建了 `LICENSE` 文件
- [ ] 添加了 `icon.png`（可选）
- [ ] LSP 二进制文件包含在包中
- [ ] 在本地测试过 `.vsix` 文件
- [ ] 版本号正确
- [ ] 所有功能正常工作
- [ ] 没有明显的 bug

## 🎉 快速命令

```bash
# 一键发布（假设已配置）
cd vscode-extension
npm run compile && vsce package && vsce publish

# 或者分步
npm run compile          # 编译 TypeScript
vsce package            # 打包
code --install-extension aether-lsp-0.1.0.vsix  # 本地测试
vsce publish            # 发布
```

祝你发布顺利！🚀
