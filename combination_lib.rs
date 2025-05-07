pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if k == 0 {
        result.push(Vec::new());
        return result;
    }

    if arr.len() < k {
        return result;
    }

    for i in 0..=arr.len() - k {
        let head = arr[i];
        let tail_combinations = combinations(&arr[i + 1..], k - 1);

        for tail in tail_combinations {
            let mut combo = Vec::with_capacity(k);
            combo.push(head);
            for item in tail {
                combo.push(item);
            }
            result.push(combo);
        }
    }

    result
}
