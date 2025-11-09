# Aether LSP Server

基于 Rust + tower-lsp 实现的 Language Server Protocol 服务器。

## 快速开始

### 1. 编译 LSP 服务器

```bash
cargo build --release
```

### 2. 测试服务器

```bash
# 运行测试
cargo test

# 手动测试
./target/release/aether-lsp
```

### 3. 安装 VSCode 扩展

```bash
cd vscode-extension
npm install
npm run compile

# 在 VSCode 中按 F5 启动扩展开发主机
```

### 4. 测试扩展

1. 在扩展开发主机中打开 `examples/test.aether`
2. 查看语法高亮
3. 输入代码查看自动补全
4. 悬停查看函数文档

## 下一步

- [ ] 完善 Parser 集成 (目前使用简化版)
- [ ] 添加跳转到定义功能
- [ ] 实现符号表和语义分析
- [ ] 添加代码格式化
- [ ] 集成更多内置函数文档
