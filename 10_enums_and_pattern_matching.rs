// 10_enums_and_pattern_matching.rs
// 核心内容：讲解枚举的定义，特别是强大的match表达式如何与枚举结合，处理所有可能的情况。介绍Option枚举。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 枚举（Enumerations）是一种允许你通过列出所有可能的“变体”（variants）来定义一个类型的方式。
 *
 * 1. 定义枚举
 *    - 我们使用 `enum` 关键字，后跟枚举的名字，然后是花括号内所有可能的变体。
 *    - 例如: `enum IpAddrKind { V4, V6 }`
 *
 * 2. 枚举值和数据
 *    - Rust的枚举非常强大，因为每个变体都可以选择性地存储关联的数据。
 *    - 这使得我们可以将数据直接附加到枚举的变体中，而无需依赖结构体。
 *    - 例如: `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(i32, i32, i32) }`
 *    - 这个 `Message` 枚举有四个变体：
 *      - `Quit`: 无任何关联数据。
 *      - `Move`: 包含一个匿名的结构体。
 *      - `Write`: 包含一个 `String`。
 *      - `ChangeColor`: 包含三个 `i32` 值。
 *
 * 3. `Option<T>` 枚举 - Rust如何处理空值
 *    - `Option<T>` 是一个由标准库定义的非常重要的枚举。它被用来处理一个值“可能为空”的情况。
 *    - 这是Rust避免其他语言中常见的 `null` 或 `nil` 带来的“十亿美元错误”的解决方案。
 *    - `Option<T>` 的定义大致如下：
 *      `enum Option<T> {`
 *      `    None,       // 表示没有值`
 *      `    Some(T),    // 表示有一个 T 类型的值`
 *      `}`
 *    - `T` 是一个泛型参数，我们后面会学到。现在你只需要知道 `Option<i32>` 可以是 `Some(5)` 或 `None`。
 *    - 这种设计的优点是，编译器会强制你处理 `None` 的情况，从而防止你像使用 `null` 一样，
 *      在没有检查的情况下就使用一个可能为空的值，导致运行时错误。
 *
 * 4. `match` 控制流表达式
 *    - `match` 是Rust中一个极其强大的控制流结构。它允许你将一个值与一系列的“模式”（patterns）进行比较，
 *      并根据匹配的模式执行相应的代码。
 *    - `match` 就像一个类固醇版的 `switch` 语句。
 *
 * 5. `match` 的穷尽性检查 (Exhaustiveness Checking)
 *    - `match` 最重要的特性是它是“穷尽的”（exhaustive）。这意味着你必须覆盖所有可能的情况。
 *    - 对于一个枚举，你必须为它的每一个变体都提供一个匹配分支（arm）。
 *    - 这是一项巨大的安全保障：编译器确保你在编译时就已经处理了所有可能发生的情况。
 *
 * 6. `_` 通配符 (Catch-all Pattern)
 *    - 如果你不想列出所有可能的变体（比如只想处理少数几个），可以使用 `_` 通配符。
 *    - `_` 会匹配任何未被指定的值，并且它不会将值绑定到变量。
 *    - `other` 变量名也可以用作通配符，但它会将值绑定到 `other` 变量上。
 *
 * 7. `if let` 简洁控制流
 *    - 有时，你只关心 `match` 中的某一个分支，而其他情况都做同样的处理。
 *    - `if let` 语法可以看作是 `match` 的一种语法糖，它允许你以一种不那么冗长的方式来处理这种情况。
 *    - `if let Some(value) = an_option { ... }` 等价于 `match an_option { Some(value) => ..., None => (), }`
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 1. 定义一个枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 变体包含一个 UsState 类型的数据
}

// 另一个枚举，用于 Quarter
#[derive(Debug)] // 这个注解让我们能打印出枚举
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

// 4. 使用 match 的函数
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // match 分支可以是一个代码块
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // `state` 变量绑定了 Quarter 变体中的 UsState 值
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 3. 使用 Option<T> 和 match
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // `i` 绑定了 Some 中的 i32 值
    }
}

fn main() {
    let my_coin = Coin::Penny;
    println!("Value is: {}", value_in_cents(my_coin));

    let my_quarter = Coin::Quarter(UsState::Alaska);
    println!("Value is: {}", value_in_cents(my_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("5 plus one is: {:?}", six); // 打印 Some(6)
    println!("None plus one is: {:?}", none); // 打印 None

    // 6. 使用 `_` 通配符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // `_` 匹配所有其他情况 (1, 2, 4, 5, 6, 8, 9, ...)
    }

    // 7. 使用 `if let`
    let config_max = Some(3u8);
    // 如果 config_max 是 Some(max)，则将值绑定到 max 并执行代码块
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // 这比写下面的 match 更简洁：
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // 练习2：
    let some_string: Option<String> = Some(String::from("testSome"));
    // 匹配some的形式
    if let Some(s) = some_string {
        println!("\t字符串的长度是: {}", s.len());
    }

    // 匹配None
    let none_string: Option<String> = None;
    // 匹配None的形式
    if let Some(s) = none_string{
        println!("匹配失败，不会被打印")
    }else {
        println!("\t没有匹配，因为值为None")
    }
    println!("\n-----------------------------------\n");
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

// 练习1：
enum TrafficLight{
    Red,
    Yellow,
    Green
}
fn get_duration(t1:TrafficLight) -> u8{
    match t1 {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 3,
        TrafficLight::Green => 45
    }
}
/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 定义并匹配枚举:
 *    创建一个名为 `TrafficLight` 的枚举，它有三个变体：`Red`, `Yellow`, `Green`。
 *    编写一个函数 `get_duration`，它接收一个 `&TrafficLight` 的引用，并使用 `match` 表达式
 *    返回每种灯应该持续的时间（`u8` 类型，单位秒），例如：红灯60秒，黄灯3秒，绿灯45秒。
 *
 * 2. 使用 `if let` 处理 `Option`:
 *    创建一个 `Option<String>` 类型的变量。
 *    如果变量是 `Some(s)`，使用 `if let` 打印出字符串 `s` 的长度。
 *    如果变量是 `None`，则什么也不做。
 *    尝试用 `Some` 和 `None` 两种情况来测试你的代码。
 *
 */