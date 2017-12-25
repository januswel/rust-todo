pub mod todo {
    pub struct Todo {
        name: String,
        is_done: bool,
    }

    impl Todo {
        pub fn new(name_: &str) -> Todo {
            Todo {
                name: name_.to_string(),
                is_done: false,
            }
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }
        pub fn is_done(&self) -> bool {
            self.is_done
        }
        pub fn done(& mut self) {
            self.is_done = true
        }
        pub fn undone(& mut self) {
            self.is_done = false
        }
    }
}
