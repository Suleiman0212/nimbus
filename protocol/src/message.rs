use bincode::{Decode, Encode};

use crate::error::MessageError;

#[derive(Encode, Decode, Debug)]
pub enum Message {
    LoginRequest {
        login: String,
        password: String,
    },
    LoginAnswer {
        login_successfully: bool,
    },
    LoginSessionRequest,
    LoginSessionAnswer {
        session_exists: bool,
    },
    FileMetaRequest {
        file_path: String,
    },
    FileMetaAnswer {
        file_size: Result<u64, String>,
    },
    FileDownloadRequest {
        file_path: String,
    },
    FileDownloadAnswer {
        file_data: Result<Vec<u8>, String>,
    },
    FileUploadRequest {
        file_size: u64,
    },
    FileUploadAction {
        file_path: Result<String, String>,
        file_data: Option<Vec<u8>>,
    },
    FileDeleteRequest {
        file_path: String,
    },
    FileDeleteAnswer {
        file_name: Result<String, String>,
    },
}

impl Message {
    pub fn to_bytes(&self) -> Result<Vec<u8>, MessageError> {
        Ok(bincode::encode_to_vec(self, bincode::config::standard())
            .map_err(|_| MessageError::EncodeError)?)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, MessageError> {
        let message: Message = bincode::decode_from_slice(&data, bincode::config::standard())
            .map_err(|_| MessageError::DecodeError)?
            .0;
        Ok(message)
    }
}
