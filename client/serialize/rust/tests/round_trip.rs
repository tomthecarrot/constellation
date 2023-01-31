use tp_serialize::{Deserializer, DeserializerBuilder, Serializer};

use eyre::WrapErr;
use flatbuffers::FlatBufferBuilder;
use tp_client::baseline::{Baseline, BaselineKind};
use tp_client::contract::properties::dynamic::DynTpProperty;
use tp_client::contract::{Contract, ContractDataHandle, ContractId};
use tp_contract_example::ExampleContract;

struct EmptyContract {
    handle: ContractDataHandle,
}
impl Contract for EmptyContract {
    type States = ();

    type Channels = ();

    const ID: ContractId = ContractId {
        name: "empty",
        version: (0, 0, 0),
    };

    fn new(handle: tp_client::contract::ContractDataHandle) -> Self {
        Self { handle }
    }

    fn states(&self) -> &Self::States {
        &()
    }

    fn channels(&self) -> &Self::Channels {
        &()
    }

    fn handle(&self) -> tp_client::contract::ContractDataHandle {
        self.handle
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Fields {
    u8_0: u8,
    u8_1: u8,
    i8_0: i8,
    i8_1: i8,
    f32_0: f32,
    f32_1: f32,
    str_0: String,
}

#[test]
fn test_round_trip() -> eyre::Result<()> {
    let mut fields = Vec::new();
    for i in 0..10 {
        let f = Fields {
            // handles are not consistent across baselines, so we use this field for ID
            u8_0: i,
            u8_1: u8::MAX - i,
            i8_0: -(i as i8 + 1),
            i8_1: -(i as i8 + 2),
            f32_0: i as f32 + 0.1,
            f32_1: i as f32 + 0.2,
            str_0: i.to_string(),
        };
        fields.push(f);
    }

    let (empty_contract, example_contract, baseline) = create_baseline(&fields);
    check_matches_fields(&fields, &example_contract, &baseline)
        .wrap_err("`create_baseline` doesn't seem to be correct.")?;

    let bytes = {
        let mut serializer = Serializer::new(FlatBufferBuilder::new(), &baseline);
        serializer
            .serialize(&example_contract)
            .wrap_err("Failed to serialize ExampleContract")?;
        serializer
            .serialize(&empty_contract)
            .wrap_err("Failed to serialize EmptyContract")?;
        serializer.finish().finished_data().to_vec()
    };
    let (de_empty_contract, de_example_contract, de_baseline) = {
        let mut builder = DeserializerBuilder::new(&bytes, BaselineKind::Main)
            .wrap_err("Failed to create DeserializerBuilder")?;
        let de_example_contract: ExampleContract = builder
            .register_contract()
            .wrap_err("Failed to register ExampleContract")?;
        let de_empty_contract: EmptyContract = builder
            .register_contract()
            .wrap_err("Failed to register EmptyContract")?;
        let mut deserializer = builder.finish();

        deserializer
            .deserialize_objects(&de_example_contract)
            .wrap_err("Failed to deserialize objects in ExampleContract")?;
        deserializer
            .deserialize_objects(&de_empty_contract)
            .wrap_err("Failed to deserialize objects in EmptyContract")?;

        let de_baseline = deserializer
            .finish()
            .wrap_err("Failed to finish deserialization")?;
        (de_empty_contract, de_example_contract, de_baseline)
    };

    // Validate that EmptyContract and its objects were deserialized propery
    {
        let cd = de_baseline.contract_data(de_empty_contract.handle())?;
        assert_eq!(
            cd.id(),
            EmptyContract::ID,
            "ID of deserialized EmptyContract did not match"
        );
        assert_eq!(
            cd.objects().len(),
            1,
            "Expected deserialized EmptyContract to have exactly 1 object"
        );
    }

    // Validate that all ExampleContract objects in the two baselines match
    // TODO: Currently this does not check if handle types are correct!
    check_matches_fields(&fields, &de_example_contract, &de_baseline)?;
    check_matches_fields(&fields, &example_contract, &de_baseline)?;
    check_matches_fields(&fields, &de_example_contract, &baseline)?;

    Ok(())
}

fn create_baseline(fields: &[Fields]) -> (EmptyContract, ExampleContract, Baseline) {
    let mut b = Baseline::new(BaselineKind::Main);

    let empty_c: EmptyContract = b.register_contract().expect("Faild to register contract");
    let empty_obj = b
        .object_create(&empty_c, [].into_iter(), [].into_iter())
        .expect("Failed to create object");

    let c: ExampleContract = b.register_contract().expect("Failed to register contract");

    let mut objs = Vec::with_capacity(fields.len());
    for f in fields.iter() {
        let states = [
            DynTpProperty::Primitive((f.u8_0).into()),
            DynTpProperty::Primitive((f.u8_1).into()),
            DynTpProperty::Primitive((f.i8_0).into()),
            DynTpProperty::Primitive((f.i8_1).into()),
            DynTpProperty::Primitive((f.f32_0).into()),
            DynTpProperty::Primitive((f.f32_1).into()),
            DynTpProperty::Primitive((f.str_0.to_owned()).into()),
            DynTpProperty::Primitive(empty_obj.into()),
            DynTpProperty::Primitive(empty_c.handle().into()),
        ];
        let obj = b
            .object_create(&c, states.into_iter(), [].into_iter())
            .expect("Failed to create object");
        objs.push(obj);
    }

    (empty_c, c, b)
}

fn check_matches_fields(fields: &[Fields], c: &ExampleContract, b: &Baseline) -> eyre::Result<()> {
    let mut fields = fields.to_vec();
    let cd = b
        .contract_data(c.handle())
        .expect("Failed to get contract data from handle");

    assert_eq!(fields.len(), cd.objects().len(), "# of objects mismatched");

    for obj_h in cd.objects() {
        let obj_h = *obj_h;
        let u8_0_h = b
            .bind_state(c.states().u8_0(), obj_h)
            .expect("Failed to bind u8_0");
        let u8_1_h = b
            .bind_state(c.states().u8_1(), obj_h)
            .expect("Failed to bind u8_1");
        let i8_0_h = b
            .bind_state(c.states().i8_0(), obj_h)
            .expect("Failed to bind i8_0");
        let i8_1_h = b
            .bind_state(c.states().i8_1(), obj_h)
            .expect("Failed to bind i8_1");
        let f32_0_h = b
            .bind_state(c.states().f32_0(), obj_h)
            .expect("Failed to bind f32_0");
        let f32_1_h = b
            .bind_state(c.states().f32_1(), obj_h)
            .expect("Failed to bind f32_1");
        let str_0_h = b
            .bind_state(c.states().str_0(), obj_h)
            .expect("Failed to bind str_0");

        let baseline_fields = Fields {
            u8_0: b.state(u8_0_h).expect("Failed to get u8_0").value,
            u8_1: b.state(u8_1_h).expect("Failed to get u8_1").value,
            i8_0: b.state(i8_0_h).expect("Failed to get i8_0").value,
            i8_1: b.state(i8_1_h).expect("Failed to get i8_1").value,
            f32_0: b.state(f32_0_h).expect("Failed to get f32_0").value,
            f32_1: b.state(f32_1_h).expect("Failed to get f32_1").value,
            str_0: b.state(str_0_h).expect("Failed to get str_0").value.clone(),
        };

        fields.remove(
            fields
                .iter()
                .position(|f| *f == baseline_fields)
                .expect("baseline contained an unmatched `Fields`"),
        );
    }

    // No fields should be left, because all fields were matched with baseline data
    assert_eq!(fields.len(), 0);

    Ok(())
}
