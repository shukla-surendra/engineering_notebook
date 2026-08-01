fn unique_paths(m: i32, n: i32) -> i32 {
    if m == 1 || n == 1 {
        return 1;
    }
    unique_paths(m - 1, n) + unique_paths(m, n - 1)
}

fn unique_paths_from_origin(r: i32, c: i32, m: i32, n: i32) -> i32 {
    if r == m - 1 && c == n - 1 {
        return 1;
    }
    if r >= m || c >= n {
        return 0;
    }
    unique_paths_from_origin(r + 1, c, m, n) + unique_paths_from_origin(r, c + 1, m, n)
}

fn main() {
    println!("{}", unique_paths(3, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    // (m, n, expected)
    const CASES: &[(i32, i32, i32)] = &[
        (3, 7, 28),  // problem.md example
        (3, 2, 3),   // problem.md example
        (1, 1, 1),   // single cell, one trivial path
        (1, 5, 1),   // single row, only one straight-line path
        (5, 1, 1),   // single column, only one straight-line path
        (7, 3, 28),  // symmetric to (3, 7)
        (3, 3, 6),   // small square grid
    ];

    #[test]
    fn matches_expected_path_counts() {
        for &(m, n, expected) in CASES {
            assert_eq!(
                unique_paths(m, n),
                expected,
                "unique_paths({m}, {n}) should be {expected}"
            );
        }
    }
}
