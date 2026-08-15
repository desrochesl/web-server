pub mod requests {
    use std::fmt::Display;

    pub struct HTTPRequest {
        pub req_type: String,
        pub msg: HTML,
    }

    pub struct HTML {
        pub tes: String,
    }

    impl Display for HTML {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<div>{}</div>", self.tes)
        }
    }

    impl From<&HTML> for String {
        fn from(value: &HTML) -> Self {
            value.tes.to_string()
        }
    }

    impl Display for HTTPRequest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg_len: usize = format!("{}", self.msg).len();
            write!(
                f,
                "HTTP/1.1 {} OK\nContent-Length: {}\nContent-Type: text/html\n\n{}",
                self.req_type,
                msg_len,
                self.msg.to_string(),
            )
        }
    }
}
