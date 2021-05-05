#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NamedResult<T, E> {
    pub name: String,
    pub result: Result<T, E>,
}
