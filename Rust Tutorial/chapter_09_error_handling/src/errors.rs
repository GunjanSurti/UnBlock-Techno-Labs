use std::fs::File;
use std::io::ErrorKind;

pub fn err() {
    let result = File::open("hello.txt");
    let rs = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opeaning the file: {:?}", other_error),
        },
    };
    // File::open returns io::Error type of strict inside Err()
    // io::Error has struct method called "kind"
    // This struct has a method kind that we can call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet. So we match on greeting_file_result, but we also have an inner match on error.kind().

    // If value returned by "error.kind()" is the "NotFound" variant of the ErrorKind enum
    // then we try to create the file with "File::create", it could also fail so we wrote a
    // panic!() code
}

/****************************Alternatives to Using match with Result<T, E>***********************/
pub fn err_match() {
    // using closures more in chapter 10
    let result = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound  {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creatint hes file: {:?}",error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

/*
std::io::error
 // size = 1, align = 0x1
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    FilesystemLoop,
    StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    FilesystemQuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
    Uncategorized,
}
A list specifying general categories of I/O error.

This list is intended to grow over time and it is not recommended to exhaustively match against it.

It is used with the [io::Error] type.

Handling errors and matching on ErrorKind
In application code, use match for the ErrorKind values you are expecting; use _ to match "all other errors".

In comprehensive and thorough tests that want to verify that a test doesn't return any known incorrect error kind, you may want to cut-and-paste the current full list of errors from here into your test code, and then match _ as the correct case. This seems counterintuitive, but it will make your tests more robust. In particular, if you want to verify that your code does produce an unrecognized error kind, the robust solution is to check for all the recognized error kinds and fail in those cases.

*/
