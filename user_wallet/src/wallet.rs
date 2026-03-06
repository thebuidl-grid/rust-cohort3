pub trait FinancialAccount {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

pub struct UserWallet {
    pub owner: String,
    balance: f64,
}

impl UserWallet {
    pub fn new(owner: String, initial_balance: f64) -> Self {
        Self {
            owner,
            balance: initial_balance,
        }
    }
}

// Implementation of the trait for UserWallet
impl FinancialAccount for UserWallet {
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("+ ${:.2} deposited to {}'s wallet.", amount, self.owner);
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err(format!("Overdraft blocked! Balance is ${:.2}", self.balance));
        }
        self.balance -= amount;
        println!("- ${:.2} withdrawn from {}'s wallet.", amount, self.owner);
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}