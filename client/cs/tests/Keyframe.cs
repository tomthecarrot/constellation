using Channels = Teleportal.Client.Contract.Properties.Channels;
using Sys = System;
using Xunit;

public class TestKeyframe
{
    private readonly Xunit.Abstractions.ITestOutputHelper output;

    public TestKeyframe(Xunit.Abstractions.ITestOutputHelper output)
    {
        this.output = output;
    }

    [Fact]
    public void TestScalar()
    {
        var kf = new Channels.Keyframe_U8(10, 1.0);
        Sys.Console.WriteLine(kf.ToString());

        Assert.Equal(10, kf.Value);
        Assert.Equal(1.0, kf.Time);

        kf.Dispose();
        // Sys.Console.WriteLine(kf.time); // aborts
    }

    [Fact]
    public void TestVec()
    {
        var v = new Channels.RVec_Keyframe_U8();

        var kf0 = new Channels.Keyframe_U8(0, 0.0);
        var kf1 = new Channels.Keyframe_U8(1, 0.5);
        var kf2 = new Channels.Keyframe_U8(2, 1.0);
        var kf3 = new Channels.Keyframe_U8(3, 1.5);

        v.push(kf0);
        v.push(kf1);
        v.push(kf2);
        v.push(kf3);

        Assert.Equal(0, v[0].Value);
        Assert.Equal(0.0, v[0].Time);

        Assert.Equal(1, v[1].Value);
        Assert.Equal(0.5, v[1].Time);

        Assert.Equal(2, v[2].Value);
        Assert.Equal(1.0, v[2].Time);

        Assert.Equal(3, v[3].Value);
        Assert.Equal(1.5, v[3].Time);

    }
}
