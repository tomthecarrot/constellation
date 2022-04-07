using System;

namespace rsharp
{
    public unsafe class RustBox<T> where T : unmanaged, IDisposable
    {
        private T* ptr;

        public RustBox(T* ptr)
        {
            this.ptr = ptr;
        }

        public void Dispose()
        {

        }
    }
}
