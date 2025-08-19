trait Monad<T> {
    type Wrapped<U>: Monad<U>;
    fn identity(value: T) -> Self::Wrapped<T>;
    fn bind<R, F>(self, func: F) -> Self::Wrapped<R>
    where
        F: FnOnce(T) -> Self::Wrapped<R>;
}

struct Real<T> {
    value: T,
}

enum Maybe<T> {
    Real(Real<T>),
    Nonthing,
}

impl<T> Maybe<T> {
    fn nothing() -> Maybe<T> {
        Maybe::Nonthing
    }
}
impl<T> Monad<T> for Maybe<T> {
    type Wrapped<U> = Maybe<U>;
    fn identity(_value: T) -> Self::Wrapped<T> {
        Maybe::Real(Real { value: _value })
    }
    fn bind<R, F>(self, func: F) -> Self::Wrapped<R>
    where
        F: FnOnce(T) -> Self::Wrapped<R>,
    {
        match self {
            Maybe::Nonthing => Maybe::Nonthing,
            Maybe::Real(value) => func(value.value),
        }
    }
}

#[test]
fn test_monad() {
    let add_one = |value: i32| Maybe::identity(value + 1);
    let something = Maybe::identity(10);
    let eleven = something.bind(add_one);
    match eleven {
        Maybe::Nonthing => unreachable!(),
        Maybe::Real(value) => assert_eq!(11, value.value),
    }
    let nothing: Maybe<i32> = Maybe::nothing().bind(add_one);
    match nothing {
        Maybe::Nonthing => (),
        Maybe::Real(_) => unreachable!(),
    }
}
