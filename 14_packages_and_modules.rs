// 14_packages_and_modules.rs
// 核心内容：如何使用mod组织代码，use关键字的用法，以及如何将模块分散到不同文件中。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * Rust 提供了一套强大的模块系统，用于将代码组织成逻辑单元，并控制代码的可见性（公有/私有）。
 *
 * 核心术语：
 * - 包 (Package): Cargo 的一个功能，允许你构建、测试和分享 crate。一个包包含一个或多个 crate。
 * - Crate: 一个编译单元，是 Rust 中最小的代码编译单位。一个 crate 可以生成一个可执行文件或一个库。
 * - 模块 (Module): 在一个 crate 内部，你可以使用模块来组织和划分代码，控制作用域和隐私。
 *
 * 1. `mod` 关键字 - 定义模块
 *    - `mod` 关键字用于定义一个新的模块。模块可以嵌套。
 *    - 模块形成了一个层级结构，称为“模块树”（module tree）。Crate 的根（通常是 `main.rs` 或 `lib.rs`）是这个树的根。
 *
 * 2. 路径 (Paths) - 引用模块中的项
 *    - 你可以使用路径来引用另一个模块中的代码项（函数、结构体等）。
 *    - 绝对路径: 从 crate 根开始，使用 `crate::` 开头。
 *    - 相对路径: 从当前模块开始，使用 `self::` 或 `super::` (父模块)。
 *
 * 3. `pub` 关键字 - 控制隐私
 *    - 默认情况下，模块中的所有项（函数、结构体、枚举、模块等）都是私有的（private）。
 *    - 这意味着父模块无法访问子模块的私有项。
 *    - 使用 `pub` 关键字可以使一个项变为公有的（public），从而允许外部代码访问它。
 *    - 对于 `struct`，即使结构体本身是 `pub`，它的字段默认也是私有的。你需要单独将字段标记为 `pub`。
 *    - 对于 `enum`，如果枚举是 `pub`，它的所有变体也都是 `pub`。
 *
 * 4. `use` 关键字 - 将路径引入作用域
 *    - 每次都写长长的路径会很繁琐。`use` 关键字可以将一个路径引入当前的本地作用域。
 *    - 引入后，你就可以直接使用该项，而无需写出其完整路径。
 *    - 惯用法：
 *      - 对于函数，通常直接引入到函数级别：`use crate::front_of_house::hosting::add_to_waitlist;`
 *      - 对于结构体、枚举和其他项，通常引入到其父模块级别：`use std::collections::HashMap;`
 *    - `as` 关键字：如果引入的两个类型同名，可以使用 `as` 来提供一个新的本地名称（别名）。
 *    - `pub use`: 重导出（re-exporting）。这使得外部代码可以通过你的模块路径来访问一个你引入的项。
 *
 * 5. 将模块分散到不同文件中
 *    - 当模块变得很大时，你可能想把它们放到单独的文件中。
 *    - 如果你有一个名为 `front_of_house` 的模块，你可以将其代码放入 `src/front_of_house.rs` 文件中。
 *    - 在 `main.rs` (或 `lib.rs`) 中，你只需要声明模块：`mod front_of_house;`
 *    - Rust 编译器会自动查找并加载 `src/front_of_house.rs` 文件。
 *    - 如果模块还有子模块，例如 `hosting`，你可以创建 `src/front_of_house/hosting.rs` 文件。
 *      或者，如果 `front_of_house` 模块本身有代码，你可以创建 `src/front_of_house/mod.rs` 文件来存放它。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 假设这是 `main.rs` 或 `lib.rs` (crate root)

// 这是一个名为 `front_of_house` 的模块
mod front_of_house {
    // 模块 `hosting` 是 `front_of_house` 的子模块
    // `pub` 使得外部可以访问 `hosting` 模块
    pub mod hosting {
        // `pub` 使得外部可以调用 `add_to_waitlist` 函数
        pub fn add_to_waitlist() {
            println!("Added to waitlist.");
            // 可以调用同模块下的私有函数
            seat_at_table();
        }

        fn seat_at_table() {
            println!("Seated at table.");
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// `use` 关键字将 `add_to_waitlist` 函数的路径引入作用域
// 这是绝对路径
use crate::front_of_house::hosting::add_to_waitlist;
// 也可以使用相对路径 `use self::front_of_house::hosting::add_to_waitlist;`

fn eat_at_restaurant() {
    // 1. 使用绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 2. 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();

    // 3. 因为我们上面 `use` 了，所以可以直接调用
    add_to_waitlist();
}

// --- 另一个例子：结构体和枚举的隐私 ---
mod back_of_house {
    // 结构体本身是 pub
    pub struct Breakfast {
        // 字段也必须是 pub 才能在外部访问
        pub toast: String,
        seasonal_fruit: String, // 这个字段是私有的
    }

    impl Breakfast {
        // 我们需要一个公有的关联函数来创建实例，因为 `seasonal_fruit` 是私有的
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 如果枚举是 pub，它的所有变体都是 pub
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn order_food() {
    // 实例化 back_of_house::Breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 可以访问公有字段
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 不能访问私有字段
    // meal.seasonal_fruit = String::from("blueberries"); // 这会报错！

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {
    eat_at_restaurant();
    order_food();
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 拆分文件:
 *    - 创建一个新的 Cargo 项目。
 *    - 将 `front_of_house` 模块的代码移动到 `src/front_of_house.rs` 文件中。
 *    - 在 `src/main.rs` 中，使用 `mod front_of_house;` 来声明并加载这个模块。
 *    - 确保程序仍然可以编译和运行。
 *
 * 2. 创建子模块文件:
 *    - 在 `src/front_of_house.rs` 的基础上，将 `hosting` 模块的代码移动到
 *      `src/front_of_house/hosting.rs` 文件中。
 *    - 你需要在 `src/front_of_house.rs` 中使用 `pub mod hosting;` 来声明它。
 *    - 再次确认程序可以正常工作。这个练习能帮助你理解多层级的文件组织方式。
 * 这个练习需要自己做，主要是关于包管理的，代码无法呈现，详情看https://github.com/EurekaO-O/Rust-learn，第14小节的学习
 */
// 最终的main.rs code(只需要复制然后测试能不能跑通):
// 练习1&练习2：
// mod front_of_house;
// use crate::front_of_house::hosting::add_to_waitlist;
// // 也可以使用相对路径 `use self::front_of_house::hosting::add_to_waitlist;`

// fn eat_at_restaurant() {
//     // 1. 使用绝对路径调用
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 2. 使用相对路径调用
//     front_of_house::hosting::add_to_waitlist();

//     // 3. 因为我们上面 `use` 了，所以可以直接调用
//     add_to_waitlist();
// }

// // --- 另一个例子：结构体和枚举的隐私 ---
// mod back_of_house {
//     // 结构体本身是 pub
//     pub struct Breakfast {
//         // 字段也必须是 pub 才能在外部访问
//         pub toast: String,
//         seasonal_fruit: String, // 这个字段是私有的
//     }

//     impl Breakfast {
//         // 我们需要一个公有的关联函数来创建实例，因为 `seasonal_fruit` 是私有的
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     // 如果枚举是 pub，它的所有变体都是 pub
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// fn order_food() {
//     // 实例化 back_of_house::Breakfast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 可以访问公有字段
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // 不能访问私有字段
//     // meal.seasonal_fruit = String::from("blueberries"); // 报错

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// fn main() {
//     eat_at_restaurant();
//     order_food();
// }