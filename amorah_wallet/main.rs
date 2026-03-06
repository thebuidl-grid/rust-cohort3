use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone)]
struct Wallet {
    balance: f64,
}

impl Wallet {
    pub fn new(initial_balance: f64) -> Self {
        assert!(initial_balance >= 0.0, "Initial balance must be non-negative");
        Wallet { balance: initial_balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        assert!(amount > 0.0, "Deposit amount must be positive");
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive");
        }
        if amount > self.balance {
            return Err("Insufficient balance for withdrawal");
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    gender: Gender,
    wallet: Wallet,
}

impl User {
    pub fn new(name: String, age: u32, gender: Gender, initial_balance: f64) -> Self {
        assert!(!name.is_empty(), "Name must not be empty");
        assert!(age > 0, "Age must be positive");
        User {
            name,
            age,
            gender,
            wallet: Wallet::new(initial_balance),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn gender(&self) -> &Gender {
        &self.gender
    }

    pub fn deposit_to_wallet(&mut self, amount: f64) {
        self.wallet.deposit(amount);
    }

    pub fn withdraw_from_wallet(&mut self, amount: f64) -> Result<(), &'static str> {
        self.wallet.withdraw(amount)
    }

    pub fn wallet_balance(&self) -> f64 {
        self.wallet.balance()
    }

    pub fn transfer_to(&mut self, recipient: &mut User, amount: f64) -> Result<(), &'static str> {
        assert!(amount > 0.0, "Transfer amount must be positive");
        self.wallet.withdraw(amount)?;
        recipient.wallet.deposit(amount);
        Ok(())
    }
}

pub fn run() {
    let mut users: Vec<User> = Vec::new();

    let amorah = User::new("amorah".to_string(), 37, Gender::Male, 1000.0);
    users.push(amorah);

    let johnny = User::new("Johnny".to_string(), 25, Gender::Male, 50.0);
    users.push(johnny);

    let rose = User::new("rose".to_string(), 28, Gender::Female, 300.0);
    users.push(rose);

    let james = User::new("james".to_string(), 28, Gender::Other, 300.0);
    users.push(james);

    if let Some(amorah) = users.get_mut(0) {
        amorah.deposit_to_wallet(50.0);
    }

    if let Some(johnny) = users.get_mut(1) {
        let withdrawal_result = johnny.withdraw_from_wallet(20.0);
        if let Err(err) = withdrawal_result {
            println!("Withdrawal failed: {}", err);
        }
    }

    {
        let (first, rest) = users.split_at_mut(1);
        let amorah = &mut first[0];
        let johnny = &mut rest[0];

        let num = 200.0;
        println!("Balances before transfer of {}: ", num);
        println!("  ");
        println!("{}'s balance: {}", amorah.name(), amorah.wallet_balance());
        println!("{}'s balance: {}", johnny.name(), johnny.wallet_balance());
        println!("  ");

        let _ = amorah.transfer_to(johnny, num);

        println!("Balances after transfer of {}: ", num);
        println!("  ");
        println!("{}'s balance: {}", amorah.name(), amorah.wallet_balance());
        println!("{}'s balance: {}", johnny.name(), johnny.wallet_balance());
        println!("  ");
    }

    for user in &users {
        println!(
            "User: {}, Age: {}, Gender: {:?}, Balance: {}",
            user.name(),
            user.age(),
            user.gender(),
            user.wallet_balance()
        );
    }
}

fn main() {
    run();
}
