use roaring::bitmap::RoaringBitmap;
use byteorder::{BigEndian, ByteOrder};

pub enum OrderedMsgIds {
    PlusOne(u32),
    MinusOne(u32),
    Full(RoaringBitmap),
    Delta(RoaringBitmap, RoaringBitmap),
}

impl OrderedMsgIds {
    pub fn get_value(&self) -> Vec<u8> {
        match self {
            &OrderedMsgIds::PlusOne(msg_id) => {
                let mut ret: [u8; 1 + 4] = [0; 1 + 4];
                ret[0] = 1;
                BigEndian::write_u32(&mut ret[1..], msg_id);
                ret.to_vec()
            }
            &OrderedMsgIds::MinusOne(msg_id) => {
                let mut ret: [u8; 1 + 4] = [0; 1 + 4];
                ret[0] = 2;
                BigEndian::write_u32(&mut ret[1..], msg_id);
                ret.to_vec()
            }
            &OrderedMsgIds::Full(ref map) => {
                let size = map.serialized_size();
                let mut ret: Vec<u8> = Vec::with_capacity(1 + size);
                ret.push(3);
                map.serialize_into(&mut ret).unwrap();
                ret
            }
            &OrderedMsgIds::Delta(ref add_map, ref remove_map) => {
                let add_size = add_map.serialized_size();
                let remove_size = remove_map.serialized_size();

                let mut ret: Vec<u8> = Vec::with_capacity(1 + 4 + 4 + add_size + remove_size);
                ret.push(4);
                let mut aa: Vec<u8> = vec![0; 4];
                let mut bb: Vec<u8> = vec![0; 4];
                BigEndian::write_u32(&mut aa[..], add_size as u32);
                BigEndian::write_u32(&mut bb[..], remove_size as u32);
                ret.extend(aa);
                ret.extend(bb);
                add_map.serialize_into(&mut ret).unwrap();
                remove_map.serialize_into(&mut ret).unwrap();
                ret
            }
        }
    }

    pub fn merge(blocks: Vec<OrderedMsgIds>) -> Option<OrderedMsgIds> {
        let mut ret = RoaringBitmap::default();
        for block in &blocks {
            block.add(&mut ret);
        }

        for block in &blocks {
            block.remove(&mut ret);
        }
        Some(OrderedMsgIds::Full(ret))
    }

    fn add(&self, msgs: &mut RoaringBitmap) {
        match self {
            &OrderedMsgIds::PlusOne(msg_id) => {
                msgs.insert(msg_id);
            }
            &OrderedMsgIds::MinusOne(_) => {}
            &OrderedMsgIds::Full(ref map) => {
                msgs.union_with(map);
            }
            &OrderedMsgIds::Delta(ref add_map, _) => {
                msgs.union_with(add_map);
            }
        }
    }

    fn remove(&self, msgs: &mut RoaringBitmap) {
        match self {
            &OrderedMsgIds::PlusOne(_) => {}
            &OrderedMsgIds::MinusOne(msg_id) => {
                msgs.remove(msg_id);
            }
            &OrderedMsgIds::Full(_) => {}
            &OrderedMsgIds::Delta(_, ref remove_map) => {
                msgs.union_with(remove_map);
            }
        }
    }
}
