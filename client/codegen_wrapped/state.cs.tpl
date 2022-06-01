public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Ptr?.p ?? IntPtr.Zero);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
    set
    {
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Ptr?.p ?? IntPtr.Zero, value);
    }
}
