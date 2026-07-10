#![cfg(feature = "ratatui")]

use ratatui_core::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect, VerticalAlignment},
    style::{Color, Style},
    widgets::Widget,
};

#[test]
fn math_size_matches_rendered_text() {
    let math = txm::ratatui::Math::new("ab").expect("math creation failed");
    let size = math.size();

    assert_eq!(size.width, 2);
    assert_eq!(size.height, 1);
}

#[test]
fn math_renders_with_alignment_style_and_offset() {
    let math = txm::ratatui::Math::new("ab")
        .expect("math creation failed")
        .style(Style::default().fg(Color::Red))
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center);

    let area = Rect::new(2, 1, 6, 5);
    let mut buffer = Buffer::empty(area);

    (&math).render(area, &mut buffer);

    assert_eq!(buffer[(4, 3)].symbol(), "a");
    assert_eq!(buffer[(5, 3)].symbol(), "b");
    assert_eq!(buffer[(4, 3)].fg, Color::Red);
}

#[test]
fn math_clips_to_area_bounds() {
    let math = txm::ratatui::Math::new("abcd").expect("math creation failed");
    let area = Rect::new(0, 0, 2, 1);
    let mut buffer = Buffer::empty(area);

    (&math).render(area, &mut buffer);

    assert_eq!(buffer[(0, 0)].symbol(), "a");
    assert_eq!(buffer[(1, 0)].symbol(), "b");
}

#[test]
fn math_returns_error_for_invalid_input() {
    assert!(txm::ratatui::Math::new("{x").is_err());
}
