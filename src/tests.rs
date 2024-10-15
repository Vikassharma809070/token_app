// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_tokens() {
        let from = String::from("alice");
        let to = String::from("bob");
        let amount = 50;
        
        let result = transfer(from.clone(), to.clone(), amount);
        assert!(result);
        assert_eq!(balance_of(from), 0);
        assert_eq!(balance_of(to), 50);
    }

    #[test]
    fn test_insufficient_balance() {
        let from = String::from("alice");
        let to = String::from("bob");
        let amount = 100;
        
        let result = transfer(from.clone(), to.clone(), amount);
        assert!(!result);
    }
}
