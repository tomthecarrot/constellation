using System;
using System.Runtime.InteropServices;

namespace Teleportal
{
    /// Stuff imported by PInvoke
    namespace C_FFI
    {

        public class Metadata
        {
#if UNITY_IOS && !UNITY_EDITOR
            internal const string LIBRARY_NAME = "__Internal";
#else
            internal const string LIBRARY_NAME = "tp_rsharp";
#endif
        }

        public struct MyType
        {
            public int a;
            public float b;

            [DllImport(Metadata.LIBRARY_NAME)]
            internal static extern unsafe MyType* MyType__new(int a, float b);

            [DllImport(Metadata.LIBRARY_NAME)]
            internal static extern unsafe void MyType__drop(MyType* ptr);
        }
    }

    namespace Rsharp
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

        public unsafe class MyType : Rsharp.OwnedHandle<C_FFI.MyType>
        {
            public C_FFI.MyType* _ptr;

            public MyType(int a, float b)
            {
                this._ptr = C_FFI.MyType.MyType__new(a, b);
            }

            public MyType(C_FFI.MyType* ptr)
            {
                this._ptr = ptr;
            }

            public int A => (*this._ptr).a;

            public float B => (*this._ptr).b;

            public void Dispose()
            {
                C_FFI.MyType.MyType__drop(this._ptr);
            }

            public C_FFI.MyType* Ptr
            {
                get => this._ptr;
                set => this._ptr = value;
            }
        }
    }

}
