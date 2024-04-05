macro_rules! enum_display_impl {
    ($name: ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = serde_json::to_string(self).unwrap();
                write!(f, "{}", &s[1..s.len() - 1])
            }
        }
    };
}

pub(crate) use enum_display_impl;
