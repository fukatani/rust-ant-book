fn to_binary(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut res = Vec::new();
    while num != 0 {
        res.push(num % 2);
        num /= 2;
    }
    res.iter()
        .rev()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}


fn main() {
  assert_eq!(to_binary(5), "101".to_string());
  assert_eq!(to_binary(0), "0".to_string());
  assert_eq!(to_binary(15), "1111".to_string());
}
