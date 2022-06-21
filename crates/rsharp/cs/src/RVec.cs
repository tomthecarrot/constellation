using System;

namespace RSharp
{

    /// T is the c# wrapped data type stored *inside* the RVec, V is the C#
    /// shared struct of the monomorphized RVec
    public abstract class RVec<T, V> : SharedWrapper<V>
        where T : class
        where V : unmanaged
    {

        public RVec(V shared) : base(shared, OwnershipSemantics.Owned)
        { }

        /// Provided as a convient helper that can be invoked in a `base()` call
        protected static V NewHelper(Action<IntPtr> nativeNew)
        {
            V inner = default;
            unsafe
            {
                IntPtr intPtr = new IntPtr(&inner);
                nativeNew(intPtr);
            }
            return inner;
        }

        abstract public void push(T e);

        abstract public T this[int index]
        {
            get;
            set;
        }
    }
}
