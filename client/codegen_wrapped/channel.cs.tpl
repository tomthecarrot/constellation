public unsafe {{type_cs}} Keyframes
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Value(this.Inner?.p ?? IntPtr.Zero);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
}

public double Time
{
    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe{{type_platform}}Time(this.Inner?.p ?? IntPtr.Zero);
}
