mod domain;
mod infrastructure;

use domain::create_account;

use crate::infrastructure::account_repo::{AccountRepo, InMemoryAccountRepo};

fn main() {
    let mut repo = InMemoryAccountRepo::new();

    let account_name1 = String::from("Bank");
    let create_account_request1 = create_account::CreateAccountRequest {
        name: account_name1,
    };
    let create_account_response1 =
        create_account::create_account(&mut repo, create_account_request1);

    let account_name2 = String::from("Bank2");
    let create_account_request2 = create_account::CreateAccountRequest {
        name: account_name2,
    };
    let create_account_response2 =
        create_account::create_account(&mut repo, create_account_request2);

    let account_name3 = String::from("");
    let create_account_request3 = create_account::CreateAccountRequest {
        name: account_name3,
    };
    let create_account_response3 =
        create_account::create_account(&mut repo, create_account_request3);

    let in_repo = repo.fetch_all().unwrap();
    println!("{:?}", in_repo);

    println!("{:?}", create_account_response1);
    println!("{:?}", create_account_response2);
    println!("{:?}", create_account_response3);
}
