#[derive(Debug, PartialEq)]
pub struct AccountName(String);

impl TryFrom<String> for AccountName {
    type Error = &'static str;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.len() > 0 {
            Ok(AccountName(name))
        } else {
            Err("Name is empty")
        }
    }
}

impl From<AccountName> for String {
    fn from(name: AccountName) -> String {
        name.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_account_name() {
        let name = AccountName::try_from("test".to_string()).unwrap();
        assert_eq!(name.0, "test");
    }

    #[test]
    fn test_account_name_empty() {
        let name = AccountName::try_from("".to_string());
        assert_eq!(name, Err("Name is empty"));
    }
}
