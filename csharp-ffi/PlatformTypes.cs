using System;

namespace Teleportal.Platform
{
    public struct EngineRef {
        private IntPtr ptr;

        public IntPtr Ptr
        {
            get
            {
                return this.ptr;
            }
        }

        public EngineRef(IntPtr ptr)
        {
            this.ptr = ptr;
        }
    }

    public struct ContractHandle {
        private IntPtr ptr;

        public IntPtr Ptr
        {
            get
            {
                return this.ptr;
            }
        }

        public ContractHandle(IntPtr ptr)
        {
            this.ptr = ptr;
        }
    }

    public struct ObjectHandle {
        private IntPtr ptr;

        public IntPtr Ptr
        {
            get
            {
                return this.ptr;
            }
        }

        public ObjectHandle(IntPtr ptr)
        {
            this.ptr = ptr;
        }
    }

    public struct StateHandle<T> {
        private IntPtr ptr;

        public IntPtr Ptr
        {
            get
            {
                return this.ptr;
            }
        }

        public StateHandle(IntPtr ptr)
        {
            this.ptr = ptr;
        }
    }
}
