use std::collections::HashMap;

pub fn sock_merchant(ar: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for sock in ar {
        *map.entry(sock).or_insert(0) += 1;
    }

    map.values().map(|v| v / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let arr = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(arr), 3);
    }
}