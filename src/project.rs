use core::{marker::PhantomData, pin::Pin};

use crate::generics::Type;

/// Trait indicating the inner elements or fields can be projected.
///
/// This trait is implemented on tuples with up to 7 elements.
///
/// # Safety
///
/// This trait is `unsafe`, as indicating structural pin-projection requires
/// ensuring invariants for safety.  This trait is also sealed, so consumers of
/// this library cannot implement it.
pub unsafe trait Project: Sized {
    type Projected<'a>
    where
        Self: 'a;
    type ProjectedMut<'a>
    where
        Self: 'a;

    const TYPE: Type<Self>;
}

macro_rules! project {
    ($type:expr; $($ty:ident),*) => {
        unsafe impl<$($ty),*> Project for ($($ty,)*) {
            type Projected<'a> = ($(Pin<&'a $ty>,)*) where Self: 'a;
            type ProjectedMut<'a> = ($(Pin<&'a mut $ty>,)*) where Self: 'a;

            const TYPE: Type<Self> = $type;
        }
    };
}

project!(Type::<Self>::Unit(PhantomData););
project!(Type::<Self>::TupleA; A);
project!(Type::<Self>::TupleB; A, B);
project!(Type::<Self>::TupleC; A, B, C);
project!(Type::<Self>::TupleD; A, B, C, D);
project!(Type::<Self>::TupleE; A, B, C, D, E);
project!(Type::<Self>::TupleF; A, B, C, D, E, F);
project!(Type::<Self>::TupleG; A, B, C, D, E, F, G);
