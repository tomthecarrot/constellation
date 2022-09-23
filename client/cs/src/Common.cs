using IDisposable = System.IDisposable;
using IntPtr = System.IntPtr;
using OwnershipSemantics = RSharp.OwnershipSemantics;

namespace Teleportal.Client
{
    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "constellation";
#endif
    }


    namespace Contract.Properties
    {
        sealed partial class ToManaged
        {
            public static unsafe IntPtr f(OwnershipSemantics ownershipSemantics, IntPtr ptr)
            {
                return ptr;
            }

            public static unsafe bool f(OwnershipSemantics ownershipSemantics, bool* ptr)
            {
                return *ptr;
            }

            public static unsafe byte f(OwnershipSemantics ownershipSemantics, byte* ptr)
            {
                return *ptr;
            }

            public static unsafe ushort f(OwnershipSemantics ownershipSemantics, ushort* ptr)
            {
                return *ptr;
            }

            public static unsafe uint f(OwnershipSemantics ownershipSemantics, uint* ptr)
            {
                return *ptr;
            }

            public static unsafe ulong f(OwnershipSemantics ownershipSemantics, ulong* ptr)
            {
                return *ptr;
            }

            public static unsafe sbyte f(OwnershipSemantics ownershipSemantics, sbyte* ptr)
            {
                return *ptr;
            }

            public static unsafe short f(OwnershipSemantics ownershipSemantics, short* ptr)
            {
                return *ptr;
            }

            public static unsafe int f(OwnershipSemantics ownershipSemantics, int* ptr)
            {
                return *ptr;
            }

            public static unsafe long f(OwnershipSemantics ownershipSemantics, long* ptr)
            {
                return *ptr;
            }

            public static unsafe float f(OwnershipSemantics ownershipSemantics, float* ptr)
            {
                return *ptr;
            }

            public static unsafe double f(OwnershipSemantics ownershipSemantics, double* ptr)
            {
                return *ptr;
            }
        }

    }


}
