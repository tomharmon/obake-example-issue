use std::borrow::Cow;
// comment the below `use` to get this to compile
use foo::Foo;

pub struct Batz(String);

impl<T> From<T> for Batz
where
    T: Into<Cow<'static, str>>,
{
    fn from(s: T) -> Self {
        Self(s.into().to_string())
    }
}
