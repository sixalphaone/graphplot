use std::fmt::{Display, Write};

/// Genererer en SVG-representasjon av et "release notes"-kort.
///
/// # Arguments
/// * `attributes` - Attributter med eller ute markering.
/// * `scale` - Skalerer hele kortet, men holder `object` uendret.
/// * `object`- Valgfritt objekt/bilde som (<g>, width, height).
pub fn attribute_card<S: Display, T: Display>(
    vertical_title: S,
    blackbox_title: S,
    attribute_title: S,
    attributes: &[(T, bool)],
    scale: Option<f32>,
    object: Option<(S, f32, f32)>,
) -> String {
    let scale = scale.unwrap_or(1.0);

    // --- 1. Definer dimensjoner og stiler ---
    let vertical_label_font_size = 26.0;
    let title_font_size = 16.0;
    let attribute_title_font_size = 12.0;
    let attributes_font_size = 10.0;

    // Boks-dimensjoner
    let padding = 10.0;
    let black_box_x = padding;
    let black_box_y = padding;
    let corner_radius = 10.0;
    let (mut card_width, mut card_height, mut black_box_width) = (400.0, 220.0, 280.0);
    if let Some((_, width, height)) = object {
        let dw = (card_width * 0.05) - width / scale;
        let dh = (card_height * 0.3) - height / scale;
        if dw < 0.0 {
            card_width += dw.abs();
            black_box_width += dw.abs();
        }
        if dh < 0.0 {
            card_height += dh.abs();
        }
    }
    let black_box_height = card_height - 2.0 * padding;

    // Farger
    let bg_color = "#EEEEEE";
    let black_box_color = "black";
    let text_color_light = "#F0F0F0"; // For tekst på svart bakgrunn
    let text_color_purple = "#C5A0FF"; // For "API demo"
    let text_color_dark = "#333333"; // For vertikal label

    // Skrifttyper
    let font_family_mono = "IBM Plex Mono, 'Courier New', monospace";

    // --- 2. Initialiser SVG og tegn bakgrunner ---
    let mut svg = String::new();
    let _ = write!(svg, r#"<g><g transform="scale({scale}, {scale})">"#);

    // Grå bakgrunn
    let _ = write!(
        svg,
        r#"<rect x="0" y="0" width="{}" height="{}" fill="{}"/>"#,
        card_width, card_height, bg_color
    );

    // Svart, avrundet boks
    let _ = write!(
        svg,
        r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="{}"/>"#,
        black_box_x, black_box_y, black_box_width, black_box_height, corner_radius, black_box_color
    );

    // --- 3. Tegn vertikal label ---
    // Finner midtpunktet av det grå området til høyre
    let label_x = card_width - padding;
    let label_y = padding;
    let _ = write!(
        svg,
        r#"<text transform="rotate(90, {label_x}, {label_y})" text-anchor="start" dominant-baseline="hanging" font-family="{font_family_mono}" font-size="{vertical_label_font_size}" font-weight="bold" fill="{text_color_dark}">"#
    );

    // Splitter label på mellomrom for å stable ord (f.eks. "Release" og "Notes")
    let parts: Vec<String> = vertical_title.to_string().split("\n").map(|s| s.to_string()).collect();
    let line_height = vertical_label_font_size;
    for part in parts.iter() {
        let _ = write!(svg, r#"<tspan x="{label_x}" dy="{line_height}">{part}</tspan>"#);
    }
    let _ = write!(svg, "</text>");

    // --- 4. Statisk tekst inni svart boks (fra bildekontekst) ---
    let text_x_base = black_box_x + 20.0;
    let text_y_base = black_box_y + 30.0;

    // Blackbox title
    let _ = write!(
        svg,
        r#"<text x="{text_x_base}" y="{text_y_base}" font-family="{font_family_mono}" font-size="{title_font_size}" fill="{text_color_light}">{blackbox_title}</text>"#,
    );

    // Attribute title og "↓"
    let list_line_height = 14.0;
    let list_start_y = (black_box_y + black_box_height) - (list_line_height * (2.0 + attributes.len() as f32));

    let _ = write!(
        svg,
        r#"<text x="{text_x_base}" y="{list_start_y}" font-family="{font_family_mono}" font-size="{attribute_title_font_size}" fill="{text_color_light}">{attribute_title}</text>"#
    );

    // --- 5. Dynamisk 'attributes'-liste ---
    let mut current_y = list_start_y + list_line_height / 2.0; // Start Y-posisjon etter "↓"
    let list_item_x = text_x_base; // Grunnlinje for innrykk
    let prompt_x = list_item_x + 5.0; // x-pos for ">"
    let text_x = prompt_x + 15.0; // x-pos for list item
    for (attr, mark) in attributes.iter() {
        current_y += list_line_height; // Flytt ned for neste linje

        // Sjekk for spesiell styling fra bildet
        let (fill_color, prompt) = if *mark { (text_color_purple, ">") } else { (text_color_light, "•") };

        // Bruk <tspan> for å justere "prompt" (>) og selve teksten
        let _ = write!(
            svg,
            r#"<text y="{}" font-family="{}" font-size="{}" fill="{}" xml:space="preserve">"#,
            current_y, font_family_mono, attributes_font_size, fill_color
        );

        if !prompt.is_empty() {
            let _ = write!(svg, r#"<tspan x="{}">{}</tspan>"#, prompt_x, prompt);
        }
        let _ = write!(svg, r#"<tspan x="{}">{}</tspan>"#, text_x, attr);

        let _ = write!(svg, "</text>");
    }
    let _ = write!(svg, "</g>");

    // ... 6. Plasser objekt
    if let Some((object_svg, width, height)) = object {
        let cx = scale * ((black_box_x + 0.6 * black_box_width) - ((width / scale) / 2.0));
        let cy = scale * ((black_box_y + 0.45 * black_box_height) - ((height / scale) / 2.0));
        let _ = write!(svg, r#"<g transform="translate({cx}, {cy})">{object_svg}</g>"#);
    }

    // return
    let _ = write!(svg, "</g>");
    svg
}
