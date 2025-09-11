// 16_error_handling_result.rs
// 核心内容：详细讲解Result<T, E>枚举，以及如何优雅地处理可恢复的错误，包括?操作符。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 大多数错误并不严重到需要整个程序停止运行。对于这类可恢复的错误，Rust 提供了 `Result<T, E>` 枚举。
 *
 * 1. `Result<T, E>` 枚举
 *    - `Result` 枚举被定义为有两个变体：`Ok` 和 `Err`。
 *      `enum Result<T, E> {`
 *      `    Ok(T),    // 包含了成功时返回的值，类型为 T`
 *      `    Err(E),   // 包含了失败时返回的错误，类型为 E`
 *      `}`
 *    - `T` 和 `E` 是泛型参数：`T` 代表成功时返回值的类型，`E` 代表失败时错误的类型。
 *
 * 2. 处理 `Result`
 *    - 和 `Option` 一样，`Result` 的设计强迫你在编译时就处理可能发生的错误。
 *    - a) `match` 表达式:
 *      这是最基本、最明确的处理方式。你可以为 `Ok(value)` 和 `Err(error)` 两种情况提供不同的处理逻辑。
 *
 *    - b) `unwrap()` 和 `expect()`:
 *      - `unwrap()`: 是一个快捷方法。如果 `Result` 是 `Ok(value)`，它会返回 `value`。
 *        如果 `Result` 是 `Err(error)`，它会 `panic!`。这主要用于示例和测试。
 *      - `expect("message")`: 与 `unwrap()` 类似，但它允许你提供一个自定义的 `panic!` 消息。
 *        这能让错误信息更明确，更容易追踪 bug。
 *
 * 3. `?` 操作符 - 传播错误 (Propagating Errors)
 *    - 当你在一个返回 `Result` 的函数中调用另一个返回 `Result` 的函数时，你通常希望如果内部函数出错了，
 *      外部函数也立即停止并返回这个错误。这个过程称为“传播错误”。
 *    - `?` 操作符就是为此设计的语法糖。
 *    - 如果一个 `Result` 的值是 `Ok(value)`，`?` 操作符会从表达式中提取出 `value`。
 *    - 如果值是 `Err(e)`，`?` 操作符会立即从整个函数中 `return` 这个 `Err(e)` 值。
 *
 *    - **重要**: `?` 操作符只能在返回类型为 `Result`、`Option` 或其他实现了 `Try` trait 的类型的函数中使用。
 *      你不能在 `main` 函数（它返回 `()`）中直接使用 `?`，除非你修改 `main` 函数的签名。
 *
 * 4. `?` 操作符与 `Option<T>`
 *    - `?` 操作符也可以用于 `Option<T>`。
 *    - 如果 `Option` 是 `Some(value)`，它会返回 `value`。
 *    - 如果 `Option` 是 `None`，它会立即从函数返回 `None`。
 *
 * 5. 何时使用 `panic!` vs `Result`
 *    - 这是一个关键的设计决策。
 *    - 如果一个函数可能因为调用者应该预料到并处理的原因而失败（例如，解析一个格式错误的字符串），
 *      那么返回 `Result` 是合适的。
 *    - 如果失败意味着调用者代码中有 bug，或者程序进入了一种无法修复的无效状态，
 *      那么 `panic!` 更合适。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
// 练习2：
fn main() -> Result<(), Box<dyn Error>> {
    // // 2. 处理 Result
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         // 使用 match 来处理不同的错误类型
    //         match error.kind() {
    //             std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Problem creating the file: {:?}", e),
    //             },
    //             other_error => {
    //                 panic!("Problem opening the file: {:?}", other_error);
    //             }
    //         }
    //     }
    // };

    // 使用 unwrap() 和 expect()
    // let f_unwrap = File::open("hello.txt").unwrap(); // 如果文件不存在会 panic
    // let f_expect = File::open("hello.txt").expect("Failed to open hello.txt"); // 提供自定义 panic 消息

    // 3. 演示传播错误
    // match read_username_from_file() {
    //     Ok(username) => println!("Username from file: {}", username),
    //     Err(e) => println!("Error reading username: {}", e),
    // }

    // 练习1:
    match parse_positive_integer("100") {
        Ok(number) => println!("  => 成功! 解析出的正整数是: {}", number),
        Err(e) => println!("  => 失败! 错误信息是: {}", e),
    }

    // 练习2：
    read_username_from_file ()?;
    Ok(())

    
}

// 这是一个返回 Result 的函数
// `?` 操作符让代码非常简洁
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?; // 如果 open 失败，? 会立即返回 Err
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 如果 read_to_string 失败，? 会立即返回 Err
    Ok(s) // 如果一切顺利，返回 Ok(s)
}

// 上面的函数可以被链式调用写得更短
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 最短的版本：使用标准库函数
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

// 练习1：
fn parse_positive_integer(s :&str) -> Result<i32, String> {
    // 1.调用parse()然后用match处理返回的Result
    match s.parse::<i32>() {
        // 2.如果解析成功进入OK分支，直接返回数字
        Ok(num) => {
            // 3. 检查数字是否为正数
            if num > 0 {
                // 4. 如果是正数，返回一个包裹着 num 的 Ok
                Ok(num)
            } else {
                // 5. 如果不是正数，返回一个包含错误信息的 Err
                Err(format!("解析成功，但数字 '{}' 不是正数。", num))
            }
        }
        // 6. 如果解析失败，进入 Err 分支
        Err(_) => {
            // 7. 返回一个包含通用错误信息的 Err
            Err(format!("解析失败：'{}' 不是一个有效的整数。", s))
        }
    }
}
/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 解析字符串为整数:
 *    编写一个函数 `parse_positive_integer`，它接收一个 `&str`。
 *    - 尝试将字符串解析为一个 `i32` (可以使用 `s.parse::<i32>()`)。
 *    - 如果解析成功，检查这个数字是否是正数。
 *    - 如果是正数，返回 `Ok(number)`。
 *    - 如果解析失败，或者数字不是正数，返回一个 `Err`，其中包含一个描述性 `String` 错误信息。
 *    在 `main` 中用有效和无效的输入来测试这个函数。
 *
 * 2. 修改 `main` 函数的返回类型:
 *    为了能在 `main` 函数中使用 `?` 操作符，你可以将其签名修改为 `fn main() -> Result<(), Box<dyn std::error::Error>>`。
 *    `Box<dyn std::error::Error>` 是一种“trait 对象”，可以代表任何类型的错误。
 *    修改 `main` 函数的签名，然后在 `main` 中直接调用 `read_username_from_file()?` 并打印结果，
 *    体会 `?` 带来的便利。
 *
 */