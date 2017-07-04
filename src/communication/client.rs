use nanomsg;



pub struct Client {
    socket: nanomsg::Socket
}

impl Client {
    pub fn new(addr: &str) -> Result<Client,nanomsg::result::Error> {
        let mut socket = nanomsg::Socket::new(nanomsg::Protocol::Pair)?;
        socket.connect(addr)?;
        Ok(Client {
            socket
        })
    }

}
