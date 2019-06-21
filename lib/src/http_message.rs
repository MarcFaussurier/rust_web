pub struct Header {
    key: String,
    value: String
}

pub struct HttpMessage {
    request_line: String,
    headers: Vec<Header>,
    body: String
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
        request_line: String::from(""),
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
                output.request_line = current_key_buffer.clone();
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
    output = format!("{}{}\n", output, message.request_line);
    for x in message.headers.iter() {
        output = format!("{}{}:{}\n", output, x.key, x.value);
    }
    output = format!("{}{}", output, message.body);
    return output;
}
