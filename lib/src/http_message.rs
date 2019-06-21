pub struct Header {
    pub key: String,
    pub value: String
}

pub struct HttpMessage {
    pub first_line: String,
    pub headers: Vec<Header>,
    pub body: String
}

pub struct Request {
    pub req_type: String,
    pub uri: String,
    pub protocol: String
}

pub fn parse_first_line(message: &HttpMessage) -> Request {
    let mut output: Request = Request {
        req_type: String::from(""),
        uri: String::from(""),
        protocol: String::from("")
    };
    let mut first_space = true;
    let mut buf = String::from("");
    let mut pos = 0;

    for x in message.first_line.chars() {
        if x == ' ' {
            if first_space {
                output.req_type = buf.clone();
                buf = String::from("");
            } else {
                output.uri = buf.clone();
                buf = String::from("");
            }
            first_space = false;
        } else {
            buf = format!("{}{}", buf, x);
            if pos == (message.first_line.len() - 1) {
                output.protocol = buf.clone();
                buf = String::from("");
            }
        }    
        pos = pos + 1;
    }
    return output;
}

pub fn read(text: String) -> HttpMessage {
    // states
    let mut is_first_line                       = true;
    let mut two_consecutive_eol                 = false;
    let mut waiting_for_key                     = true;
    // buffers
    let mut current_key_buffer                  = String::from("");
    let mut current_value_buffer                = String::from("");


    let mut output = HttpMessage {
        first_line: String::from(""),
        headers: vec![],
        body: String::from("")
    };

    for x in text.chars() {
        if x == '\n' {
            if two_consecutive_eol {
                output.body = current_key_buffer.clone();
                break;
            }
            if is_first_line {
                output.first_line = current_key_buffer.clone();
                current_key_buffer = String::from("");
            } else {
                if !waiting_for_key { 
                    output.headers.push(Header{
                        key: current_key_buffer.clone(),
                        value: current_value_buffer.clone()
                    });
                }
            }

            current_key_buffer  = String::from("");
            current_value_buffer = String::from("");

            is_first_line = false;
            waiting_for_key = true;
            two_consecutive_eol = true;
        } else {
            // key end
            if x == ':' && waiting_for_key {
                waiting_for_key = false;
            } 
            // waiting for value
            else if !waiting_for_key 
            {
                current_value_buffer = format!("{}{}", current_value_buffer, x);
            }
            // waiting for key
            else {
                current_key_buffer = format!("{}{}", current_key_buffer, x);
            }
            two_consecutive_eol = false;
        }
    }

    return output;
}

pub fn write(message: HttpMessage) -> String {
    let mut output = String::from("");
    output = format!("{}{}\n", output, message.first_line);
    for x in message.headers.iter() {
        output = format!("{}{}:{}\n", output, x.key, x.value);
    }
    output = format!("{}{}", output, message.body);
    return output;
}
