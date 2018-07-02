#[cfg(test)]
mod wordlistfactory03test {
    use wordlistfactory03::WordlistCountry03::English;
    use wordlistfactory03::WordlistCountry03::French;
    use wordlistfactory03::WordlistCountry03::Korean;
    use wordlistfactory03::WordlistFactory03;


    #[test]
    fn test_some_words() {
        assert_eq!(WordlistFactory03::get_word_list(&Korean)[2], "가난");
        assert_eq!(WordlistFactory03::get_word_list(&French)[2047], "zoologie");
    }

    #[test]
    fn test_index() {
        assert_eq!(WordlistFactory03::get_index("about", &English).unwrap(), 3);
    }
}