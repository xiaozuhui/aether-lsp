//! Completion provider for Aether language

use crate::builtins;
use crate::parser::ParsedDocument;
use tower_lsp::lsp_types::*;

pub fn get_completions(_doc: &ParsedDocument, _position: Position) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // 关键字补全
    completions.extend(get_keyword_completions());

    // 内置函数补全 (从 builtins 模块自动生成)
    completions.extend(builtins::builtin_to_completion_items());

    // TODO: 从文档符号表添加用户定义的函数和变量
    // completions.extend(doc.symbols.to_completion_items());

    completions
}

/// Get keyword completions
fn get_keyword_completions() -> Vec<CompletionItem> {
    let keywords = vec![
        ("Set", "变量赋值", "Set VAR value"),
        ("Func", "函数定义", "Func NAME(params) { ... }"),
        ("Return", "返回值", "Return value"),
        ("If", "条件判断", "If (condition) { ... }"),
        ("Elif", "否则如果", "Elif (condition) { ... }"),
        ("Else", "否则", "Else { ... }"),
        ("While", "循环", "While (condition) { ... }"),
        ("For", "遍历", "For VAR In collection { ... }"),
        ("In", "循环关键字", "For X In [1,2,3] { ... }"),
        ("Break", "跳出循环", "Break"),
        ("Continue", "继续下一次循环", "Continue"),
        ("Generator", "生成器定义", "Generator NAME(params) { ... }"),
        ("Yield", "生成值", "Yield value"),
        ("Lazy", "惰性求值", "Lazy NAME(expr)"),
        ("Force", "强制求值", "Force(lazy_value)"),
        ("Switch", "分支", "Switch (value) { Case x: ... }"),
        ("Case", "分支情况", "Case value: statements"),
        ("Default", "默认分支", "Default: statements"),
        ("Import", "导入模块", "Import {NAME} From \"path\""),
        ("Export", "导出符号", "Export NAME"),
        ("From", "导入来源", "Import X From \"path\""),
        ("As", "别名", "Import X As Y From \"path\""),
        ("Lambda", "匿名函数", "Lambda X -> expr"),
        ("True", "布尔真", "True"),
        ("False", "布尔假", "False"),
        ("Null", "空值", "Null"),
    ];

    keywords
        .into_iter()
        .map(|(keyword, desc, example)| CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(desc.to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**{}**\n\n{}\n\n```aether\n{}\n```", keyword, desc, example),
            })),
            insert_text: Some(keyword.to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect()
}
