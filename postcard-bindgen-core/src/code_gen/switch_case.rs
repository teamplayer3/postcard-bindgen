use genco::{lang::Lang, quote, tokens::FormatInto, Tokens};

pub struct Case<L>
where
    L: Lang,
{
    pub(super) case: Tokens<L>,
    pub(super) body: Tokens<L>,
    pub(super) break_after: bool,
}

impl<L> Case<L>
where
    L: Lang,
{
    pub fn new(case: impl FormatInto<L>, body: impl FormatInto<L>) -> Self {
        Self {
            case: quote!($case),
            body: quote!($body),
            break_after: true,
        }
    }

    pub fn new_without_break(case: impl FormatInto<L>, body: impl FormatInto<L>) -> Self {
        Self {
            case: quote!($case),
            body: quote!($body),
            break_after: false,
        }
    }
}

impl<L, C, B> From<(C, B)> for Case<L>
where
    L: Lang,
    C: FormatInto<L>,
    B: FormatInto<L>,
{
    fn from((case, body): (C, B)) -> Self {
        Self::new(case, body)
    }
}

pub struct DefaultCase<L>
where
    L: Lang,
{
    pub(super) body: Tokens<L>,
    pub(super) break_after: bool,
}

impl<L> DefaultCase<L>
where
    L: Lang,
{
    pub fn new_without_break(body: impl FormatInto<L>) -> Self {
        Self {
            body: quote!($body),
            break_after: false,
        }
    }
}

pub struct SwitchCase<L>
where
    L: Lang,
{
    pub(crate) cases: Vec<Case<L>>,
    pub(crate) default_case: Option<DefaultCase<L>>,
    pub(crate) switch_arg: Tokens<L>,
}

impl<L> SwitchCase<L>
where
    L: Lang,
{
    pub fn new(switch_arg: impl FormatInto<L>) -> Self {
        Self {
            cases: Vec::new(),
            switch_arg: quote!($switch_arg),
            default_case: None,
        }
    }

    pub fn extend_cases<C: Into<Case<L>>>(&mut self, cases: impl Iterator<Item = C>) {
        self.cases.extend(cases.into_iter().map(Into::into));
    }

    pub fn default_case(&mut self, default_case: DefaultCase<L>) {
        self.default_case = Some(default_case);
    }
}
