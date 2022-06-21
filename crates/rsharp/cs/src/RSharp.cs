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


    public enum OwnershipSemantics
    {
        Owned,
        SharedRef,
        MutRef,
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

    public abstract class SharedWrapper<T> : IDisposable where T : struct
    {
        private T? inner;

        private OwnershipSemantics ownershipSemantics;

        // ---- Properties ----
        public OwnershipSemantics OwnershipSemantics
        {
            get => this.ownershipSemantics;
        }

        public T? Inner
        {
            get => this.inner;
            set => this.inner = value;
        }

        // ---- Lifetime management ----

        public unsafe SharedWrapper(T inner, OwnershipSemantics ownershipSemantics)
        {
            this.inner = inner;
            this.ownershipSemantics = ownershipSemantics;
        }

        public void Dispose()
        {
            if (this.ownershipSemantics == OwnershipSemantics.Owned && this.inner != null)
            {
                NativeDrop(this.inner.Value);
            }
            this.inner = null;
        }

        ~SharedWrapper()
        {
            this.Dispose();
        }

        /// Implemented by subclasses to provide the exact native destructor to call
        abstract protected void NativeDrop(T inner);
    }

    public abstract class OpaqueWrapper<T> : IDisposable
    {
        private Ptr<T>? inner;

        private OwnershipSemantics ownershipSemantics;

        // ---- Properties ----
        public OwnershipSemantics OwnershipSemantics
        {
            get => this.ownershipSemantics;
        }

        public Ptr<T>? Inner
        {
            get => this.inner;
            set => this.inner = value;
        }

        // ---- Lifetime management ----

        public unsafe OpaqueWrapper(Ptr<T> inner, OwnershipSemantics ownershipSemantics)
        {
            this.inner = inner;
            this.ownershipSemantics = ownershipSemantics;
        }

        public void Dispose()
        {
            if (this.ownershipSemantics == OwnershipSemantics.Owned && this.inner != null)
            {
                NativeDrop(this.inner.Value);
            }
            this.inner = null;
        }

        ~OpaqueWrapper()
        {
            this.Dispose();
        }

        /// Implemented by subclasses to provide the exact native destructor to call
        abstract protected void NativeDrop(Ptr<T> inner);
    }

    public class InvalidDropException : Exception
    { }
}
