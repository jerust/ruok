//! 函数指针

#![allow(dead_code)]

use std::cmp::Ordering;

/// 代码复用
///
/// 接收两个整数和一个函数指针作为参数, 可以在不同的上下文中复用这个通用逻辑, 而不需要为每种操作编写新的函数
fn calculator(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

/// 动态行为
///
/// 函数指针允许动态决定在某个操作中执行哪个函数, 在需要动态配置的场景中非常有用, 例如, 排序算法通过函数指针来选择不同的比较函数
fn sorter(sequence: &mut [i32], compare: fn(&i32, &i32) -> Ordering) {
    sequence.sort_by(compare)
}

fn ascending(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}

fn descending(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master() {
        // 函数项和函数指针的区别

        // 函数项
        let fni = sum;
        // 函数项实现了Copy特征
        let op1 = fni;
        let op2 = fni;
        // 函数项没有实现Eq特征, 不能用于比较
        // ❎ op1 == op2
        // 函数项没有实现Pointer特征, 不能打印地址
        // ❎ println!("{:p}", op1);
        // ❎ println!("{:p}", op2);
        // 函数项调用
        assert_eq!(op1(1, 2), op2(2, 1));

        // 函数指针, 通过类型注释强制将函数项转换成函数指针
        let fnp: fn(i32, i32) -> i32 = sum;
        // 函数指针实现了Copy特征
        let op1 = fnp;
        let op2 = fnp;
        // 函数指针实现了Eq特征, 可以用于比较
        assert_eq!(op1, op2);
        // 函数指针实现了Pointer特征, 可以打印地址
        // ✅ println!("{:p}", op1); // 0x1029d69f0
        // ✅ println!("{:p}", op2); // 0x1029d69f0
        // 函数指针调用
        assert_eq!(op1(1, 2), op2(2, 1));
    }

    #[test]
    fn code_reuse() {
        assert_eq!(calculator(0, 0, sum), calculator(0, 0, sub));
    }

    #[test]
    fn dynamic_behavior() {
        let mut sequences: [i32; 6] = [3, 1, 4, 1, 5, 9];

        sorter(&mut sequences, ascending);
        assert_eq!(sequences, [1, 1, 3, 4, 5, 9]);

        sorter(&mut sequences, descending);
        assert_eq!(sequences, [9, 5, 4, 3, 1, 1]);

        sequences.sort_by(|a, b| a.cmp(b));
        assert_eq!(sequences, [1, 1, 3, 4, 5, 9]);
    }
}
