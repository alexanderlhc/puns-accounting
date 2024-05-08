use crate::domain::{
    create_account::Account,
    value_objects::{account_id::AccountId, account_name::AccountName},
};

#[derive(Debug)]
pub enum AccountRepoError {
    CreateError(String),
}

pub trait AccountRepo {
    fn create(&mut self, name: &String) -> Result<u32, AccountRepoError>;
    fn fetch(&self, id: u32) -> Result<Account, AccountRepoError>;
    fn fetch_all(&self) -> Result<Vec<Account>, AccountRepoError>;
}

pub struct InMemoryAccountRepo {
    accounts: Vec<Account>,
}

impl InMemoryAccountRepo {
    pub fn new() -> Self {
        InMemoryAccountRepo {
            accounts: Vec::new(),
        }
    }
}

impl AccountRepo for InMemoryAccountRepo {
    fn create(&mut self, name: &String) -> Result<u32, AccountRepoError> {
        let id = self.accounts.len() as u32 + 1;
        let account = Account::new(
            AccountId(id),
            AccountName::try_from(name.clone())
                .map_err(|_| AccountRepoError::CreateError("Name is empty".to_string()))?,
        );
        self.accounts.push(account);
        Ok(id)
    }

    fn fetch(&self, _id: u32) -> Result<Account, AccountRepoError> {
        todo!()
    }

    fn fetch_all(&self) -> Result<Vec<Account>, AccountRepoError> {
        let accounts = self.accounts.iter().cloned().collect();
        Ok(accounts)
    }
}
