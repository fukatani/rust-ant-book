#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

fn main() {
    let s = read::<String>()
        .chars()
        .map(|x| x.to_digit(36).unwrap() as usize - 10)
        .collect::<Vec<_>>();
    let n = read::<usize>();
    let mut trie = Trie::new();
    for i in 0..n {
        let t = read::<String>()
            .chars()
            .map(|x| x.to_digit(36).unwrap() as usize - 10)
            .collect::<Vec<_>>();
        trie.add(&t);
    }
    trie.build();

    let accum = trie.calc_accum();
    let mut pos = 0;

    let mut ans = 0;
    for ch in s {
        pos = trie.trans(pos, ch);
        if accum[pos] > 0 {
            ans += 1;
            pos = 0;
        }
    }
    println!("{}", ans);
}

const F: usize = 26;

#[derive(Default, Clone, Copy, Debug)]
struct TrieNode {
    fail: usize,
    next: [Option<usize>; F],
    count: i64,
}

#[derive(Default, Clone, Debug)]
struct Trie {
    nodes: Vec<TrieNode>,
}

// Aho-Corasick
impl Trie {
    fn new() -> Trie {
        Trie {
            nodes: vec![TrieNode::default()],
        }
    }

    fn add(&mut self, word: &Vec<usize>) {
        let mut cur = 0;
        for &ch in word {
            if let Some(next) = self.nodes[cur].next[ch] {
                cur = next;
            } else {
                let next = self.nodes.len();
                self.nodes.push(TrieNode::default());
                self.nodes[cur].next[ch] = Some(next);
                cur = next;
            }
        }
        self.nodes[cur].count += 1;
    }

    fn build(&mut self) {
        let mut q = VecDeque::new();
        for i in 0..F {
            if let Some(x) = self.nodes[0].next[i] {
                q.push_back(x);
            }
        }
        while let Some(cur) = q.pop_front() {
            for next_ch in 0..F {
                if let Some(next_node) = self.nodes[cur].next[next_ch] {
                    let mut fail = self.nodes[cur].fail;
                    while fail > 0 && self.nodes[fail].next[next_ch] == None {
                        fail = self.nodes[fail].fail;
                    }
                    self.nodes[next_node].fail = self.nodes[fail].next[next_ch].unwrap_or(0);
                    q.push_back(next_node);
                }
            }
        }
    }

    fn trans(&self, start: usize, ch: usize) -> usize {
        let mut cur = start;
        while cur != 0 && self.nodes[cur].next[ch] == None {
            cur = self.nodes[cur].fail;
        }
        self.nodes[cur].next[ch].unwrap_or(0)
    }

    fn calc_accum(&self) -> Vec<i64> {
        let mut accum = self.nodes.iter().map(|&x| x.count).collect::<Vec<_>>();
        let mut q = VecDeque::new();
        q.push_back(0);

        while let Some(cur) = q.pop_front() {
            accum[cur] += accum[self.nodes[cur].fail];

            for next_ch in 0..F {
                if let Some(next_node) = self.nodes[cur].next[next_ch] {
                    accum[next_node] += accum[cur];
                    q.push_back(next_node);
                }
            }
        }
        accum
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
