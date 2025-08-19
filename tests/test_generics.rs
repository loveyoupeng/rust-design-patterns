//! # The Monad Pattern
//!
//! This module demonstrates the Monad pattern in Rust, a powerful abstraction for sequencing
//! computations, especially those involving side effects, context, or error handling.
//!
//! A Monad is a design pattern that provides a way to chain operations together while
//! encapsulating the context of those operations. It allows for a clean and composable way
//! to handle common programming patterns like nullability, asynchronous operations,
//! state management, or error propagation.
//!
//! ## Monad Concept from Category Theory
//!
//! In category theory, a monad is an endofunctor (a functor from a category to itself)
//! along with two natural transformations:
//!
//! 1.  **Unit** (or `return`/`identity`): A natural transformation from the identity functor to the monad.
//!     This operation takes a plain value and "lifts" it into the monadic context.
//!     In the provided `Monad` trait, this corresponds to the `identity` function.
//!
//! 2.  **Multiplication** (or `join`): A natural transformation from the composition of the monad with itself to the monad.
//!     This operation "flattens" a nested monadic context. While not directly exposed as `join`,
//!     the `bind` operation implicitly performs this flattening by chaining computations.
//!
//! These operations must satisfy certain associativity and identity laws (monad laws)
//! to ensure predictable behavior. The `bind` operation (also known as `flatMap` or `>>=`)
//! is a central part of the monad, allowing sequential application of functions that
//! return monadic values.
//!
//! ## `Monad` Trait
//!
//! The `Monad` trait defines the core operations:
//!
//! -   `identity(value: T) -> Self::Wrapped<T>`: Lifts a value `T` into the monadic context `Self::Wrapped<T>`.
//!     This is the "unit" or "return" operation.
//! -   `bind<R, F>(self, func: F) -> Self::Wrapped<R>`: Applies a function `func` that takes a value `T`
//!     from the current monadic context and returns a new monadic context `Self::Wrapped<R>`.
//!     This operation handles the unwrapping of the current context, applying the function,
//!     and wrapping the result into a new context, effectively sequencing operations.
//!
//! ## `Maybe` Sample Implementation
//!
//! The `Maybe` enum is a classic example of a monad, representing the presence or absence of a value.
//! It's analogous to `Option` in Rust or `Nullable` types in other languages.
//!
//! -   `Maybe::Real(Real<T>)`: Represents the presence of a value `T`.
//! -   `Maybe::Nonthing`: Represents the absence of a value.
//!
//! The `Maybe` implementation of the `Monad` trait demonstrates how it encapsulates nullability:
//!
//! -   `identity(_value: T)`: Creates a `Maybe::Real` from a given value, lifting it into the `Maybe` context.
//! -   `bind<R, F>(self, func: F)`:
//!     -   If `self` is `Maybe::Nonthing`, it propagates the `Nonthing` state, effectively short-circuiting
//!         the computation.
//!     -   If `self` is `Maybe::Real(value)`, it extracts the inner `value.value`, applies the `func`
//!         to it, and returns the resulting `Maybe<R>`, automatically handling the potential `Nonthing`
//!         result from `func`.
//!
//! This allows for chaining operations on potentially null values without explicit null checks at each step.
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
