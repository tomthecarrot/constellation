use bimap::BiHashMap;
use tp_client::contract::ContractDataHandle;

pub struct IdxMapper {
    pub contract: BiHashMap<usize, ContractDataHandle>,
}
