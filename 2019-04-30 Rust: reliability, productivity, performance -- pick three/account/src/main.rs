fn main() {
    let mut account = Account::with_balance(100.0);
    account
        .withdraw(15.0)
        .and_then(|()| {
            println!("account balance: {}", account.value());
            Ok(())
        })
        .unwrap_or_else(|e| {
            println!("failed to withdraw: {:?}", e);
        });
}

struct Account {
    balance: f32,
}

impl Account {
    fn with_balance(balance: f32) -> Self {
        Self { balance }
    }

    fn withdraw(&mut self, amount: f32) -> Result<(), WithdrawalError> {
        if amount < 0.0 {
            Err(WithdrawalError::InvalidAmount(amount))
        } else if amount > self.balance {
            Err(WithdrawalError::NotEnoughBalance)
        } else {
            self.balance -= amount;
            Ok(())
        }
    }
}

impl Asset for Account {
    fn value(&self) -> f32 {
        self.balance
    }
}

#[derive(Debug, PartialEq)]
enum WithdrawalError {
    InvalidAmount(f32),
    NotEnoughBalance,
}

trait Asset {
    fn value(&self) -> f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn withdraw() {
        let mut account = Account::with_balance(100.0);
        account.withdraw(15.0).unwrap();
    }

    #[test]
    fn invalid_amount() {
        let mut account = Account::with_balance(100.0);
        assert_eq!(
            account.withdraw(-15.0),
            Err(WithdrawalError::InvalidAmount(-15.0))
        );
    }

    #[test]
    #[should_panic]
    fn not_enough() {
        let mut account = Account::with_balance(100.0);
        account.withdraw(90.0).unwrap();        
    }
}
