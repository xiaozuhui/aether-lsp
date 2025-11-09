#!/bin/bash
# 打包 VSCode 扩展为 .vsix 文件

set -e

echo "=========================================="
echo "打包 Aether LSP VSCode 扩展"
echo "=========================================="

# 检查是否安装了 vsce
if ! command -v vsce &> /dev/null; then
    echo "❌ vsce 未安装"
    echo "📥 正在安装 @vscode/vsce..."
    npm install -g @vscode/vsce
fi

# 构建 LSP 服务器
echo ""
echo "📦 构建 LSP 服务器..."
cargo build --release

echo ""
echo "✅ LSP 服务器编译完成"

# 构建并打包扩展
echo ""
echo "📦 构建 VSCode 扩展..."
cd vscode-extension

if [ ! -d "node_modules" ]; then
    echo "📥 安装 npm 依赖..."
    npm install
fi

echo "🔨 编译 TypeScript..."
npm run compile

echo ""
echo "📦 打包扩展为 .vsix..."
vsce package

VSIX_FILE=$(ls -t *.vsix | head -1)

echo ""
echo "=========================================="
echo "✅ 打包完成!"
echo "=========================================="
echo ""
echo "生成的文件: vscode-extension/$VSIX_FILE"
echo ""
echo "安装方法:"
echo "  1. 命令行安装:"
echo "     code --install-extension vscode-extension/$VSIX_FILE"
echo ""
echo "  2. VSCode UI 安装:"
echo "     - 打开 VSCode"
echo "     - Cmd+Shift+X 打开扩展面板"
echo "     - 点击 ... → 从 VSIX 安装..."
echo "     - 选择 $VSIX_FILE"
echo ""
echo "  3. 开发模式测试:"
echo "     - 按 F5 启动扩展开发主机"
echo ""
