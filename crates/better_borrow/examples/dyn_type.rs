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

impl<'b> BBorrowMut<'b, DynRef<'b>> for Dyn {
    type BorrowedMut = DynMut<'b>;

    fn borrow_mut<'a>(&'a mut self) -> Self::BorrowedMut
    where
        'a: 'b,
    {
        todo!()
    }
}

fn main() {
    let d_ref;
    let d_mut;
    {
        let mut d = Dyn::String(String::from("hello"));

        d_ref = d.borrow();
        println!("{d_ref:?}");
        d_mut = d.borrow_mut();
        println!("{d_mut:?}")
    };
    // The following two lines would cause an error, because `d` does not live
    // long enough:
    // println!("{d_ref:?}")
    // println!("{d_mut:?}")
}
