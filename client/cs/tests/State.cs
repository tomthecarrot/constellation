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
        var st = new States.State_I8(new RBox_I8(-20));
        output.WriteLine(st.ToString());

        Assert.Equal(-20, st.Value.Value);

        st.Value = new RBox_I8(10);
        Assert.Equal(10, st.Value.Value);

        st.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var val = st.Value; });
    }

    [Fact]
    public void TestF64()
    {
        var b = new RBox_F64(20181.530152399);
        var st = new States.State_F64(b);
        // `b` should be moved from!
        Assert.Null(b.Inner);
        Assert.Equal(20181.530152399, st.Value.Value);

        st.Value = new RBox_F64(-56817.5919827);
        Assert.Equal(-56817.5919827, st.Value.Value);

        st.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var val = st.Value; });
    }

    [Fact]
    public void TestString()
    {
        var rs = new RString("Hello!");
        var st = new States.State_String(rs);
        output.WriteLine(st.ToString());

        Assert.Equal("Hello!", st.Value.Value);

        // Ensure that RString was moved into the State.
        Assert.Null(rs.Inner);
        rs.Dispose();
        Assert.Equal("Hello!", st.Value.Value);

        st.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => st.Value);
    }
}
