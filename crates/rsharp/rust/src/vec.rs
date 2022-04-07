/// A C-FFI compatible version of a rust [`Vec`].
///
/// Its important to note that this type maintains the same alignment and allocator
/// invariants as `Vec`. It is unsound to create a `RustVec` that doesn't satisfy
/// these invariants.
#[repr(C)]
pub struct RustVec<T> {
    ptr: *mut T,
    /// capacity is in # of items, not bytes
    capacity: usize,
    len: usize,
}
impl<T> RustVec<T> {
    /// # Safety
    /// The caller must ensure that the arguments satisfy the same safety requirements
    /// as [`Vec::from_raw_parts`]
    pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize, len: usize) -> Self {
        Self { ptr, capacity, len }
    }
}
impl<T> From<Vec<T>> for RustVec<T> {
    fn from(mut other: Vec<T>) -> Self {
        let ptr = other.as_mut_ptr();
        let capacity = other.capacity();
        let len = other.len();
        let _ = other.leak();
        Self { ptr, capacity, len }
    }
}
impl<T> Drop for RustVec<T> {
    fn drop(&mut self) {
        // Safety: `RustVec` already upholds the invariants of `from_raw_parts`.
        unsafe { Vec::from_raw_parts(self.ptr, self.len, self.capacity) };
    }
}
impl<T> From<RustVec<T>> for Vec<T> {
    /// Converts the FFI-safe `RustVec` into a native rust [`Vec`].
    ///
    /// # Safety
    /// The caller must ensure that `self` satisfies all the safety requirements of
    /// [`Vec::from_raw_parts`]
    fn from(other: RustVec<T>) -> Self {
        // Safety: `RustVec` already upholds the invariants of `from_raw_parts`
        unsafe { Vec::from_raw_parts(other.ptr, other.len, other.capacity) }
    }
}
