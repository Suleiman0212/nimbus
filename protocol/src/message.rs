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
        file_path: String,
        file_data: Vec<u8>,
    },
    FileUploadAnswer {
        uploaded: bool,
    },
    FileDeleteRequest {
        file_path: String,
    },
    FileDeleteAnswer {
        deleted: Result<bool, String>,
    },
    FileListRequest {
        path: String,
    },
    FileListAnswer {
        files: Vec<String>,
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
