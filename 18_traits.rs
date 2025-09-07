// 18_traits.rs
// 核心内容：[Rust核心] 讲解Trait（类似于接口），如何定义、实现和使用Trait来定义共享行为。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * Trait 是 Rust 中实现代码复用和抽象的核心机制。它定义了一组方法签名，用于描述某种类型所能提供的行为。
 *
 * 1. 定义 Trait
 *    - 我们使用 `trait` 关键字来定义一个 trait，后面跟着 trait 的名字。
 *    - 在 `{}` 内部，我们声明方法签名，这些签名描述了实现该 trait 的类型所必须拥有的行为。
 *    - 例如: `pub trait Summary { fn summarize(&self) -> String; }`
 *
 * 2. 为类型实现 Trait
 *    - 我们使用 `impl TraitName for TypeName` 语法来为某个具体的类型实现 trait。
 *    - 在 `impl` 块中，我们必须提供 trait 中定义的所有方法的具体实现。
 *    - 实现 trait 的一个重要规则是“孤儿规则”（Orphan Rule）：
 *      你只能为你自己的 crate 中定义的 trait 或类型实现 trait。
 *      即，你不能为外部 crate 的类型（如 `Vec`）实现一个外部 crate 的 trait（如 `Display`）。
 *      这个规则确保了外部 crate 的代码不会破坏你的代码，反之亦然。
 *
 * 3. Trait 作为参数 (Trait Bounds)
 *    - 我们可以使用 trait 来约束函数参数的类型。这使得函数可以接受任何实现了特定 trait 的类型。
 *    - `pub fn notify(item: &impl Summary) { ... }`
 *      这表示 `item` 参数可以是任何实现了 `Summary` trait 的类型的引用。
 *    - `pub fn notify<T: Summary>(item: &T) { ... }`
 *      这是更通用的“trait bound”语法，与上面等价。当有多个 trait 约束时，这种语法更清晰。
 *      `fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32`
 *    - `where` 子句: 当泛型和 trait 约束变得复杂时，可以使用 `where` 子句来提高可读性。
 *      `fn some_function<T, U>(t: &T, u: &U) -> i32`
 *      `    where T: Display + Clone,`
 *      `          U: Clone + Debug`
 *
 * 4. 返回实现了 Trait 的类型
 *    - 你也可以在函数返回值中使用 `impl Trait` 语法，来表示函数返回一个实现了某个 trait 的类型，
 *      但你不希望（或不能）写出它的具体类型。
 *    - `fn returns_summarizable() -> impl Summary { ... }`
 *    - 这对于返回闭包或迭代器非常有用，因为它们的具体类型可能非常复杂或无法写出。
 *
 * 5. Trait 的默认实现
 *    - 有时，为 trait 中的某些方法提供一个默认的行为是很有用的。
 *    - 在定义 trait 时，你可以直接为一个方法提供一个默认实现。
 *    - 这样，在为类型实现 trait 时，你可以选择不重写这个方法，直接使用它的默认行为。
 *
 * 6. 派生 Trait (Derive)
 *    - Rust 编译器能够为某些特定的 trait 自动生成实现代码。
 *    - 我们通过 `#[derive(...)]` 注解来告诉编译器这样做。
 *    - 常见的可派生 trait 包括：
 *      - `Debug`: 允许使用 `{:?}` 格式化打印。
 *      - `Clone`, `Copy`: 控制类型的复制行为。
 *      - `PartialEq`, `Eq`, `PartialOrd`, `Ord`: 用于比较。
 *      - `Hash`: 用于 `HashMap` 的键。
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

// 1. 定义一个 Trait
pub trait Summary {
    // 必须实现的方法
    fn summarize_author(&self) -> String;

    // 带有默认实现的方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 定义两个不同的结构体
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

// 2. 为 NewsArticle 实现 Summary Trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // summarize 方法使用默认实现
}

// 2. 为 Tweet 实现 Summary Trait
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 重写 summarize 的默认实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 3. Trait 作为参数
// 这个函数可以接受任何实现了 Summary trait 的类型的引用
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 4. 返回实现了 Trait 的类型
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            author: String::from("Iceburgh"),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }
    // 注意：虽然看起来返回了不同类型，但在 Rust 1.75 及以后版本，
    // 编译器可以推断出一个统一的匿名类型，只要所有分支返回的类型都实现了该 trait。
    // 在旧版本中，这需要更复杂的技巧（如 Box<dyn Trait>）。
}

fn main() {
    let tweet = Tweet {
        username: String::from("johndoe"),
        content: String::from("Hello, this is my first tweet!"),
    };

    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        author: String::from("Jane Doe"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    // 使用 notify 函数
    notify(&tweet);
    notify(&article);

    // 使用返回 impl Trait 的函数
    let summary = returns_summarizable(true);
    println!("\nReturned summary: {}", summary.summarize());
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 创建一个 `Drawable` Trait:
 *    定义一个名为 `Drawable` 的 trait，它有一个 `draw` 方法，签名是 `fn draw(&self);`。
 *    创建两个结构体：`Button` 和 `Screen`。
 *    `Screen` 结构体有一个字段 `components`，其类型是 `Vec<Box<dyn Drawable>>`。
 *    `Box<dyn Drawable>` 是一个“trait 对象”，它允许我们在 `Vec` 中存储不同类型但都实现了 `Drawable` trait 的实例。
 *    为 `Button` 实现 `Drawable` trait，让它的 `draw` 方法打印出 "Drawing a button."。
 *    为 `Screen` 实现一个 `run` 方法，该方法遍历其所有 `components` 并调用每个组件的 `draw` 方法。
 *
 * 2. 实现标准库的 `Display` Trait:
 *    为你在 `09_structs.rs` 中创建的 `Rectangle` 结构体实现 `std::fmt::Display` trait。
 *    `Display` trait 需要你实现 `fmt` 方法，它允许你使用 `{}` 格式化操作符来打印你的结构体。
 *    实现 `fmt` 方法，使其打印出类似 "Rectangle (width: 30, height: 50)" 的格式。
 *
 */