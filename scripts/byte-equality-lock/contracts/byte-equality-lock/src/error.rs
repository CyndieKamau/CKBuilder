use ckb_std::error::SysError;

#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    WrongSecret,
    WaitFailure,
    InvalidFd,
    OtherEndClosed,
    MaxVmsSpawned,
    MaxFdsCreated,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        match err {
            SysError::IndexOutOfBound => Self::IndexOutOfBound,
            SysError::ItemMissing => Self::ItemMissing,
            SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            SysError::Encoding => Self::Encoding,
            SysError::WaitFailure => Self::WaitFailure,
            SysError::InvalidFd => Self::InvalidFd,
            SysError::OtherEndClosed => Self::OtherEndClosed,
            SysError::MaxVmsSpawned => Self::MaxVmsSpawned,
            SysError::MaxFdsCreated => Self::MaxFdsCreated,
            SysError::Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}
