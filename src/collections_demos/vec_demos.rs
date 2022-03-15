pub mod demos {

    pub fn use_vec() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in &mut v {
            *i += 50
        }
    }
}
