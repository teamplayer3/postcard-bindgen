pub mod impls;

use genco::prelude::js::Tokens;

pub trait AccessorGenerateable {
    fn gen_ser_accessor(
        &self,
        field_access: ser::InnerTypeAccess,
        field_accessor: ser::FieldAccessor,
    ) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens;

    fn gen_ty_check(
        &self,
        field_access: ty_check::FieldAccess,
        inner_access: ty_check::InnerTypeAccess,
    ) -> Tokens;
}

pub mod ser {
    use genco::{prelude::JavaScript, quote_in, tokens::FormatInto};

    use crate::code_gen::JS_ENUM_VARIANT_VALUE;

    #[derive(Debug, Clone, Copy)]
    pub enum FieldAccessor<'a> {
        Object(&'a str),
        Array(usize),
        Direct,
    }

    impl FormatInto<JavaScript> for FieldAccessor<'_> {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    FieldAccessor::Array(i) => [$i],
                    FieldAccessor::Object(n) => .$n,
                    FieldAccessor::Direct => ()
                })
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum InnerTypeAccess {
        Direct,
        EnumInner,
    }

    impl FormatInto<JavaScript> for InnerTypeAccess {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    InnerTypeAccess::Direct => (),
                    InnerTypeAccess::EnumInner => .$JS_ENUM_VARIANT_VALUE
                })
            }
        }
    }
}

pub mod des {
    use genco::{
        prelude::{js::Tokens, JavaScript},
        quote_in,
        tokens::FormatInto,
    };

    #[derive(Debug, Clone, Copy)]
    pub enum FieldAccessor<'a> {
        Object(&'a str),
        Array,
        None,
    }

    impl<'a> FormatInto<JavaScript> for FieldAccessor<'a> {
        fn format_into(self, tokens: &mut Tokens) {
            quote_in! { *tokens =>
                $(match self {
                    Self::Array | Self::None => (),
                    Self::Object(n) => $n:$[' '],
                })
            }
        }
    }
}

pub mod ty_check {
    use genco::{
        prelude::JavaScript,
        quote_in,
        tokens::{quoted, FormatInto},
    };

    use crate::code_gen::JS_ENUM_VARIANT_VALUE;

    #[derive(Clone)]
    pub enum AvailableCheck<'a> {
        Object(&'a str, InnerTypeAccess),
        None,
    }

    impl<'a> AvailableCheck<'a> {
        pub fn from_field_access_and_inner_type_access(
            fa: FieldAccess<'a>,
            ita: InnerTypeAccess,
        ) -> Self {
            match fa {
                FieldAccess::Array(_) => Self::None,
                FieldAccess::Object(name) => Self::Object(name, ita),
            }
        }
    }

    impl FormatInto<JavaScript> for AvailableCheck<'_> {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    AvailableCheck::Object(field_name, inner_access) => $(quoted(field_name)) in v$inner_access,
                    AvailableCheck::None => ()
                })
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum FieldAccess<'a> {
        Object(&'a str),
        Array(usize),
    }

    impl FormatInto<JavaScript> for FieldAccess<'_> {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    FieldAccess::Array(i) => [$i],
                    FieldAccess::Object(n) => .$n
                })
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum InnerTypeAccess {
        Direct,
        EnumInner,
    }

    impl FormatInto<JavaScript> for InnerTypeAccess {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    InnerTypeAccess::Direct => (),
                    InnerTypeAccess::EnumInner => .$JS_ENUM_VARIANT_VALUE
                })
            }
        }
    }
}
