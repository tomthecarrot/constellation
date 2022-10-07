using Baseline = Teleportal.Client.Baseline;
using Xunit;
using InvalidOperationException = System.InvalidOperationException;

public class TestBallContract
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestBallContract(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void Test()
    {
        var baseline = new Baseline(true);
        var ballContract = BallContract.Register(baseline);
        var ballStates = ballContract.States;
        var ballObject = ballContract.ObjectCreate(
            baseline,
            420.69f, 0f, 0f,
            0, 0, 0,
            0f, 0f, 0f,
            0
        );

        var stateHandle = baseline.BindStateF32(ballStates.PosX, ballObject);
        var state = baseline.State(stateHandle);

        Assert.Equal(420.69f, state.Value.Value);

        ballObject.Dispose();

        Assert.Throws<InvalidOperationException>(() => { var _ = baseline.BindStateF32(ballStates.PosX, ballObject); });
    }
}
