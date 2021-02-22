#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::convert::TryInto;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_example_coap_message() {
        // This is just a sanity test
        unsafe {
            coap_startup();

            let mut ainfo: *mut libc::addrinfo;
            let node = std::ffi::CString::new("coap.me").unwrap();
            let service = std::ffi::CString::new("5683").unwrap();
            let hints = libc::addrinfo {
                ai_flags: 0,
                ai_family: libc::AF_UNSPEC,
                ai_socktype: libc::SOCK_DGRAM,
                ai_protocol: 0,
                ai_addrlen: 0,
                ai_addr: std::ptr::null_mut(),
                ai_canonname: std::ptr::null_mut(),
                ai_next: std::ptr::null_mut(),
            };
            let mut res: *mut libc::addrinfo = std::ptr::null_mut();
            let error = libc::getaddrinfo(node.as_ptr(), service.as_ptr(), &hints, &mut res);
            if error != 0 {
                panic!("getaddrinfo: {:?}\n", libc::gai_strerror(error));
            }

            let mut dst: Option<coap_address_t> = None;
            ainfo = res;
            while ainfo != std::ptr::null_mut() {
                let de_ainfo = *ainfo;
                let addr = *de_ainfo.ai_addr;

                match de_ainfo.ai_family {
                    libc::AF_INET => {
                        let sock: sockaddr_in = std::mem::transmute(addr);
                        dst = Some(coap_address_t {
                            size: (*ainfo).ai_addrlen,
                            addr: coap_address_t__bindgen_ty_1 { sin: sock },
                        });
                    }
                    libc::AF_INET6 => {}
                    _ => {}
                };

                ainfo = (*ainfo).ai_next;
            }

            libc::freeaddrinfo(res);

            let listen_addr: *const coap_address_t = std::ptr::null_mut();
            let ctx = coap_new_context(listen_addr);
            assert!(!ctx.is_null());

            let local_if: *const coap_address_t = std::ptr::null_mut();
            let ptr_dst: *const coap_address_t = &(dst.unwrap());
            let session = coap_new_client_session(ctx, local_if, ptr_dst, COAP_PROTO_UDP as u8);
            assert!(!session.is_null());

            let pdu = coap_pdu_init(
                COAP_MESSAGE_CON as u8,
                coap_request_t_COAP_REQUEST_GET as u8,
                0,
                coap_session_max_pdu_size(session),
            );
            assert!(!pdu.is_null());

            let message = std::ffi::CString::new("hello").unwrap();
            coap_add_option(
                pdu,
                COAP_OPTION_URI_PATH as u16,
                5,
                message.as_ptr() as *const u8,
            );

            coap_send(session, pdu);

            coap_session_release(session);
            coap_free_context(ctx);
            coap_cleanup();
        }
    }
}
