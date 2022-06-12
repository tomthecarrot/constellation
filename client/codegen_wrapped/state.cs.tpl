public unsafe {{type_cs}}{{ptr_literal}} ValueBoxed
{
    get
    {
        return generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
    }
    set
    {
        generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, value);
    }
}

public unsafe {{type_cs}} Value
{
    get
    {
        return ToManaged.f(OwnershipSemantics.SharedRef, this.ValueBoxed);
    }
}
