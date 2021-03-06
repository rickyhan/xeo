use std::net::SocketAddr;

type SrcAddr = SocketAddr;
type DstAddr = SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkRequest {
    GetPubKey { src: SrcAddr },
    PubKeyResponse { src: SrcAddr, key: Vec<u8> },
}

impl NetworkRequest {
    pub fn get_pub_key(src: SrcAddr) -> NetworkRequest {
        NetworkRequest::GetPubKey {
            src
        }
    }

    pub fn ty(&self) -> &'static str {
        use self::NetworkRequest::*;
        match self {
            GetPubKey { .. } => "GetPubKey",
            PubKeyResponse { .. } => "PubKeyResponse",
        }
    }
}
