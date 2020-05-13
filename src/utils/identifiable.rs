pub trait Identifiable {
    type Id;

    fn id(&self) -> Self::Id;
}
