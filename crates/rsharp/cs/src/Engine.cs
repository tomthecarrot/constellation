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

        [DllImport(LIBRARY_NAME)]
        public static extern tp_client__contract__properties__channels__Keyframe_U8

        public struct tp_client__contract__properties__channels__Keyframe_U8 { }

        private EngineRef engine;
        private ContractHandle defaultContractHandle;

        public Engine()
        {
            this.engine = new EngineRef(teleportal_engine_init());
            this.defaultContractHandle = new ContractHandle(teleportal_engine_get_contract_ffi_testing(this.engine.Ptr));
            Test();
        }

        private void Test()
        {
            ObjectHandle testObject = CreateObjectWithTestContract();
            StateHandle<byte> testState = GetStateHandle/*<byte>*/(testObject, 0);
            byte testValue = GetStateValue/*<byte>*/(testState);
            Debug.Log($"[Teleportal] U8 test: {testValue}");
        }

        public ObjectHandle CreateObjectWithTestContract()
        {
            return new ObjectHandle(teleportal_engine_create_object(this.engine.Ptr, this.defaultContractHandle.Ptr));
        }

        // private StateHandle<T> GetStateHandle(ObjectHandle objectHandle, int index)
        // {
        //     return GetStateHandle<T>(objectHandle, index);
        // }

        public StateHandle<byte> GetStateHandle(ObjectHandle objectHandle, int index)
        {
            return new StateHandle<byte>(teleportal_engine_get_state_handle_u8(this.engine.Ptr, objectHandle.Ptr, 0));
        }

        // public T GetStateValue<T>(StateHandle<T> stateHandle)
        // {
        //     return GetStateValue((dynamic) stateHandle);
        // }

        private byte GetStateValue(StateHandle<byte> stateHandle)
        {
            return Engine.teleportal_engine_get_state_value_u8(this.engine.Ptr, stateHandle.Ptr);
        }

    }

}
