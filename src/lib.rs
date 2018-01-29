pub fn look_up_word(word: &str) {
    println!("the word to look up is {}", &word)
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
