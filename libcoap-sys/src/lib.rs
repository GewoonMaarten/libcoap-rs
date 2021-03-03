/*!
 * Copyright 2021 Maarten de Klerk
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    fn resolve_address(host: &str, port: &str) -> Option<coap_address_t> {
        let mut ainfo: *mut libc::addrinfo;
        let node = std::ffi::CString::new(host).unwrap();
        let service = std::ffi::CString::new(port).unwrap();
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
        let error = unsafe { libc::getaddrinfo(node.as_ptr(), service.as_ptr(), &hints, &mut res) };
        if error != 0 {
            panic!("getaddrinfo: {:?}\n", unsafe { libc::gai_strerror(error) });
        }

        let mut dst: Option<coap_address_t> = None;
        ainfo = res;
        while ainfo != std::ptr::null_mut() {
            let de_ainfo = unsafe { *ainfo };
            let addr = unsafe { *de_ainfo.ai_addr };
            let addr_len = unsafe { (*ainfo).ai_addrlen };

            match de_ainfo.ai_family {
                libc::AF_INET => {
                    let sock: sockaddr_in = unsafe { std::mem::transmute(addr) };
                    dst = Some(coap_address_t {
                        size: addr_len,
                        addr: coap_address_t__bindgen_ty_1 { sin: sock },
                    });
                }
                libc::AF_INET6 => {}
                _ => {}
            };

            ainfo = unsafe { (*ainfo).ai_next };
        }

        unsafe { libc::freeaddrinfo(res) };

        dst
    }

    #[test]
    fn check_metadata() {
        assert_eq!(LIBCOAP_PACKAGE_NAME, b"libcoap\0");
        assert_eq!(LIBCOAP_PACKAGE_VERSION, b"4.3.0\0");
    }

    #[test]
    fn send_example_coap_message() {
        // This is just a sanity test
        // Copy of https://github.com/obgm/libcoap-minimal/blob/master/client.cc
        unsafe {
            coap_startup();

            let dst = resolve_address("coap.me", "5683");

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

    #[test]
    fn start_server() {
        // This is just a sanity test
        // Copy of https://github.com/obgm/libcoap-minimal/blob/master/client.cc
        unsafe {
            let dst = resolve_address("localhost", "5683").unwrap();
            let ctx = coap_new_context(std::ptr::null());
            assert!(!ctx.is_null());

            let proto = (COAP_PROTO_UDP as u32).try_into().unwrap();
            let endpoint = coap_new_endpoint(ctx, &dst, proto);
            assert!(!endpoint.is_null());

            coap_free_context(ctx);
            coap_cleanup();
        }
    }
}
