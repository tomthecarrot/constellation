using System.Runtime.InteropServices;
using RSharp;

namespace Teleportal.Client.Contract.Properties
{
    namespace C_ffi
    {
        struct RBox_U8 // Cannot have functions in just namespace
        {
            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe byte* tp_client__contract__properties__Box_U8__new(byte* value);
            internal static unsafe byte* new_(byte* value)
            {
                return tp_client__contract__properties__Box_U8__new(value);
            }

            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe void tp_client__contract__properties__Box_U8__drop(byte* value);
            internal static unsafe void drop(byte* value)
            {
                tp_client__contract__properties__Box_U8__drop(value);
            }
        }
    }

    public unsafe class RBox_U8 : RSharp.OwnedHandle<byte>
    {
        private byte* _ptr;

        public RBox_U8(byte value)
        {
            this._ptr = C_ffi.RBox_U8.new_(&value);
        }

        public void Dispose()
        {
            if (null == this._ptr)
            {
                return;
            }

            C_ffi.RBox_U8.drop(this._ptr);
            this._ptr = null;
        }

        ~RBox_U8()
        {
            Dispose();
        }

        public byte* Ptr
        {
            get => this._ptr;
            set => this._ptr = value;
        }
    }
}
