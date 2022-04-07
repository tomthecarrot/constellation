using System;


namespace RSharp
{
    /// An `OwnedHandle<T>` owns a rust `*mut T`, so it will also be in charge
    /// of dropping it. To do anything useful with an instance of this class,
    /// use the `Ptr` property.
    public unsafe interface OwnedHandle<T> : IDisposable where T : unmanaged
    {
        /// Gets a rust `*mut T`
        T* Ptr
        {
            get; set;
        }
    }
}
