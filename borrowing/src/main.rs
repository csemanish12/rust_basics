fn main() {
    // Immutable
    let x: i32 = 5;
    let r: &i32 = &x;
    println!("Value of x: {}", x);
    println!("Value of r: {}", r);

    // mutable reference
    let mut var1 = 10;
    let var1_borrower: &mut i32 = &mut var1; // mutable borrow

    *var1_borrower += 1;  
    
    println!("Value of var1: {}", var1); // immutable borrow is happening here. cannot do both at the same time
    // println!("Value of var1_borrower: {}", var1_borrower); // mutable borrow used


    // demonstrating on one mutable reference or many immutable references
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // immutable borrow to check balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(50.0);


    // immutable borrow to check balance
    account.check_balance();


}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    // borrow happening here are in their own scope and therefore don't overlap
    // there is no simulataneus mutable and immutable borrowing
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has balance {}", self.owner, self.balance);
    }

}
