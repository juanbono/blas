/// See https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html
pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
}

impl<T: ?Sized> Upcast<T> for T {
    fn upcast(&self) -> &T {
        self
    }
}

pub trait UpcastMut<T: ?Sized> {
    fn upcast_mut(&mut self) -> &mut T;
}

impl<T: ?Sized> UpcastMut<T> for T {
    fn upcast_mut(&mut self) -> &mut T {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Upcast;

    trait Reader {}
    trait Writer: Reader {}

    struct Foo;

    impl Reader for Foo {}

    #[test]
    fn test_trait_upcasting() {}
}
