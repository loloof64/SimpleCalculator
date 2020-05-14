extern crate iui;
use iui::controls::{Combobox, HorizontalBox, Label};
use iui::prelude::*;

use std::fmt;

#[derive(Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Times,
}

impl Operator {
    fn from(value: i8) -> Option<Self> {
        match value {
            0 => Some(Operator::Plus),
            1 => Some(Operator::Minus),
            2 => Some(Operator::Times),
            _ => None,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_to_print = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "x",
        };
        write!(f, "{}", string_to_print)
    }
}

fn build_digits_combo_box(ui: &UI) -> Combobox {
    let combo = Combobox::new(&ui);
    for value in 1..=9 {
        combo.append(&ui, format!("{}", value).as_str());
    }
    combo
}

fn build_operators_combo_box(ui: &UI) -> Combobox {
    let combo = Combobox::new(&ui);
    for value in vec!["+", "-", "x"] {
        combo.append(&ui, value);
    }
    combo
}

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");

    let mut selector_operand_1 = build_digits_combo_box(&ui);
    let mut selector_operand_2 = build_digits_combo_box(&ui);
    let mut selector_operator = build_operators_combo_box(&ui);
    let label_equal = Label::new(&ui, "=");
    let mut label_result = Label::new(&ui, "");

    let mut operand_1: Option<i8> = None;
    let mut operand_2: Option<i8> = None;
    let mut operator: Option<Operator> = None;

    let mut update_result = |op1, op2, operator| {
        if let Some(v1) = op1 {
            if let Some(v2) = op2 {
                if let Some(op) = operator {
                    let result = match op {
                        Operator::Plus => v1 + v2,
                        Operator::Minus => v1 - v2,
                        Operator::Times => v1 * v2,
                    };
                    label_result.set_text(&ui, format!("{}", result).as_str());
                }
            }
        }
    };

    selector_operand_1.on_selected(&ui, |index| {
        operand_1 = Some((index + 1) as i8);
        // switching from mutable borrow to immutable borrow
        let operand_1 = operand_1.clone();
        update_result(operand_1, operand_2, operator);
    });

    selector_operand_2.on_selected(&ui, |index| {
        operand_2 = Some((index + 1) as i8);
        // switching from mutable borrow to immutable borrow
        let operand_2 = operand_2.clone();
        update_result(operand_1, operand_2, operator);
    });

    selector_operator.on_selected(&ui, |index| {
        operator = Operator::from(index as i8);
        // switching from mutable borrow to immutable borrow
        let operator = operator.clone();
        update_result(operand_1, operand_2, operator);
    });

    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);
    hbox.append(&ui, selector_operand_1, LayoutStrategy::Compact);
    hbox.append(&ui, selector_operator, LayoutStrategy::Compact);
    hbox.append(&ui, selector_operand_2, LayoutStrategy::Compact);
    hbox.append(&ui, label_equal, LayoutStrategy::Compact);
    hbox.append(&ui, label_result, LayoutStrategy::Compact);

    let mut win = Window::new(&ui, "Simple calc", 200, 40, WindowType::NoMenubar);
    win.set_child(&ui, hbox);
    win.show(&ui);
    ui.main();
}
