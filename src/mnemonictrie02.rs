use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    chars: HashMap<char, Node>,
    idx: Option<i32>,
}


#[derive(Debug)]
pub struct MnemonicTrie {
    size: i32,
    root: Node,
}

impl MnemonicTrie {
    pub fn new() -> MnemonicTrie {
        MnemonicTrie {
            size: 0,
            root: Node::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            let moved = current_node;
            current_node = moved
                .chars
                .entry(c)
                .or_insert(Node::default());
        }
        current_node.idx = Some(self.size);
        self.size += 1;
    }

    pub fn get_word_index(&mut self, word: String) -> Option<i32> {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            let moved = current_node;
            if let Some(node) = moved.chars.get_mut(&c) {
                current_node = node;
            } else {
                return None;
            }
        }
        current_node.idx
    }
}



