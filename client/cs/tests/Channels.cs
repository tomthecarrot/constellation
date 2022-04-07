using Channels = Teleportal.Client.Contract.Properties.Channels;
using Sys = System;
using Xunit;

public class TestChannels
{
    [Fact]
    public void TestKeyframe()
    {
        var kf = new Channels.Keyframe_U8(10, 1.0);
        Sys.Console.WriteLine(kf.ToString());

        Assert.Equal(10, kf.value);
        Assert.Equal(1.0, kf.time);

        kf.Dispose();
        // Sys.Console.WriteLine(kf.time); // aborts
    }
}
