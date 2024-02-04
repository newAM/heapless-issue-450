#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodeMsgExample<'a> {
    pub msg_str: &'a str,
    pub msg_random_num: u64,
}

fn main() {
    let node_msg_example = NodeMsgExample {
        msg_str: "node_2",
        msg_random_num: 123456,
    };

    let json_heapless: heapless::String<32> =
        serde_json_core::to_string(&node_msg_example).unwrap();

    let json_str: &str = json_heapless.as_str();

    let value = zenoh::value::Value::from(json_str);
}
