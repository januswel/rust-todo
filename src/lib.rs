pub mod todo {
    pub struct Todo<'a> {
        pub name: &'a str,
        pub is_done: bool,
    }

    impl<'a> Todo<'a> {
        pub fn get_name(&self) -> &'a str {
            self.name
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
