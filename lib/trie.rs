use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TrieNode<K>
where
    K: Eq + Hash + Clone + Copy,
{
    ch: K,
    count: usize,
    children: HashMap<K, TrieNode<K>>,
}

impl<K> TrieNode<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn with_char(ch: K) -> TrieNode<K> {
        TrieNode {
            ch: ch,
            count: 0,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Trie<K>
where
    K: Eq + Hash + Clone + Copy,
{
    node: TrieNode<K>,
    init: K,
}

impl<K> Trie<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn new(init: K) -> Trie<K> {
        let node = TrieNode::with_char(init);
        Trie {
            node: node,
            init: init,
        }
    }

    fn add_word(&mut self, word: &[K]) {
        let mut cur = &mut self.node;
        for &ch in word {
            assert!(self.init != ch);
            if !cur.children.contains_key(&ch) {
                let node = TrieNode::with_char(ch);
                cur.children.insert(ch, node);
            }
            cur = cur.children.get_mut(&ch).unwrap();
        }
        cur.count += 1;
    }

    fn count(&self, word: &[K]) -> usize {
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

fn main() {
    let mut trie = Trie::new('#');
    trie.add_word(&vec!['a', 'b', 'c']);
    trie.add_word(&vec!['a', 'b', 'c']);
    assert_eq!(trie.count(&vec!['a', 'b', 'c']), 2);
    assert_eq!(trie.count(&vec!['a', 'b']), 0);
}
