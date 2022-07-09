using States = Teleportal.Client.Contract.Properties.States;
using Sys = System;
using Xunit;
using RSharp;
using Baseline = Teleportal.Client.Baseline;

public class TestStateId
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestStateId(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    public void TestContractDataHandle()
    {
        var baseline = new Baseline(true);
        var contract = baseline.registerContractExample();
        var s = contract.States;

        var u8_0 = s.U8_0;
        var u8_1 = s.U8_1;
        var i8_0 = s.I8_0;
        var i8_1 = s.I8_1;
        var f32_0 = s.F32_0;
        var f32_1 = s.F32_1;

        Assert.Equal(contract.Handle, u8_0.Contract);
        Assert.Equal(contract.Handle, u8_1.Contract);
        Assert.Equal(contract.Handle, i8_0.Contract);
        Assert.Equal(contract.Handle, i8_1.Contract);
        Assert.Equal(contract.Handle, f32_0.Contract);
        Assert.Equal(contract.Handle, f32_1.Contract);

        u8_0.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = u8_0.Contract; });
        u8_1.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = u8_1.Contract; });
        i8_0.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = i8_0.Contract; });
        i8_0.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = i8_1.Contract; });
        f32_0.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = f32_0.Contract; });
        f32_0.Dispose();
        Assert.Throws<Sys.InvalidOperationException>(() => { var _ = f32_1.Contract; });
    }
}
