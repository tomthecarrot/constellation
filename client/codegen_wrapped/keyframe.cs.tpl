public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Value(this.Inner.Value.p);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
}

public double Time
{
    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Time(this.Inner.Value.p);
}
