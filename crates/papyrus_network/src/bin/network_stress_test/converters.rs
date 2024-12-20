use std::time::{Duration, SystemTime};

struct StressTestMessage {
    id: u32,
    payload: Vec<u8>,
    time: SystemTime,
}

impl From<StressTestMessage> for Vec<u8> {
    fn from(value: StressTestMessage) -> Self {
        let StressTestMessage { id, mut payload, time } = value;
        let id = id.to_be_bytes().to_vec();
        let time = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let seconds = time.as_secs().to_be_bytes().to_vec();
        let nanos = time.subsec_nanos().to_be_bytes().to_vec();
        payload.extend(id);
        payload.extend(seconds);
        payload.extend(nanos);
        payload
    }
}

impl From<Vec<u8>> for StressTestMessage {
    // This auto implements TryFrom<Vec<u8>> for StressTestMessage
    fn from(mut value: Vec<u8>) -> Self {
        let vec_size = value.len();
        let payload_size = vec_size - 12;
        let id_and_time = value.split_off(payload_size);
        let id = u32::from_be_bytes(id_and_time[0..4].try_into().unwrap());
        let seconds = u64::from_be_bytes(id_and_time[4..12].try_into().unwrap());
        let nanos = u32::from_be_bytes(id_and_time[12..16].try_into().unwrap());
        let time = SystemTime::UNIX_EPOCH + Duration::new(seconds, nanos);
        StressTestMessage { id, payload: value, time }
    }
}
