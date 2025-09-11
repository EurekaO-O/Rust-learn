// 练习2：
// `pub` 使得外部可以调用 `add_to_waitlist` 函数
        // `pub` 使得外部可以调用 `add_to_waitlist` 函数
        pub fn add_to_waitlist() {
            println!("Added to waitlist.");
            // 可以调用同模块下的私有函数
            seat_at_table();
        }

        fn seat_at_table() {
            println!("Seated at table.");
        }