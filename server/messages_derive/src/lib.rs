use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use proc_macro::TokenStream;

/// Derive macro generating an impl of the trait ReceivableMessage.
#[proc_macro_derive(ReceivableMessage)]
pub fn derive_receivable_message(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl ReceivableMessage for #ident {}

        impl Default for #ident {
            fn default() -> Self {
                Self::NONE
            }
        }
    };
    output.into()
}

/// Derive macro generating an impl of the trait SendableMessage, and config it to send messages using the renet's reliable channel.
///
/// The reliable channel is designed for sending small messages when message ordering is crucial and when messages must be delivered.
#[proc_macro_derive(SendableMessage)]
pub fn derive_reliable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SendableMessage for #ident {
            fn get_channel_id() -> u8 {
                Channel::Reliable.id()
            }   
        }
    };
    output.into()
}

/// Derive macro generating an impl of the trait SendableMessage, and config it to send messages using the renet's chunk channel.
///
/// The chunk channel is designed for sending big messages, like map data,
/// that aren't frequent. It can be used when the message is large and must be delivered. 
/// 
/// The message is sliced into multiple chunks so that it can be sent in multiple frames instead of sending all of it in one packet.
#[proc_macro_derive(SendableChunkMessage)]
pub fn derive_chunk(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SendableMessage for #ident {
            fn get_channel_id() -> u8 {
                Channel::Chunk.id()
            }   
        }
    };
    output.into()
}

/// Derive macro generating an impl of the trait SendableMessage, and config it to send messages using the renet's unreliable channel.
/// 
/// The unreliable channel is designed for sending small pieces of data frequently.
/// This channel is unreliable, so there is a small chance that messages will not be received at all. 
/// There are no guarantees about the order in which clients will receive messages.
#[proc_macro_derive(SendableFrequentMessage)]
pub fn derive_unreliable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SendableMessage for #ident {
            fn get_channel_id() -> u8 {
                Channel::Unreliable.id()
            }   
        }
    };
    output.into()
}

