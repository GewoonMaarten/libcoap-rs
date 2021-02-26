use std::mem;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct CoapContext {
    context: std::ptr::NonNull<ffi::coap_context_t>,
}

fn socket_addr(addr: &SocketAddr) -> ffi::coap_address_t {
    match addr {
        SocketAddr::V4(ref addr) => {
            let sockaddr = ffi::sockaddr_in {
                sin_family: libc::AF_INET as ffi::sa_family_t,
                sin_port: addr.port().to_be(),
                sin_addr: ffi::in_addr {
                    s_addr: u32::from_ne_bytes(addr.ip().octets()),
                },
                sin_zero: [0; 8],
            };
            ffi::coap_address_t {
                addr: ffi::coap_address_t__bindgen_ty_1 { sin: sockaddr },
                size: mem::size_of::<ffi::sockaddr_in>() as ffi::socklen_t,
            }
        }
        SocketAddr::V6(ref addr) => {
            let sockaddr = ffi::sockaddr_in6 {
                sin6_family: libc::AF_INET as ffi::sa_family_t,
                sin6_port: addr.port().to_be(),
                sin6_addr: ffi::in6_addr {
                    __in6_u: ffi::in6_addr__bindgen_ty_1 {
                        __u6_addr8: addr.ip().octets(),
                    },
                },
                sin6_flowinfo: addr.flowinfo(),
                sin6_scope_id: addr.scope_id(),
            };
            ffi::coap_address_t {
                addr: ffi::coap_address_t__bindgen_ty_1 { sin6: sockaddr },
                size: mem::size_of::<ffi::sockaddr_in6>() as ffi::socklen_t,
            }
        }
    }
}

impl CoapContext {
    pub fn new(server_address: &str) -> Self {
        let coap_addrs: Vec<_> = server_address
            .to_socket_addrs()
            .expect("Unable to resolve address")
            .map(|addr| socket_addr(&addr))
            .collect();

        let coap_addr = &coap_addrs[0];
        let context = unsafe { ffi::coap_new_context(coap_addr) };
        Self {
            context: std::ptr::NonNull::new(context).expect("Could not create context."),
        }
    }

    // pub fn set_app_data<T>(&self, mut data: T) {
    //     let data = &mut data as *mut _ as *mut libc::c_void;
    //     unsafe { ffi::coap_set_app_data(self.context.as_ptr(), data) };
    // }

    // pub fn get_app_data<T>(&self) -> T {
    //     let data = unsafe { ffi::coap_get_app_data(self.context.as_ptr()) };
    // }
}

impl Drop for CoapContext {
    fn drop(&mut self) {
        unsafe { ffi::coap_free_context(self.context.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_context() {
        CoapContext::new("localhost:6001");
    }

    #[test]
    #[should_panic(expected = "Unable to resolve address")]
    fn create_new_context_with_invalid_addr() {
        CoapContext::new("lochost:6001");
    }
}
