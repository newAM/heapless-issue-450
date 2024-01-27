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

    let jsonstr: heapless::String<32> = serde_json_core::to_string(&node_msg_example).unwrap();

    println!("json: {jsonstr}");
}
