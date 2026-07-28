use core::marker::PhantomData;

#[non_exhaustive]
#[derive(Debug)]
pub enum Type<T> {
    Unit(PhantomData<fn() -> T>),
    TupleA,
    TupleB,
    TupleC,
    TupleD,
    TupleE,
    TupleF,
    TupleG,
}

pub trait Generics {
    type A;
    type B;
    type C;
    type D;
    type E;
    type F;
    type G;
}

macro_rules! generics {
    ($($generic:ident),* $(,)?) => {
        generics!($($generic),*; $($generic),*);
    };
    ($($generic:ident),*; $(,)?) => {
        generics!($($generic),*; ());
    };
    ($($generic:ident),*; $a:ty $(,)?) => {
        generics!($($generic),*; $a, ());
    };
    ($($generic:ident),*; $a:ty, $b:ty $(,)?) => {
        generics!($($generic),*; $a, $b, ());
    };
    ($($generic:ident),*; $a:ty, $b:ty, $c:ty $(,)?) => {
        generics!($($generic),*; $a, $b, $c, ());
    };
    ($($generic:ident),*; $a:ty, $b:ty, $c:ty, $d:ty $(,)?) => {
        generics!($($generic),*; $a, $b, $c, $d, ());
    };
    ($($generic:ident),*; $a:ty, $b:ty, $c:ty, $d:ty, $e:ty $(,)?) =>
    {
        generics!($($generic),*; $a, $b, $c, $d, $e, ());
    };
    ($($generic:ident),*; $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty $(,)?) => {
        generics!($($generic),*; $a, $b, $c, $d, $e, $f, ());
    };
    (
        $($generic:ident),*;
        $a:ty,
        $b:ty,
        $c:ty,
        $d:ty,
        $e:ty,
        $f:ty,
        $g:ty $(,)?
    ) => {
        impl<$($generic),*> Generics for Type<($($generic,)*)> {
            type A = $a;
            type B = $b;
            type C = $c;
            type D = $d;
            type E = $e;
            type F = $f;
            type G = $g;
        }
    };
}

generics!();
generics!(A);
generics!(A, B);
generics!(A, B, C);
generics!(A, B, C, D);
generics!(A, B, C, D, E);
generics!(A, B, C, D, E, F);
generics!(A, B, C, D, E, F, G);
