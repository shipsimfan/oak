use std::{borrow::Cow, str::FromStr};

/// The type of list for filtering scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterListType {
    /// Only logs scopes in the list
    Whitelist,

    /// Logs all scopes except those in the list
    Blacklist,
}

pub struct UnknownFilterListType;

impl FilterListType {
    /// Checks if `item` should be allowed based on the `filter_list` and this type
    pub(super) fn filter(&self, item: &str, filter_list: &[Cow<'static, str>]) -> bool {
        let mut contains = false;
        for filter in filter_list {
            if filter == item {
                contains = true;
                break;
            }
        }

        match self {
            FilterListType::Blacklist => !contains,
            FilterListType::Whitelist => contains,
        }
    }
}

impl FromStr for FilterListType {
    type Err = UnknownFilterListType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const TYPES: &[(&str, FilterListType)] = &[
            ("white", FilterListType::Whitelist),
            ("whitelist", FilterListType::Whitelist),
            ("black", FilterListType::Blacklist),
            ("blacklist", FilterListType::Blacklist),
        ];

        fn case_insensitive_compare(a: &str, b: &str) -> bool {
            if a.len() != b.len() {
                return false;
            }

            let mut a = a.chars();
            let mut b = b.chars();
            while let Some(a) = a.next() {
                let b = b.next().unwrap();

                if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                    return false;
                }
            }

            true
        }

        for (type_str, r#type) in TYPES {
            if case_insensitive_compare(s, type_str) {
                return Ok(*r#type);
            }
        }

        Err(UnknownFilterListType)
    }
}

impl std::error::Error for UnknownFilterListType {}

impl std::fmt::Display for UnknownFilterListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown filter list type")
    }
}

impl std::fmt::Debug for UnknownFilterListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
