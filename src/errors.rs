#![allow(missing_docs)]
use rustc_serialize::json;
use std::io;

error_chain! {

    foreign_links {
        json::BuilderError, Json;
    }

    errors {
        StreamWriteError(err: io::Error, msg: String) {
            description("error writing to a stream")
            display("{}: {}", err, msg)
        }

        FileReadError(err: io::Error, filename: String) {
            description("error reading from a file")
            display("{}: {}", err, filename)
        }

        UnexpectedDataType(data: String) {
            description("unexpected data type")
            display("{}", data)
        }

        UnexpectedNodeType(t: String) {
            description("unexpected node type")
            display("{}", t)
        }
    }
}
