#![windows_subsystem = "windows"]
use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

pub enum Operator {
    Plus,
    Minus,
    Times,
}

pub struct AppModel {
    result: Option<i16>
}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
    UpdateComputation,
}

#[widget]
impl Widget for AppWin {
    fn model() -> AppModel {
        AppModel{ 
            result: None
        }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            AppMsg::UpdateComputation => self.update_result(),
            AppMsg::Quit => gtk::main_quit()
        }
    }

    fn update_result(&mut self) {
        if let Some(v1_str) = self.cb_operand_1.get_active_text() {
            if let Some(v2_str) = self.cb_operand_2.get_active_text() {
                if let Some(ref op_str) = self.cb_operator.get_active_text() {
                    let v1 = v1_str.parse::<i16>().unwrap_or_default();
                    let v2 = v2_str.parse::<i16>().unwrap_or_default();
                    self.model.result = Some(match op_str.as_str() {
                        "+" => (v1 + v2) as i16,
                        "-" => (v1 - v2) as i16,
                        "*" => (v1 * v2) as i16,
                        _ => 0_i16
                    });
                }
            }
        }
    }

    fn init_view(&mut self) {
        for digit in 1..=9 {
            let digit_str = format!("{}", digit);
            let digit_str = digit_str.as_str();
            self.cb_operand_1.append(Some(digit_str), digit_str);
            self.cb_operand_2.append(Some(digit_str), digit_str);
        }

        for letter in vec!["+", "-", "*"] {
            self.cb_operator.append(Some(letter), letter);
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                #[name="cb_operand_1"]
                gtk::ComboBoxText {
                    changed(source) => {
                        AppMsg::UpdateComputation
                    }
                },
                #[name="cb_operator"]
                gtk::ComboBoxText {
                    changed(source) => {
                        AppMsg::UpdateComputation
                    }
                },
                #[name="cb_operand_2"]
                gtk::ComboBoxText {
                    changed(source) => {
                        AppMsg::UpdateComputation
                    }
                },
                gtk::Label {
                    text: &self.model.result.unwrap_or_default().to_string(),
                    justify: gtk::Justification::Center,
                },
            },
            delete_event(_, _) => (AppMsg::Quit, Inhibit(false)),
        },
    }
}

fn main() {
    AppWin::run(()).expect("Failed to start application !");
}
