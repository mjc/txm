use ratatui_core::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect, Size, VerticalAlignment},
    style::Style,
    widgets::Widget,
};

#[derive(Debug, Clone)]
pub struct Math {
    rendered: String,
    style: Style,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Math {
    pub fn new(input: &str) -> Result<Self, crate::ParseError> {
        let rendered = crate::render(input)?;
        Ok(Self {
            rendered,
            style: Style::default(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        })
    }

    pub fn size(&self) -> Size {
        let (width, height) = rendered_size(&self.rendered);
        Rect::new(0, 0, width, height).as_size()
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl Widget for &Math {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let (render_width, render_height) = rendered_size(&self.rendered);
        if render_width == 0 || render_height == 0 {
            return;
        }

        let (content_x, draw_x, visible_width) =
            align_horizontal_span(render_width, area.width, self.horizontal_alignment);
        let (content_y, draw_y, visible_height) =
            align_vertical_span(render_height, area.height, self.vertical_alignment);

        let lines: Vec<&str> = self.rendered.lines().collect();

        for row in 0..visible_height {
            let source_row = content_y + row;
            let line = lines[source_row as usize];
            let visible: String = line
                .chars()
                .skip(content_x as usize)
                .take(visible_width as usize)
                .collect();
            buf.set_string(area.x + draw_x, area.y + draw_y + row, visible, self.style);
        }
    }
}

fn rendered_size(rendered: &str) -> (u16, u16) {
    let mut width = 0u16;
    let mut height = 0u16;

    for line in rendered.lines() {
        width = width.max(u16::try_from(line.chars().count()).unwrap_or(u16::MAX));
        height = height.saturating_add(1);
    }

    (width, height)
}

fn align_horizontal_span(
    content: u16,
    area: u16,
    alignment: HorizontalAlignment,
) -> (u16, u16, u16) {
    let visible = content.min(area);

    if content <= area {
        let draw = match alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => (area - content) / 2,
            HorizontalAlignment::Right => area - content,
        };
        (0, draw, visible)
    } else {
        let content_start = match alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => (content - area) / 2,
            HorizontalAlignment::Right => content - area,
        };
        (content_start, 0, visible)
    }
}

fn align_vertical_span(content: u16, area: u16, alignment: VerticalAlignment) -> (u16, u16, u16) {
    let visible = content.min(area);

    if content <= area {
        let draw = match alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => (area - content) / 2,
            VerticalAlignment::Bottom => area - content,
        };
        (0, draw, visible)
    } else {
        let content_start = match alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => (content - area) / 2,
            VerticalAlignment::Bottom => content - area,
        };
        (content_start, 0, visible)
    }
}
