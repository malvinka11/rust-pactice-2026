pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let l = a.iter().copied().reduce(|x, y| lcm(x, y)).unwrap();
    let g = b.iter().copied().reduce(|x, y| gcd(x, y)).unwrap();

    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(a, b), 3);
    }
}