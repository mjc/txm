use std::process::Command;

#[test]
fn prints_usage_without_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Usage: txm [LaTeX input]\n"
    );
}

#[test]
fn boxes_simple_identifier() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("x")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌───┐\n│   │\n│ x │\n│   │\n└───┘\n"
    );
}

#[test]
fn boxes_wide_identifier() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("你")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌────┐\n│    │\n│ 你 │\n│    │\n└────┘\n"
    );
}

#[test]
fn boxes_adjacent_wide_identifiers() {
    let output = Command::new(env!("CARGO_BIN_EXE_txm"))
        .arg("你你")
        .output()
        .expect("failed to run txm");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "┌──────┐\n│      │\n│ 你你 │\n│      │\n└──────┘\n"
    );
}

#[test]
fn render_returns_raw_lines_for_simple_identifier() {
    let rendered = txm::render("x");

    assert_eq!(rendered, "x\n");
}
