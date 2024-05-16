use ratatui::{prelude::*, Frame};
use std::rc::Rc;

#[derive(Clone)]
pub struct LayoutArea {
    pub main_area: Rc<[Rect]>,
    pub sub_main_area: Rc<[Rect]>,
    pub left_area: Rc<[Rect]>,
    pub right_area: Rc<[Rect]>,
}

impl LayoutArea {
    pub fn new(frame: &Frame) -> Self {
        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.size());

        let sub_main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(2), Constraint::Fill(6)])
            .split(main_area[0]);

        let left_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(3),
                Constraint::Fill(3),
                Constraint::Fill(3),
            ])
            .split(sub_main_area[0]);
        let right_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(2),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(sub_main_area[1]);
        LayoutArea {
            main_area,
            sub_main_area,
            left_area,
            right_area,
        }
    }
}
