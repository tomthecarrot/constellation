using System;
using System.Runtime.InteropServices;

/// Stuff imported by PInvoke
namespace c_ffi
{

    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "tp_client";
#endif
    }

    public struct MyType
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        internal static extern unsafe MyType* MyType__new(int a, float b);

        [DllImport(Metadata.LIBRARY_NAME)]
        internal static extern unsafe MyType* MyType__drop(MyType* ptr);
    }
}

namespace rsharp
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

    namespace ffi
    {
        public unsafe class MyType : rsharp.OwnedHandle<c_ffi.MyType>
        {
            public c_ffi.MyType* _ptr;

            public MyType(int a, float b)
            {
                this._ptr = c_ffi.MyType.MyType__new(a, b);
            }

            public MyType(c_ffi.MyType* ptr)
            {
                this._ptr = ptr;
            }

            public void Dispose()
            {
                c_ffi.MyType.MyType__drop(this._ptr);
            }

            public c_ffi.MyType* Ptr
            {
                get => _ptr;
                set => _ptr = value;
            }
        }
    }
