use ratatui::{prelude::*, widgets::*};

const MAX_ELEMENT_ROW: usize = 4;

#[derive(Clone)]
pub struct CustomTable<'a> {
    rows: Vec<Vec<String>>,
    header: Vec<String>,
    block: Block<'a>,
    header_style: Style,
    header_block_style: Style,
    rows_style: Style,
    highlight_style: Style,
    style: Style,

    constraints_col: Vec<Constraint>,
}

impl<'a> CustomTable<'a> {
    pub fn new() -> Self {
        CustomTable {
            rows: Vec::new(),
            header: Vec::new(),
            block: Block::default(),
            header_style: Style::default(),
            header_block_style: Style::default(),
            rows_style: Style::default(),
            highlight_style: Style::default(),
            style: Style::default(),
            constraints_col: Vec::new(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }
    pub fn header(mut self, header: Vec<String>) -> Self {
        self.header = header;

        self.constraints_col = if self.header.len() > MAX_ELEMENT_ROW {
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ]
        } else {
            self.header
                .iter()
                .map(|_| Constraint::Fill(1))
                .collect::<Vec<_>>()
        };
        self
    }
    pub fn rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }
    pub fn rows_style(mut self, style: Style) -> Self {
        self.rows_style = style;
        self
    }
    pub fn header_block_style(mut self, style: Style) -> Self {
        self.header_block_style = style;
        self
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer, state: &mut CustomTableState) {
        let header_block = Block::default()
            .style(self.header_style)
            .borders(Borders::BOTTOM)
            .border_style(self.header_block_style);

        let rects = Layout::horizontal(&self.constraints_col)
            .flex(layout::Flex::Center)
            .split(Rect::new(
                area.x + 1,
                area.y + 1,
                area.width - 1,
                area.height - 1,
            ));

        for index in 0..self.constraints_col.len() {
            let header_title = self.header.get(state.offset_x + index).unwrap();
            let mut line = Line::from(Span::from(header_title).style(self.header_style));

            if let Some((x, y)) = state.position {
                if x == index && y == 0 {
                    // highlight of col selected
                    line = Line::from(Span::from(header_title).style(self.highlight_style));
                }
            }
            line.render(*rects.get(index).unwrap(), buf);
        }

        header_block.render(Rect::new(area.x, area.y, area.width - 2, 3), buf);
    }

    fn render_rows(&self, area: Rect, buf: &mut Buffer, state: &mut CustomTableState) {
        // we need to set max element in col because we don't know
        // the size of the area otherwise
        state.max_element_in_col = (area.height - 4) as usize;
        // avoid calculating constraint directly here
        let col_size = ((area.width - 2) as usize / state.max_element_in_row) as u16;

        for (row_index, result) in self.rows
            [state.offset_y..=state.offset_y + state.max_element_in_col]
            .iter()
            .enumerate()
        {
            for col_index in 0..self.constraints_col.len() {
                let item_rect = Rect::new(
                    area.x + 1 + (col_index as u16) * col_size,
                    area.y + 3 + row_index as u16,
                    col_size,
                    1,
                );
                let content = result.get(state.offset_x + col_index).unwrap();
                let mut line = Line::from(Span::from(content).style(self.rows_style));

                if let Some((x, y)) = state.position {
                    if x == col_index && y == row_index + 1 {
                        line = Line::from(Span::from(content).style(self.highlight_style));
                    }
                }
                line.render(item_rect, buf);
            }
        }
    }
}

impl StatefulWidget for CustomTable<'_> {
    type State = CustomTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        self.render_header(area, buf, state);
        self.render_rows(area, buf, state);

        self.block.render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTableState {
    pub offset_x: usize,
    pub offset_y: usize,
    pub position: Option<(usize, usize)>,
    pub header_length: usize,
    pub content_length: usize,

    pub max_element_in_row: usize,
    pub max_element_in_col: usize,
}

impl CustomTableState {
    pub fn new(header_length: usize, content_length: usize) -> Self {
        let max_element_in_row = if header_length > MAX_ELEMENT_ROW {
            MAX_ELEMENT_ROW
        } else {
            header_length
        };
        CustomTableState {
            offset_x: 0,
            offset_y: 0,
            position: None,
            header_length,
            content_length,
            max_element_in_row,
            max_element_in_col: 0,
        }
    }
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.content_length = content_length;
        self
    }
    pub fn header_length(mut self, header_length: usize) -> Self {
        self.header_length = header_length;
        let max_element_row = if header_length > MAX_ELEMENT_ROW {
            MAX_ELEMENT_ROW
        } else {
            header_length
        };
        self.max_element_in_row = max_element_row;
        self
    }

    pub fn next_col(&mut self) {
        if let Some((pos, y)) = self.position {
            if pos == self.max_element_in_row - 1 {
                if self.offset_x + pos < self.header_length - 1 {
                    self.offset_x = self.offset_x.saturating_add(1);
                }
            } else {
                self.position = Some((pos.saturating_add(1), y));
            }
        } else {
            self.position = Some((0, 0));
        }
    }
    pub fn prev_col(&mut self) {
        if let Some((pos, y)) = self.position {
            if pos == 0 {
                self.offset_x = self.offset_x.saturating_sub(1);
            }
            self.position = Some((pos.saturating_sub(1), y));
        }
    }
    pub fn next_row(&mut self) {
        if let Some((x, pos)) = self.position {
            if pos == self.max_element_in_col {
                if self.offset_y + pos < self.content_length - 1 {
                    self.offset_y = self.offset_y.saturating_add(1);
                }
            } else {
                self.position = Some((x, pos.saturating_add(1)));
            }
        } else {
            self.position = Some((0, 0));
        }
    }
    pub fn prev_row(&mut self) {
        if let Some((x, pos)) = self.position {
            if pos == 1 && self.offset_y > 0 {
                self.offset_y = self.offset_y.saturating_sub(1);
            } else {
                self.position = Some((x, pos.saturating_sub(1)));
            }
        } else {
            self.position = Some((0, 0));
        }
    }

    pub fn selected(&self) -> Option<(usize, usize)> {
        self.position
    }
    pub fn select(&mut self, new_pos: Option<(usize, usize)>) {
        self.position = new_pos;
    }
}
