#![feature(generic_associated_types)]

fn main() {
    println!("Hello, world!");
}

trait MonoFunctor {
    type Unwrapped;
    fn map<F>(self, f: F) -> Self
        where F: FnMut(Self::Unwrapped) -> Self::Unwrapped;
}

impl<A> MonoFunctor for Option<A> {
    type Unwrapped = A;
    fn map<F: FnMut(A) -> A>(self, mut f: F) -> Option<A> {
        match self {
            Some(a) => Some(f(a)),
            None => None,
        }
    }
}

trait Functor {
    type Unwrapped;
    type Wrapped<B>: Functor; // GAT

    fn map<F, B>(self, f: F) -> Self::Wrapped<B>
        where F: FnMut(Self::Unwrapped) -> B;
}

impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Wrapped<B> = Option<B>;

    fn map<F: FnMut(A) -> B, B>(self, mut f: F) -> Option<B> {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

#[test]
fn test_option_map() {
    assert_eq!(Some(5).map(|x| x + 1), Some(6));
    assert_eq!(None.map(|x: i32| x + 1), None);
}

//trait HktFunctor {
//    fn map<A, B, F: FnMut(A) -> B>(self: Self<A>, f: F) -> Self<B>;
//}
//
//impl HktFunctor for Option {
//    fn map<A, B, F: FnMut(A) -> B>(self: Option<A>, f: F) -> Option<B> {
//        match self {
//            Some(a) => Some(f(a)),
//            None => None,
//        }
//    }
//}
