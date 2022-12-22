use tokio_tungstenite::{
    tungstenite::protocol::{self, WebSocketConfig},
    WebSocketStream,
};
use futures_util::{future, ready, FutureExt, Sink, Stream, TryFutureExt};
use hyper::upgrade::OnUpgrade;
use std::fmt;
use std::borrow::Cow;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A WebSocket message.
///
/// This will likely become a `non-exhaustive` enum in the future, once that
/// language feature has stabilized.
#[derive(Eq, PartialEq, Clone)]
pub struct Message {
    inner: protocol::Message,
}

impl Message {
    /// Construct a new Text `Message`.
    pub fn text<S: Into<String>>(s: S) -> Message {
        Message {
            inner: protocol::Message::text(s),
        }
    }

    /// Construct a new Binary `Message`.
    pub fn binary<V: Into<Vec<u8>>>(v: V) -> Message {
        Message {
            inner: protocol::Message::binary(v),
        }
    }

    /// Construct a new Ping `Message`.
    pub fn ping<V: Into<Vec<u8>>>(v: V) -> Message {
        Message {
            inner: protocol::Message::Ping(v.into()),
        }
    }

    /// Construct a new Pong `Message`.
    ///
    /// Note that one rarely needs to manually construct a Pong message because the underlying tungstenite socket
    /// automatically responds to the Ping messages it receives. Manual construction might still be useful in some cases
    /// like in tests or to send unidirectional heartbeats.
    pub fn pong<V: Into<Vec<u8>>>(v: V) -> Message {
        Message {
            inner: protocol::Message::Pong(v.into()),
        }
    }

    /// Construct the default Close `Message`.
    pub fn close() -> Message {
        Message {
            inner: protocol::Message::Close(None),
        }
    }

    /// Construct a Close `Message` with a code and reason.
    pub fn close_with(code: impl Into<u16>, reason: impl Into<Cow<'static, str>>) -> Message {
        Message {
            inner: protocol::Message::Close(Some(protocol::frame::CloseFrame {
                code: protocol::frame::coding::CloseCode::from(code.into()),
                reason: reason.into(),
            })),
        }
    }

    /// Returns true if this message is a Text message.
    pub fn is_text(&self) -> bool {
        self.inner.is_text()
    }

    /// Returns true if this message is a Binary message.
    pub fn is_binary(&self) -> bool {
        self.inner.is_binary()
    }

    /// Returns true if this message a is a Close message.
    pub fn is_close(&self) -> bool {
        self.inner.is_close()
    }

    /// Returns true if this message is a Ping message.
    pub fn is_ping(&self) -> bool {
        self.inner.is_ping()
    }

    /// Returns true if this message is a Pong message.
    pub fn is_pong(&self) -> bool {
        self.inner.is_pong()
    }

    /// Try to get the close frame (close code and reason)
    pub fn close_frame(&self) -> Option<(u16, &str)> {
        if let protocol::Message::Close(Some(ref close_frame)) = self.inner {
            Some((close_frame.code.into(), close_frame.reason.as_ref()))
        } else {
            None
        }
    }

    /// Try to get a reference to the string text, if this is a Text message.
    pub fn to_str(&self) -> Result<&str, ()> {
        match self.inner {
            protocol::Message::Text(ref s) => Ok(s),
            _ => Err(()),
        }
    }

    /// Return the bytes of this message, if the message can contain data.
    pub fn as_bytes(&self) -> &[u8] {
        match self.inner {
            protocol::Message::Text(ref s) => s.as_bytes(),
            protocol::Message::Binary(ref v) => v,
            protocol::Message::Ping(ref v) => v,
            protocol::Message::Pong(ref v) => v,
            protocol::Message::Close(_) => &[],
            protocol::Message::Frame(ref frame) => frame.payload(),
        }
    }

    /// Destructure this message into binary data.
    pub fn into_bytes(self) -> Vec<u8> {
        self.inner.into_data()
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl From<Message> for Vec<u8> {
    fn from(m: Message) -> Self {
        m.into_bytes()
    }
}
/// Creates a Websocket Filter.
///
/// The yielded `Ws` is used to finish the websocket upgrade.
///
/// # Note
///
/// This filter combines multiple filters internally, so you don't need them:
///
/// - Method must be `GET`
/// - Header `connection` must be `upgrade`
/// - Header `upgrade` must be `websocket`
/// - Header `sec-websocket-version` must be `13`
/// - Header `sec-websocket-key` must be set.
///
/// If the filters are met, yields a `Ws`. Calling `Ws::on_upgrade` will
/// return a reply with:
///
/// - Status of `101 Switching Protocols`
/// - Header `connection: upgrade`
/// - Header `upgrade: websocket`
/// - Header `sec-websocket-accept` with the hash value of the received key.
pub fn ws() {
    
}
/// WebSocket
pub struct WebSocket {
    inner: WebSocketStream<hyper::upgrade::Upgraded>,
}

impl WebSocket {
    pub(crate) async fn from_raw_socket(
        upgraded: hyper::upgrade::Upgraded,
        role: protocol::Role,
        config: Option<protocol::WebSocketConfig>,
    ) -> Self {
        WebSocketStream::from_raw_socket(upgraded, role, config)
            .map(|inner| WebSocket { inner })
            .await
    }
}
// ===== Rejections =====

/// Connection header did not include 'upgrade'
#[derive(Debug)]
pub struct MissingConnectionUpgrade;

impl fmt::Display for MissingConnectionUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection header did not include 'upgrade'")
    }
}

impl ::std::error::Error for MissingConnectionUpgrade {}
