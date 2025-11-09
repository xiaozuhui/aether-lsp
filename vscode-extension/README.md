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
