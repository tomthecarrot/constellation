using System;
using System.Runtime.InteropServices;

namespace RSharp
{
    // ---- Unsigned ----

    public class RBox_Bool : OpaqueWrapper<bool>
    {
        public unsafe RBox_Bool(bool value) : base(new Ptr<bool>((IntPtr)rsharp__Box_Bool__new(value)), OwnershipSemantics.Owned)
        { }

        public unsafe RBox_Bool(Ptr<bool> inner, OwnershipSemantics ownershipSemantics) : base(inner, ownershipSemantics)
        { }

        protected override unsafe void NativeDrop(Ptr<bool> inner)
        {
            rsharp__Box_Bool__drop((bool*)inner.p);
        }

        // -- C interop

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe bool* rsharp__Box_Bool__new(bool value);

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_Bool__drop(bool* value);
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

    // ---- Other ----

    public struct RBox_ObjectHandle
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe IntPtr rsharp__Box__ObjectHandle__new(IntPtr value);
        public static unsafe IntPtr new_(IntPtr value)
        {
            return rsharp__Box__ObjectHandle__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_ObjectHandle__drop(IntPtr value);
        public static unsafe void drop(IntPtr value)
        {
            rsharp__Box_ObjectHandle__drop(value);
        }
    }

    public struct RBox_ContractDataHandle
    {
        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe IntPtr rsharp__Box__ContractDataHandle__new(IntPtr value);
        public static unsafe IntPtr new_(IntPtr value)
        {
            return rsharp__Box__ContractDataHandle__new(value);
        }

        [DllImport(Metadata.LIBRARY_NAME)]
        private static extern unsafe void rsharp__Box_ContractDataHandle__drop(IntPtr value);
        public static unsafe void drop(IntPtr value)
        {
            rsharp__Box_ContractDataHandle__drop(value);
        }
    }
}
