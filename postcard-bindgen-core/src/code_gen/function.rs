use genco::{lang::Lang, quote, tokens::FormatInto, Tokens};

#[macro_export]
macro_rules! function_args {
    ($(($n:expr, $t:expr)),+ $(,)?) => {

        [$(({
            use genco::tokens::FormatInto;
            let mut tokens = genco::Tokens::new();
            $n.format_into(&mut tokens);
            tokens
        }, {
            use genco::tokens::FormatInto;
            let mut tokens = genco::Tokens::new();
            $t.format_into(&mut tokens);
            tokens
        })),+]
    };
    ($($x:expr),+ $(,)?) => {

        [$({
            use genco::tokens::FormatInto;
            let mut tokens = genco::Tokens::new();
            $x.format_into(&mut tokens);
            tokens
        }),+]
    };

}

pub struct FunctionArg<L>
where
    L: Lang,
{
    pub(super) name: Tokens<L>,
    pub(super) r#type: Option<Tokens<L>>,
}

impl<L> FunctionArg<L>
where
    L: Lang,
{
    pub fn new(name: impl FormatInto<L>, r#type: impl FormatInto<L>) -> Self {
        Self {
            name: quote!($name),
            r#type: Some(quote!($r#type)),
        }
    }

    pub fn new_untyped(name: impl FormatInto<L>) -> Self {
        Self {
            name: quote!($name),
            r#type: None,
        }
    }
}

impl<N, T, L> From<(N, T)> for FunctionArg<L>
where
    L: Lang,
    N: FormatInto<L>,
    T: FormatInto<L>,
{
    fn from((name, r#type): (N, T)) -> Self {
        Self::new(name, r#type)
    }
}

impl<L> From<Tokens<L>> for FunctionArg<L>
where
    L: Lang,
{
    fn from(name: Tokens<L>) -> Self {
        Self::new_untyped(name)
    }
}

pub trait ToArgs<L>
where
    L: Lang,
{
    fn to_args(self) -> Vec<FunctionArg<L>>;
}

impl<L, A> ToArgs<L> for Vec<A>
where
    L: Lang,
    A: Into<FunctionArg<L>>,
{
    fn to_args(self) -> Vec<FunctionArg<L>> {
        self.into_iter().map(|a| a.into()).collect()
    }
}

impl<L> ToArgs<L> for FunctionArg<L>
where
    L: Lang,
{
    fn to_args(self) -> Vec<FunctionArg<L>> {
        vec![self]
    }
}

impl<L> ToArgs<L> for ()
where
    L: Lang,
{
    fn to_args(self) -> Vec<FunctionArg<L>> {
        Vec::new()
    }
}

impl<L, F, const N: usize> ToArgs<L> for [F; N]
where
    L: Lang,
    F: Into<FunctionArg<L>>,
{
    fn to_args(self) -> Vec<FunctionArg<L>> {
        self.into_iter().map(|f| f.into()).collect()
    }
}

impl<'a, L, F, const N: usize> ToArgs<L> for &'a [F; N]
where
    L: Lang,
    &'a F: Into<FunctionArg<L>>,
{
    fn to_args(self) -> Vec<FunctionArg<L>> {
        self.iter().map(|f| f.into()).collect()
    }
}

pub struct Function<L>
where
    L: Lang,
{
    pub(super) args: Vec<FunctionArg<L>>,
    pub(super) name: Tokens<L>,
    pub(super) body: Tokens<L>,
    pub(super) return_type: Option<Tokens<L>>,
    pub(super) doc_string: Option<String>,
}

impl<L> Function<L>
where
    L: Lang,
{
    pub fn new(
        name: impl FormatInto<L>,
        args: impl ToArgs<L>,
        body: impl FormatInto<L>,
        return_type: impl FormatInto<L>,
    ) -> Self {
        Self {
            name: quote!($name),
            args: args.to_args(),
            return_type: Some(quote!($return_type)),
            body: quote!($body),
            doc_string: None,
        }
    }

    pub fn new_untyped(
        name: impl FormatInto<L>,
        args: impl ToArgs<L>,
        body: impl FormatInto<L>,
    ) -> Self {
        Self {
            name: quote!($name),
            args: args.to_args(),
            return_type: None,
            body: quote!($body),
            doc_string: None,
        }
    }

    pub fn with_doc_string(mut self, doc_string: impl ToString) -> Self {
        self.doc_string = Some(doc_string.to_string());
        self
    }
}
