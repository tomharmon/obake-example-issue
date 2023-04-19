#[obake::versioned]
#[obake(version("0.1.0"))]
pub struct Foo {
    s: String,
}

impl Foo {
    fn new(s: String) -> Self {
        Self { s }
    }
}
