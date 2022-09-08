using Xunit;
using RSharp;
using InvalidOperationException = System.InvalidOperationException;


namespace test
{

    public class TestRBox
    {
        [Fact]
        public void TestU8()
        {
            var u8 = new RBox_U8(10);
            Assert.Equal(10, u8.Value);
            u8.Dispose();
            Assert.Throws<InvalidOperationException>(() => u8.Value);
        }
    }

}
