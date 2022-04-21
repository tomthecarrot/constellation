pub mod channels;
pub mod dynamic;
pub mod rbox;
pub mod states;
pub mod traits;

macro_rules! prop_iter {
    ($iter_name:ident, $trait_name:ident, $dyn_name:ident) => {
        pub struct $iter_name<S: $trait_name> {
            contract: $crate::contract::ContractDataHandle,
            pos: usize,
            phantom: std::marker::PhantomData<S>,
        }
        impl<S: $trait_name> $iter_name<S> {
            pub fn new(contract: $crate::contract::ContractDataHandle) -> Self {
                Self {
                    contract,
                    pos: 0,
                    phantom: Default::default(),
                }
            }
        }

        impl<S: $trait_name> Iterator for $iter_name<S> {
            type Item = $dyn_name;

            fn next(&mut self) -> Option<Self::Item> {
                let prop_type = S::enumerate_types().get(self.pos).copied()?;
                let result = $dyn_name::new(self.contract, self.pos, prop_type);
                self.pos += 1;
                Some(result)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let n_remaining = S::enumerate_types().len() - self.pos;
                (n_remaining, Some(n_remaining))
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let n_fields = S::enumerate_types().len();
                let new_pos = self.pos + n;
                if new_pos >= n_fields {
                    self.pos = n_fields;
                    return None;
                }
                let prop_type = S::enumerate_types()
                    .get(self.pos)
                    .copied()
                    .expect("Should be impossible to be `None`");
                self.pos += 1; // also discard the element at the new position
                Some($dyn_name::new(self.contract, self.pos, prop_type))
            }
        }
    };
}

use prop_iter; // re-export for use

macro_rules! primitives {
    // repetition necessary to support multiple arguments to `macro_name`
    (; idents, $macro_name:ident, $($x:tt)+) => {
        $macro_name!(
            $($x)+,
            U8,
            U16,
            U32,
            U64,
            I8,
            I16,
            I32,
            I64,
            Bool,
            F32,
            F64,
            String,
            ObjectHandle,
            ContractDataHandle,
        );
    };
    (; types, $macro_name:ident, $($x:tt)+) => {
        $macro_name!(
            $($x)+,
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            bool,
            f32,
            f64,
            String,
            ObjectHandle,
            ContractDataHandle,
        );
    };
    (; idents, $macro_name:ident) => {
        $macro_name!(
            U8,
            U16,
            U32,
            U64,
            I8,
            I16,
            I32,
            I64,
            Bool,
            F32,
            F64,
            String,
            ObjectHandle,
            ContractDataHandle,
        );
    };
    (; types, $macro_name:ident) => {
        $macro_name!(
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            bool,
            f32,
            f64,
            String,
            ObjectHandle,
            ContractDataHandle,
        );
    };
    (idents, $macro_name:ident, $($x:tt)+) => {
        $macro_name!(
            $($x)+,
            U8,
            U16,
            U32,
            U64,
            I8,
            I16,
            I32,
            I64,
            Bool,
            F32,
            F64,
            String,
            ObjectHandle,
            ContractDataHandle,
        )
    };
    (types, $macro_name:ident, $($x:tt)+) => {
        $macro_name!(
            $($x)+,
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            bool,
            f32,
            f64,
            String,
            ObjectHandle,
            ContractDataHandle,
        )
    };
    (idents, $macro_name:ident) => {
        $macro_name!(
            U8,
            U16,
            U32,
            U64,
            I8,
            I16,
            I32,
            I64,
            Bool,
            F32,
            F64,
            String,
            ObjectHandle,
            ContractDataHandle,
        )
    };
    (types, $macro_name:ident) => {
        $macro_name!(
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            bool,
            f32,
            f64,
            String,
            ObjectHandle,
            ContractDataHandle,
        )
    };
    (idents) => {
        U8,
        U16,
        U32,
        U64,
        I8,
        I16,
        I32,
        I64,
        Bool,
        F32,
        F64,
        String,
        ObjectHandle,
        ContractDataHandle,
    };
    (types) => {
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        f32,
        f64,
        String,
        ObjectHandle,
        ContractDataHandle,
    };
}
pub(crate) use primitives;