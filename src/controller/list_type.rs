use std::borrow::Cow;

/// The type of list for filtering scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterListType {
    /// Only logs scopes in the list
    Whitelist,

    /// Logs all scopes except those in the list
    Blacklist,
}

impl FilterListType {
    /// Checks if `item` shouldn't be allowed based on the `filter_list` and this type
    pub(super) fn filter(&self, item: &str, filter_list: &[Cow<'static, str>]) -> bool {
        let mut contains = false;
        for filter in filter_list {
            if filter == item {
                contains = true;
                break;
            }
        }

        match self {
            FilterListType::Blacklist => contains,
            FilterListType::Whitelist => !contains,
        }
    }
}
