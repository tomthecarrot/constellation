using System.Runtime.InteropServices;

namespace rsharp
{
    public class FFI
    {
#if UNITY_IOS && !UNITY_EDITOR
        private const string LIBRARY_NAME = "__Internal";
#else
        private const string LIBRARY_NAME = "tp_client";
#endif

        [DllImport(LIBRARY_NAME)]
        public static extern unsafe byte tp_client__contract__properties__channels__get_keyframe_value_u8(Keyframe_U8* keyframe);
    }

    public struct Keyframe_U8 { }

    public struct Keyframe_String { }
}
