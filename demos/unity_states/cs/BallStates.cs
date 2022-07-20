using RSharp;

/// NOTE: this class is only ever a shared ref in rust
public struct BallStates
{
    readonly Ptr<BallStates> inner;

    public BallStates(Ptr<BallStates> inner)
    {
        this.inner = inner;
    }

    // TODO: implement StateId accessors
}
