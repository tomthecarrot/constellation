using System;
using System.Runtime.InteropServices;
using UnityEngine;

namespace Teleportal.Platform
{
    public class Engine
    {
#if UNITY_IOS && !UNITY_EDITOR
        private const string LIBRARY_NAME = "__Internal";
#else
        private const string LIBRARY_NAME = "tp_client";
#endif

        [DllImport(LIBRARY_NAME)]
        public static extern unsafe IntPtr teleportal_engine_init();

        [DllImport(LIBRARY_NAME)]
        public static extern unsafe IntPtr teleportal_engine_get_contract_ffi_testing(IntPtr engine);

        [DllImport(LIBRARY_NAME)]
        public static extern unsafe IntPtr teleportal_engine_create_object(IntPtr engine, IntPtr contract_handle);

        [DllImport(LIBRARY_NAME)]
        public static extern unsafe IntPtr teleportal_engine_get_state_handle_u8(IntPtr engine, IntPtr object_handle, uint state_idx);

        [DllImport(LIBRARY_NAME)]
        public static extern byte teleportal_engine_get_state_value_u8(IntPtr engine, IntPtr state_handle);

        private EngineRef engine;
        private IntPtr defaultContractHandle;

        public Engine()
        {
            this.engine = teleportal_engine_init();
            this.defaultContractHandle = teleportal_engine_get_contract_ffi_testing(this.engine);

            IntPtr testStateHandle =
            byte testStateValue = GetState<byte>(testStateHandle);
            Debug.Log($"[Teleportal] U8 test: {testStateValue}");
        }

        public IntPtr CreateObjectWithTestContract()
        {
            return teleportal_engine_create_object(this.engine, this.defaultContractHandle);
        }

        public IntPtr GetStateHandle(IntPtr objectHandle, int index)
        {
            return teleportal_engine_get_state_handle_u8(this.engine, this.objectHandle, 0);
        }

        private IntPtr GetStateHandle<T>(IntPtr objectHandle, int index)
        {
            StateHandle<T> handle = GetStateHandle
        }

        public T GetStateValue<T>(IntPtr stateHandle)
        {
            return GetStateValue((dynamic)stateHandle);
        }

        private byte GetStateValue(IntPtr stateHandle)
        {
            return Engine.teleportal_engine_get_state_value_u8(this.engine, stateHandle);
        }

    }

}
