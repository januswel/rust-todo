pub mod todo {
    #[no_mangle]
    pub struct Todo {
        name: String,
        is_done: bool,
    }

    impl Todo {
        #[no_mangle]
        pub fn new(name_: &str) -> Todo {
            Todo {
                name: name_.to_string(),
                is_done: false,
            }
        }

        #[no_mangle]
        pub fn get_name(&self) -> &String {
            &self.name
        }
        #[no_mangle]
        pub fn is_done(&self) -> bool {
            self.is_done
        }
        #[no_mangle]
        pub fn done(& mut self) {
            self.is_done = true
        }
        #[no_mangle]
        pub fn undone(& mut self) {
            self.is_done = false
        }
    }
}
