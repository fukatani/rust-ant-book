fn main() {
    let mut a = 34;
    let mut b = 12;
    let mut stan_win = true;
    loop {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if b % a == 0 {
            break;
        }
        if b > 2 * a {
            break;
        }
        b -= a;
        stan_win = !stan_win;
    }
    if stan_win {
        println!("Stan wins");
    } else {
        println!("Ollie wins");
    }
}
