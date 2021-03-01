/// Message type of the PDU.
///
/// Has the following types:
///     * `CON`: confirmable message (requires ACK/RST)
///     * `NON`: non-confirmable message (one-shot message)
///     * `ACK`: used to acknowledge confirmable messages
///     * `RST`: indicates error in received messages
#[derive(Clone, Copy)]
enum MessageType {
    CON = 0,
    NON = 1,
    ACK = 2,
    RST = 3,
}

struct PDU {
    message_type: MessageType,
}

impl PDU {
    pub fn new(message_type: MessageType) -> Option<PDU> {
        let pdu_ptr = unsafe { ffi::coap_pdu_init(message_type as u8, 0, 0, 0) };
        if pdu_ptr.is_null() {
            None
        } else {
            Some(PDU { message_type })
        }
    }
}
