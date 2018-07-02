#[cfg(test)]
mod wordlistfactory01test {
    use wordlistfactory01::*;

    #[test]
    fn test_some_words_01() {
        assert_eq!(WordlistFactory01::get_word_list("korean")[2], "가난");
        assert_eq!(WordlistFactory01::get_word_list("french")[2047], "zoologie");
    }
}