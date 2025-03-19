use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    InvalidRequestLine,
    InvalidDelay,
    MissingPayload,
    InvalidBase64,
    IoError(std::io::Error),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::InvalidRequestLine => write!(f, "Error: Invalid Request Line"),
            RequestError::InvalidDelay => write!(f, "Error: 'delay' must be a positive integer"),
            RequestError::MissingPayload => {
                write!(f, "Error: 'payload' must be a non-empty string")
            }
            RequestError::InvalidBase64 => write!(f, "Error: Invalid base64 payload"),
            RequestError::IoError(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> Self {
        RequestError::IoError(err)
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_request_error_display() {
        let error = RequestError::InvalidRequestLine;
        assert_eq!(format!("{}", error), "Error: Invalid Request Line");
    }

    #[test]
    fn test_request_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "io failure");
        let request_error: RequestError = io_error.into();
        assert!(matches!(request_error, RequestError::IoError(_)));
    }
}
