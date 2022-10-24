using States = Teleportal.Client.Contract.Properties.States;
using Baseline = Teleportal.Client.Baseline;
using InvalidOperationException = System.InvalidOperationException;
using Xunit;
using RSharp;
using ExampleContract = Teleportal.Example.Contract.ExampleContract;

public class TestObject
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestObject(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestObjectCreate()
    {
        var baseline = new Baseline(true);
        var contract = ExampleContract.register(baseline);
        var s = contract.States;
        var obj = contract.ObjectCreate(baseline, 0, 1, -1, -2, 0.0f, 1.0f, new RString("foobar"));

        var u8_0 = baseline.BindStateU8(s.U8_0, obj);
        var u8_1 = baseline.BindStateU8(s.U8_1, obj);
        var f32_0 = baseline.BindStateF32(s.F32_0, obj);
        var f32_1 = baseline.BindStateF32(s.F32_1, obj);
        var i8_0 = baseline.BindStateI8(s.I8_0, obj);
        var i8_1 = baseline.BindStateI8(s.I8_1, obj);
        var str_0 = baseline.BindStateString(s.Str_0, obj);

        Assert.Equal(0, baseline.State(u8_0).Value.Value);
        Assert.Equal(1, baseline.State(u8_1).Value.Value);
        Assert.Equal(-1, baseline.State(i8_0).Value.Value);
        Assert.Equal(-2, baseline.State(i8_1).Value.Value);
        Assert.Equal(0.0, baseline.State(f32_0).Value.Value);
        Assert.Equal(1.0, baseline.State(f32_1).Value.Value);
        Assert.Equal("foobar", baseline.State(str_0).Value.Value);

        Assert.Equal(0, baseline.StateMut(u8_0).Value.Value);
        Assert.Equal(1, baseline.StateMut(u8_1).Value.Value);
        Assert.Equal(-1, baseline.StateMut(i8_0).Value.Value);
        Assert.Equal(-2, baseline.StateMut(i8_1).Value.Value);
        Assert.Equal(0.0, baseline.StateMut(f32_0).Value.Value);
        Assert.Equal(1.0, baseline.StateMut(f32_1).Value.Value);
        Assert.Equal("foobar", baseline.StateMut(str_0).Value.Value);

        obj.Dispose();
        Assert.Throws<InvalidOperationException>(() => { var _ = baseline.BindStateU8(s.U8_0, obj); });
    }
}
