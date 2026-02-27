#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }
}

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn print_holder(holder: String) {
    println!("{:#?}", holder);
}

fn main() {
    let bank = Bank::new();

    // let other_bank = bank; // value moves to other variable
    // println!("Bank: {:#?}", bank); // hence it is not allowed

    // let account = Account::new(1, "John Doe".to_string());
    let mut account = Account::new(1, String::from("John Doe"));

    println!("{:#?}", bank);
    account = print_account(account);

    // let list_of_accounts = vec![account]; // account moved to vector list_of_accounts
    // print_account(account); // not allowed

    // let accounts = bank.accoun    // account moved to struct bank
    // print_account("{.#?}", bank.accounts); // not allowed

    // print_account("{:#?}", account.holder); // same logic

    // print_holder(account.holder);

    account = print_account(account);
}
