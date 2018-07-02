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
    pub fn new<'a>() -> MnemonicTrie {
        MnemonicTrie {
            size: 0,
            root: Node::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = moving(current_node)
                .chars
                .entry(c)
                .or_insert(Node {
                    chars: HashMap::new(),
                    idx: None,
                });
        }
        current_node.idx = Some(self.size);
        self.size = self.size + 1;
    }

    pub fn get_word_index(&mut self, word: String) -> Option<i32> {
        //let ret = None;
        let mut current_node: &Node = &self.root;

        for c in word.chars() {
            let mut option = current_node.chars.get(&c);
            match option {
                Some(n) => {
                    current_node = n;
                }
                None => return None,
            };
        }
        current_node.idx
    }
}

fn moving<T>(t: T) -> T { t }

