using generated = tp_contract_example.generated.__Internal;
using RSharp;
using System.Collections.Generic;
using ObjectHandle = Teleportal.Client.Object.ObjectHandle;
using States = Teleportal.Client.Contract.Properties.States;
using Baseline = Teleportal.Client.Baseline;
using ContractDataHandle = Teleportal.Client.Contract.ContractDataHandle;

namespace Teleportal.Example.Contract
{

    public class ExampleContract : OpaqueWrapper<ExampleContract>
    {
        public ExampleContract(Ptr<ExampleContract> inner, OwnershipSemantics ownershipSemantics)
        : base(inner, ownershipSemantics)
        { }

        override protected void NativeDrop(Ptr<ExampleContract> inner)
        {
            generated.TpContractExampleExampleContractDrop(inner.p);
        }

        public static ExampleContract register(Baseline baseline)
        {
            if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new MutabilityException("`baseline` was not mutable!");
            }

            var p = new Ptr<ExampleContract>(
                generated.TpContractExampleExampleContractRegister(
                    baseline.Inner.Value.p
                )
            );
            return new ExampleContract(p, OwnershipSemantics.Owned);
        }

        public ContractDataHandle Handle
        {
            get
            {
                var p = new Ptr<ContractDataHandle>(
                    generated.TpContractExampleExampleContractHandle(
                        this.Inner.Value.p
                    )
                );
                return new ContractDataHandle(p, OwnershipSemantics.Owned);
            }
        }

        public ExampleStates States
        {
            get
            {
                var p = new Ptr<ExampleStates>(
                    generated.TpContractExampleExampleContractStates(
                        this.Inner.Value.p
                    )
                );
                return new ExampleStates(p);
            }
        }

        public ObjectHandle ObjectCreate(Baseline baseline, byte u8_0, byte u8_1, sbyte i8_0, sbyte i8_1, float f32_0, float f32_1, RString str_0)
        {
            if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new OwnershipException("`baseline` must be mutable");
            }
            if (str_0.OwnershipSemantics != OwnershipSemantics.Owned)
            {
                throw new OwnershipException("`str_0` must be owned");
            }
            var p = new Ptr<ObjectHandle>(generated.TpContractExampleExampleContractObjectCreate(this.Inner.Value.p, baseline.Inner.Value.p, u8_0, u8_1, i8_0, i8_1, f32_0, f32_1, str_0.StealInner().p));
            return new ObjectHandle(p, OwnershipSemantics.Owned);
        }

        public void ObjectRemove(Baseline baseline, ObjectHandle obj)
        {
            if (baseline.OwnershipSemantics == OwnershipSemantics.SharedRef)
            {
                throw new OwnershipException("`baseline` must be mutable");
            }
            generated.TpContractExampleExampleContractObjectRemove(baseline.Inner.Value.p, obj.Inner.Value.p);
        }
    }

    public class ExampleStates : OpaqueWrapper<ExampleStates>
    {
        public ExampleStates(Ptr<ExampleStates> p)
        : base(p, OwnershipSemantics.SharedRef)
        { }

        override protected void NativeDrop(Ptr<ExampleStates> inner)
        {
            throw new System.InvalidOperationException("Unreachable code reached");
        }

        public States.StateId_U8 U8_0
        {
            get
            {
                var p = new Ptr<States.StateId_U8>(generated.TpContractExampleExampleStatesU8_0(this.Inner.Value.p));
                return new States.StateId_U8(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_U8 U8_1
        {
            get
            {
                var p = new Ptr<States.StateId_U8>(generated.TpContractExampleExampleStatesU8_1(this.Inner.Value.p));
                return new States.StateId_U8(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_I8 I8_0
        {
            get
            {
                var p = new Ptr<States.StateId_I8>(generated.TpContractExampleExampleStatesI8_0(this.Inner.Value.p));
                return new States.StateId_I8(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_I8 I8_1
        {
            get
            {
                var p = new Ptr<States.StateId_I8>(generated.TpContractExampleExampleStatesI8_1(this.Inner.Value.p));
                return new States.StateId_I8(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_F32 F32_0
        {
            get
            {
                var p = new Ptr<States.StateId_F32>(generated.TpContractExampleExampleStatesF32_0(this.Inner.Value.p));
                return new States.StateId_F32(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_F32 F32_1
        {
            get
            {
                var p = new Ptr<States.StateId_F32>(generated.TpContractExampleExampleStatesF32_1(this.Inner.Value.p));
                return new States.StateId_F32(p, OwnershipSemantics.Owned);
            }
        }

        public States.StateId_String Str_0
        {
            get
            {
                var p = new Ptr<States.StateId_String>(generated.TpContractExampleExampleStatesStr0(this.Inner.Value.p));
                return new States.StateId_String(p, OwnershipSemantics.Owned);
            }
        }
    }
}
