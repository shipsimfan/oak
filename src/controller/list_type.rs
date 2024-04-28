/// The type of list for filtering scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterListType {
    /// Only logs scopes in the list
    Whitelist,

    /// Logs all scopes except those in the list
    Blacklist,
}

impl Default for FilterListType {
    fn default() -> Self {
        FilterListType::Blacklist
    }
}
