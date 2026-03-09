pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (usize, usize) {
    let apples_count = apples
        .into_iter()
        .filter(|&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count();

    let oranges_count = oranges
        .into_iter()
        .filter(|&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count();

    (apples_count, oranges_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let result = count_apples_and_oranges(7, 11, 5, 15, vec![-2, 2, 1], vec![5, -6]);
        assert_eq!(result, (1, 1));
    }
}