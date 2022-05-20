public unsafe {{type_cs}} Value
{
    get
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Ptr?.p ?? IntPtr.Zero);
        return ToManaged.f(OwnershipSemantics.SharedRef, result);
    }
    set
    {
        var result = generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueMut(this.Ptr?.p ?? IntPtr.Zero);
        var ptr = ToManaged.f(OwnershipSemantics.MutRef, result);
        *ptr = value;
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Drop(ptr);
    }
}
