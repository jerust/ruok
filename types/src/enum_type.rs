#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::f64::consts;
use std::fmt;
use std::hash::Hash;

enum HttpStatusCode {
    SwitchProtocol, // 如果不显式分配第一个变体的值, 那么它的值等于0
    NotFound = 404,
    GatewayTimeout, // 如果不显式分配非第一个变体的值, 那么它的值等于上一个变体的值加1
}

enum SchedulerState<Job, Pid>
where
    Job: Eq + Hash,
    Pid: Eq + Hash,
{
    Pending(HashSet<Job>),
    Running(HashMap<Pid, Vec<Job>>),
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

#[repr(u8)] // 指定枚举变体的底层类型为u8
enum ProgramLanguage<'a> {
    Rust = 1,
    Java = 2,
    Rest(&'a str, usize),
}

impl<'a> fmt::Display for ProgramLanguage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust => write!(f, "Top1: Rust"),
            Self::Java => write!(f, "Top2: Java"),
            Self::Rest(lang, rank) => write!(f, "Top{}: {}", rank, lang),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpStatusCode, ProgramLanguage};

    #[test]
    fn http_status_code() {
        let gateway_timeout: HttpStatusCode = HttpStatusCode::GatewayTimeout;
        assert_eq!(gateway_timeout as u8, 149); // 405 - 256 = 149
    }

    #[test]
    fn program_language() {
        let rust: ProgramLanguage = ProgramLanguage::Rust;
        println!("{}", rust); // Top1: Rust

        let ruok: ProgramLanguage = ProgramLanguage::Rest("Ruok", 3);
        println!("{}", ruok); // Top3: Ruok

        // as运算符只能用于将没有任何字段的变体转换为数字类型, 以下操作无法通过编译
        // ProgramLanguage::Java as u8;
    }
}
