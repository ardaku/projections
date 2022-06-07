mod lib {
    use core::pin::Pin;

    use projections::Sp;

    // Sealed unique type to make namespace collisions impossible.
    pub struct Seal;

    pub type StructurallyPinnedStruct =
        Sp<u32, String, Option<&'static [u8]>, Seal>;

    pub struct StructurallyPinnedStructBuilder {
        pub a: u32,
        pub b: String,
        pub c: Option<&'static [u8]>,
    }

    impl StructurallyPinnedStructBuilder {
        pub fn build(self) -> StructurallyPinnedStruct {
            let Self { a, b, c } = self;
            Sp::from_a(a).with_b(b).with_c(c).with_d(Seal)
        }
    }

    pub trait StructurallyPinnedStructApi {
        fn use_pinned_types(self: Pin<&mut Self>);
    }

    impl StructurallyPinnedStructApi for StructurallyPinnedStruct {
        fn use_pinned_types(mut self: Pin<&mut Self>) {
            let _a: Pin<&mut u32> = self.as_mut().a();
            let _b: Pin<&mut String> = self.as_mut().b();
            let _c: Pin<&mut Option<&'static [u8]>> = self.as_mut().c();
        }
    }
}

use core::pin::Pin;

use lib::{StructurallyPinnedStructApi, StructurallyPinnedStructBuilder};

fn main() {
    let mut structurally_pinned_struct = StructurallyPinnedStructBuilder {
        a: 4,
        b: "eight".to_string(),
        c: Some(b"15"),
    }
    .build();

    Pin::new(&mut structurally_pinned_struct).use_pinned_types();
}
