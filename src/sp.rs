use core::{
    mem::{self, MaybeUninit},
    pin::Pin,
};

use crate::{
    generics::{Generics, Type},
    project::Project,
};

macro_rules! sp {
    ($this:ident, $ptr:ident, $generics:ty, $tuple:expr $(,)?) => {{
        let $this: Pin<&$generics> =
            unsafe { Pin::new_unchecked(&*$ptr.cast::<$generics>()) };
        #[allow(unused_variables)]
        let $this = Pin::get_ref($this);

        unsafe { mem::transmute_copy(&$tuple) }
    }};
}

macro_rules! sp_mut {
    (
        $output:ty,
        $this:ident,
        $ptr:ident,
        ($($generic:ty),* $(,)?),
        $tuple:expr $(,)?
    ) => {{
        let $this: Pin<&mut ($($generic,)*)> =
            unsafe { Pin::new_unchecked(&mut *$ptr.cast::<($($generic,)*)>()) };
        #[allow(unused_variables)]
        let $this = unsafe { Pin::get_unchecked_mut($this) };
        let mut output = MaybeUninit::<$output>::uninit();

        unsafe {
            *output.as_mut_ptr().cast::<($(Pin<&mut $generic>),* ,)>() = $tuple;
            output.assume_init()
        }
    }};
}

/// `Sp` stands for "Structurally Pinned"
///
/// Functionality is exclusively exposed as associated functions to prevent name
/// collisions with [`Pin`] methods.
#[repr(transparent)]
pub struct Sp<T>(T);

impl<T> Sp<T> {
    /// Create a new structurally-pinned wrapper.
    #[inline]
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Get a pin-projected reference to the inner value.
    #[inline]
    pub const fn get(this: Pin<&Self>) -> Pin<&T> {
        // SAFETY: Value is never moved out of the reference (pointer
        // exclusively used for projection).
        let this = Pin::get_ref(this);
        let inner = &this.0;

        // SAFETY: `self.0` is pinned whenever `self` is
        unsafe { Pin::new_unchecked(inner) }
    }

    /// Get a pin-projected mutable reference to the inner value.
    #[inline]
    pub const fn get_mut(this: Pin<&mut Self>) -> Pin<&mut T> {
        // SAFETY: Value is never moved out of the reference (pointer
        // exclusively used for projection).
        let this = unsafe { Pin::get_unchecked_mut(this) };
        let inner = &mut this.0;

        // SAFETY: `self.0` is pinned whenever `self` is
        unsafe { Pin::new_unchecked(inner) }
    }

    /// Project all elements (supports tuples with up to 7 elements).
    #[inline]
    pub const fn project(this: Pin<&Self>) -> T::Projected<'_>
    where
        T: Project,
        Type<T>: Generics,
    {
        let ptr: *const T = Sp::get(this).get_ref();

        match T::TYPE {
            Type::Unit(_) => sp!(this, ptr, (), ()),
            Type::TupleA => {
                sp!(
                    this,
                    ptr,
                    (<Type<T> as Generics>::A,),
                    (Pin::new_unchecked(&this.0),),
                )
            }
            Type::TupleB => {
                sp!(
                    this,
                    ptr,
                    (<Type<T> as Generics>::A, <Type<T> as Generics>::B),
                    (Pin::new_unchecked(&this.0), Pin::new_unchecked(&this.1)),
                )
            }
            Type::TupleC => {
                sp!(
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                    ),
                    (
                        Pin::new_unchecked(&this.0),
                        Pin::new_unchecked(&this.1),
                        Pin::new_unchecked(&this.2),
                    ),
                )
            }
            Type::TupleD => {
                sp!(
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                    ),
                    (
                        Pin::new_unchecked(&this.0),
                        Pin::new_unchecked(&this.1),
                        Pin::new_unchecked(&this.2),
                        Pin::new_unchecked(&this.3),
                    ),
                )
            }
            Type::TupleE => {
                sp!(
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                    ),
                    (
                        Pin::new_unchecked(&this.0),
                        Pin::new_unchecked(&this.1),
                        Pin::new_unchecked(&this.2),
                        Pin::new_unchecked(&this.3),
                        Pin::new_unchecked(&this.4),
                    ),
                )
            }
            Type::TupleF => {
                sp!(
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                        <Type<T> as Generics>::F,
                    ),
                    (
                        Pin::new_unchecked(&this.0),
                        Pin::new_unchecked(&this.1),
                        Pin::new_unchecked(&this.2),
                        Pin::new_unchecked(&this.3),
                        Pin::new_unchecked(&this.4),
                        Pin::new_unchecked(&this.5),
                    ),
                )
            }
            Type::TupleG => {
                sp!(
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                        <Type<T> as Generics>::F,
                        <Type<T> as Generics>::G,
                    ),
                    (
                        Pin::new_unchecked(&this.0),
                        Pin::new_unchecked(&this.1),
                        Pin::new_unchecked(&this.2),
                        Pin::new_unchecked(&this.3),
                        Pin::new_unchecked(&this.4),
                        Pin::new_unchecked(&this.5),
                        Pin::new_unchecked(&this.6),
                    ),
                )
            }
        }
    }

    /// Project all elements mutably (supports tuples with up to 7 elements).
    #[inline]
    pub const fn project_mut(this: Pin<&mut Self>) -> T::ProjectedMut<'_>
    where
        T: Project,
        Type<T>: Generics,
    {
        let ptr: *mut T = unsafe { Self::get_mut(this).get_unchecked_mut() };

        match T::TYPE {
            Type::Unit(_) => unsafe { mem::transmute_copy(&()) },
            Type::TupleA => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (<Type<T> as Generics>::A,),
                    (Pin::new_unchecked(&mut this.0),),
                )
            }
            Type::TupleB => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (<Type<T> as Generics>::A, <Type<T> as Generics>::B),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                    ),
                )
            }
            Type::TupleC => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                    ),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                        Pin::new_unchecked(&mut this.2),
                    ),
                )
            }
            Type::TupleD => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                    ),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                        Pin::new_unchecked(&mut this.2),
                        Pin::new_unchecked(&mut this.3),
                    ),
                )
            }
            Type::TupleE => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                    ),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                        Pin::new_unchecked(&mut this.2),
                        Pin::new_unchecked(&mut this.3),
                        Pin::new_unchecked(&mut this.4),
                    ),
                )
            }
            Type::TupleF => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                        <Type<T> as Generics>::F,
                    ),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                        Pin::new_unchecked(&mut this.2),
                        Pin::new_unchecked(&mut this.3),
                        Pin::new_unchecked(&mut this.4),
                        Pin::new_unchecked(&mut this.5),
                    ),
                )
            }
            Type::TupleG => {
                sp_mut!(
                    T::ProjectedMut<'_>,
                    this,
                    ptr,
                    (
                        <Type<T> as Generics>::A,
                        <Type<T> as Generics>::B,
                        <Type<T> as Generics>::C,
                        <Type<T> as Generics>::D,
                        <Type<T> as Generics>::E,
                        <Type<T> as Generics>::F,
                        <Type<T> as Generics>::G,
                    ),
                    (
                        Pin::new_unchecked(&mut this.0),
                        Pin::new_unchecked(&mut this.1),
                        Pin::new_unchecked(&mut this.2),
                        Pin::new_unchecked(&mut this.3),
                        Pin::new_unchecked(&mut this.4),
                        Pin::new_unchecked(&mut this.5),
                        Pin::new_unchecked(&mut this.6),
                    ),
                )
            }
        }
    }
}
