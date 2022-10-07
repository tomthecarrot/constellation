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
        var contract = BallContract.Register(baseline);
        var states = contract.States;
        // var obj = contract.ObjectCreate(baseline, "some_addressable_id", "Evan");

        // var stateHandle = baseline.BindStateObjectHandle(s_slot.CharacterObject, obj_slot);
        // var state = baseline.State(stateHandle);

        // Assert.Equal("Bevan", stateHandle.Value.Value);

        // obj.Dispose();

        // Assert.Throws<InvalidOperationException>(() => { var _ = baseline.BindStateObjectHandle(states.CharacterObject, obj_slot); });
    }
}
