using System;
using System.Runtime.InteropServices;

namespace RSharp
{

    /// T is the c# wrapped data type stored *inside* the RVec, V is the C#
    /// shared struct of the monomorphized RVec
    public abstract class RVec<T, V> : SharedWrapper<V>
        where T : class
        where V : struct
    {

        public RVec(V shared) : base(shared, OwnershipSemantics.Owned)
        { }

        abstract public void push(T e);

        abstract public T this[int index]
        {
            get;
            set;
        }
    }
}
