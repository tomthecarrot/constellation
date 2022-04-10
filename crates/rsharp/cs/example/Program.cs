using System;
using Teleportal.Rsharp;

class MainClass
{
    static void Main(string[] args)
    {
        MyType type = new MyType(0, 1);
        Console.WriteLine($"Hello, World! Type data: {type.A} & {type.B}");
    }
}
