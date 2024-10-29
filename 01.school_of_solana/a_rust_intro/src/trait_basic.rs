fn _main() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        heading: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("summary: {}", &self.content[0..50])
        }
    }
}
