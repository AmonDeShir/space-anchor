mod channel;
mod version;

use std::time::{Instant, Duration};
use std::{net::UdpSocket, time::SystemTime};

use renet::{RenetClient, RenetConnectionConfig, ClientAuthentication};

use std::ffi::CString;
use std::os::raw::c_char;

const PROTOCOL_ID: u64 = version!();

static mut CLIENT: Option<RenetClient> = None;
static mut RESULT: Option<Result> = None;

#[derive(Clone, Debug)]
pub struct Result {
    pub status: bool,
    pub message: String,
    pub detail: String,
}

impl Result {
    fn new(status: bool, message: &str, detail: &str) -> Self {
        Result {
            status,
            message: message.to_string(),
            detail: detail.to_string(),
        }
    }
}

unsafe fn set_error_result(message: &str, detail: String) {
    RESULT = Some(Result { status: false, message: message.to_string(), detail });
}

unsafe fn set_success_result() {
    RESULT = Some(Result { status: true, message: "success".to_string(), detail: "".to_string() });
}


#[no_mangle]
pub unsafe extern "C" fn rnet_get_result_status() -> bool {
    let result = match RESULT.clone() {
        Some(res) => res,
        None => Result::new(false, "Result is empty", "RESULT = NONE")
    };

    return result.status;
}

#[no_mangle]
pub unsafe extern "C" fn rnet_get_result_message() -> *mut c_char {
    let result = match RESULT.clone() {
        Some(res) => res,
        None => Result::new(false, "Result is empty", "RESULT = NONE")
    };

    return CString::new(result.message).unwrap().into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn rnet_get_result_detail() -> *mut c_char {
    let result = match RESULT.clone() {
        Some(res) => res,
        None => Result::new(false, "Result is empty", "RESULT = NONE")
    };

    return CString::new(result.detail).unwrap().into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn rnet_init_client() {
    let server_addr = match "127.0.0.1:2137".parse() {
        Ok(addr) => addr,
        Err(err) =>  {
            set_error_result("Cannot parse server address", format!("{:?}", err));
            return
        }
    };
    
    let socket = match UdpSocket::bind("127.0.0.1:0") {
        Ok(addr) => addr,
        Err(err) => {
            set_error_result("Cannot bind UDP socket", format!("{:?}", err));
            return
        }
    };


    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(addr) => addr,
        Err(err) => {
            set_error_result("Cannot get duration since UNIX EPOCH", format!("{:?}", err));
            return
        }
    };

    let connection_config = RenetConnectionConfig::default();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };


    CLIENT = match RenetClient::new(current_time, socket, connection_config, authentication) {
        Ok(client) => Some(client),
        Err(err) => {
            set_error_result("Cannot create renet client", format!("{:?}", err));
            return
        }
    };

    set_success_result();
}

#[no_mangle]
pub extern "C" fn rnet_time_now() -> u64 {
    Instant::now().elapsed().as_millis() as u64
}

#[no_mangle]
pub unsafe extern "C" fn rnet_get_messages(duration: u64, channel: u8) -> *const *const u8 {
    let mut messages = Vec::new();

    let client = match &mut CLIENT {
        Some(client) => client,
        _ => {
            set_error_result("Client isn't initializatied! (Run rnet_init_client before!)", format!("Option(CLIENT) == None"));
            return std::ptr::null();
        }
    };
    
    if let Err(err) = client.update(Duration::from_millis(duration)) {
        set_error_result("Cannot pdate client", format!("{:?}", err));
        return std::ptr::null();
    }

    if client.is_connected() {
        let mut message = Vec::new();

        while let Some(data) = client.receive_message(channel) {
            if data == [0] {
                let result = message.into_boxed_slice();
                let ptr = result.as_ptr();

                messages.push(ptr);
                std::mem::forget(result);
                message = Vec::new();
            } else {
                message.extend_from_slice(&data);
            }
        }
        if !message.is_empty() {
            let result = message.into_boxed_slice();
            let ptr = result.as_ptr();

            messages.push(ptr);
            std::mem::forget(result);
        }
    }

    messages.push(std::ptr::null());

    let result = messages.into_boxed_slice();
    let ptr = result.as_ptr();
    std::mem::forget(result);

    set_success_result();

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn rnet_send_messages() {
    let client = match &mut CLIENT {
        Some(client) => client,
        _ => {
            set_error_result("Client isn't initializatied! (Run rnet_init_client before!)", format!("Option(CLIENT) == None"));
            return;
        }
    };

    if let Err(err) = client.send_packets() {
        set_error_result("Cannot send packets", format!("{:?}", err));
        return;
    }

    set_success_result();
}

#[no_mangle]
pub unsafe extern "C" fn rnet_send_message(channel_id: u8, msg_ptr: *const u8, msg_len: usize) {
    let client = match &mut CLIENT {
        Some(client) => client,
        _ => {
            set_error_result("Client isn't initializatied! (Run rnet_init_client before!)", format!("Option(CLIENT) == None"));
            return;
        }
    };

    let msg_slice = std::slice::from_raw_parts(msg_ptr, msg_len);
    let msg_str = std::str::from_utf8_unchecked(msg_slice);

    client.send_message(channel_id, msg_str.to_owned());
    set_success_result();
}

#[no_mangle]
pub extern "C" fn rnet_get_reliable_channel_id() -> u8 {
    channel::Channel::Reliable.id()
}

#[no_mangle]
pub extern "C" fn rnet_get_unreliable_channel_id() -> u8 {
    channel::Channel::Unreliable.id()
}

#[no_mangle]
pub extern "C" fn rnet_get_chunk_channel_id() -> u8 {
    channel::Channel::Chunk.id()
}