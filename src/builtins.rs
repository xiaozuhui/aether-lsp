//! Built-in functions registry for Aether language
//!
//! This module contains definitions for all 200+ built-in functions

use tower_lsp::lsp_types::*;

pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub examples: &'static [&'static str],
}

/// Get all built-in functions
pub fn get_builtin_functions() -> Vec<BuiltinFunction> {
    vec![
        // === I/O Functions ===
        BuiltinFunction {
            name: "PRINTLN",
            signature: "PRINTLN(value...)",
            description: "打印值到控制台并换行",
            category: "IO",
            examples: &["PRINTLN(\"Hello World\")", "PRINTLN(MY_VAR, MY_VAR2)"],
        },
        BuiltinFunction {
            name: "PRINT",
            signature: "PRINT(value...)",
            description: "打印值到控制台(不换行)",
            category: "IO",
            examples: &["PRINT(\"Result: \")", "PRINT(RESULT)"],
        },
        BuiltinFunction {
            name: "INPUT",
            signature: "INPUT(prompt)",
            description: "读取用户输入",
            category: "IO",
            examples: &["Set NAME INPUT(\"Enter your name: \")"],
        },
        // === Array Functions ===
        BuiltinFunction {
            name: "MAP",
            signature: "MAP(array, function)",
            description: "对数组每个元素应用函数",
            category: "Array",
            examples: &["Set DOUBLED MAP(NUMBERS, Lambda X -> (X * 2))"],
        },
        BuiltinFunction {
            name: "FILTER",
            signature: "FILTER(array, predicate)",
            description: "过滤数组元素",
            category: "Array",
            examples: &["Set EVENS FILTER(NUMBERS, Lambda X -> ((X % 2) == 0))"],
        },
        BuiltinFunction {
            name: "REDUCE",
            signature: "REDUCE(array, function, initial)",
            description: "归约数组为单一值",
            category: "Array",
            examples: &["Set SUM REDUCE(NUMBERS, Lambda (ACC, X) -> (ACC + X), 0)"],
        },
        BuiltinFunction {
            name: "LENGTH",
            signature: "LENGTH(array_or_string)",
            description: "返回数组或字符串的长度",
            category: "Array",
            examples: &["Set LEN LENGTH([1, 2, 3])", "Set STR_LEN LENGTH(\"hello\")"],
        },
        BuiltinFunction {
            name: "PUSH",
            signature: "PUSH(array, element)",
            description: "添加元素到数组末尾",
            category: "Array",
            examples: &["PUSH(MY_ARR, 42)"],
        },
        BuiltinFunction {
            name: "POP",
            signature: "POP(array)",
            description: "移除并返回数组最后一个元素",
            category: "Array",
            examples: &["Set LAST POP(MY_ARR)"],
        },
        BuiltinFunction {
            name: "SORT",
            signature: "SORT(array)",
            description: "排序数组(升序)",
            category: "Array",
            examples: &["Set SORTED SORT([3, 1, 4, 1, 5])"],
        },
        BuiltinFunction {
            name: "REVERSE",
            signature: "REVERSE(array)",
            description: "反转数组",
            category: "Array",
            examples: &["Set REVERSED REVERSE([1, 2, 3])"],
        },
        BuiltinFunction {
            name: "JOIN",
            signature: "JOIN(array, separator)",
            description: "用分隔符连接数组元素为字符串",
            category: "Array",
            examples: &["Set CSV JOIN([\"a\", \"b\", \"c\"], \",\")"],
        },
        BuiltinFunction {
            name: "RANGE",
            signature: "RANGE(start, end)",
            description: "生成数字范围数组",
            category: "Array",
            examples: &["Set NUMS RANGE(1, 10)"],
        },
        BuiltinFunction {
            name: "SUM",
            signature: "SUM(array)",
            description: "计算数组元素总和",
            category: "Array",
            examples: &["Set TOTAL SUM([1, 2, 3, 4, 5])"],
        },
        BuiltinFunction {
            name: "MIN",
            signature: "MIN(array)",
            description: "返回数组最小值",
            category: "Array",
            examples: &["Set MINIMUM MIN([3, 1, 4, 1, 5])"],
        },
        BuiltinFunction {
            name: "MAX",
            signature: "MAX(array)",
            description: "返回数组最大值",
            category: "Array",
            examples: &["Set MAXIMUM MAX([3, 1, 4, 1, 5])"],
        },
        // === String Functions ===
        BuiltinFunction {
            name: "SPLIT",
            signature: "SPLIT(string, separator)",
            description: "分割字符串为数组",
            category: "String",
            examples: &["Set PARTS SPLIT(\"a,b,c\", \",\")"],
        },
        BuiltinFunction {
            name: "UPPER",
            signature: "UPPER(string)",
            description: "转换为大写",
            category: "String",
            examples: &["Set UPPER UPPER(\"hello\")"],
        },
        BuiltinFunction {
            name: "LOWER",
            signature: "LOWER(string)",
            description: "转换为小写",
            category: "String",
            examples: &["Set LOWER LOWER(\"HELLO\")"],
        },
        BuiltinFunction {
            name: "TRIM",
            signature: "TRIM(string)",
            description: "去除首尾空格",
            category: "String",
            examples: &["Set TRIMMED TRIM(\"  hello  \")"],
        },
        BuiltinFunction {
            name: "REPLACE",
            signature: "REPLACE(string, old, new)",
            description: "替换子串",
            category: "String",
            examples: &["Set REPLACED REPLACE(\"hello\", \"l\", \"r\")"],
        },
        BuiltinFunction {
            name: "STARTSWITH",
            signature: "STARTSWITH(string, prefix)",
            description: "检查是否以指定前缀开始",
            category: "String",
            examples: &["Set IS_PREFIX STARTSWITH(\"hello\", \"he\")"],
        },
        BuiltinFunction {
            name: "ENDSWITH",
            signature: "ENDSWITH(string, suffix)",
            description: "检查是否以指定后缀结束",
            category: "String",
            examples: &["Set IS_SUFFIX ENDSWITH(\"hello\", \"lo\")"],
        },
        BuiltinFunction {
            name: "SUBSTRING",
            signature: "SUBSTRING(string, start, length)",
            description: "提取子串",
            category: "String",
            examples: &["Set SUB SUBSTRING(\"hello\", 1, 3)"],
        },
        BuiltinFunction {
            name: "FORMAT",
            signature: "FORMAT(template, args...)",
            description: "格式化字符串",
            category: "String",
            examples: &["Set MSG FORMAT(\"Hello {}, you are {} years old\", NAME, AGE)"],
        },
        // === Math Functions ===
        BuiltinFunction {
            name: "ABS",
            signature: "ABS(number)",
            description: "返回绝对值",
            category: "Math",
            examples: &["Set ABSOLUTE ABS(-5)"],
        },
        BuiltinFunction {
            name: "FLOOR",
            signature: "FLOOR(number)",
            description: "向下取整",
            category: "Math",
            examples: &["Set FLOORED FLOOR(3.7)"],
        },
        BuiltinFunction {
            name: "CEIL",
            signature: "CEIL(number)",
            description: "向上取整",
            category: "Math",
            examples: &["Set CEILED CEIL(3.2)"],
        },
        BuiltinFunction {
            name: "ROUND",
            signature: "ROUND(number)",
            description: "四舍五入",
            category: "Math",
            examples: &["Set ROUNDED ROUND(3.5)"],
        },
        BuiltinFunction {
            name: "SQRT",
            signature: "SQRT(number)",
            description: "计算平方根",
            category: "Math",
            examples: &["Set ROOT SQRT(16)"],
        },
        BuiltinFunction {
            name: "POW",
            signature: "POW(base, exponent)",
            description: "计算幂",
            category: "Math",
            examples: &["Set POWER POW(2, 3)"],
        },
        BuiltinFunction {
            name: "LOG",
            signature: "LOG(number)",
            description: "计算自然对数",
            category: "Math",
            examples: &["Set LN LOG(2.718)"],
        },
        BuiltinFunction {
            name: "LOG10",
            signature: "LOG10(number)",
            description: "计算以10为底的对数",
            category: "Math",
            examples: &["Set LG LOG10(100)"],
        },
        BuiltinFunction {
            name: "SIN",
            signature: "SIN(radians)",
            description: "计算正弦值",
            category: "Math",
            examples: &["Set SINE SIN(1.57)"],
        },
        BuiltinFunction {
            name: "COS",
            signature: "COS(radians)",
            description: "计算余弦值",
            category: "Math",
            examples: &["Set COSINE COS(0)"],
        },
        BuiltinFunction {
            name: "TAN",
            signature: "TAN(radians)",
            description: "计算正切值",
            category: "Math",
            examples: &["Set TANGENT TAN(0.785)"],
        },
        BuiltinFunction {
            name: "RANDOM",
            signature: "RANDOM()",
            description: "生成 0-1 之间的随机数",
            category: "Math",
            examples: &["Set RAND RANDOM()"],
        },
        // === Type Functions ===
        BuiltinFunction {
            name: "TYPE",
            signature: "TYPE(value)",
            description: "返回值的类型字符串",
            category: "Type",
            examples: &["Set T TYPE(42)"],
        },
        BuiltinFunction {
            name: "STRING",
            signature: "STRING(value)",
            description: "转换为字符串",
            category: "Type",
            examples: &["Set STR STRING(42)"],
        },
        BuiltinFunction {
            name: "NUMBER",
            signature: "NUMBER(string_or_value)",
            description: "转换为数字",
            category: "Type",
            examples: &["Set NUM NUMBER(\"42\")"],
        },
        BuiltinFunction {
            name: "ISNUMBER",
            signature: "ISNUMBER(value)",
            description: "检查是否为数字",
            category: "Type",
            examples: &["Set IS_NUM ISNUMBER(42)"],
        },
        BuiltinFunction {
            name: "ISSTRING",
            signature: "ISSTRING(value)",
            description: "检查是否为字符串",
            category: "Type",
            examples: &["Set IS_STR ISSTRING(\"hello\")"],
        },
        BuiltinFunction {
            name: "ISARRAY",
            signature: "ISARRAY(value)",
            description: "检查是否为数组",
            category: "Type",
            examples: &["Set IS_ARR ISARRAY([1, 2])"],
        },
        BuiltinFunction {
            name: "ISDICT",
            signature: "ISDICT(value)",
            description: "检查是否为字典",
            category: "Type",
            examples: &["Set IS_DICT ISDICT({\"key\": \"value\"})"],
        },
        // === Dict Functions ===
        BuiltinFunction {
            name: "KEYS",
            signature: "KEYS(dict)",
            description: "返回字典所有键",
            category: "Dict",
            examples: &["Set ALL_KEYS KEYS(MY_DICT)"],
        },
        BuiltinFunction {
            name: "VALUES",
            signature: "VALUES(dict)",
            description: "返回字典所有值",
            category: "Dict",
            examples: &["Set ALL_VALUES VALUES(MY_DICT)"],
        },
        BuiltinFunction {
            name: "ITEMS",
            signature: "ITEMS(dict)",
            description: "返回键值对数组",
            category: "Dict",
            examples: &["Set PAIRS ITEMS(MY_DICT)"],
        },
        BuiltinFunction {
            name: "HASKEY",
            signature: "HASKEY(dict, key)",
            description: "检查字典是否包含指定键",
            category: "Dict",
            examples: &["Set HAS HASKEY(MY_DICT, \"name\")"],
        },
        // === JSON Functions ===
        BuiltinFunction {
            name: "JSONPARSE",
            signature: "JSONPARSE(json_string)",
            description: "解析JSON字符串",
            category: "JSON",
            examples: &["Set DATA JSONPARSE(\"{\\\"name\\\": \\\"Alice\\\"}\")"],
        },
        BuiltinFunction {
            name: "JSONSTRINGIFY",
            signature: "JSONSTRINGIFY(value)",
            description: "将值转换为JSON字符串",
            category: "JSON",
            examples: &["Set JSON JSONSTRINGIFY(MY_DATA)"],
        },
        // === Date/Time Functions ===
        BuiltinFunction {
            name: "NOW",
            signature: "NOW()",
            description: "返回当前时间戳",
            category: "DateTime",
            examples: &["Set TIMESTAMP NOW()"],
        },
        BuiltinFunction {
            name: "FORMATDATE",
            signature: "FORMATDATE(timestamp, format)",
            description: "格式化时间戳",
            category: "DateTime",
            examples: &["Set DATE_STR FORMATDATE(NOW(), \"%Y-%m-%d\")"],
        },
        BuiltinFunction {
            name: "SLEEP",
            signature: "SLEEP(seconds)",
            description: "暂停执行指定秒数",
            category: "DateTime",
            examples: &["SLEEP(1)"],
        },
    ]
}

/// Convert builtin functions to LSP completion items
pub fn builtin_to_completion_items() -> Vec<CompletionItem> {
    get_builtin_functions()
        .into_iter()
        .map(|func| {
            let detail = format!("{} - {}", func.signature, func.category);
            let doc = format!(
                "{}\n\n**分类**: {}\n\n**示例**:\n```aether\n{}\n```",
                func.description,
                func.category,
                func.examples.join("\n")
            );

            CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(detail),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc,
                })),
                insert_text: Some(format!("{}($1)", func.name)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            }
        })
        .collect()
}
