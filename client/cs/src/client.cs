using System;
using System.Runtime.InteropServices;
using generated = Client.generated;

namespace Teleportal.Client
{
    public enum OwnershipSemantics
    {
        Owned,
        SharedRef,
        MutRef,
    }
}

namespace Teleportal.Client.Contract.Properties
{
    class ToManaged
    {
        public static unsafe Channels.Keyframe_U8 f(OwnershipSemantics ownershipSemantics, Channels.Keyframe_U8_Ptr ptr)
        {
            return new Channels.Keyframe_U8(ptr, ownershipSemantics);
        }


        public static unsafe byte f(OwnershipSemantics ownershipSemantics, byte* ptr)
        {
            return *ptr;
        }

        public static unsafe double f(OwnershipSemantics ownershipSemantics, double* ptr)
        {
            return *ptr;
        }

    }

}

namespace Teleportal.Client.Contract.Properties.Channels
{

    /// Type-safe wrapper around `IntPtr` for `Keyframe_U8`
    public struct Keyframe_U8_Ptr
    {
        public readonly IntPtr p;

        public Keyframe_U8_Ptr(IntPtr p)
        {
            this.p = p;
        }
    }

    public struct SomeSharedStruct
    { }

    public class Keyframe_U8 : IDisposable
    {
        private Keyframe_U8_Ptr? ptr;

        private OwnershipSemantics ownershipSemantics;

        public OwnershipSemantics OwnershipSemantics
        {
            get => this.ownershipSemantics;
        }

        public Keyframe_U8_Ptr? Ptr
        {
            get => this.ptr;
            set => this.ptr = value;
        }

        public unsafe byte value
        {
            get
            {
                byte* result = generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Value(this.ptr?.p ?? IntPtr.Zero);
                return ToManaged.f(OwnershipSemantics.SharedRef, result);
            }
        }

        public double time
        {
            get => generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Time(this.ptr?.p ?? IntPtr.Zero);
        }

        public unsafe Keyframe_U8(byte value, double time)
            : this(new Keyframe_U8_Ptr(generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8New(RSharp.RBox_U8.new_(value), time)), OwnershipSemantics.Owned)
        { }

        internal unsafe Keyframe_U8(Keyframe_U8_Ptr ptr, OwnershipSemantics ownershipSemantics)
        {
            this.ptr = ptr;
            this.ownershipSemantics = ownershipSemantics;
        }

        public void Dispose()
        {
            if ((null != this.ptr) && (this.ownershipSemantics == OwnershipSemantics.Owned))
            {
                generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Drop(this.ptr?.p ?? IntPtr.Zero);
                this.ptr = null;
            }
        }

        ~Keyframe_U8()
        {
            this.Dispose();
        }
    }

}
