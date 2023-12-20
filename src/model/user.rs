use super::common::*;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
  #[validate(length(min = 1, max = 20), custom = "validate_username")]
  pub user_name: String,
  #[validate(length(min = 8, max = 20))]
  pub password: String,
  // #[validate(email, contains = "@mail.ntou.edu.tw")]
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
  pub user_name: String,
  pub password_hash: String,
  pub email: String,
  pub user_role: UserRole,
  pub verified: bool,
  pub verification_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
  #[validate(length(min = 1, max = 20), custom = "validate_username")]
  pub user_name: String,
  #[validate(length(min = 8, max = 20))]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
  RegularUser,
  Admin,
}

impl FromRow<'_, SqliteRow> for UserInfo {
  fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
    Ok(UserInfo {
      user_name: row.try_get("user_name")?,
      password_hash: row.try_get("password_hash")?,
      email: row.try_get("email")?,
      user_role: row.try_get("user_role")?,
      verified: row.try_get("verified")?,
      verification_token: row.try_get("verification_token")?,
    })
  }
}

impl<'r> Decode<'r, Sqlite> for UserRole {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
    let value = <&str as Decode<Sqlite>>::decode(value)?;

    match value {
      "RegularUser" => Ok(UserRole::RegularUser),
      "Admin" => Ok(UserRole::Admin),
      _ => Err("Invalid UserRole".into()),
    }
  }
}

impl Type<Sqlite> for UserRole {
  fn type_info() -> SqliteTypeInfo {
    <&str as Type<Sqlite>>::type_info()
  }
}

impl ToString for UserRole {
  fn to_string(&self) -> String {
    match *self {
      UserRole::RegularUser => "RegularUser".to_owned(),
      UserRole::Admin => "Admin".to_owned(),
    }
  }
}

impl FromStr for UserRole {
  type Err = std::io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "RegularUser" => Ok(UserRole::RegularUser),
      "Admin" => Ok(UserRole::Admin),
      _ => Err(std::io::Error::new(
        ErrorKind::InvalidInput,
        "Provided string does not match any UserRole variant",
      )),
    }
  }
}

fn validate_username(user_name: &str) -> Result<(), ValidationError> {
  let regex = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();

  if !regex.is_match(user_name) {
    return Err(ValidationError::new(
      "Username must only contain letters and numbers",
    ));
  }

  Ok(())
}
