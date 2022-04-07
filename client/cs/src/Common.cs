namespace Teleportal.Client
{
    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "tp_client";
#endif
    }
}
