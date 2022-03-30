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
        public static extern unsafe IntPtr teleportal_engine_get_state_handle_u8(IntPtr engine, IntPtr object_handle, IntPtr contract_handle, uint state_idx);

        [DllImport(LIBRARY_NAME)]
        public static extern byte teleportal_engine_get_state_value_u8(IntPtr engine, IntPtr state_handle);

        private static Engine shared; // singleton reference
        private IntPtr engine;
        private IntPtr objectHandle;
        private IntPtr contractHandle;

        public Engine()
        {
            Engine.shared = this;
            this.engine = teleportal_engine_init();
            this.contractHandle = teleportal_engine_get_contract_ffi_testing(this.engine);
            this.objectHandle = teleportal_engine_create_object(this.engine, this.contractHandle);
            
            IntPtr testStateHandle = teleportal_engine_get_state_handle_u8(this.engine, this.objectHandle, this.contractHandle, 0);
            byte testStateValue = GetState<byte>(testStateHandle);
            Debug.Log($"[Teleportal] U8 test: {testStateValue}");
        }

        public T GetState<T>(IntPtr stateHandle)
        {
            return GetState((dynamic) stateHandle);
        }

        private static byte GetState(IntPtr stateHandle)
        {
            return Engine.teleportal_engine_get_state_value_u8(Engine.shared.engine, stateHandle);
        }

    }

}
