mod data_structures;
mod filter;
mod io;
mod parsers;
mod process;

pub use crate::data_structures::address::Address;
pub use crate::data_structures::auth_result::AuthResult;
pub use crate::data_structures::event::Event;
pub use crate::data_structures::filter_kind::FilterKind;
pub use crate::data_structures::filter_phase::FilterPhase;
pub use crate::data_structures::filter_response::FilterResponse;
pub use crate::data_structures::mail_result::MailResult;
pub use crate::data_structures::method::Method;
pub use crate::data_structures::smtp_status::SmtpStatusCode;
pub use crate::data_structures::subsystem::SubSystem;
pub use crate::data_structures::timeval::TimeVal;
pub use crate::filter::Filter;
pub use crate::parsers::entry::{FilterEntry, ReportEntry};

use crate::parsers::handshake::parse_handshake;
use std::sync::mpsc::channel;
use std::thread;

const BUFFER_SIZE: usize = 4096;

#[macro_export]
macro_rules! register {
    ($name: ident) => {
        fn $name(&self) -> bool {
            return true;
        }
    };
}

macro_rules! recv {
    ($rx: ident) => {
        match $rx.recv() {
            Ok(b) => b,
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        }
    };
}

pub fn run_filter<T>(user_object: &mut T)
where
    T: Filter,
{
    // IO init
    let (tx, rx) = channel::<Vec<u8>>();
    thread::spawn(move || {
        io::read_stdin(&tx);
    });

    // Handshake
    let mut handshake_buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
    let handshake = loop {
        let buffer = recv!(rx);
        handshake_buffer.extend_from_slice(&buffer);
        if let Ok((_, handshake)) = parse_handshake(&handshake_buffer) {
            break handshake;
        }
    };
    handshake_reply(user_object, handshake.subsystem);

    // Read and process input
    loop {
        let buffer = recv!(rx);
        if let Err(msg) = process::line(user_object, &buffer) {
            log::error!("{}", msg);
        }
    }
}

macro_rules! handshake_register {
    ($obj: ident, $func: ident, $subsystem: expr, $type: expr, $name: expr) => {
        if $obj.$func() {
            println!("register|{}|{}|{}", $type, $subsystem.to_string(), $name);
        }
    };
}

fn handshake_reply<T>(obj: &mut T, ss: SubSystem)
where
    T: Filter,
{
    handshake_register!(obj, has_report_link_connect, ss, "report", "link-connect");
    println!("register|ready");
}