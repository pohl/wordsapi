pub fn look_up_word(word: &str, token: &str) {
    println!("the word to look up is {}, the token is {}", &word, &token)
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
