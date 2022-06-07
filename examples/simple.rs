mod lib {
    use core::pin::Pin;

    use projections::Sp;

    pub struct StructurallyPinnedStruct(
        Pin<Box<Sp<u32, String, Option<&'static [u8]>>>>,
    );

    impl StructurallyPinnedStruct {
        pub fn new(a: u32, b: String, c: Option<&'static [u8]>) -> Self {
            let inner = Sp::from_a(a).with_b(b).with_c(c);
            Self(Box::pin(inner))
        }

        pub fn use_pinned_types(&mut self) {
            let _a: Pin<&mut u32> = self.0.as_mut().a();
            let _b: Pin<&mut String> = self.0.as_mut().b();
            let _c: Pin<&mut Option<&'static [u8]>> = self.0.as_mut().c();
        }
    }
}

use lib::StructurallyPinnedStruct;

fn main() {
    let mut structurally_pinned_struct =
        StructurallyPinnedStruct::new(4, "eight".to_string(), Some(b"15"));

    structurally_pinned_struct.use_pinned_types();
}
