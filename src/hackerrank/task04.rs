pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|grade| {
            if grade < 38 {
                grade
            } else {
                let next = ((grade + 4) / 5) * 5;
                if next - grade < 3 {
                    next
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(grades), expected);
    }
}