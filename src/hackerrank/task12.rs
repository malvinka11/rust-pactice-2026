pub fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    let max = *candles.iter().max().unwrap();
    candles.iter().filter(|&&x| x == max).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_cake_candles() {
        let arr = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(arr), 2);
    }
}