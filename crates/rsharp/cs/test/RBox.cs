using Xunit;
using RSharp;
using InvalidOperationException = System.InvalidOperationException;
using Thread = System.Threading.Thread;

namespace test
{

    public class TestRBox
    {
        private readonly Xunit.Abstractions.ITestOutputHelper output;

        public TestRBox(Xunit.Abstractions.ITestOutputHelper output)
        {
            this.output = output;
        }

        [Fact]
        public void TestU8()
        {
            output.WriteLine("Starting RBox U8 test");
            var u8 = new RBox_U8(10);
            output.WriteLine("Finished RBox U8 test");
            Assert.Equal(10, u8.Value);
            u8.Dispose();
            Assert.Throws<InvalidOperationException>(() => u8.Value);
        }
    }

}
