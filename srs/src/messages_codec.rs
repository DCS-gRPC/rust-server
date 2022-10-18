use crate::message::Message;
use bytes::BytesMut;
use std::{error, fmt, io};
use tokio_util::codec::{Decoder, Encoder, LinesCodec, LinesCodecError};

pub struct MessagesCodec {
    lines_codec: LinesCodec,
}

impl MessagesCodec {
    pub fn new() -> Self {
        MessagesCodec {
            lines_codec: LinesCodec::new(),
        }
    }
}

impl Decoder for MessagesCodec {
    type Item = Message;
    type Error = MessagesCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(line) = self.lines_codec.decode(buf)? {
            match serde_json::from_str(&line) {
                Ok(msg) => Ok(Some(msg)),
                Err(err) => Err(MessagesCodecError::JsonDecode(err, line)),
            }
        } else {
            Ok(None)
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(line) = self.lines_codec.decode_eof(buf)? {
            match serde_json::from_str(&line) {
                Ok(msg) => Ok(Some(msg)),
                Err(err) => Err(MessagesCodecError::JsonDecode(err, line)),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Message> for MessagesCodec {
    type Error = MessagesCodecError;

    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_string(&msg).map_err(MessagesCodecError::JsonEncode)?;
        self.lines_codec.encode(json, buf)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum MessagesCodecError {
    JsonDecode(serde_json::Error, String),
    JsonEncode(serde_json::Error),
    LinesCodec(LinesCodecError),
    Io(io::Error),
}

impl fmt::Display for MessagesCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagesCodecError::JsonDecode(_, json) => write!(f, "failed to decode JSON: {}", json),
            MessagesCodecError::JsonEncode(_) => write!(f, "failed to encode JSON"),
            MessagesCodecError::LinesCodec(err) => err.fmt(f),
            MessagesCodecError::Io(err) => err.fmt(f),
        }
    }
}

impl error::Error for MessagesCodecError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MessagesCodecError::JsonDecode(ref err, _) => Some(err),
            MessagesCodecError::JsonEncode(ref err) => Some(err),
            MessagesCodecError::LinesCodec(ref err) => Some(err),
            MessagesCodecError::Io(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for MessagesCodecError {
    fn from(err: io::Error) -> Self {
        MessagesCodecError::Io(err)
    }
}

impl From<LinesCodecError> for MessagesCodecError {
    fn from(err: LinesCodecError) -> Self {
        MessagesCodecError::LinesCodec(err)
    }
}
