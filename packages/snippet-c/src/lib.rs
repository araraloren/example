use bindings::{component::snippet_c::send, wasi::cli::stdin::get_stdin, Guest};

pub mod bindings {
    wit_bindgen::generate!({
        world: "example",
        path: "wit",
        with: {
            "wasi:cli/stdin@0.2.0": generate,
            "wasi:io/error@0.2.0": generate,
            "wasi:io/poll@0.2.0": generate,
            "wasi:io/streams@0.2.0": generate,
        },
    });
}

pub struct Component;

impl Component {
    pub fn process_line(str: &str) -> String {
        format!("processed by wasi:cli: {}", str.to_ascii_lowercase())
    }

    pub fn send2host(line: &str) -> Result<(), String> {
        send::send(&Self::process_line(line)).map_err(|e| format!("can not send to host: {e:?}"))
    }
}

impl Guest for Component {
    fn echo() -> Result<(), String> {
        let stdin = get_stdin();
        let mut buf = String::default();

        loop {
            match stdin.read(1024) {
                Ok(bytes) => {
                    if !bytes.is_empty() {
                        buf += &String::from_utf8_lossy(&bytes);
                        if let Some((line, left)) = buf.split_once("\n\r") {
                            Self::send2host(line)?;
                            buf = left.to_string();
                        } else if let Some((line, left)) = buf.split_once("\n") {
                            Self::send2host(line)?;
                            buf = left.to_string();
                        }
                    }
                }
                Err(e) => {
                    if !buf.is_empty() {
                        Self::send2host(&buf)?;
                    }
                    return Err(format!("can not read from stdin: {e:?}"));
                }
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
