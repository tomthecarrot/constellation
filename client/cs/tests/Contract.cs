using Baseline = Teleportal.Client.Baseline;
using Xunit;
using InvalidOperationException = System.InvalidOperationException;
using ExampleContract = Teleportal.Client.Contract.ExampleContract;
using Encoding = System.Text.Encoding;

public class TestContract
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestContract(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestExampleContract()
    {
        var b = new Baseline(true);
        var c = ExampleContract.register(b);

        var handle = c.Handle;

        var c_data = b.ContractData(handle);
        var name = c_data.Id.Name;

        Assert.Equal("teleportal.example-ffi-contract", Encoding.UTF8.GetString(name));
        Assert.Equal(c_data.Id.Version, (1, 2, 3));

        var c_data2 = b.ContractData(handle);
        Assert.Equal(Encoding.UTF8.GetString(c_data.Id.Name), Encoding.UTF8.GetString(c_data2.Id.Name));
        Assert.Equal(c_data.Id.Version, c_data2.Id.Version);

        c_data.Dispose();
        c_data2.Dispose();
        Assert.Throws<InvalidOperationException>(() => c_data.Id);
        Assert.Throws<InvalidOperationException>(() => c_data2.Id);

        handle.Dispose();
        c.Dispose();
        Assert.Throws<InvalidOperationException>(() => c.Handle);
        b.Dispose();
        Assert.Throws<InvalidOperationException>(() => b.IsMain);
    }
}
