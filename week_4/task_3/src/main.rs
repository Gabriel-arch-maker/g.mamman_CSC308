struct BankAccount {
    balance: i32,
}

impl BankAccount {






















































































    fn withdraw(&self, amount: i32) -> i32 {
        &self.balance - amount
    }
    fn deposit(&self, amount: i32) -> i32 {
        &self.balance + amount
    }
}

fn main() {
    println!("Hello, world!");
}
