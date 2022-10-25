use tp_serialize::{Deserializer, Serializer};

use flatbuffers::FlatBufferBuilder;
use tp_client::baseline::{Baseline, BaselineKind};
use tp_client::contract::properties::dynamic::DynTpProperty;
use tp_client::contract::Contract;
use tp_contract_example::ExampleContract;

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
fn test_round_trip() {
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

    let (contract, baseline) = create_baseline(&fields);
    check_matches_fields(&fields, &contract, &baseline);

    let bytes = {
        let mut serializer = Serializer::new(FlatBufferBuilder::new(), &baseline);
        serializer
            .serialize(&contract)
            .expect("Failed to serialize contract");
        serializer.finish().finished_data().to_vec()
    };
    let (deserialized_contract, deserialized_baseline) = {
        let mut deserializer = Deserializer::new(&bytes, BaselineKind::Main);
        let deserialized_contract = deserializer
            .deserialize::<ExampleContract>()
            .expect("Failed to deserialize contract");
        let deserialized_baseline = deserializer.finish();
        (deserialized_contract, deserialized_baseline)
    };

    check_matches_fields(&fields, &deserialized_contract, &deserialized_baseline);
    check_matches_fields(&fields, &contract, &deserialized_baseline);
    check_matches_fields(&fields, &deserialized_contract, &baseline);
}

fn create_baseline(fields: &[Fields]) -> (ExampleContract, Baseline) {
    let mut b = Baseline::new(BaselineKind::Main);
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
        ];
        let obj = b
            .object_create(&c, states.into_iter(), [].into_iter())
            .expect("Failed to create object");
        objs.push(obj);
    }

    (c, b)
}

fn check_matches_fields(fields: &[Fields], c: &ExampleContract, b: &Baseline) {
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
}
