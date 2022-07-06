using Baseline = Teleportal.Client.Baseline;
using Xunit;
using InvalidOperationException = System.InvalidOperationException;

public class TestBaseline
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestBaseline(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestKind()
    {
        var main = new Baseline(true);
        var fork = new Baseline(false);

        Assert.True(main.IsMain);
        Assert.False(fork.IsMain);

        main.Dispose();
        fork.Dispose();

        Assert.Throws<InvalidOperationException>(() => main.IsMain);
        Assert.Throws<InvalidOperationException>(() => fork.IsMain);
    }

    [Fact]
    public void TestExampleContract()
    {
        var b = new Baseline(true);
        var c = b.registerContractExample();

        var handle = c.Handle;
        var states = c.States;

        states.Dispose();
        handle.Dispose();
        c.Dispose();
        Assert.Throws<InvalidOperationException>(() => c.Handle);
        b.Dispose();
        Assert.Throws<InvalidOperationException>(() => b.IsMain);
    }
}
