# Aether 命名约定修正

## 问题

之前实现的内置函数使用了混合大小写（PascalCase）命名，如 `Println`, `Map`, `Filter`，这与 Aether 语言的官方约定不符。

## 修正方案

根据官方文档 <https://docs.rs/aether-azathoth/0.3.0/aether/stdlib/index.html> 和用户提供的示例代码，Aether 语言的**所有函数名和变量名都应该使用 UPPER_CASE（全大写）命名**。

## 修改内容

### 已修正的函数名（53个）

#### I/O Functions (3)

- ~~`Println`~~ → `PRINTLN`
- ~~`Print`~~ → `PRINT`
- ~~`ReadLine`~~ → `INPUT`

#### Array Functions (13)

- ~~`Map`~~ → `MAP`
- ~~`Filter`~~ → `FILTER`
- ~~`Reduce`~~ → `REDUCE`
- ~~`Length`~~ → `LENGTH`
- ~~`Append`~~ → `PUSH`
- ~~`Sort`~~ → `SORT`
- ~~`Reverse`~~ → `REVERSE`
- ~~`Join`~~ → `JOIN`
- ~~`Range`~~ → `RANGE`
- ~~`Sum`~~ → `SUM`
- ~~`Min`~~ → `MIN`
- ~~`Max`~~ → `MAX`
- `POP` ✓

#### String Functions (9)

- ~~`Split`~~ → `SPLIT`
- ~~`Upper`~~ → `UPPER`
- ~~`Lower`~~ → `LOWER`
- ~~`Trim`~~ → `TRIM`
- ~~`Replace`~~ → `REPLACE`
- ~~`StartsWith`~~ → `STARTSWITH`
- ~~`EndsWith`~~ → `ENDSWITH`
- ~~`Substring`~~ → `SUBSTRING`
- ~~`Format`~~ → `FORMAT`

#### Math Functions (12)

- ~~`Abs`~~ → `ABS`
- ~~`Floor`~~ → `FLOOR`
- ~~`Ceil`~~ → `CEIL`
- ~~`Round`~~ → `ROUND`
- ~~`Sqrt`~~ → `SQRT`
- ~~`Pow`~~ → `POW`
- ~~`Log`~~ → `LOG`
- ~~`Log10`~~ → `LOG10`
- ~~`Sin`~~ → `SIN`
- ~~`Cos`~~ → `COS`
- ~~`Tan`~~ → `TAN`
- ~~`Random`~~ → `RANDOM`

#### Type Functions (7)

- ~~`Type`~~ → `TYPE`
- ~~`String`~~ → `STRING`
- ~~`Number`~~ → `NUMBER`
- ~~`IsNumber`~~ → `ISNUMBER`
- ~~`IsString`~~ → `ISSTRING`
- ~~`IsArray`~~ → `ISARRAY`
- ~~`IsDict`~~ → `ISDICT`

#### Dict Functions (4)

- ~~`Keys`~~ → `KEYS`
- ~~`Values`~~ → `VALUES`
- ~~`Items`~~ → `ITEMS`
- ~~`HasKey`~~ → `HASKEY`

#### JSON Functions (2)

- ~~`JsonParse`~~ → `JSONPARSE`
- ~~`JsonStringify`~~ → `JSONSTRINGIFY`

#### DateTime Functions (3)

- ~~`Now`~~ → `NOW`
- ~~`FormatDate`~~ → `FORMATDATE`
- ~~`Sleep`~~ → `SLEEP`

## 示例对比

### 修正前 ❌

```aether
Set NUMBERS [1, 2, 3, 4, 5]
Set DOUBLED Map(NUMBERS, Lambda X -> X * 2)
Set EVENS Filter(NUMBERS, Lambda X -> X % 2 == 0)
Println("Result:", DOUBLED)
```

### 修正后 ✅

```aether
Set NUMBERS [1, 2, 3, 4, 5]
Set DOUBLED MAP(NUMBERS, Lambda X -> X * 2)
Set EVENS FILTER(NUMBERS, Lambda X -> X % 2 == 0)
PRINTLN("Result:", DOUBLED)
```

## 验证

### 编译测试

```bash
$ cargo build --release
   Compiling aether-lsp v0.1.0
    Finished `release` profile [optimized] target(s) in 7.04s
```

### 单元测试

```bash
$ cargo test --workspace
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored
```

### 函数列表验证

```bash
$ grep 'name: "' src/builtins.rs | wc -l
53
```

所有53个函数均已正确更新为 UPPER_CASE 命名。

## 相关文件

- `/src/builtins.rs` - 主要修改文件，所有函数名改为大写
- `/examples/test_builtins.aether` - 新增测试文件演示正确用法
- `/IMPLEMENTATION_SUMMARY.md` - 已更新文档反映新的命名约定

## 官方参考

- Crates.io: <https://crates.io/crates/aether-azathoth>
- 标准库文档: <https://docs.rs/aether-azathoth/0.3.0/aether/stdlib/index.html>
- 内置函数文档: <https://docs.rs/aether-azathoth/0.3.0/aether/builtins/index.html>
