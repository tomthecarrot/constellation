public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Value(this.Ptr?.p ?? IntPtr.Zero);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
}

public double Time
{
    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Time(this.Ptr?.p ?? IntPtr.Zero);
}
