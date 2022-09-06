#[macro_export]
macro_rules! enum_str {
    (
        pub enum $name:ident {
            $($variant:ident),*,
        }
    ) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }

            pub fn from_string(string: &str) -> Option<Self> {
                match string {
                    $(stringify!($variant) => Some($name::$variant)),*,
                    _ => None
                }
            }
        }
    };
}
