#[derive(Debug)]
struct TrieNode {
    ch: char,
    count: usize,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn with_char(ch: char) -> TrieNode {
        TrieNode {
            ch: ch,
            count: 0,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Trie {
    node: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        let node = TrieNode::with_char('#');
        Trie { node: node }
    }

    fn add_word(&mut self, word: &[char]) {
        let mut cur = &mut self.node;
        for &ch in word {
            if !cur.children.contains_key(&ch) {
                let node = TrieNode::with_char(ch);
                cur.children.insert(ch, node);
            }
            cur = cur.children.get_mut(&ch).unwrap();
        }
        cur.count += 1;
    }

    fn count(&self, word: &[char]) -> usize {
        let mut cur = &self.node;
        for &ch in word {
            if !cur.children.contains_key(&ch) {
                return 0;
            }
            cur = cur.children.get(&ch).unwrap();
        }
        cur.count
    }
}
