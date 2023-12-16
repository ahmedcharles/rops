use indexmap::indexmap;

use crate::*;

impl MockTestUtil for RopsTree<Decrypted> {
    fn mock() -> Self {
        Self::Map(indexmap! {
            "hello".to_string() => RopsTree::Leaf(RopsValue::String("world!".to_string())),
            "nested_map".to_string() => RopsTree::Map(indexmap! {
                    "null_key".to_string() => RopsTree::Null,
                    "array".to_string() => RopsTree::Sequence(vec![
                        RopsTree::Leaf(RopsValue::String("string".to_string())),
                        RopsTree::Map(indexmap! {
                            "nested_map_in_array".to_string() => RopsTree::Map(indexmap!{
                                "integer".to_string() => RopsTree::Leaf(RopsValue::Integer(1234))
                            }),
                        }),
                        RopsTree::Map(indexmap!{
                            "float".to_string() => RopsTree::Leaf(RopsValue::Float(1234.56789))
                        }),
                    ]),
                }
            ),
            "booleans".to_string() => RopsTree::Sequence(vec![
                RopsTree::Leaf(RopsValue::Boolean(true)),
                RopsTree::Leaf(RopsValue::Boolean(false))
            ])
        })
    }
}
