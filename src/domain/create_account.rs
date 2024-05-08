use crate::domain::value_objects::account_id::AccountId;
use crate::domain::value_objects::account_name::AccountName;
use crate::infrastructure::account_repo::AccountRepo;

pub struct CreateAccountRequest {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CreateAccountResponse {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    NameIsEmpty,
    StorageUnknown,
}

pub fn create_account(
    repo: &mut dyn AccountRepo,
    request: CreateAccountRequest,
) -> Result<CreateAccountResponse, CreateAccountError> {
    let name = AccountName::try_from(request.name).map_err(|_| CreateAccountError::NameIsEmpty)?;

    let id = repo.create(name.as_ref());
    match id {
        Ok(id) => Ok(CreateAccountResponse {
            id,
            name: name.into(),
        }),
        Err(_) => Err(CreateAccountError::StorageUnknown),
    }
}

#[derive(Clone, Debug)]
pub struct Account {
    pub number: AccountId,
    pub name: AccountName,
}

impl Account {
    pub fn new(number: AccountId, name: AccountName) -> Self {
        Account { number, name }
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::account_repo::InMemoryAccountRepo;

    use super::*;

    #[test]
    fn test_create_account() {
        let mut repo = InMemoryAccountRepo::new();

        let request = CreateAccountRequest {
            name: "test".to_string(),
        };
        let response = create_account(&mut repo, request).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.name, "test");
    }
    #[test]
    fn test_create_account_empty() {
        let mut repo = InMemoryAccountRepo::new();

        let request = CreateAccountRequest {
            name: "".to_string(),
        };
        let response = create_account(&mut repo, request);
        assert_eq!(response, Err(CreateAccountError::NameIsEmpty));
    }
}
