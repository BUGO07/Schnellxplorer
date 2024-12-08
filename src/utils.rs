/// This function takes in a file size and returns it with an appropriate unit
pub fn size_units(size: f32) -> (String, String) {
    let display_size;
    let unit;
    if size > 1_000_000_000.0 {
        display_size = size / 1_000_000_000.0;
        unit = "GB";
    } else if size > 1_000_000.0 {
        display_size = size / 1_000_000.0;
        unit = "MB";
    } else if size > 1_000.0 {
        display_size = size / 1_000.0;
        unit = "KB";
    } else {
        display_size = size;
        unit = "B";
    };
    let size_string = if unit == "B" {
        format!("{:.0}", display_size)
    } else {
        format!("{:.2}", display_size)
    };
    (size_string, unit.to_string())
}
