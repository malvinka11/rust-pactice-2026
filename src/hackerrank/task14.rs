pub fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) -> Result<(), i32> {
    let total: i32 = bill.iter().sum();
    let fair_share = (total - bill[k]) / 2;

    if b == fair_share {
        Ok(())
    } else {
        Err(b - fair_share)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_overcharge() {
        let bill = vec![3, 10, 2, 9];
        let result = bon_appetit(bill, 1, 12);
        assert_eq!(result, Err(5));
    }

    #[test]
    fn test_bon_appetit_correct() {
        let bill = vec![3, 10, 2, 9];
        let result = bon_appetit(bill, 1, 7);
        assert_eq!(result, Ok(()));
    }
}