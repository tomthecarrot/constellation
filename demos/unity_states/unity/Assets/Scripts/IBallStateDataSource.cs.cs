public interface IBallStateDataSource
{
    BallStateDataSourceType type { get; }
    void LogCurrentData();
    float pos_x { get; }
    float pos_y { get; }
    float pos_z { get; }
    short euler_x { get; }
    short euler_y { get; }
    short euler_z { get; }
    float scale_x { get; }
    float scale_y { get; }
    float scale_z { get; }
    ulong color { get; }
}
