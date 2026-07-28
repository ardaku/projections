//! #### Create structural pin projections without unsafe or macros.
//!
//! This like
//! [pin-project-lite](https://docs.rs/pin-project-lite/latest/pin_project_lite/)
//! but more lite.
//!
//! # Why Another Pin Projection Crate?
//!
//! Because you want to create structural pin projections without macros or
//! unsafe for some reason (perhaps for fun?).  If you need functionality not
//! supported by this crate, it's worth considering using `pin-project-lite` or
//! `pin-project` instead.
//!
//! # Differences To `pin-project-lite`
//!
//! `pin-project-lite` only supports structs with named fields; `projections`
//! only supports wrapped tuple structs (up to an arity of 7).
//!
//! `pin-project-lite` only projects fields annotated with `#[pin]`;
//! `projections` always projects all fields.
//!
//! `pin-project-lite` might not have the best error messages; `projections`
//! error messages should be relatively good.
//!
//! # Getting Started
//!
//! Structurally pin a tuple inside of an [`Sp`]:
//!
//! ```rust
//! use std::pin::{pin, Pin};
//!
//! use projections::Sp;
//!
//! // Create structurally-pinned type
//! let mut sp: Pin<&mut Sp<(u32, String)>> = pin!(
//!     Sp::new((12u32, "Hi".to_string())),
//! );
//!
//! // Project entire inner tuple
//! let _inner: Pin<&(u32, String)> = Sp::get(sp.as_ref());
//! let _inner: Pin<&mut (u32, String)> = Sp::get_mut(sp.as_mut());
//!
//! // Immutable projection of tuple elements
//! let (int, string): (Pin<&u32>, Pin<&String>) = Sp::project(sp.as_ref());
//!
//! assert_eq!(*int.get_ref(), 12);
//! assert_eq!(*string.get_ref(), "Hi");
//!
//! // Mutable projection of tuple elements
//! let (int, string): (Pin<&mut u32>, Pin<&mut String>) = Sp::project_mut(sp);
//!
//! assert_eq!(*int.get_mut(), 12);
//! assert_eq!(string.get_mut(), "Hi");
//! ```
//!
//! ## Orphan Rule: Alloc
//!
//! Due to the orphan rule, either alloc, unsafe, or macros are required to
//! implement [`Future`] or other traits usually requiring pinned references
//! into a structurally-pinned type on a newtype.
//!
//! ```rust
//! use std::{pin::Pin, task::{Context, Poll}};
//!
//! use projections::Sp;
//!
//! pub struct MyFuture<F>(Pin<Box<Sp<(F,)>>>);
//!
//! impl<F> Future for MyFuture<F>
//! where
//!     F: Future
//! {
//!     type Output = F::Output;
//!
//!     fn poll(
//!         mut self: Pin<&mut Self>,
//!         cx: &mut Context<'_>,
//!     ) -> Poll<F::Output> {
//!         Sp::project_mut(self.0.as_mut()).0.poll(cx)
//!     }
//! }
//!
//! # pasts::Executor::default().block_on(async {
//! let output = MyFuture(Box::pin(Sp::new((async { "uwu" },)))).await;
//!
//! assert_eq!(output, "uwu");
//! # });
//! ```
//!
//! ## Orphan Rule: Unsafe
//!
//! Using `unsafe` to get around the orphan rule (with the [`as_repr`] crate):
//!
//! ```rust
//! use std::{pin::Pin, task::{Context, Poll}};
//!
//! use as_repr::AsRepr;
//! use projections::Sp;
//!
//! // Marker to prevent consumers from invalidating invariants
//! struct PrivateMarker;
//!
//! #[repr(transparent)]
//! pub struct MyFuture<F>(Sp<(F, PrivateMarker)>);
//!
//! // SAFETY: `MyFuture` is `repr(transparent)`
//! unsafe impl<F> AsRepr<Pin<&mut Sp<(F, PrivateMarker)>>>
//!     for Pin<&mut MyFuture<F>>
//! {}
//!
//! impl<F> Future for MyFuture<F>
//! where
//!     F: Future
//! {
//!     type Output = F::Output;
//!
//!     fn poll(
//!         mut self: Pin<&mut Self>,
//!         cx: &mut Context<'_>,
//!     ) -> Poll<F::Output> {
//!         let mut sp: Pin<&mut Sp<(F, PrivateMarker)>>
//!             = as_repr::as_repr(self);
//!
//!         Sp::project_mut(sp.as_mut()).0.poll(cx)
//!     }
//! }
//!
//! # pasts::Executor::default().block_on(async {
//! let output = MyFuture(Sp::new((async { "uwu" }, PrivateMarker))).await;
//!
//! assert_eq!(output, "uwu");
//! # });
//! ```
//!
//! ## Orphan Rule: Macros
//!
//! Using macros to get around the orphan rule (with the [`as_repr`] crate):
//!
//! ```rust
//! use std::{pin::Pin, task::{Context, Poll}};
//!
//! use as_repr::AsRepr;
//! use projections::Sp;
//!
//! // Marker to prevent consumers from invalidating invariants
//! struct PrivateMarker;
//!
//! as_repr::transparent_newtype! {
//!     pub struct MyFuture<F>(Sp<(F, PrivateMarker)>);
//! }
//!
//! impl<F> Future for MyFuture<F>
//! where
//!     F: Future
//! {
//!     type Output = F::Output;
//!
//!     fn poll(
//!         mut self: Pin<&mut Self>,
//!         cx: &mut Context<'_>,
//!     ) -> Poll<F::Output> {
//!         let mut sp: Pin<&mut Sp<(F, PrivateMarker)>>
//!             = as_repr::as_repr(self);
//!
//!         Sp::project_mut(sp.as_mut()).0.poll(cx)
//!     }
//! }
//!
//! # pasts::Executor::default().block_on(async {
//! let output = MyFuture(Sp::new((async { "uwu" }, PrivateMarker))).await;
//!
//! assert_eq!(output, "uwu");
//! # });
//! ```
//!
//! [`as_repr`]: https://docs.rs/as_repr

use core::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
    pin::Pin,
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

#[doc(hidden)]
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

#[doc(hidden)]
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
