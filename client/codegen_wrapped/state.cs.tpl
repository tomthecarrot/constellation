public {{type_cs}}{{ptr_literal}} ValueBoxed
{
    get
    {
        unsafe
        {
            return generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}Value(this.Inner.Value.p);
        }
    }
    set
    {
        unsafe
        {
            generated.__Internal.TpClientContractPropertiesStatesState{{type_platform}}ValueSet(this.Inner.Value.p, value);
        }
    }
}

public {{type_cs}} Value
{
    get
    {
        unsafe
        {
            return ToManaged.f(OwnershipSemantics.SharedRef, this.ValueBoxed);
        }
    }
}
