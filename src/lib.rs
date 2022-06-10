//! Safe structural pin projections without macros.
//!
//! This like
//! [pin-project-lite](https://docs.rs/pin-project-lite/latest/pin_project_lite/)
//! but more lite.
//!
//! # Why
//! Because you want safe structural pin projections without macros for some
//! reason.
//!
//! # Getting Started
//! Here's an example of how you would create a public struct in your API that
//! uses pin projection internally via [`Sp`].  This one goes out of it's way
//! to not allocate (the [allocating version](#allocating-version) is simpler).
//!
//! ```rust
#![doc = include_str!("../examples/noalloc.rs")]
//! ```
//! 
//! # Allocating Version
//! ```rust
#![doc = include_str!("../examples/simple.rs")]
//! ```

#![no_std]

use core::pin::Pin;

/// Sp stands for Structurally Pinned
///
/// Up to 8 generics can be supplied for 8 structurally pinned fields.
pub struct Sp<A, B = (), C = (), D = (), E = (), F = (), G = (), H = ()> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
}

impl<A> Sp<A> {
    /// Create a new `Sp` with a single field
    pub fn from_a(a: A) -> Self {
        Sp {
            a,
            b: (),
            c: (),
            d: (),
            e: (),
            f: (),
            g: (),
            h: (),
        }
    }

    /// Add field `B` to an `Sp`
    pub fn with_b<B>(self, b: B) -> Sp<A, B> {
        Sp {
            a: self.a,
            b,
            c: (),
            d: (),
            e: (),
            f: (),
            g: (),
            h: (),
        }
    }
}

impl<A, B> Sp<A, B> {
    /// Add field `C` to an `Sp`
    pub fn with_c<C>(self, c: C) -> Sp<A, B, C> {
        Sp {
            a: self.a,
            b: self.b,
            c,
            d: (),
            e: (),
            f: (),
            g: (),
            h: (),
        }
    }
}

impl<A, B, C> Sp<A, B, C> {
    /// Add field `D` to an `Sp`
    pub fn with_d<D>(self, d: D) -> Sp<A, B, C, D> {
        Sp {
            a: self.a,
            b: self.b,
            c: self.c,
            d,
            e: (),
            f: (),
            g: (),
            h: (),
        }
    }
}

impl<A, B, C, D> Sp<A, B, C, D> {
    /// Add field `E` to an `Sp`
    pub fn with_e<E>(self, e: E) -> Sp<A, B, C, D, E> {
        Sp {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e,
            f: (),
            g: (),
            h: (),
        }
    }
}

impl<A, B, C, D, E> Sp<A, B, C, D, E> {
    /// Add field `F` to an `Sp`
    pub fn with_f<F>(self, f: F) -> Sp<A, B, C, D, E, F> {
        Sp {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f,
            g: (),
            h: (),
        }
    }
}

impl<A, B, C, D, E, F> Sp<A, B, C, D, E, F> {
    /// Add field `G` to an `Sp`
    pub fn with_g<G>(self, g: G) -> Sp<A, B, C, D, E, F, G> {
        Sp {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            g,
            h: (),
        }
    }
}

impl<A, B, C, D, E, F, G> Sp<A, B, C, D, E, F, G> {
    /// Add field `H` to an `Sp`
    pub fn with_h<H>(self, h: H) -> Sp<A, B, C, D, E, F, G, H> {
        Sp {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            g: self.g,
            h,
        }
    }
}

impl<A, B, C, D, E, F, G, H> Sp<A, B, C, D, E, F, G, H> {
    /// Get a `Pin<&mut A>`
    pub fn a(self: Pin<&mut Self>) -> Pin<&mut A> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.a) }
    }

    /// Get a `Pin<&mut B>`
    pub fn b(self: Pin<&mut Self>) -> Pin<&mut B> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.b) }
    }

    /// Get a `Pin<&mut C>`
    pub fn c(self: Pin<&mut Self>) -> Pin<&mut C> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.c) }
    }

    /// Get a `Pin<&mut D>`
    pub fn d(self: Pin<&mut Self>) -> Pin<&mut D> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.d) }
    }

    /// Get a `Pin<&mut E>`
    pub fn e(self: Pin<&mut Self>) -> Pin<&mut E> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.e) }
    }

    /// Get a `Pin<&mut F>`
    pub fn f(self: Pin<&mut Self>) -> Pin<&mut F> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.f) }
    }

    /// Get a `Pin<&mut G>`
    pub fn g(self: Pin<&mut Self>) -> Pin<&mut G> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.g) }
    }

    /// Get a `Pin<&mut H>`
    pub fn h(self: Pin<&mut Self>) -> Pin<&mut H> {
        // unsafe: This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.h) }
    }
}
