fn derangement(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 0,
        _ => (n - 1) * (derangement(n - 1) + derangement(n - 2)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derangement_6() {
        assert_eq!(derangement(6), 265);
    }

    #[test]
    fn derangement_9() {
        assert_eq!(derangement(9), 133496);
    }

    #[test]
    fn derangement_14() {
        assert_eq!(derangement(14), 32071101049);
    }
}
