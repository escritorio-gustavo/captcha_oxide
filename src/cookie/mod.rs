use itertools::Itertools;
use std::borrow::Cow;

pub struct Cookie<'a>(pub Cow<'a, str>, pub Cow<'a, str>);

impl<'a> Cookie<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Self(name.into(), value.into())
    }

    pub fn stringify(&self) -> Cow<'a, str> {
        Cow::Owned(format!("{}={}", self.0, self.1))
    }
}

impl<'a, T, U> From<(T, U)> for Cookie<'a>
where
    T: ToString,
    U: ToString,
{
    fn from(value: (T, U)) -> Self {
        Cookie::new(value.0.to_string(), value.1.to_string())
    }
}

pub struct Cookies<'a> {
    pub iter: Box<[Cookie<'a>]>,
}

impl<'a> Cookies<'a> {
    pub fn stringify(&self) -> Cow<'a, str> {
        self.iter.iter().map(Cookie::stringify).join(";").into()
    }
}

impl<'a, I> FromIterator<I> for Cookies<'a>
where
    I: Into<Cookie<'a>>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self {
            iter: iter.into_iter().map(Into::into).collect(),
        }
    }
}
