use crate::controller::Controller;
use crate::http_message::HttpMessage;

pub struct ControllerStack {
    pub controllers: Vec<Controller>
}

impl ControllerStack {
    pub fn handle(&self, input: HttpMessage) -> HttpMessage {
        let output = HttpMessage {
            first_line: String::from("HTTP/1.1 200 OK"),
            headers: vec![],
            body: String::from("")
        };

        for x in self.controllers.iter() {

        }

        return output;
    }
}
