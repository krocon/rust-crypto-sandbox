#[cfg(test)]
mod wordlistfactory02test {
    use wordlistfactory02::WordlistCountry02::French;
    use wordlistfactory02::WordlistCountry02::Korean;
    use wordlistfactory02::WordlistFactory02;


    #[test]
    fn test_some_words_02() {
        assert_eq!(WordlistFactory02::get_word_list(&Korean)[2], "가난");
        assert_eq!(WordlistFactory02::get_word_list(&French)[2047], "zoologie");
    }
}