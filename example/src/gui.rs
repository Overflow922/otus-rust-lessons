use client::SmartSocketClient;
use iced::{
    widget::{button, column, text},
    Alignment, Application, Command, Element, Settings, Theme,
};
use network::utils::ConnectResult;
pub struct Counter {
    stats: String,
    tooltip: String,
}

pub fn run() {
    let _ = Counter::run(Settings::default());
}

#[derive(Debug, Clone)]
pub enum Message {
    TurnOnPressed,
    TurnOffPressed,
    GetStatisticsPressed,
    ReceivedStatistics(String),
    TooltipUpdate(String),
}

impl Counter {
    async fn connect() -> ConnectResult<SmartSocketClient> {
        SmartSocketClient::new("127.0.0.1:33445").await
    }
}
impl Application for Counter {
    type Message = Message;
    type Executor = iced_futures::backend::native::tokio::Executor;
    type Theme = Theme;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self {
                stats: String::new(),
                tooltip: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SmartSocket App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TurnOnPressed => {
                println!("run");
                Command::perform(
                    async {
                        println!("run");
                        let mut con = Counter::connect().await.unwrap();
                        con.turn_on().await
                    },
                    |val| {
                        Message::TooltipUpdate(match val {
                            Ok(s) => s,
                            Err(e) => e.to_string(),
                        })
                    },
                )
            }
            Message::TurnOffPressed => {
                println!("turn off pressed");
                Command::perform(
                    async {
                        let mut con = Counter::connect().await.unwrap();
                        con.turn_off().await
                    },
                    |val| {
                        Message::TooltipUpdate(match val {
                            Ok(s) => s,
                            Err(e) => e.to_string(),
                        })
                    },
                )
            }
            Message::GetStatisticsPressed => {
                println!("statistics pressed");
                Command::perform(
                    async {
                        let mut con = Counter::connect().await.unwrap();
                        con.status().await
                    },
                    |val| {
                        Message::ReceivedStatistics(match val {
                            Ok(s) => s,
                            Err(e) => e.to_string(),
                        })
                    },
                )
            }
            Message::ReceivedStatistics(s) => {
                self.stats = s;
                Command::none()
            }
            Message::TooltipUpdate(s) => {
                self.tooltip = s;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(self.tooltip.clone()).size(15),
            button("Turn On").on_press(Message::TurnOnPressed),
            button("Turn Off").on_press(Message::TurnOffPressed),
            button("Show stats").on_press(Message::GetStatisticsPressed),
            text(self.stats.clone()).size(50),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
