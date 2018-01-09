//! A simple, thread-safe, and async-friendly IRC client library.

pub mod conn;
pub mod data;
pub mod reactor;
pub mod server;
pub mod transport;

pub mod prelude {
    //! A client-side IRC prelude, re-exporting the complete high-level IRC client API.
    //!
    //! # Structure
    //! A connection to an IRC server is represented by an `IrcServer` which is configured using a
    //! `Config` struct that defines data such as which server to connect to, on what port, and
    //! using what nickname. The `Server` trait provides an API for actually interacting with the
    //! server once a connection has been established. This API intentionally offers only a single
    //! method to send `Commands` because it makes it easy to see the whole set of possible
    //! interactions with a server. The `ServerExt` trait addresses this deficiency by defining a
    //! number of methods that provide a more clear and succinct interface for sending various
    //! common IRC commands to the server.
    //!
    //! The various `proto` types capture details of the IRC protocol that are used throughout the
    //! client API. `Message`, `Command`, and `Response` are used to send and receive messages along
    //! the connection, and are naturally at the heart of communication in the IRC protocol.
    //! `Capability` and `NegotiationVersion` are used to determine (with the server) what IRCv3
    //! functionality to enable for the connection. Certain parts of the API offer suggestions for
    //! extensions that will improve the user experience, and give examples of how to enable them
    //! using `Capability`. `Mode`, `ChannelMode`, and `UserMode` are used in a high-level API for
    //! dealing with IRC channel and user modes. They appear in methods for sending mode commands,
    //! as well as in the parsed form of received mode commands.

    pub use client::data::Config;
    pub use client::reactor::IrcReactor;
    pub use client::server::{EachIncomingExt, IrcServer, Server};
    pub use client::server::utils::ServerExt;
    pub use proto::{Capability, ChannelExt, Command, Message, NegotiationVersion, Response};
    pub use proto::{ChannelMode, Mode, UserMode};

    pub use futures::{Future, Stream};
}
