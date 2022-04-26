using System.Runtime.InteropServices;

namespace RSharp
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
