#[cfg(test)]
mod mnemonictrietest02 {
    use mnemonictrie02::MnemonicTrie;

    #[test]
    fn test_trie() {
        let mut trie = MnemonicTrie::new();
        trie.add_word("foo".to_string());
        trie.add_word("foobar".to_string());
        trie.add_word("letter".to_string());
        trie.add_word("cage".to_string());

        assert_eq!(trie.get_word_index("foo".to_string()), Some(0));
        assert_eq!(trie.get_word_index("foobar".to_string()), Some(1));
        assert_eq!(trie.get_word_index("letter".to_string()), Some(2));
        assert_eq!(trie.get_word_index("cage".to_string()), Some(3));

        assert_eq!(trie.get_word_index("xxxx".to_string()), None);
        assert_eq!(trie.get_word_index("foob".to_string()), None);
    }
}