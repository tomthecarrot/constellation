using States = Teleportal.Client.Contract.Properties.States;
using Sys = System;
using Xunit;
using RSharp;

public class TestString
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestString(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestString1()
    {
        var st = new States.State_String(RString.FromManaged("Hello!"));
        // output.WriteLine(st.ToString());

        // Assert.Equal("Hello!", st.Value.Value);

        // st.Dispose();
        // Assert.Throws<Sys.InvalidOperationException>(() => { var val = st.Value; });
    }
}
