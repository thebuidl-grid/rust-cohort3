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

    // READ: Get user details
    fn get_user(&self, account_number: u32) -> Option<&User> {
        self.wallet_details.get(&account_number)
    }

    // READ: List all users
    fn list_all_users(&self) -> Vec<&User> {
        self.wallet_details.values().collect()
    }

    // UPDATE: Update user information
    fn update_user(&mut self, account_number: u32, name: Option<String>, bank: Option<Bank>) -> Status {
        match self.wallet_details.get_mut(&account_number) {
            Some(user) => {
                if let Some(new_name) = name {
                    user.name = new_name;
                }
                if let Some(new_bank) = bank {
                    user.bank = new_bank;
                }
                Status::Success
            }
            None => Status::AccountNotFound,
        }
    }

    // DELETE: Remove user from wallet
    fn remove_user(&mut self, account_number: u32) -> Status {
        match self.wallet_details.remove(&account_number) {
            Some(_) => Status::Success,
            None => Status::AccountNotFound,
        }
    }
}

fn main() {
    let mut wallet = Wallet::new();

    // CREATE: Add users
    let user1 = User::new("Uche".to_string(), Bank::Kuda, 1001, 5_000);
    let user2 = User::new("Ada".to_string(), Bank::Opay, 1002, 8_500);
    let user3 = User::new("Chidi".to_string(), Bank::PalmPay, 1003, 10_000);

    wallet.add_user(user1);
    wallet.add_user(user2);
    wallet.add_user(user3);
    println!("=== CREATE: Added 3 users ===\n");

    // READ: Get specific user
    println!("=== READ: Get user 1001 ===");
    if let Some(user) = wallet.get_user(1001) {
        println!("{:?}\n", user);
    }

    // READ: List all users
    println!("=== READ: List all users ===");
    for user in wallet.list_all_users() {
        println!("{:?}", user);
    }
    println!();

    // UPDATE: Update user information
    println!("=== UPDATE: Change user 1002 name and bank ===");
    let update_status = wallet.update_user(1002, Some("Ada Obi".to_string()), Some(Bank::Moniepoint));
    println!("Update status: {:?}", update_status);
    if let Some(user) = wallet.get_user(1002) {
        println!("Updated user: {:?}\n", user);
    }

    // Deposit and withdraw operations
    let deposit_status = wallet.deposit_to(1001, 4_000);
    let withdraw_status = wallet.withdraw_from(1002, 7_000);

    println!("=== TRANSACTIONS ===");
    println!("Deposit status: {:?}", deposit_status);
    println!("Withdraw status: {:?}", withdraw_status);
    println!("1001 balance: {:?}", wallet.balance_of(1001));
    println!("1002 balance: {:?}\n", wallet.balance_of(1002));

    // DELETE: Remove user
    println!("=== DELETE: Remove user 1003 ===");
    let delete_status = wallet.remove_user(1003);
    println!("Delete status: {:?}", delete_status);
    println!("Remaining users: {}\n", wallet.list_all_users().len());

    // Try to access deleted user
    println!("=== Verify deletion ===");
    match wallet.get_user(1003) {
        Some(_) => println!("User 1003 still exists"),
        None => println!("User 1003 successfully deleted"),
    }
}
