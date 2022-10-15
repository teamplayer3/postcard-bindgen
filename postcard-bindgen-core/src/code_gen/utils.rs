use genco::{prelude::js::Tokens, quote};

pub fn semicolon_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join (; ) => $part))
}

pub fn comma_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join (, ) => $part))
}

pub fn line_brake_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ($['\n']) => $part))
}

#[allow(unused)]
pub fn joined_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    parts.into_iter().fold(Tokens::new(), |mut res, p| {
        res.append(p);
        res
    })
}

pub fn and_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ( && ) => $part))
}

pub fn or_chain(parts: impl IntoIterator<Item = Tokens>) -> Tokens {
    quote!($(for part in parts join ( || ) => $part))
}

#[cfg(test)]
mod test {
    use genco::quote;

    use super::{and_chain, or_chain};

    #[test]
    fn test_and_chain() {
        let parts = vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            and_chain(parts).to_string().unwrap(),
            "true === true && false === false"
        )
    }

    #[test]
    fn test_or_chain() {
        let parts = vec![quote!(true === true), quote!(false === false)];
        assert_eq!(
            or_chain(parts).to_string().unwrap(),
            "true === true || false === false"
        )
    }
}
