pub trait AsOption: AsRef<str> {
    fn empty_as_none(self) -> Option<String>;
}
impl<S> AsOption for S
where
    S: AsRef<str>,
{
    fn empty_as_none(self) -> Option<String> {
        let s: &str = self.as_ref();
        if s.is_empty() {
            None
        } else {
            Some(s.to_string())
        }
    }
}
