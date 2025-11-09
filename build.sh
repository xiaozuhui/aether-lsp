#!/bin/bash
set -e

echo "=========================================="
echo "构建 Aether LSP Server"
echo "=========================================="

# 构建 Rust LSP 服务器
echo "📦 编译 LSP 服务器..."
cargo build --release

echo ""
echo "✅ LSP 服务器编译完成: target/release/aether-lsp"

# 构建 VSCode 扩展
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
echo "=========================================="
echo "✅ 构建完成!"
echo "=========================================="
echo ""
echo "下一步:"
echo "1. 在 VSCode 中打开此项目"
echo "2. 按 F5 启动扩展开发主机"
echo "3. 在新窗口中打开 examples/test.aether"
echo "4. 测试语法高亮和自动补全功能"
echo ""
echo "或者直接测试 LSP 服务器:"
echo "  ./target/release/aether-lsp"
