#[derive(Debug)]   // This is a derive attribute that allows us to print the struct with the debug format
struct Account{
    id: u32,
    balance: i32,
    holder: String,
}

impl Account{
    fn new(id:u32, holder: String) -> Self{
        Account{id, balance: 0, holder}
    }
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Self{
        Bank {accounts: vec![]}
    }
}

fn print_account(account:Account){
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, "Alice".to_string());

    print_account(account);
}
