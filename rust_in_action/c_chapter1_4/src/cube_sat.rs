#[derive(Debug)]
pub struct Message {
    pub to: u64,
    pub content: String,
}

#[derive(Debug)]
pub struct Mailbox {
    pub messages: Vec<Message>,
}



impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                // 수정을 일으키는 강력한 안티패턴이지만, 다음줄에 Some(msg)로 반환시키므로 컴파일러가 용인해준다.
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct CubeSat {
    pub id: u64,
    pub mailbox: Mailbox,
}

impl CubeSat {
    pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}


#[derive(Debug)]
pub struct GroundStation {
    pub radio_freq: f64
}

impl GroundStation {
    // &self가 있으면 인스턴스에서도 사용할 수 있다.
    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] } }
    }
    pub fn send(
        &self,
        mailbox: &mut Mailbox,
        msg: Message,
    ) {
        mailbox.post(msg)
    }
}


impl Clone for CubeSat {
    fn clone(&self) -> Self {
        todo!()
    }
}


// 내장 구현
#[derive(Copy,Debug)]
enum StatusMessage {
    Ok,
}

// Clone 직접 구현
impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}