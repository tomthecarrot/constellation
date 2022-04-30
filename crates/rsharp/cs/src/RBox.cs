using System;
using System.Runtime.InteropServices;

namespace RSharp
{
    // ---- Unsigned ----

    public struct RBox_Bool
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe bool* rsharp__Box_Bool__new(bool value);
        public static unsafe bool* new_(bool value)
        {
            return rsharp__Box_Bool__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_Bool__drop(bool* value);
        public static unsafe void drop(bool* value)
        {
            rsharp__Box_Bool__drop(value);
        }
    }

    public struct RBox_U8
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe byte* rsharp__Box_U8__new(byte value);
        public static unsafe byte* new_(byte value)
        {
            return rsharp__Box_U8__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_U8__drop(byte* value);
        public static unsafe void drop(byte* value)
        {
            rsharp__Box_U8__drop(value);
        }
    }

    public struct RBox_U16
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe ushort* rsharp__Box_U16__new(ushort value);
        public static unsafe ushort* new_(ushort value)
        {
            return rsharp__Box_U16__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_U16__drop(ushort* value);
        public static unsafe void drop(ushort* value)
        {
            rsharp__Box_U16__drop(value);
        }
    }

    public struct RBox_U32
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe uint* rsharp__Box_U32__new(uint value);
        public static unsafe uint* new_(uint value)
        {
            return rsharp__Box_U32__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_U32__drop(uint* value);
        public static unsafe void drop(uint* value)
        {
            rsharp__Box_U32__drop(value);
        }
    }

    public struct RBox_U64
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe ulong* rsharp__Box_U64__new(ulong value);
        public static unsafe ulong* new_(ulong value)
        {
            return rsharp__Box_U64__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_U64__drop(ulong* value);
        public static unsafe void drop(ulong* value)
        {
            rsharp__Box_U64__drop(value);
        }
    }

    // ---- Signed ----

    public struct RBox_I8
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe sbyte* rsharp__Box_I8__new(sbyte value);
        public static unsafe sbyte* new_(sbyte value)
        {
            return rsharp__Box_I8__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_I8__drop(sbyte* value);
        public static unsafe void drop(sbyte* value)
        {
            rsharp__Box_I8__drop(value);
        }
    }

    public struct RBox_I16
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe short* rsharp__Box_I16__new(short value);
        public static unsafe short* new_(short value)
        {
            return rsharp__Box_I16__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_I16__drop(short* value);
        public static unsafe void drop(short* value)
        {
            rsharp__Box_I16__drop(value);
        }
    }

    public struct RBox_I32
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe int* rsharp__Box_I32__new(int value);
        public static unsafe int* new_(int value)
        {
            return rsharp__Box_I32__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_I32__drop(int* value);
        public static unsafe void drop(int* value)
        {
            rsharp__Box_I32__drop(value);
        }
    }

    public struct RBox_I64
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe long* rsharp__Box_I64__new(long value);
        public static unsafe long* new_(long value)
        {
            return rsharp__Box_I64__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_I64__drop(long* value);
        public static unsafe void drop(long* value)
        {
            rsharp__Box_I64__drop(value);
        }
    }

    // ---- Floating point ----

    public struct RBox_F32
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe float* rsharp__Box_F32__new(float value);
        public static unsafe float* new_(float value)
        {
            return rsharp__Box_F32__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_F32__drop(float* value);
        public static unsafe void drop(float* value)
        {
            rsharp__Box_F32__drop(value);
        }
    }

    public struct RBox_F64
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe double* rsharp__Box_F64__new(double value);
        public static unsafe double* new_(double value)
        {
            return rsharp__Box_F64__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_F64__drop(double* value);
        public static unsafe void drop(double* value)
        {
            rsharp__Box_F64__drop(value);
        }
    }

    // ---- Opaque ----

    public struct RBox_ContractDataHandle
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_ContractDataHandle__drop(IntPtr value);
        public static unsafe void drop(IntPtr value)
        {
            rsharp__Box_ContractDataHandle__drop(value);
        }
    }

    public struct RBox_ObjectHandle
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_ObjectHandle__drop(IntPtr value);
        public static unsafe void drop(IntPtr value)
        {
            rsharp__Box_ObjectHandle__drop(value);
        }
    }

}
