pub trait Identifier: Send + Sync {
    fn parse(&self) -> (&str, &str);
}

impl Identifier for String {
    fn parse(&self) -> (&str, &str) {
        let mut split = self.split(":");
        (split.next().unwrap(), split.next().unwrap())
    }
}
