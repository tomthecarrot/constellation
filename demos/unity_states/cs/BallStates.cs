using RSharp;
using States = Teleportal.Client.Contract.Properties.States;
using ffi = unity_states.generated.__Internal;

/// NOTE: this class is only ever a shared ref in rust
public struct BallStates
{
    readonly Ptr<BallStates> inner;

    public BallStates(Ptr<BallStates> inner)
    {
        this.inner = inner;
    }

    public States.StateId_F32 PosX
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_pos_x(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_F32 PosY
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_pos_y(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_F32 PosZ
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_pos_z(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_I16 EulerX
    {
        get => new States.StateId_I16(
            new Ptr<States.StateId_I16>(ffi.BallStates_euler_x(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_I16 EulerY
    {
        get => new States.StateId_I16(
            new Ptr<States.StateId_I16>(ffi.BallStates_euler_y(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_I16 EulerZ
    {
        get => new States.StateId_I16(
            new Ptr<States.StateId_I16>(ffi.BallStates_euler_z(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_F32 ScaleX
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_scale_x(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_F32 ScaleY
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_scale_y(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_F32 ScaleZ
    {
        get => new States.StateId_F32(
            new Ptr<States.StateId_F32>(ffi.BallStates_scale_z(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }

    public States.StateId_U64 Color
    {
        get => new States.StateId_U64(
            new Ptr<States.StateId_U64>(ffi.BallStates_color(this.inner.p)),
            OwnershipSemantics.Owned
        );
    }
}
