//! Better Borrow provides a more advanced borrow trait. It operates in the same
//! spirit as [`std::borrow::Borrow`], but without the restriction that the
//! return type must be itself a reference.

/// Allows borrowing a `MyOwned` as a `MyRef<'a>` instead of as `&'a MyOwned`.
///
/// # Lifetimes
/// The `'b` lifetime only exists so that implementors of this trait have access
/// to the lifetime in their implementation. Because `'a: 'b` in the `borrow(&'a self)`
/// function, implementors can use the `'b` lifetime to parameterize their choice of
/// `Borrow` such that their reference type lives no longer than `'b`, and by extension,
/// no longer than `&'a self`.
///
/// # Example
/// ```
/// # use better_borrow::BBorrow;
/// struct MyRef<'inner>(&'inner str);
/// impl<'b> BBorrow<'b, MyRef<'b>> for String {
///     fn borrow<'a>(&'a self) -> MyRef<'b>
///     where
///         'a: 'b,
///     {
///         MyRef(self.as_str()) // store the &'a str in the `MyRef`
///     }
/// }
///
/// //...
///
/// let my_ref: MyRef;
/// {
///     let s = String::from("hello");
///     my_ref = s.borrow();
///     assert_eq!(my_ref.0, "hello")
/// }
/// // The following gives a borrow checker error, because `s` has been dropped
/// // assert_eq!(my_ref.0, "hello")
/// ```
pub trait BBorrow<'b, Borrowed>
where
    Borrowed: ?Sized,
{
    fn borrow<'a>(&'a self) -> Borrowed
    where
        'a: 'b;
}

impl<'b, B, T> BBorrow<'b, &'b B> for T
where
    T: std::borrow::Borrow<B> + ?Sized,
    B: ?Sized,
{
    fn borrow<'a>(&'a self) -> &'b B
    where
        'a: 'b,
    {
        self.borrow()
    }
}

/// Mutable version of [`BBorrow`]. Allows borrowing a `MyOwned` as a `MyMut<'a>`
/// instead of as a `&'a mut MyOwned`.
///
/// # Lifetimes
/// The `'b` lifetime only exists so that implementors of this trait have access
/// to the lifetime in their implementation. Because `'a: 'b` in the
/// `borrow_mut(&'a mut self)` function, implementors can use the `'b` lifetime
/// to parameterize their choice of `BorrowedMut` such that their reference type
/// lives no longer than `'b`, and by extension, no longer than `&'a mut self`.
///
/// # Example
/// ```
/// # use better_borrow::{BBorrow, BBorrowMut};
/// # struct MyRef<'inner>(&'inner str);
/// # impl<'b> BBorrow<'b, MyRef<'b>> for String {
/// #     fn borrow<'a>(&'a self) -> MyRef<'b> where 'a: 'b { todo!() }
/// # }
/// // ...assuming `BBorrow<MyRef>` has alredy been implemented for `String`
/// struct MyMut<'inner>(&'inner mut str);
/// impl<'b> BBorrowMut<'b, MyMut<'b>> for String {
///     type Borrowed = MyRef<'b>;
///
///     fn borrow_mut<'a>(&'a mut self) -> MyMut<'b>
///     where
///         'a: 'b,
///     {
///         MyMut(self.as_mut_str()) // store the &'a mut str in the `MyMut`
///     }
/// }
///
/// //...
///
/// let my_mut: MyMut;
/// {
///     let mut s = String::from("hello");
///     my_mut = s.borrow_mut();
///     assert_eq!(my_mut.0, "hello")
/// }
/// // The following gives a borrow checker error, because `s` has been dropped
/// // assert_eq!(my_mut.0, "hello")
/// ```
pub trait BBorrowMut<'b, BorrowedMut>: BBorrow<'b, Self::Borrowed>
where
    BorrowedMut: ?Sized,
{
    type Borrowed: ?Sized;

    fn borrow_mut<'a>(&'a mut self) -> BorrowedMut
    where
        'a: 'b;
}

impl<'b, B, T> BBorrowMut<'b, &'b mut B> for T
where
    T: std::borrow::BorrowMut<B> + ?Sized,
    B: ?Sized,
{
    type Borrowed = &'b B;

    fn borrow_mut<'a>(&'a mut self) -> &'b mut B
    where
        'a: 'b,
    {
        self.borrow_mut()
    }
}
