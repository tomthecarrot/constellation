using System.Runtime.InteropServices;
using Teleportal.Client;
using P = Teleportal.Client.Contract.Properties;

namespace Teleportal.Client.Contract.Properties.Channels
{
    namespace C_ffi
    {
        public struct Keyframe_U8
        {
            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe Keyframe_U8* tp_client__contract__properties__channels__Keyframe_U8__new(byte* value, double time);
            internal static unsafe Keyframe_U8* new_(byte* value, double time)
            {
                return tp_client__contract__properties__channels__Keyframe_U8__new(value, time);
            }

            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe void tp_client__contract__properties__channels__Keyframe_U8__drop(Keyframe_U8* ptr);
            internal static unsafe void drop(Keyframe_U8* ptr)
            {
                tp_client__contract__properties__channels__Keyframe_U8__drop(ptr);
            }

            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe byte* tp_client__contract__properties__channels__Keyframe_U8__value(Keyframe_U8* ptr);
            internal static unsafe byte* value(Keyframe_U8* ptr)
            {
                return tp_client__contract__properties__channels__Keyframe_U8__value(ptr);
            }

            [DllImport(Metadata.LIBRARY_NAME)]
            private static extern unsafe double tp_client__contract__properties__channels__Keyframe_U8__time(Keyframe_U8* ptr);
            internal static unsafe double time(Keyframe_U8* ptr)
            {
                return tp_client__contract__properties__channels__Keyframe_U8__time(ptr);
            }
        }
    }

    public unsafe class Keyframe_U8 : RSharp.OwnedHandle<C_ffi.Keyframe_U8>
    {
        private C_ffi.Keyframe_U8* _ptr;

        public Keyframe_U8(byte value, double time)
        {
            var box = new P.RBox_U8(value);
            this._ptr = C_ffi.Keyframe_U8.new_(box.Ptr, time);
            box.Ptr = null; // We prevent double free by actually moving the pointer.
        }

        public void Dispose()
        {
            if (null == this._ptr)
            {
                return;
            }

            C_ffi.Keyframe_U8.drop(this._ptr);
            this._ptr = null;
        }

        ~Keyframe_U8()
        {
            Dispose();
        }

        public C_ffi.Keyframe_U8* Ptr
        {
            get => this._ptr;
            set => this._ptr = value;
        }

        public byte value
        {
            get => *C_ffi.Keyframe_U8.value(this._ptr);
        }

        public double time
        {
            get => C_ffi.Keyframe_U8.time(this._ptr);
        }
    }
}
