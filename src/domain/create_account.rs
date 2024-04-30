use crate::domain::value_objects::account_name::AccountName;

pub struct CreateAccountRequest {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CreateAccountResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    NameIsEmpty,
}

pub fn create_account(
    request: CreateAccountRequest,
) -> Result<CreateAccountResponse, CreateAccountError> {
    let name = AccountName::try_from(request.name).map_err(|_| CreateAccountError::NameIsEmpty)?;

    Ok(CreateAccountResponse {
        id: 1,
        name: name.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_account() {
        let request = CreateAccountRequest {
            name: "test".to_string(),
        };
        let response = create_account(request).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.name, "test");
    }
    #[test]
    fn test_create_account_empty() {
        let request = CreateAccountRequest {
            name: "".to_string(),
        };
        let response = create_account(request);
        assert_eq!(response, Err(CreateAccountError::NameIsEmpty));
    }
}
