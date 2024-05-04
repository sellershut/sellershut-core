#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "surrealdb", any(feature = "users", feature = "categories")))]
use surrealdb_core::sql::Table as SurrealdbTable;

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
/// An enum representing tables available in the database
#[derive(Debug)]
pub enum Table {
    #[cfg(feature = "categories")]
    /// Category
    Category,
    #[cfg(feature = "users")]
    /// User
    User,
    #[cfg(feature = "users")]
    /// Oauth Session
    OauthSession,
    #[cfg(feature = "users")]
    /// Oauth Account Provider
    OauthAccountProvider,
    #[cfg(feature = "users")]
    /// Oauth Account
    OauthAccount,
}
#[cfg(any(feature = "users", feature = "categories"))]
impl std::fmt::Display for Table {
    #[cfg(any(feature = "users", feature = "categories"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "categories")]
                Table::Category => "category",
                #[cfg(feature = "users")]
                Table::User => "user",
                #[cfg(feature = "users")]
                Table::OauthSession => "oauth_session",
                #[cfg(feature = "users")]
                Table::OauthAccountProvider => "oauth_account_provider",
                #[cfg(feature = "users")]
                Table::OauthAccount => "oauth_account",
            }
        )
    }
}

#[cfg(all(feature = "surrealdb", any(feature = "users", feature = "categories")))]
impl From<Table> for SurrealdbTable {
    fn from(value: Table) -> Self {
        SurrealdbTable::from(value.to_string())
    }
}
