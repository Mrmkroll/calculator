use iced::{
    button, executor, text_input,
    window::{self, Position},
    Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Rule,
    Settings, Text, TextInput, Toggler,
};

pub fn main() -> iced::Result {
    Calculator::run(Settings {
        window: window::Settings {
            size: (420, 480),
            position: Position::Default,
            min_size: None,
            max_size: None,
            resizable: false,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        default_font: Some(include_bytes!("../fonts/RobotoMono-Light.ttf")),
        ..Settings::default()
    })
}

pub fn check_len(value: String) -> bool {
    if 1 < value.len() {
        return false
    }
    match &value.find("0") {
        Some(0) => return true,
        _ => return false,
    }
}

pub fn swap(value: String) -> usize {
    let val = value.trim_start_matches("0x");
    if val.len() == 0 {
        let num = 0;
        return num
    }
    let num = usize::from_str_radix(val, 16).unwrap();
    return num
}

pub fn calc(toggler_value: bool, calc_flag: usize ,calc_value: f64 , input_value: String) -> String {
    if toggler_value {
        match calc_flag {
            1 => {
                let input_value = format!("{:X}", (calc_value + swap(format!("{}{}", String::from("0x"), input_value)) as f64) as usize);
                return input_value;
            }
            2 => {
                let input_value = format!("{:X}", (calc_value - swap(format!("{}{}", String::from("0x"), input_value)) as f64) as usize);
                return input_value;
            }
            3 => {
                let input_value = format!("{:X}", (calc_value * swap(format!("{}{}", String::from("0x"), input_value)) as f64) as usize);
                return input_value;
            }
            4 => {
                let input_value = format!("{:X}", (calc_value / swap(format!("{}{}", String::from("0x"), input_value)) as f64) as usize);
                return input_value;
            }
            _ => return input_value,
        }
    } else {
        match calc_flag {
            1 => {
                let input_value = (calc_value + input_value.parse::<f64>().unwrap()).to_string();
                return input_value;
            }
            2 => {
                let input_value = (calc_value - input_value.parse::<f64>().unwrap()).to_string();
                return input_value;
            }
            3 => {
                let input_value = (calc_value * input_value.parse::<f64>().unwrap()).to_string();
                return input_value;
            }
            4 => {
                let input_value = (calc_value / input_value.parse::<f64>().unwrap()).to_string();
                return input_value;
            }
            _ => return input_value,
        }
    }
}

#[derive(Default)]
struct Calculator {
    _main: style::MainColor,
    _sub: style::SubColor,
    input: text_input::State,
    input_value: String,
    input_init: bool,
    calc_flag: usize,
    calc_value: f64,
    mem_value: f64,
    toggler_value: bool,
    button_00: button::State,
    button_0: button::State,
    button_1: button::State,
    button_2: button::State,
    button_3: button::State,
    button_4: button::State,
    button_5: button::State,
    button_6: button::State,
    button_7: button::State,
    button_8: button::State,
    button_9: button::State,
    button_a: button::State,
    button_b: button::State,
    button_c: button::State,
    button_d: button::State,
    button_e: button::State,
    button_f: button::State,
    button_dot: button::State,
    button_equal: button::State,
    button_plus: button::State,
    button_minus: button::State,
    button_mul: button::State,
    button_div: button::State,
    button_ca: button::State,
    button_del: button::State,
    button_not: button::State,
    button_rm: button::State,
    button_cm: button::State,
    button_mp: button::State,
    button_mm: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TogglerToggled(bool),
    ButtonDoubleZeroPressed,
    ButtonZeroPressed,
    ButtonOnePressed,
    ButtonTwoPressed,
    ButtonThreePressed,
    ButtonFourPressed,
    ButtonFivePressed,
    ButtonSixPressed,
    ButtonSevenPressed,
    ButtonEightPressed,
    ButtonNinePressed,
    ButtonAPressed,
    ButtonBPressed,
    ButtonCPressed,
    ButtonDPressed,
    ButtonEPressed,
    ButtonFPressed,
    ButtonDotPressed,
    ButtonEqualPressed,
    ButtonPlusPressed,
    ButtonMinusPressed,
    ButtonMulPressed,
    ButtonDivPressed,
    ButtonDelPressed,
    ButtonCAPressed,
    ButtonNotPressed,
    ButtonRMPressed,
    ButtonCMPressed,
    ButtonMPPressed,
    ButtonMMPressed,
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Calculator::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                let value = value.to_ascii_uppercase().to_string();
                for c in value.bytes() {
                    if 47 < c && c < 58 || self.toggler_value && 64 < c && c < 71 {
                        if check_len((&value).to_string()) {
                            self.input_value = (&value[1..]).to_string();
                        } else {
                            self.input_value = (&value).to_string();
                        }
                    } else {
                        if check_len((&value).to_string()) {
                            self.input_value = (&value[1..value.len()-1]).to_string();
                        } else {
                            self.input_value = (&value[..value.len()-1]).to_string();
                        }
                    }
                }
            }
            Message::TogglerToggled(value) => {
                self.toggler_value = value;
                if self.toggler_value {
                    self.input_value = format!("{:X}", self.input_value.parse::<f64>().unwrap() as isize);
                } else {
                    self.input_value = (usize::from_str_radix(&self.input_value, 16).unwrap()).to_string();
                }
            }
            Message::ButtonDoubleZeroPressed => {
                if !check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = format!("{}{}", self.input_value, String::from("00"));
                    self.input_init = false;
                }
            }
            Message::ButtonZeroPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("0");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("0"));
                }
                self.input_init = false;
            }
            Message::ButtonOnePressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("1");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("1"));
                }
                self.input_init = false;
            }
            Message::ButtonTwoPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("2");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("2"));
                }
                self.input_init = false;
            }
            Message::ButtonThreePressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("3");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("3"));
                }
                self.input_init = false;
            }
            Message::ButtonFourPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("4");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("4"));
                }
                self.input_init = false;
            }
            Message::ButtonFivePressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("5");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("5"));
                }
                self.input_init = false;
            }
            Message::ButtonSixPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("6");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("6"));
                }
                self.input_init = false;
            }
            Message::ButtonSevenPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("7");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("7"));
                }
                self.input_init = false;
            }
            Message::ButtonEightPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("8");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("8"));
                }
                self.input_init = false;
            }
            Message::ButtonNinePressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("9");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("9"));
                }
                self.input_init = false;
            }
            Message::ButtonAPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("A");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("A"));
                }
                self.input_init = false;
            }
            Message::ButtonBPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("B");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("B"));
                }
                self.input_init = false;
            }
            Message::ButtonCPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("C");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("C"));
                }
                self.input_init = false;
            }
            Message::ButtonDPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("D");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("D"));
                }
                self.input_init = false;
            }
            Message::ButtonEPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("E");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("E"));
                }
                self.input_init = false;
            }
            Message::ButtonFPressed => {
                if check_len((&self.input_value).to_string()) || self.input_init {
                    self.input_value = String::from("F");
                } else {
                    self.input_value = format!("{}{}", self.input_value, String::from("F"));
                }
                self.input_init = false;
            }
            Message::ButtonDotPressed => {
                self.input_value = format!("{}{}", self.input_value, String::from("."));
                self.input_init = false;
            }
            Message::ButtonEqualPressed => {
                self.input_value = calc(self.toggler_value, self.calc_flag, self.calc_value, (&self.input_value).to_string());
                self.input_init = true;
                self.calc_flag = 0;
            }
            Message::ButtonPlusPressed => {
                self.input_value = calc(self.toggler_value, self.calc_flag, self.calc_value, (&self.input_value).to_string());
                self.input_init = true;
                self.calc_flag = 1;
                if self.toggler_value {
                    self.calc_value = swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.calc_value = self.input_value.parse::<f64>().unwrap();
                }
            }
            Message::ButtonMinusPressed => {
                self.input_value = calc(self.toggler_value, self.calc_flag, self.calc_value, (&self.input_value).to_string());
                self.input_init = true;
                self.calc_flag = 2;
                if self.toggler_value {
                    self.calc_value = swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.calc_value = self.input_value.parse::<f64>().unwrap();
                }
            }
            Message::ButtonMulPressed => {
                self.input_value = calc(self.toggler_value, self.calc_flag, self.calc_value, (&self.input_value).to_string());
                self.input_init = true;
                self.calc_flag = 3;
                if self.toggler_value {
                    self.calc_value = swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.calc_value = self.input_value.parse::<f64>().unwrap();
                }
            }
            Message::ButtonDivPressed => {
                self.input_value = calc(self.toggler_value, self.calc_flag, self.calc_value, (&self.input_value).to_string());
                self.input_init = true;
                self.calc_flag = 4;
                if self.toggler_value {
                    self.calc_value = swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.calc_value = self.input_value.parse::<f64>().unwrap();
                }
            }
            Message::ButtonDelPressed => {
                self.input_value = String::from("0");
            }
            Message::ButtonCAPressed => {
                self.input_value = String::from("0");
                self.calc_value = 0 as f64;
                self.mem_value = 0 as f64;
                self.input_init = false;
            }
            Message::ButtonNotPressed => {
                self.input_init = false;
                if self.toggler_value {
                    self.input_value = format!("{:X}", !swap(format!("{}{}", String::from("0x"), self.input_value)))
                } else {
                    match &self.input_value.find(".") {
                        None => self.input_value = (!self.input_value.parse::<usize>().unwrap()).to_string(),
                        _ => self.input_value = (!(self.input_value.parse::<f64>().unwrap() as usize)).to_string(),
                    }
                }
            }
            Message::ButtonRMPressed => {
                self.input_init = false;
                if self.toggler_value {
                    self.input_value = format!("{:X}", self.mem_value as usize);
                } else {
                    self.input_value = self.mem_value.to_string();
                }
            }
            Message::ButtonCMPressed => self.mem_value = 0 as f64,
            Message::ButtonMPPressed => {
                if self.toggler_value {
                    self.mem_value = self.mem_value + swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.mem_value = self.mem_value + self.input_value.parse::<f64>().unwrap();
                }
            }
            Message::ButtonMMPressed => {
                if self.toggler_value {
                    self.mem_value = self.mem_value - swap(format!("{}{}", String::from("0x"), self.input_value)) as f64;
                } else {
                    self.mem_value = self.mem_value - self.input_value.parse::<f64>().unwrap();
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let len = self.input_value.len();
        if len == 0 {
            self.input_value = String::from("0");
        } else if 33 < len {
            self.input_value = (&self.input_value[..len-1]).to_string();
        }
        let text_input = TextInput::new(
            &mut self.input,
            "",
            &self.input_value,
            Message::InputChanged)
            .padding(10)
            .size(20)
            .style(self._main);

        let toggler = Toggler::new(
            self.toggler_value,
            String::from(""),
            Message::TogglerToggled)
            .width(Length::Shrink)
            .style(self._main);

        let mut button_a = Button::new(&mut self.button_a, Text::new("A"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_a = button_a.on_press(Message::ButtonAPressed)
        };

        let mut button_b = Button::new(&mut self.button_b, Text::new("B"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_b = button_b.on_press(Message::ButtonBPressed)
        };

        let mut button_c = Button::new(&mut self.button_c, Text::new("C"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_c = button_c.on_press(Message::ButtonCPressed)
        };

        let mut button_d = Button::new(&mut self.button_d, Text::new("D"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_d = button_d.on_press(Message::ButtonDPressed)
        };

        let mut button_e = Button::new(&mut self.button_e, Text::new("E"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_e = button_e.on_press(Message::ButtonEPressed)
        };

        let mut button_f = Button::new(&mut self.button_f, Text::new("F"))
            .padding([10, 22])
            .style(self._main);

        if self.toggler_value {
            button_f = button_f.on_press(Message::ButtonFPressed)
        };

        let button_00 = Button::new(&mut self.button_00, Text::new("00"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonDoubleZeroPressed)
            .style(self._main);

        let button_0 = Button::new(&mut self.button_0, Text::new("0"))
            .padding([10, 22])
            .on_press(Message::ButtonZeroPressed)
            .style(self._main);

        let mut button_dot = Button::new(&mut self.button_dot, Text::new("."))
            .padding([10, 22])
            .style(self._main);

        if !self.toggler_value {
            button_dot = button_dot.on_press(Message::ButtonDotPressed)
        };

        let button_1 = Button::new(&mut self.button_1, Text::new("1"))
            .padding([10, 22])
            .on_press(Message::ButtonOnePressed)
            .style(self._main);

        let button_2 = Button::new(&mut self.button_2, Text::new("2"))
            .padding([10, 22])
            .on_press(Message::ButtonTwoPressed)
            .style(self._main);

        let button_3 = Button::new(&mut self.button_3, Text::new("3"))
            .padding([10, 22])
            .on_press(Message::ButtonThreePressed)
            .style(self._main);

        let button_4 = Button::new(&mut self.button_4, Text::new("4"))
            .padding([10, 22])
            .on_press(Message::ButtonFourPressed)
            .style(self._main);

        let button_5 = Button::new(&mut self.button_5, Text::new("5"))
            .padding([10, 22])
            .on_press(Message::ButtonFivePressed)
            .style(self._main);

        let button_6 = Button::new(&mut self.button_6, Text::new("6"))
            .padding([10, 22])
            .on_press(Message::ButtonSixPressed)
            .style(self._main);

        let button_7 = Button::new(&mut self.button_7, Text::new("7"))
            .padding([10, 22])
            .on_press(Message::ButtonSevenPressed)
            .style(self._main);

        let button_8 = Button::new(&mut self.button_8, Text::new("8"))
            .padding([10, 22])
            .on_press(Message::ButtonEightPressed)
            .style(self._main);

        let button_9 = Button::new(&mut self.button_9, Text::new("9"))
            .padding([10, 22])
            .on_press(Message::ButtonNinePressed)
            .style(self._main);

        let button_equal = Button::new(&mut self.button_equal, Text::new("="))
            .padding([10, 22])
            .on_press(Message::ButtonEqualPressed)
            .style(self._sub);

        let button_plus = Button::new(&mut self.button_plus, Text::new("+"))
            .padding([10, 22])
            .on_press(Message::ButtonPlusPressed)
            .style(self._sub);

        let button_minus = Button::new(&mut self.button_minus, Text::new("-"))
            .padding([10, 22])
            .on_press(Message::ButtonMinusPressed)
            .style(self._sub);

        let button_mul = Button::new(&mut self.button_mul, Text::new("×"))
            .padding([10, 22])
            .on_press(Message::ButtonMulPressed)
            .style(self._sub);

        let button_div = Button::new(&mut self.button_div, Text::new("÷"))
            .padding([10, 22])
            .on_press(Message::ButtonDivPressed)
            .style(self._sub);

        let button_ca = Button::new(&mut self.button_ca, Text::new("CA"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonCAPressed)
            .style(self._sub);

        let button_del = Button::new(&mut self.button_del, Text::new("Del"))
            .padding([10, 13])
            .on_press(Message::ButtonDelPressed)
            .style(self._sub);

        let button_not = Button::new(&mut self.button_not, Text::new("NOT"))
            .padding([10, 13])
            .on_press(Message::ButtonNotPressed)
            .style(self._sub);

        let button_cm = Button::new(&mut self.button_cm, Text::new("CM"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonCMPressed)
            .style(self._sub);

        let button_rm = Button::new(&mut self.button_rm, Text::new("RM"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonRMPressed)
            .style(self._sub);

        let button_mp = Button::new(&mut self.button_mp, Text::new("M+"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonMPPressed)
            .style(self._sub);

        let button_mm = Button::new(&mut self.button_mm, Text::new("M-"))
            .padding([10, 17, 10, 18])
            .on_press(Message::ButtonMMPressed)
            .style(self._sub);

        let content = Column::new()
            .spacing(20)
            .padding(40)
            .max_width(400)
            .push(Row::new().spacing(10).push(text_input))
            .push(Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new("Dec"))
                            .push(toggler)
                            .push(Text::new("Hex")),
                    ),
            )
            .push(Column::new()
                    .width(Length::Fill)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(Row::new()
                            .push(Column::new()
                                    .width(Length::Shrink)
                                    .spacing(10)
                                    .push(Row::new().spacing(10).push(button_a))
                                    .push(Row::new().spacing(10).push(button_b))
                                    .push(Row::new().spacing(10).push(button_c))
                                    .push(Row::new().spacing(10).push(button_d))
                                    .push(Row::new().spacing(10).push(button_e))
                                    .push(Row::new().spacing(10).push(button_f)),
                            )
                            .push(Column::new()
                                    .height(Length::Units(290))
                                    .push(Rule::vertical(20).style(self._main)),
                            )
                            .push(Column::new()
                                    .spacing(10)
                                    .push(Row::new().spacing(10)
                                            .push(button_cm).push(button_rm).push(button_ca).push(button_del),
                                    )
                                    .push(Row::new()
                                            .spacing(10)
                                            .push(button_mp).push(button_mm).push(button_not).push(button_div),
                                    )
                                    .push(Row::new()
                                            .spacing(10)
                                            .push(button_7).push(button_8).push(button_9).push(button_mul),
                                    )
                                    .push(Row::new()
                                            .spacing(10)
                                            .push(button_4).push(button_5).push(button_6).push(button_minus),
                                    )
                                    .push(Row::new()
                                            .spacing(10)
                                            .push(button_1).push(button_2).push(button_3).push(button_plus),
                                    )
                                    .push(Row::new()
                                            .spacing(10)
                                            .push(button_0).push(button_00).push(button_dot).push(button_equal),
                                    ),
                            ),
                    ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self._main)
            .into()
    }
}

mod style {
    use iced::{button, container, rule, text_input, toggler};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MainColor {
        Dark,
    }

    impl MainColor {
        pub const ALL: [MainColor; 1] = [MainColor::Dark];
    }

    impl Default for MainColor {
        fn default() -> MainColor {
            MainColor::Dark
        }
    }

    impl<'a> From<MainColor> for Box<dyn container::StyleSheet + 'a> {
        fn from(_main: MainColor) -> Self {
            main_color::Container.into()
        }
    }

    impl<'a> From<MainColor> for Box<dyn text_input::StyleSheet + 'a> {
        fn from(_main: MainColor) -> Self {
            main_color::TextInput.into()
        }
    }

    impl<'a> From<MainColor> for Box<dyn button::StyleSheet + 'a> {
        fn from(_main: MainColor) -> Self {
            main_color::Button.into()
        }
    }

    impl From<MainColor> for Box<dyn toggler::StyleSheet> {
        fn from(_main: MainColor) -> Self {
            main_color::Toggler.into()
        }
    }

    impl From<MainColor> for Box<dyn rule::StyleSheet> {
        fn from(_main: MainColor) -> Self {
            main_color::Rule.into()
        }
    }

    mod main_color {
        use iced::{button, container, rule, text_input, toggler, Color};

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x95 as f32 / 255.0,
            0xAC as f32 / 255.0,
            0xFF as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x4B as f32 / 255.0,
            0x5B as f32 / 255.0,
            0x96 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                    text_color: Color::WHITE.into(),
                    ..container::Style::default()
                }
            }
        }

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: SURFACE.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: ACCENT,
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: Color { a: 0.3, ..ACCENT },
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.4, 0.4, 0.4)
            }

            fn value_color(&self) -> Color {
                Color::WHITE
            }

            fn selection_color(&self) -> Color {
                ACTIVE
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: ACTIVE.into(),
                    border_radius: 3.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: HOVERED.into(),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1.0,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }

        pub struct Toggler;

        impl toggler::StyleSheet for Toggler {
            fn active(&self, is_active: bool) -> toggler::Style {
                toggler::Style {
                    background: if is_active { ACTIVE } else { SURFACE },
                    background_border: None,
                    foreground: if is_active { Color::WHITE } else { ACTIVE },
                    foreground_border: None,
                }
            }

            fn hovered(&self, is_active: bool) -> toggler::Style {
                toggler::Style {
                    background: if is_active { ACTIVE } else { SURFACE },
                    background_border: None,
                    foreground: if is_active {
                        Color {
                            a: 0.5,
                            ..Color::WHITE
                        }
                    } else {
                        Color { a: 0.5, ..ACTIVE }
                    },
                    foreground_border: None,
                }
            }
        }

        pub struct Rule;

        impl rule::StyleSheet for Rule {
            fn style(&self) -> rule::Style {
                rule::Style {
                    color: SURFACE,
                    width: 2,
                    radius: 1.0,
                    fill_mode: rule::FillMode::Padded(15),
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SubColor {
        DarkBlue,
    }

    impl SubColor {
        pub const ALL: [SubColor; 1] = [SubColor::DarkBlue];
    }

    impl Default for SubColor {
        fn default() -> SubColor {
            SubColor::DarkBlue
        }
    }

    impl<'a> From<SubColor> for Box<dyn button::StyleSheet + 'a> {
        fn from(_sub: SubColor) -> Self {
            sub_color::Button.into()
        }
    }

    mod sub_color {
        use iced::{button, Color};

        const ACTIVE: Color = Color::from_rgb(
            0x32 as f32 / 255.0,
            0x40 as f32 / 255.0,
            0x70 as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x27 as f32 / 255.0,
            0x33 as f32 / 255.0,
            0x5D as f32 / 255.0,
        );

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: ACTIVE.into(),
                    border_radius: 3.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: HOVERED.into(),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1.0,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }
    }
}
