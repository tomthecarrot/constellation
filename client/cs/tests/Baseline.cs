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
}
