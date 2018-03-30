pub struct MsgIndexer;

use Msg;
use mutation::IndexMutation;
impl MsgIndexer {
    pub fn index(msg: &Msg) -> IndexMutation {
        let mut msg_index = IndexMutation::new();
        for header in &msg.headers {
            match header.0.as_str() {
                "From" => {
                }
                _ => {}
            }
        }
        msg_index
    }
}
