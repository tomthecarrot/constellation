use better_borrow::{BBorrow, BBorrowMut};

use derive_more::From;

#[derive(Debug)]
pub enum Dyn {
    String(String),
    U8(Vec<u8>),
}

#[derive(Debug, From)]
pub enum DynRef<'a> {
    String(&'a str),
    U8(&'a [u8]),
}

impl<'b> BBorrow<'b, DynRef<'b>> for Dyn {
    fn borrow<'a>(&'a self) -> DynRef<'b>
    where
        'a: 'b,
    {
        match self {
            Self::String(v) => v.as_str().into(),
            Self::U8(v) => v.as_slice().into(),
        }
    }
}

#[derive(Debug, From)]
pub enum DynMut<'a> {
    String(&'a mut str),
    U8(&'a mut [u8]),
}

impl<'b> BBorrowMut<'b, DynMut<'b>> for Dyn {
    type Borrowed = DynRef<'b>;

    fn borrow_mut<'a>(&'a mut self) -> DynMut<'b>
    where
        'a: 'b,
    {
        match self {
            Self::String(v) => v.as_mut_str().into(),
            Self::U8(v) => v.as_mut_slice().into(),
        }
    }
}

fn main() {
    let d = Dyn::String(String::from("hello"));
    foo(d);
}

fn foo<B>(mut d: B)
where
    // this is a HRTB because we say that this generic bound is upheld for *every*
    // lifetime `'b`. Otherwise since `'b` is completely unconstrained, so the
    // implementation must assume that the caller might provide a `'b` that is
    // `'static`, which would make implementing the function impossible.
    for<'b> B: BBorrowMut<'b, DynMut<'b>> + BBorrow<'b, DynRef<'b>>,
{
    let d_ref: DynRef;
    let d_mut: DynMut;
    {
        d_ref = d.borrow(); // if `'b` was `'static` this would be an error
        println!("{d_ref:?}");
        d_mut = d.borrow_mut();
        println!("{d_mut:?}")
    };
    // The following two lines would cause an error, because `d` does not live
    // long enough:
    // println!("{d_ref:?}")
    // println!("{d_mut:?}")
}
