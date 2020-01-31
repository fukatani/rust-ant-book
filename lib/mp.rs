struct MP<T: std::cmp::PartialEq> {
    pi: Vec<i32>,
    s: Vec<T>,
}

impl<T: std::cmp::PartialEq> MP<T> {
    fn new(s: Vec<T>) -> MP<T> {
        let mut pi = vec![0; s.len() + 1];
        pi[0] = -1;
        let mut j = -1;
        for i in 0..s.len() {
            while j >= 0 && s[i] != s[j as usize] {
                j = pi[j as usize];
            }
            j += 1;
            pi[i + 1] = j;
        }
        MP { pi: pi, s: s }
    }

    fn matched_indexes(&self, t: &Vec<T>) -> Vec<usize> {
        let mut j = 0;
        let mut res = vec![];
        for i in 0..t.len() {
            while j >= 0 && self.s[j as usize] != t[i] {
                j = self.pi[j as usize];
            }
            j += 1;
            if j as usize == self.s.len() {
                res.push(1 + i - j as usize);
                j = self.pi[j as usize];
            }
        }
        res
    }
}

fn main() {
    let mp = MP::new("aabaabaaa".chars().collect::<Vec<_>>());
    assert_eq!(mp.pi, vec![-1, 0, 1, 0, 1, 2, 3, 4, 5, 2]);

    let mp = MP::new("aab".chars().collect::<Vec<_>>());
    let t = "aabaabaaa".chars().collect::<Vec<_>>();
    assert_eq!(mp.matched_indexes(&t), vec![0, 3]);

    let mp = MP::new("aabaa".chars().collect::<Vec<_>>());
    let t = "aabaabaaa".chars().collect::<Vec<_>>();
    assert_eq!(mp.matched_indexes(&t), vec![0, 3]);
}
