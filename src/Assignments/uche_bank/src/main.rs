use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Bank {
    Opay,
    PalmPay,
    Kuda,
    Moniepoint,
}

#[derive(Debug)]
enum Status {
    Success,
    InsufficientFunds,
    AccountNotFound,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    bank: Bank,
    account_number: u32,
    balance: u64,
}

#[derive(Debug, Default)]
struct Wallet {
    wallet_details: HashMap<u32, User>,
}

impl User {
    fn new(name: String, bank: Bank, account_number: u32, balance: u64) -> Self {
        Self {
            name,
            bank,
            account_number,
            balance,
        }
    }

    fn deposit(&mut self, amount: u64) -> u64 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: u64) -> Status {
        if self.balance < amount {
            Status::InsufficientFunds
        } else {
            self.balance -= amount;
            Status::Success
        }
    }
}

impl Wallet {
    fn new() -> Self {
        Self {
            wallet_details: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: User) {
        self.wallet_details.insert(user.account_number, user);
    }

    fn deposit_to(&mut self, account_number: u32, amount: u64) -> Status {
        match self.wallet_details.get_mut(&account_number) {
            Some(user) => {
                user.deposit(amount);
                Status::Success
            }
            None => Status::AccountNotFound,
        }
    }

    fn withdraw_from(&mut self, account_number: u32, amount: u64) -> Status {
        match self.wallet_details.get_mut(&account_number) {
            Some(user) => user.withdraw(amount),
            None => Status::AccountNotFound,
        }
    }

    fn balance_of(&self, account_number: u32) -> Option<u64> {
        self.wallet_details.get(&account_number).map(|user| user.balance)
    }
}

fn main() {
    let mut wallet = Wallet::new();

    let user1 = User::new("Uche".to_string(), Bank::Kuda, 1001, 5_000);
    let user2 = User::new("Ada".to_string(), Bank::Opay, 1002, 8_500);

    wallet.add_user(user1);
    wallet.add_user(user2);

    let deposit_status = wallet.deposit_to(1001, 4_000);
    let withdraw_status = wallet.withdraw_from(1002, 7_000);

    println!("Deposit status: {:?}", deposit_status);
    println!("Withdraw status: {:?}", withdraw_status);
    println!("1001 balance: {:?}", wallet.balance_of(1001));
    println!("1002 balance: {:?}", wallet.balance_of(1002));
}
