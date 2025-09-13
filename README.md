# Rust 核心概念入门教程

欢迎来到这个循序渐进的 Rust 学习教程！本仓库旨在为聪明的编程初学者提供一套完整的、基于文件的 Rust 入门课程。

教程的风格受到了 [Rustlings](https://github.com/rust-lang/rustlings) 的启发，强调动手实践。每个文件都是一个独立、可运行的课程，包含了详细的概念讲解、丰富的代码示例和用于巩固知识的课后挑战。

## 教程结构

本教程涵盖了从基础到 Rust 核心特性的 19 个主题，严格按照学习曲线递进排序：

1.  **基础入门**
    -   [`01_hello_cargo.rs`](./01_hello_cargo.rs): 介绍 Cargo，Rust 的构建工具和包管理器。
    -   [`02_variables_and_mutability.rs`](./02_variables_and_mutability.rs): 变量、不可变性与 `mut` 关键字。
    -   [`03_scalar_data_types.rs`](./03_scalar_data_types.rs): 整型、浮点型、布尔型和字符型。
    -   [`04_compound_data_types.rs`](./04_compound_data_types.rs): 元组（Tuple）和数组（Array）。
    -   [`05_functions.rs`](./05_functions.rs): 函数定义、参数、返回值以及语句与表达式的区别。
    -   [`06_flow_control.rs`](./06_flow_control.rs): `if-else` 表达式和多种循环 (`loop`, `while`, `for`)。

2.  **Rust 核心概念**
    -   [`07_ownership.rs`](./07_ownership.rs): **[核心]** 所有权三大法则：所有者、移动（Move）和克隆（Clone）。
    -   [`08_references_and_borrowing.rs`](./08_references_and_borrowing.rs): 引用（&）和借用（Borrowing），包括可变与不可变引用。
    -   [`09_structs.rs`](./09_structs.rs): 定义和实例化结构体，以及为其实现方法（`impl`）。
    -   [`10_enums_and_pattern_matching.rs`](./10_enums_and_pattern_matching.rs): 枚举的定义，强大的 `match` 表达式和 `Option` 枚举。

3.  **常用集合与数据处理**
    -   [`11_collections_vector.rs`](./11_collections_vector.rs): 动态数组 `Vec<T>`。
    -   [`12_collections_string.rs`](./12_collections_string.rs): `String` 类型与 `&str` 字符串切片的区别。
    -   [`13_collections_hashmap.rs`](./13_collections_hashmap.rs): 键值对集合 `HashMap<K, V>`。

4.  **代码组织与错误处理**
    -   [`14_packages_and_modules.rs`](./14_packages_and_modules.rs): 使用 `mod` 和 `use` 组织代码。
    -   [`15_error_handling_panic.rs`](./15_error_handling_panic.rs): 不可恢复的错误与 `panic!`。
    -   [`16_error_handling_result.rs`](./16_error_handling_result.rs): 可恢复的错误与 `Result<T, E>` 枚举，以及 `?` 操作符。

5.  **高级特性**
    -   [`17_generics.rs`](./17_generics.rs): 使用泛型减少代码重复。
    -   [`18_traits.rs`](./18_traits.rs): **[核心]** 使用 Trait 定义共享行为，类似于接口。
    -   [`19_lifetimes.rs`](./19_lifetimes.rs): **[核心]** 生命周期，Rust 内存安全的终极保障。

## 如何使用本教程

每个 `.rs` 文件都是一个独立的课程。要学习和运行某一课的内容，请遵循以下步骤：

1.  **选择课程**: 从列表中选择你感兴趣的 `.rs` 文件，例如 `07_ownership.rs`。
2.  **阅读概念**: 打开文件，仔细阅读文件头部的注释，里面有对核心概念的详细讲解。
3.  **复制代码**: 复制该文件的 **全部** 内容。
4.  **粘贴到 `main.rs`**: 打开 `src/main.rs` 文件，清空其内容，然后将刚刚复制的代码粘贴进去。
5.  **运行代码**: 在你的终端中，确保路径位于项目根目录，然后执行：
    ```bash
    cargo run
    ```
6.  **观察输出**: 查看程序的运行结果，并对照代码和注释进行理解。
7.  **完成挑战**: 尝试完成文件末尾注释中的“练习挑战”，动手修改或扩展代码以巩固所学。

## 环境要求

你需要安装 Rust 工具链，它包含了 Rust 编译器 `rustc` 和包管理器 `cargo`。

-   **安装地址**: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

---

祝你学习愉快！如果你发现任何错误或有改进建议，欢迎随时提出 Issue。