#!/bin/bash
# VSCode Extension Publishing Script

set -e

echo "🚀 Aether LSP 扩展发布脚本"
echo "================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否在正确的目录
if [ ! -f "../Cargo.toml" ]; then
    echo -e "${RED}错误: 请在 vscode-extension 目录运行此脚本${NC}"
    exit 1
fi

echo -e "${YELLOW}步骤 1/6: 构建 LSP 服务器 (Release)${NC}"
cd ..
cargo build --release
cd vscode-extension

echo ""
echo -e "${YELLOW}步骤 2/6: 准备二进制文件${NC}"
mkdir -p bin
echo "复制 macOS 二进制文件..."
cp ../target/release/aether-lsp bin/

# 如果存在 Windows 二进制，也复制
if [ -f "../target/release/aether-lsp.exe" ]; then
    echo "复制 Windows 二进制文件..."
    cp ../target/release/aether-lsp.exe bin/
fi

echo ""
echo -e "${YELLOW}步骤 3/6: 安装依赖${NC}"
npm install

echo ""
echo -e "${YELLOW}步骤 4/6: 编译 TypeScript${NC}"
npm run compile

echo ""
echo -e "${YELLOW}步骤 5/6: 检查必需文件${NC}"
MISSING_FILES=()

if [ ! -f "README.md" ]; then
    MISSING_FILES+=("README.md")
fi

if [ ! -f "LICENSE" ]; then
    MISSING_FILES+=("LICENSE")
fi

if [ ! -f "CHANGELOG.md" ]; then
    echo -e "${YELLOW}警告: 建议创建 CHANGELOG.md${NC}"
fi

if [ ! -f "icon.png" ]; then
    echo -e "${YELLOW}警告: 建议添加 icon.png (128x128)${NC}"
fi

if [ ${#MISSING_FILES[@]} -ne 0 ]; then
    echo -e "${RED}错误: 缺少必需文件:${NC}"
    for file in "${MISSING_FILES[@]}"; do
        echo "  - $file"
    done
    echo ""
    echo "请参考 PUBLISH_GUIDE.md 创建这些文件"
    exit 1
fi

# 检查 package.json 中的必需字段
echo "检查 package.json..."
if ! grep -q '"publisher"' package.json; then
    echo -e "${RED}错误: package.json 缺少 'publisher' 字段${NC}"
    exit 1
fi

if ! grep -q '"license"' package.json; then
    echo -e "${RED}错误: package.json 缺少 'license' 字段${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}步骤 6/6: 打包扩展${NC}"

# 检查是否安装了 vsce
if ! command -v vsce &> /dev/null; then
    echo -e "${RED}错误: vsce 未安装${NC}"
    echo "运行: npm install -g @vscode/vsce"
    exit 1
fi

vsce package

echo ""
echo -e "${GREEN}✅ 打包完成!${NC}"
echo ""
echo "生成的文件:"
ls -lh *.vsix | tail -1

echo ""
echo -e "${GREEN}下一步:${NC}"
echo "1. 本地测试: code --install-extension aether-lsp-*.vsix"
echo "2. 发布到市场: vsce publish"
echo ""
echo "查看完整指南: ../PUBLISH_GUIDE.md"
