pub fn create(text: &str) -> String {
    let len = text.len();
    let horizontal_line = "─".repeat(len + 2);

    let top = format!("  ╭{}╮", horizontal_line);
    let middle = format!("  │ {} │", text);
    let bottom = format!("  ╰{}╯", horizontal_line);

    // Bubble pointer (refined ASCII)
    let pointer = "      ▼";

    format!("{}\n{}\n{}\n{}", top, middle, bottom, pointer)
}
