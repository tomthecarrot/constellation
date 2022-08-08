using System;

namespace RSharp
{
    // TODO[SER-446]
    public sealed class RString : OpaqueWrapper<RString>
    {
        public RString(Ptr<RString> inner) : base(inner, OwnershipSemantics.Owned) { }

        override protected void NativeDrop(Ptr<RString> inner)
        {
            // generated.__Internal.TpClientObjectObjectHandleDrop(inner.p);
        }
    }

}
