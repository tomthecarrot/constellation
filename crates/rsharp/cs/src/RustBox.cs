using System;
using Functional.Option;

namespace rsharp
{
    public class RustBox<T>
    {
        private IRustBox<T> box;
    }

    /// C# version of C++ CRTP. Allows referencing implementing type.
    /// NOTE: `TSelf` should be the implementing type, DO NOT try to circumvent this
    interface IRustBox<TSelf> : IDisposable// where TSelf : IRustBox<TSelf>
    {
        // ref readonly Option<TSelf> get();
        // ref Option<TSelf> getMut();
    }

    public class RustBoxU8 : IRustBox<RustBoxInnerU8>
    {
        private int count = 0;
        private readonly Option<RustBoxInnerU8_RO> innerRO;
        private Option<RustBoxInnerU8_RW> innerRW;

        private Option<RustBoxInnerU8_RO> noneRO;
        private Option<RustBoxInnerU8_RW> noneRW;

        public unsafe RustBoxU8(byte* ptr)
        {
            this.innerRO = Option.Some(new RustBoxInnerU8_RO(ptr));
            this.innerRW = Option.Some(new RustBoxInnerU8_RW(ptr));
            this.noneRO = Option.None;
            this.noneRW = Option.None;
        }

        public ref readonly Option<RustBoxInnerU8_RO> get()
        {
            if (this.count == 0)
            {
                this.count++;
                return ref this.innerRO;
            }

            return ref this.noneRO;
        }

        public ref Option<RustBoxInnerU8_RW> getMut()
        {
            if (this.count == 0)
            {
                this.count++;
                return ref this.innerRW;
            }

            return ref this.noneRW;
        }

        public void ReleaseInner(RustBoxInner<byte> inner)
        {
            this.count--;
        }

        public void Dispose()
        {

        }
    }

    public interface RustBoxInner<T> { }

    public interface RustBoxInnerU8 : RustBoxInner<byte> { }

    public unsafe interface RustBoxInner_RO<T> : RustBoxInnerU8
    {
        public T get();
    }

    public unsafe interface RustBoxInner_RW<T> : RustBoxInner_RO<T> where T : unmanaged
    {
        public T* getMut();
    }

    public unsafe struct RustBoxInnerU8_RO : RustBoxInner_RO<byte>
    {
        private RustBoxU8 boxSuper;
        // private readonly byte* ptr;

        internal RustBoxInnerU8_RO(RustBoxU8 boxSuper/*, byte* ptr*/)
        {
            this.boxSuper = boxSuper;
            // this.ptr = ptr;
        }

        public readonly byte get()
        {
            return *this.ptr;
        }
    }

    public unsafe struct RustBoxInnerU8_RW : RustBoxInner_RW<byte>
    {
        private byte* ptr;

        internal RustBoxInnerU8_RW(byte* ptr)
        {
            this.ptr = ptr;
        }

        public readonly byte get()
        {
            return *this.ptr;
        }

        public byte* getMut()
        {
            return this.ptr;
        }
    }

    // public unsafe class MyType1 : IRustBox<MyType>
    // {
    //     private MangledType* ptr;

    //     public MyType* get()
    //     {

    //     }
    // }

    // public unsafe struct MangledType
    // {
    //     // this is the C struct
    // }

    // public unsafe class RustBoxU8 : IRustBox<byte>
    // {
    //     private byte* ptr;

    //     public void Dispose()
    //     {

    //     }
    // }

}
