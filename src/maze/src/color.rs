/// Parses "rgb(r, g, b)" or "rgba(r, g, b, a)" strings.
pub fn parse_rgb_color(rgb_str: &str) -> Result<wgpu::Color, String> {
    let trimmed = rgb_str.trim();

    // Extract the content between the parentheses
    let inner = if let Some(stripped) = trimmed.strip_prefix("rgba(") {
        stripped.strip_suffix(')').ok_or("Missing closing parenthesis for rgba")?
    } else if let Some(stripped) = trimmed.strip_prefix("rgb(") {
        stripped.strip_suffix(')').ok_or("Missing closing parenthesis for rgb")?
    } else {
        return Err("String is not in rgb() or rgba() format".to_string());
    };

    // Split the components by the comma
    let components: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if components.len() < 3 || components.len() > 4 {
        return Err(format!("Invalid number of components: {}", components.len()));
    }

    // Parse R, G, B as u8 values
    let r = components[0].parse::<u8>().map_err(|e| e.to_string())?;
    let g = components[1].parse::<u8>().map_err(|e| e.to_string())?;
    let b = components[2].parse::<u8>().map_err(|e| e.to_string())?;

    // Parse Alpha. It can be a float (0-1) or an integer (0-255).
    // CSS standard is a float for rgba(), but some older implementations might differ.
    // Let's assume float. If no alpha, default to 1.0.
    let a = if components.len() == 4 {
        components[3].parse::<f32>().map_err(|e| e.to_string())?
    } else {
        1.0
    };

    Ok(wgpu::Color {
        r: r as f64 / 255.0,
        g: g as f64 / 255.0,
        b: b as f64 / 255.0,
        a: a as f64,
    })
}
