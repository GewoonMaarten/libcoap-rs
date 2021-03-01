use std::mem;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct CoapContext<'a> {
    context: std::ptr::NonNull<ffi::coap_context_t>,
    _phantom: std::marker::PhantomData<&'a std::ptr::NonNull<ffi::coap_context_t>>,
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

impl<'a> CoapContext<'a> {
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
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set_app_data<T>(&self, data: &'a mut T) {
        let data = data as *mut _ as *mut libc::c_void;
        unsafe { ffi::coap_set_app_data(self.context.as_ptr(), data) };
    }

    pub fn get_app_data<T>(&self) -> Option<&T> {
        let data = unsafe { ffi::coap_get_app_data(self.context.as_ptr()) };
        if data.is_null() {
            None
        } else {
            let data = data as *mut T;
            //TODO: Fix safety comment, when data lifetime has been fixed.
            // SAFETY: coap_context_t can only store a pointer to our data (which
            // we know is correct and we check if the pointer still point to a
            // valid memory space). And we can turn in into a safe reference.
            let data = unsafe { &*data };
            Some(data)
        }
    }
}

impl<'a> Drop for CoapContext<'a> {
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

    #[test]
    fn store_and_retrieve() {
        let context = CoapContext::new("localhost:6001");

        context.set_app_data(&mut 42);
        let result: i32 = *context.get_app_data().unwrap();
        assert_eq!(result, 42);

        context.set_app_data(&mut "test");
        let result: &str = *context.get_app_data().unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    //TODO: This test should not compile
    fn store_drop_and_retrieve() {
        let context = CoapContext::new("localhost:6001");
        let mut data = String::from("test");
        context.set_app_data(&mut data);
        drop(data); // Data is dropped here
        let x = context // But used here
            .get_app_data::<String>()
            .expect("Value has been dropped");
        assert_eq!(x, "test")
    }
}
