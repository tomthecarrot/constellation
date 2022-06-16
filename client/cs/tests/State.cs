using States = Teleportal.Client.Contract.Properties.States;
using Sys = System;
using Xunit;
using RSharp;

public class TestState
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestState(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestI8()
    {
        var st = new States.State_I8(-20);
        Sys.Console.WriteLine(st.ToString());

        Assert.Equal(-20, st.Value.SharedValue);

        st.Value = new RBox_I8(10);
        Assert.Equal(10, st.Value.SharedValue);
    }

    [Fact]
    public void TestF64()
    {
        var st = new States.State_F64(20181.530152399);

        Assert.Equal(20181.530152399, st.Value.SharedValue);

        st.Value = new RBox_F64(-56817.5919827);
        Assert.Equal(-56817.5919827, st.Value.SharedValue);
    }
}
