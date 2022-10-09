#![no_std]

use postcard_bindgen::PostcardBindings;
use serde::Serialize;

#[derive(Serialize, PostcardBindings)]
pub struct A1Meta {
    name: &'static str,
    version: u16,
    payload: &'static [u8],
}

#[derive(Serialize, PostcardBindings)]
pub enum Packet {
    A1(A1Meta),
}

#[derive(Serialize, PostcardBindings)]
pub struct Protocol {
    packet: Packet,
}
