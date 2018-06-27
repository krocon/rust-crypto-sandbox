


#[cfg(test)]
mod wordlistfactorytest {

    use wordlistfactory::WordlistFactory;

    #[test]
    fn test_some_words () {
        assert_eq!(WordlistFactory::get_word_list("Korean")[2], "가난");
        assert_eq!(WordlistFactory::get_word_list("French")[2047], "zoologie");
    }
}