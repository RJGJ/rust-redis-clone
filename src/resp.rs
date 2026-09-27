use std::io::{self, BufRead, Read};

#[derive(Debug)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>),
    Null,
}

fn invalid_data(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message);
}

fn read_line<R: BufRead>(reader: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();

    let bytes_read = reader.read_until(b'\n', &mut line)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    if !line.ends_with(b"\r\n") {
        return Err(invalid_data("RESP line must end with CRLF"));
    }

    line.truncate(line.len() - 2);

    Ok(Some(line))
}

fn parse_integer(bytes: &[u8]) -> io::Result<i64> {
    let text = std::str::from_utf8(bytes) //
        .map_err(|_| invalid_data("Invalid utf8 integer"))?;
}

// TODO
