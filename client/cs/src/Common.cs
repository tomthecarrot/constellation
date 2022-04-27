using IDisposable = System.IDisposable;
using IntPtr = System.IntPtr;

namespace Teleportal.Client
{
    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "tp_client";
#endif
    }

    public enum OwnershipSemantics
    {
        Owned,
        SharedRef,
        MutRef,
    }


    namespace Contract.Properties
    {
        sealed partial class ToManaged
        {
            public static unsafe byte f(OwnershipSemantics ownershipSemantics, byte* ptr)
            {
                return *ptr;
            }

            public static unsafe double f(OwnershipSemantics ownershipSemantics, double* ptr)
            {
                return *ptr;
            }
        }

        /// Type-safe wrapper around `IntPtr` for `T`
        public readonly struct Ptr<T>
        {
            public readonly IntPtr p;

            public Ptr(IntPtr p)
            {
                this.p = p;
            }
        }

        public abstract class Wrapper<T> : IDisposable
        {
            private Ptr<T>? ptr;
            private OwnershipSemantics ownershipSemantics;

            // ---- Properties ----
            public OwnershipSemantics OwnershipSemantics
            {
                get => this.ownershipSemantics;
            }

            public Ptr<T>? Ptr
            {
                get => this.ptr;
                set => this.ptr = value;
            }

            // ---- Lifetime management ----

            public unsafe Wrapper(Ptr<T> ptr, OwnershipSemantics ownershipSemantics)
            {
                this.ptr = ptr;
                this.ownershipSemantics = ownershipSemantics;
            }

            public void Dispose()
            {
                if (this.ownershipSemantics == OwnershipSemantics.Owned && this.ptr != null)
                {
                    NativeDrop(this.ptr.Value);
                }
                this.ptr = null;
            }

            ~Wrapper()
            {
                this.Dispose();
            }

            /// Implemented by subclasses to provide the exact native destructor to call
            abstract protected void NativeDrop(Ptr<T> ptr);
        }
    }

}
