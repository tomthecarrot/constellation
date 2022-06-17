public unsafe ContractDataHandle Contract
{
    get
    {
        var raw_ptr = generated.__Internal.TpClientContractPropertiesStatesStateId{{inner_mangled_name}}Contract(this.Inner.Value.p);
        var ptr = new Ptr<ContractDataHandle>(raw_ptr);
        return new ContractDataHandle(ptr);
    }
}
