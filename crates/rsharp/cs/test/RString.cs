using Sys = System;
using Xunit;
using RSharp;

public class TestString
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestString(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestString1()
    {
        // var rs = new RString("Hello!");
        // output.WriteLine(rs.ToString());

        // Assert.Equal("Hello!", rs.Value);

        // rs.Dispose();
        // Assert.Throws<Sys.InvalidOperationException>(() => { var val = rs; });
    }
}
