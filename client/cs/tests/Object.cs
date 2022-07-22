using States = Teleportal.Client.Contract.Properties.States;
using Baseline = Teleportal.Client.Baseline;
using InvalidOperationException = System.InvalidOperationException;
using Xunit;
using RSharp;

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
        var contract = baseline.registerContractExample();
        var s = contract.States;
        var obj = contract.ObjectCreate(baseline, 0, 1, -1, -2, 0.0f, 1.0f);

        var u8_0 = baseline.BindStateU8(s.U8_0, obj);
        var u8_1 = baseline.BindStateU8(s.U8_1, obj);
        var f32_0 = baseline.BindStateF32(s.F32_0, obj);
        var f32_1 = baseline.BindStateF32(s.F32_1, obj);
        var i8_0 = baseline.BindStateI8(s.I8_0, obj);
        var i8_1 = baseline.BindStateI8(s.I8_1, obj);

        Assert.Equal(0, baseline.State(u8_0).Value.Value);
        Assert.Equal(1, baseline.State(u8_1).Value.Value);
        Assert.Equal(-1, baseline.State(i8_0).Value.Value);
        Assert.Equal(-2, baseline.State(i8_1).Value.Value);
        Assert.Equal(0.0, baseline.State(f32_0).Value.Value);
        Assert.Equal(1.0, baseline.State(f32_1).Value.Value);

        Assert.Equal(0, baseline.StateMut(u8_0).Value.Value);
        Assert.Equal(1, baseline.StateMut(u8_1).Value.Value);
        Assert.Equal(-1, baseline.StateMut(i8_0).Value.Value);
        Assert.Equal(-2, baseline.StateMut(i8_1).Value.Value);
        Assert.Equal(0.0, baseline.StateMut(f32_0).Value.Value);
        Assert.Equal(1.0, baseline.StateMut(f32_1).Value.Value);

        obj.Dispose();
        Assert.Throws<InvalidOperationException>(() => { var _ = baseline.BindStateU8(s.U8_0, obj); });
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
        Assert.Throws<InvalidOperationException>(() => { var val = st.Value; });
    }
}
