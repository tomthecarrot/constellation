using System;


namespace RSharp
{

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

        /// Transfers ownership of `Inner`, and returns it.
        ///
        /// This "steals" the inner data by returning it and setting `Inner` in this
        /// instance to `null`. This is important to avoid double-free and
        /// use-after-free bugs.
        public T StealInner()
        {
            var result = this.inner.Value;
            this.inner = null;
            return result;
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

        /// Transfers ownership of `Inner`, and returns it.
        ///
        /// This "steals" the inner data by returning it and setting `Inner` in this
        /// instance to `null`. This is important to avoid double-free and
        /// use-after-free bugs.
        public Ptr<T> StealInner()
        {
            var result = this.inner.Value;
            this.inner = null;
            return result;
        }

        /// Implemented by subclasses to provide the exact native destructor to call.
        abstract protected void NativeDrop(Ptr<T> inner);
    }
}
