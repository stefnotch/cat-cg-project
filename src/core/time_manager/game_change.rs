pub trait GameChange
where
    Self: Sync + Send,
{
    // dyn trait is interesting https://doc.rust-lang.org/error_codes/E0038.html#method-references-the-self-type-in-its-parameters-or-return-type
    fn is_similar(&self, other: &Self) -> bool
    where
        Self: Sized;
}
