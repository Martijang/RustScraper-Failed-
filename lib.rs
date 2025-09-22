
use std::net::TcpStream;
use std::io::prelude::*;

pub struct Sock {
    pub buffer: [u8; 4096],
}
fn return_str(arg: &[u8]) -> String {
        return String::from_utf8_lossy(&arg).parse().expect("Cannot parse");
}

impl Sock {
    pub async fn connect_to_target(&mut self, addr: String, arg: i8) -> Result<(), std::io::Error> {
        // if addr.find("/").is_some() {
        //     let mut v = vec![];
        //     let fmt = addr.split("/");
        //     for chars in fmt {
        //         v.push(chars);
                
        //     }
        //     let fmtted = format!("{}:80/{}", v.pop().expect("Empty"), v.pop().expect("Empty"));
        //     println!("{}", fmtted);
        //     let mut stream = TcpStream::connect(fmtted)?;
        //     stream.write(b"GET / HTTP/1.1\r\n\r\nContent-Length: 0\r\n\r\n")?;
        //     stream.read(&mut self.buffer)?;
        // }
        let mut stream = TcpStream::connect(&addr).await?;
        stream.write(b"GET / HTTP/1.1\r\n\r\nContent-Length: 0\r\n\r\n")?;
        stream.read(&mut self.buffer)?;

        if arg == 0 {
            println!("{}", String::from_utf8_lossy(&self.buffer[0..20]));
        }else if arg == 1{
            println!("{}", String::from_utf8_lossy(&self.buffer));
        }
        Ok(())
    }

    pub fn analize_components(&mut self) {
        let buff: String = return_str(&self.buffer);
        let mut components: Vec<String> = Vec::new();

        for item in buff.trim().split(' ') {
            components.push(item.to_owned());
        }

        let mut div = 0;
        let mut span = 0;
        let mut a = 0;
        let mut p = 0;
        let mut img = 0;
        let mut h1 = 0;
        let mut h2 = 0;
        let mut h3 = 0;
        let mut h4 = 0;
        let mut h5 = 0;
        let mut src = 0;

        while let Some(parts) = components.pop() {
            match parts.as_str() {
               part if part.contains("<div") => {
                   div += 1;
                }
                part if part.contains("<span") => {
                    span += 1;
                }
                part if part.contains("<a") => {
                    a += 1;
                }
                part if part.contains("<p") => {
                    p += 1;
                }
                part if part.contains("<img") => {
                    img += 1;
                }
                part if part.contains("<h1")|| parts.contains("<H1") => {
                    h1 += 1;
                }
                part if part.contains("<h2") || parts.contains("<H2") => {
                    h2 += 1;
                }
                part if part.contains("<h3") || parts.contains("H3") => {
                    h3 += 1;
                }
                part if part.contains("<h4") || parts.contains("H4") => {
                    h4 += 1;
                }
                part if part.contains("<h5") || parts.contains("H5") => {
                    h5 += 1;
                }
                part if part.contains("<src") => {
                   src += 1;
                }
                _ => {}
            }
        }

    println!(
        " 
        h1:  {}
        h2:  {}
        h3:  {}
        h4:  {}
        h5:  {}
        src: {}
        img: {}
        a:   {}
        p:   {}
        div: {}
        span: {}
        ",
        h1, h2, h3, h4, h5, src, img, a, p, div, span
        );
    }
}