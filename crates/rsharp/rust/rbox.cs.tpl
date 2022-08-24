using System;
using System.Runtime.InteropServices;

namespace RSharp
{

    public class RBox_{{type_platform}} : OpaqueWrapper<{{type_cs}}>
    {
        public unsafe RBox_{{type_platform}}({{type_cs}} value) : base(new Ptr<{{type_cs}}>((IntPtr)rsharp__Box_{{type_platform}}__new(value)), OwnershipSemantics.Owned)
        { }

        public unsafe RBox_{{type_platform}}(Ptr<{{type_cs}}> inner, OwnershipSemantics ownershipSemantics) : base(inner, ownershipSemantics)
        { }

        protected override unsafe void NativeDrop(Ptr<{{type_cs}}> inner)
        {
            rsharp__Box_{{type_platform}}__drop(({{type_cs}}{{ptr_literal}})inner.p);
        }

        public unsafe {{type_cs}} Value {
            get => {{ptr_literal}}({{type_cs}}{{ptr_literal}})this.Inner.Value.p;
        }


        // -- C interop

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe {{type_cs}}{{ptr_literal}} rsharp__Box_{{type_platform}}__new({{type_cs}} value);

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_{{type_platform}}__drop({{type_cs}}{{ptr_literal}} value);
    }

}
