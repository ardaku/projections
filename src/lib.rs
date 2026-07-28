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

#![no_std]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    rustdoc::redundant_explicit_links
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/v2/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/v2/res/icon.svg"
)]

mod generics;
mod project;
mod sp;

pub use self::{project::Project, sp::Sp};
