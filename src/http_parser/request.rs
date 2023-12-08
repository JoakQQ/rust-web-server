use std::{collections::HashMap, io::Read, net::TcpStream, str};

static mut MAX_HEADER_SIZE: usize = 1024;

pub fn parse_request(
    mut stream: &TcpStream,
) -> Result<(HashMap<String, String>, Vec<u8>), &'static str> {
    const HEADERS_END_WINDOW: &[u8; 4] = &[13, 10, 13, 10];
    const WINDOW_SIZE: usize = 1024;

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut headers_ended: bool = false;
    let mut halt: bool = false;
    let mut reached_max_headers_size = false;

    let mut body_buf: Vec<u8> = Vec::new();
    let mut headers_buf: Vec<u8> = Vec::new();

    while !halt {
        let mut buf = [0u8; WINDOW_SIZE];
        if let Err(err) = stream.read(&mut buf) {
            eprintln!("failed to read buffer from http request: {:?}", err);
            return Err("failed to read buffer from http request");
        }
        if buf[WINDOW_SIZE - 1] == 0 {
            halt = true;
        } else if reached_max_headers_size {
            continue;
        }

        if headers_ended {
            body_buf.extend(buf.into_iter().take_while(|&b| b != 0).collect::<Vec<u8>>());
        } else {
            let mut temp_buf: Vec<u8> = Vec::new();
            let mut skip: usize = 0;
            if !headers_buf.is_empty() {
                temp_buf.extend_from_slice(&headers_buf[headers_buf.len() - 3..]);
                skip = 3;
            }
            temp_buf.extend_from_slice(&buf);

            if let Some(index) = temp_buf
                .windows(HEADERS_END_WINDOW.len())
                .position(|window| window == HEADERS_END_WINDOW)
            {
                headers_ended = true;

                headers_buf.extend(
                    temp_buf
                        .iter()
                        .take(index)
                        .skip(skip)
                        .cloned()
                        .collect::<Vec<u8>>(),
                );
                body_buf.extend(
                    temp_buf
                        .into_iter()
                        .skip(index + 4)
                        .take_while(|&b| b != 0u8)
                        .collect::<Vec<u8>>(),
                );
            } else {
                headers_buf.extend(temp_buf.into_iter().skip(skip).collect::<Vec<u8>>());
            }

            if headers_buf.len() > unsafe { MAX_HEADER_SIZE } {
                reached_max_headers_size = true;
            }
        }
    }

    if reached_max_headers_size {
        return Err("max headers size reached");
    }

    let mut is_first_line = true;
    match str::from_utf8(&headers_buf) {
        Ok(lines) => {
            for line in lines.split("\r\n") {
                if is_first_line {
                    let mut failed = false;
                    let mut line_iter: str::Split<'_, &str> = line.split(" ");
                    headers.insert(
                        "Method".to_string(),
                        line_iter
                            .next()
                            .unwrap_or_else(|| {
                                failed = true;
                                ""
                            })
                            .to_string(),
                    );
                    headers.insert(
                        "Path".to_string(),
                        line_iter
                            .next()
                            .unwrap_or_else(|| {
                                failed = true;
                                ""
                            })
                            .to_string(),
                    );
                    headers.insert(
                        "Protocol".to_string(),
                        line_iter
                            .next()
                            .unwrap_or_else(|| {
                                failed = true;
                                ""
                            })
                            .to_string(),
                    );
                    if failed {
                        return Err("failed to convert header first line");
                    }
                    is_first_line = false;
                } else {
                    if line.contains(": ") {
                        let mut failed = false;
                        let mut line_iter: str::Split<'_, &str> = line.split(":");
                        headers.insert(
                            line_iter
                                .next()
                                .unwrap_or_else(|| {
                                    failed = true;
                                    ""
                                })
                                .to_string(),
                            line_iter
                                .next()
                                .unwrap_or_else(|| {
                                    failed = true;
                                    ""
                                })
                                .trim()
                                .to_string(),
                        );
                        if failed {
                            return Err("failed to convert header");
                        }
                    } else {
                        return Err("malformed header, without ':'");
                    }
                }
            }
        }
        Err(_) => return Err("failed to convert header buffer to string"),
    }
    Ok((headers, body_buf))
}
