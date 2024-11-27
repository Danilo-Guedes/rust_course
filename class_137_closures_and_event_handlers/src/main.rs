#[derive(Debug)]
enum ButtonData {
    Count(i32),
    Message(String),
}

struct Button<F>
where
    F: Fn(&mut ButtonData),
{
    on_click: F,
    data: ButtonData,
}

impl<F> Button<F>
where
    F: Fn(&mut ButtonData),
{
    fn new(on_click: F, data: ButtonData) -> Self {
        Button { on_click, data }
    }

    fn click(&mut self) {
        (self.on_click)(&mut self.data);
    }

    fn set_message(&mut self, message: &str) {
        if let ButtonData::Message(ref mut msg) = self.data {
            *msg = message.to_string();
        }
    }
}

fn main() {
    let mut subscribe_btn = Button::new(
        |btn_data| {
            if let ButtonData::Count(sub_count) = btn_data {
                *sub_count += 1;
                println!("Subscribed!! Total subscription {}", sub_count);
            }
        },
        ButtonData::Count(0),
    );

    subscribe_btn.click();
    subscribe_btn.click();
    subscribe_btn.click();

    let mut send_btn = Button::new(
        |btn_data| {
            if let ButtonData::Message(msg) = btn_data {
                println!("Your message sent: {}", msg);
            }
        },
        ButtonData::Message(String::new()),
    );

    send_btn.set_message("Hi, Please call me!!");
    send_btn.click();
}
