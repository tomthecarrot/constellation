using RSharp;
using ffi = unity_states.generated.__Internal;
using Baseline = Teleportal.Client.Baseline;
using ContractDataHandle = Teleportal.Client.Contract.ContractDataHandle;
using ObjectHandle = Teleportal.Client.Object.ObjectHandle;

public class BallContract : OpaqueWrapper<BallContract>
{
    unsafe BallContract(Ptr<BallContract> inner) : base(inner, OwnershipSemantics.Owned) { }

    unsafe BallContract Register(Baseline baseline)
    {
        if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
        {
            throw new MutabilityException("`baseline` was not mutable!");
        }
        var p = new Ptr<BallContract>(ffi.BallContract_register(baseline.Inner.Value.p));
        return new BallContract(p);
    }

    unsafe void Unregister(Baseline baseline)
    {
        if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
        {
            throw new MutabilityException("`baseline` was not mutable!");
        }
        ffi.BallContract_unregister(baseline.Inner.Value.p, this.Handle.StealInner().p);
    }

    override protected void NativeDrop(Ptr<BallContract> inner)
    {
        ffi.BallContract_drop(inner.p);
    }

    ContractDataHandle Handle
    {
        get
        {
            var p = new Ptr<ContractDataHandle>(ffi.BallContract_handle(this.Inner.Value.p));
            return new ContractDataHandle(p);
        }
    }

    BallStates States
    {
        get
        {
            var p = new Ptr<BallStates>(ffi.BallContract_states(this.Inner.Value.p));
            return new BallStates(p);
        }
    }

    ObjectHandle ObjectCreate(
        Baseline baseline,
        float pos_x,
        float pos_y,
        float pos_z,
        short euler_x,
        short euler_y,
        short euler_z,
        float scale_x,
        float scale_y,
        float scale_z,
        ulong color
    )
    {
        if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
        {
            throw new MutabilityException("`baseline` was not mutable!");
        }
        var p = new Ptr<ObjectHandle>(ffi.BallContract_object_create(
            baseline.Inner.Value.p,
            this.Handle.StealInner().p,
            pos_x, pos_y, pos_z,
            euler_x, euler_y, euler_z,
            scale_x, scale_y, scale_z,
            color
        ));
        return new ObjectHandle(p);
    }

    void ObjectRemove(Baseline baseline, ContractDataHandle contract)
    {
        if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
        {
            throw new MutabilityException("`baseline was not mutable!");
        }

        ffi.BallContract_object_remove(baseline.Inner.Value.p, contract.StealInner().p);
    }
}
