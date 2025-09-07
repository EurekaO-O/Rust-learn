// 02_variables_and_mutability.rs
// 核心内容：讲解变量的声明（let）、不可变性（immutability）的核心概念，以及如何使用mut关键字使其可变。

/*
 * =====================================================================================
 * 核心概念讲解 (Comments Section)
 * =====================================================================================
 *
 * 在任何编程语言中，变量都是基础。Rust在处理变量时有一个非常重要且独特的特性：默认不可变性。
 *
 * 1. 变量声明 (let)
 *    - 在Rust中，我们使用 `let` 关键字来声明一个变量。
 *    - 例如: `let x = 5;` 这行代码创建了一个名为 `x` 的变量，并把它绑定到值 `5` 上。
 *
 * 2. 默认不可变性 (Immutability by Default)
 *    - 当你像上面那样声明一个变量后，它的值是不可变的（immutable）。这意味着一旦一个值被绑定到变量上，你就不能再改变它了。
 *    - 尝试修改它会导致编译错误！例如：
 *      `let x = 5;`
 *      `x = 6; // 这会报错！`
 *    - 为什么这么设计？这是Rust为了编写更安全、更并发的代码而做出的核心设计决策之一。
 *      不可变性可以让你更容易地推理代码，因为你知道一个值在程序的某个作用域内不会被意外改变。
 *
 * 3. 可变性 (Mutability)
 *    - 当然，我们经常需要能够改变值的变量。为了实现这一点，你可以在变量名前加上 `mut` 关键字。
 *    - 例如: `let mut y = 5;`
 *      `y = 6; // 这是完全可以的！`
 *    - 使用 `mut` 是你向编译器和其他开发者明确表示：“这个变量的值在后面可能会改变”。
 *
 * 4. 常量 (Constants)
 *    - 常量与不可变变量类似，但有一些区别。常量使用 `const` 关键字声明，并且必须显式地注明类型。
 *    - `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
 *    - 常量在整个程序的生命周期内都有效，并且其值必须是一个编译时就能确定的常量表达式。
 *    - 命名约定上，常量通常使用全大写和下划线。
 *
 * 5. 变量遮蔽 (Shadowing)
 *    - Rust允许你使用 `let` 声明一个与之前变量同名的新变量。这个过程称为“遮蔽”（shadowing）。
 *    - `let x = 5;`
 *    - `let x = x + 1; // 这里，新的x遮蔽了旧的x`
 *    - 遮蔽与 `mut` 不同。遮蔽实际上是创建了一个全新的变量，我们可以用它来改变值的类型，而 `mut` 变量则不能改变类型。
 *      `let spaces = "   ";`
 *      `let spaces = spaces.len(); // 从字符串类型变为数字类型，这是合法的！`
 *
 */

// =====================================================================================
// 代码示例 (Code Section)
// =====================================================================================

fn main() {
    // 1. 不可变变量
    let x = 5;
    println!("The value of x is: {}", x);
    // 下面这行代码如果取消注释，将会导致编译错误
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    // println!("The value of x is now: {}", x);

    // 2. 可变变量
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20; // 因为 y 是 mut，所以可以修改
    println!("The new value of y is: {}", y);

    // 3. 常量
    // 常量必须在声明时指定类型，例如 u32 (32位无符号整数)
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    // 4. 遮蔽 (Shadowing)
    let z = 5;
    println!("The value of z is: {}", z);

    // 在同一个作用域内，使用 let 再次声明 z
    let z = z * 2; // 新的 z (值为10) 遮蔽了旧的 z (值为5)
    println!("The value of z after shadowing is: {}", z);

    // 遮蔽允许我们改变变量的类型
    let spaces = "   ";       // spaces 是一个字符串切片
    let spaces = spaces.len(); // spaces 现在是一个数字
    println!("The number of spaces is: {}", spaces);

    // 如果我们对 mut 变量尝试做同样的事情，就会报错
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len(); // error[E0308]: mismatched types
}

/*
 * =====================================================================================
 * 练习挑战 (Challenge Section)
 * =====================================================================================
 *
 * 1. 修复错误:
 *    下面的代码有一个错误。请只添加一个 `mut` 关键字来修复它。
 *
 *    fn challenge_one() {
 *        let temperature = 30;
 *        println!("The temperature is {} degrees.", temperature);
 *        temperature = 25;
 *        println!("The temperature changed to {} degrees.", temperature);
 *    }
 *
 * 2. 使用遮蔽:
 *    声明一个名为 `value` 的变量，并将其绑定到一个字符串上，比如 "one"。
 *    打印它。
 *    然后，使用遮蔽将 `value` 绑定到这个字符串的长度上。
 *    再次打印它，观察输出和类型的变化。
 *
 */