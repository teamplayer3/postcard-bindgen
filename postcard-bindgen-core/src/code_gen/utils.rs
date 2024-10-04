use genco::{lang::Lang, quote, Tokens};

pub fn semicolon_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join (; ) => $part))
}

pub fn comma_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join (, ) => $part))
}

pub fn line_break_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join ($['\n']) => $part))
}

#[allow(unused)]
pub fn joined_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    parts.into_iter().fold(Tokens::new(), |mut res, p| {
        res.append(p);
        res
    })
}

pub fn and_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join ( && ) => $part))
}

pub fn or_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join ( || ) => $part))
}

pub fn colon_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join ( : ) => $part))
}

pub fn divider_chain<L: Lang>(parts: impl IntoIterator<Item = Tokens<L>>) -> Tokens<L> {
    quote!($(for part in parts join ( | ) => $part))
}

pub fn wrapped_brackets<L: Lang>(inner: Tokens<L>) -> Tokens<L> {
    quote!(($inner))
}

pub fn wrapped_curly_brackets<L: Lang>(inner: Tokens<L>) -> Tokens<L> {
    quote!({ $inner })
}

#[cfg(test)]
mod test {
    use genco::quote;

    use super::{and_chain, or_chain};

    #[test]
    fn test_and_chain() {
        let parts: Vec<genco::prelude::Tokens<()>> =
            vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            and_chain(parts).to_string().unwrap(),
            "true === true && false === false"
        )
    }

    #[test]
    fn test_or_chain() {
        let parts: Vec<genco::prelude::Tokens<()>> =
            vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            or_chain(parts).to_string().unwrap(),
            "true === true || false === false"
        )
    }
}
