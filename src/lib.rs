pub mod expr;
pub mod formatter;
pub mod lexer;
pub mod parser;
pub mod syntax;

// use crate::lexer::RawToken;
// use logos::Logos;
// use serde::Deserialize;
// use std::io::Write;

// //Config File Definition
// #[derive(Debug, Deserialize, Default)]
// pub struct Config {
//     #[serde(default)]
//     indent_style: IndentStyle,
// }

// #[derive(Deserialize, Debug)]
// pub struct IndentStyle {
//     indent_width: usize,
//     column_width: usize,
// }

// impl Default for IndentStyle {
//     fn default() -> Self {
//         IndentStyle {
//             indent_width: 4,
//             column_width: 92,
//         }
//     }
// }
