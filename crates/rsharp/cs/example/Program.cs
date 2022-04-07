using System;
using RSharp;
using System.Runtime.InteropServices;

/// Stuff imported by PInvoke
namespace C_FFI
{

    public class Metadata
    {
#if UNITY_IOS && !UNITY_EDITOR
        internal const string LIBRARY_NAME = "__Internal";
#else
        internal const string LIBRARY_NAME = "rsharp";
#endif
    }

    public struct MyType
    {
        public int a;
        public float b;

        [DllImport(Metadata.LIBRARY_NAME)]
        internal static extern unsafe MyType* MyType__new(int a, float b);

        [DllImport(Metadata.LIBRARY_NAME)]
        internal static extern unsafe void MyType__drop(MyType* ptr);
    }
}

public unsafe class MyType : RSharp.OwnedHandle<C_FFI.MyType>
{
    public C_FFI.MyType* _ptr;

    public MyType(int a, float b)
    {
        this._ptr = C_FFI.MyType.MyType__new(a, b);
    }

    public MyType(C_FFI.MyType* ptr)
    {
        this._ptr = ptr;
    }

    public int A => (*this._ptr).a;

    public float B => (*this._ptr).b;

    public void Dispose()
    {
        C_FFI.MyType.MyType__drop(this._ptr);
    }

    public C_FFI.MyType* Ptr
    {
        get => this._ptr;
        set => this._ptr = value;
    }
}

class MainClass
{
    static void Main(string[] args)
    {
        MyType type = new MyType(0, 1);
        Console.WriteLine($"Hello, World! Type data: {type.A} & {type.B}");
    }
}
