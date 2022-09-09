using System;
using System.Runtime.InteropServices;
using System.Text;

namespace RSharp
{

    public sealed class RString : OpaqueWrapper<RString>
    {
        public RString(Ptr<RString> inner) : base(inner, OwnershipSemantics.Owned) { }
        public RString(string managedString) : this(NewHelper(managedString)) { }

        private static Ptr<RString> NewHelper(string managedString)
        {
            var utf8 = new UTF8Encoding();
            byte[] bytes = utf8.GetBytes(managedString);
            unsafe
            {
                IntPtr rbox;
                fixed (byte* first_element = &bytes[0])
                {
                    SliceU8 raw_str = new SliceU8(first_element, (ulong)bytes.Length);
                    rbox = rsharp__String__copy_utf8(raw_str);
                }
                return new Ptr<RString>(rbox);
            }
        }

        public string Value
        {
            get
            {
                SliceU8 slice = rsharp__String__value(this.Inner.Value.p);
                var utf8 = new UTF8Encoding();
                unsafe
                {
                    return utf8.GetString(slice.ptr, (int)slice.len);
                }
            }
        }

        override protected void NativeDrop(Ptr<RString> inner)
        {
            rsharp__String__drop(inner.p);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe SliceU8 rsharp__String__value(IntPtr s);

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__String__drop(IntPtr s);

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe IntPtr rsharp__String__copy_utf8(SliceU8 slice);

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe IntPtr rsharp__String__copy_utf16(SliceU16 slice);

        [StructLayout(LayoutKind.Sequential)]
        private unsafe struct SliceU8
        {
            public byte* ptr;
            public ulong len;

            public SliceU8(byte* ptr, ulong len)
            {
                this.ptr = ptr;
                this.len = len;
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private unsafe struct SliceU16
        {
            public ushort* ptr;
            public ulong len;

            public SliceU16(ushort* ptr, ulong len)
            {
                this.ptr = ptr;
                this.len = len;
            }
        }
    }
}
