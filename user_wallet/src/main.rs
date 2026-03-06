
mod wallet;
use wallet::{UserWallet, FinancialAccount};

fn main() {
    let mut my_wallet = UserWallet::new(String::from("Alex"), 500.0);

    // Using trait methods
    my_wallet.deposit(150.0);

    match my_wallet.withdraw(100.0) {
        Ok(_) => println!("Withdrawal confirmed."),
        Err(e) => println!("Error: {}", e),
    }

    println!("Current Balance: ${:.2}", my_wallet.balance());
}