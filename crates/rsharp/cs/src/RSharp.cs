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

    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "rsharp";
#endif
    }
}
