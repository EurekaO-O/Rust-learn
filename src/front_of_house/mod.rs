// src/front_of_house/mod.rs
// 告诉编译器，我有一个叫 `hosting` 的公共子模块，
// 请去同目录下的 `hosting.rs` 文件里加载它的代码。
pub mod hosting;
// serving 模块的代码还留在这里。
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}