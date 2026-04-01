use serde::Serialize;
use std::collections::BTreeMap;
use ttf_parser::{name_id, Face, GlyphId, PlatformId, Tag};
use wasm_minimal_protocol::*;

initiate_protocol!();

const SAMPLE_RASTER_PPEM: [u16; 4] = [16, 32, 64, u16::MAX];
#[derive(Serialize, Clone)]
struct GlyphCategory {
    name: String,
    chars: String,
}

#[derive(Serialize, Clone)]
struct RectMeta {
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    width: i16,
    height: i16,
}

#[derive(Serialize, Clone)]
struct LineMetricsMeta {
    position: i16,
    thickness: i16,
}

#[derive(Serialize, Clone)]
struct ScriptMetricsMeta {
    x_size: i16,
    y_size: i16,
    x_offset: i16,
    y_offset: i16,
}

#[derive(Serialize, Clone)]
struct FontMetrics {
    units_per_em: u16,
    ascender: i16,
    descender: i16,
    height: i16,
    line_gap: i16,
    typographic_ascender: Option<i16>,
    typographic_descender: Option<i16>,
    typographic_line_gap: Option<i16>,
    vertical_ascender: Option<i16>,
    vertical_descender: Option<i16>,
    vertical_height: Option<i16>,
    vertical_line_gap: Option<i16>,
    x_height: Option<i16>,
    cap_height: Option<i16>,
    bbox: RectMeta,
    italic_angle: Option<f32>,
    underline: Option<LineMetricsMeta>,
    strikeout: Option<LineMetricsMeta>,
    subscript: Option<ScriptMetricsMeta>,
    superscript: Option<ScriptMetricsMeta>,
}

#[derive(Serialize, Clone)]
struct PermissionsMeta {
    level: Option<String>,
    is_subsetting_allowed: bool,
    is_bitmap_embedding_allowed: bool,
}

#[derive(Serialize, Clone)]
struct NameRecordMeta {
    platform_id: String,
    encoding_id: u16,
    language_id: u16,
    language: String,
    name_id: u16,
    name_id_label: String,
    is_unicode: bool,
    value: Option<String>,
    raw_length: usize,
}

#[derive(Serialize, Clone)]
struct NameTableMeta {
    records: Vec<NameRecordMeta>,
    selected: BTreeMap<String, String>,
}

#[derive(Serialize, Clone)]
struct VariationAxisMeta {
    tag: String,
    name: Option<String>,
    name_id: u16,
    min_value: f32,
    default_value: f32,
    max_value: f32,
    hidden: bool,
}

#[derive(Serialize, Clone)]
struct VariationMeta {
    axes: Vec<VariationAxisMeta>,
    coordinates: Vec<i16>,
    has_non_default_coordinates: bool,
}

#[derive(Serialize, Clone)]
struct RasterImageMeta {
    pixels_per_em: u16,
    format: String,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    data_length: usize,
}

#[derive(Serialize, Clone)]
struct SvgImageMeta {
    start_glyph_id: u16,
    end_glyph_id: u16,
    data_length: usize,
    compressed: bool,
}

#[derive(Serialize, Clone)]
struct GlyphMeta {
    id: u16,
    name: Option<String>,
    horizontal_advance: Option<u16>,
    vertical_advance: Option<u16>,
    horizontal_side_bearing: Option<i16>,
    vertical_side_bearing: Option<i16>,
    y_origin: Option<i16>,
    bounding_box: Option<RectMeta>,
    raster_image: Option<RasterImageMeta>,
    svg_image: Option<SvgImageMeta>,
    is_color_glyph: bool,
}

#[derive(Serialize, Clone)]
struct CodepointGlyphMeta {
    char: String,
    codepoint: u32,
    glyph_id: u16,
    glyph_name: Option<String>,
    variation_selector: Option<String>,
    variation_glyph_id: Option<u16>,
}

#[derive(Serialize, Clone)]
struct CollectionFaceMeta {
    index: u32,
    name: String,
    postscript_name: Option<String>,
    style: String,
    weight: u16,
    width: u16,
    number_of_glyphs: u16,
    is_variable: bool,
}

#[derive(Serialize, Clone)]
struct CapabilitiesMeta {
    is_regular: bool,
    is_italic: bool,
    is_oblique: bool,
    is_bold: bool,
    is_monospaced: bool,
    is_variable: bool,
    has_raster_images: bool,
    has_svg_images: bool,
    has_color_glyphs: bool,
    color_palettes: Option<u16>,
}

#[derive(Serialize, Clone)]
struct FontMeta {
    name: String,
    family: Option<String>,
    typographic_family: Option<String>,
    subfamily: Option<String>,
    typographic_subfamily: Option<String>,
    full_name: Option<String>,
    unique_id: Option<String>,
    postscript_name: Option<String>,
    postscript_name_prefix: Option<String>,
    compatible_full_name: Option<String>,
    wws_family: Option<String>,
    wws_subfamily: Option<String>,
    light_background_palette_name: Option<String>,
    dark_background_palette_name: Option<String>,
    #[serde(rename = "type")]
    font_type: String,
    style: String,
    weight: u16,
    width: u16,
    author: Option<String>,
    designer_url: Option<String>,
    manufacturer: Option<String>,
    vendor_url: Option<String>,
    version: Option<String>,
    description: Option<String>,
    sample_text: Option<String>,
    trademark: Option<String>,
    license: Option<String>,
    copyright: Option<String>,
    license_url: Option<String>,
    permissions: PermissionsMeta,
    styles_count: u32,
    collection_faces: Option<Vec<CollectionFaceMeta>>,
    number_of_glyphs: u16,
    metrics: FontMetrics,
    capabilities: CapabilitiesMeta,
    variations: VariationMeta,
    tables_present: Vec<String>,
    table_sizes: BTreeMap<String, usize>,
    name_table: NameTableMeta,
    glyphs: Option<Vec<GlyphCategory>>,
    codepoint_samples: Vec<CodepointGlyphMeta>,
    glyph_name_index: BTreeMap<String, u16>,
    glyph_details: Vec<GlyphMeta>,
}

fn rect_meta(rect: ttf_parser::Rect) -> RectMeta {
    RectMeta {
        x_min: rect.x_min,
        y_min: rect.y_min,
        x_max: rect.x_max,
        y_max: rect.y_max,
        width: rect.width(),
        height: rect.height(),
    }
}

fn line_metrics_meta(metrics: ttf_parser::LineMetrics) -> LineMetricsMeta {
    LineMetricsMeta {
        position: metrics.position,
        thickness: metrics.thickness,
    }
}

fn script_metrics_meta(metrics: ttf_parser::ScriptMetrics) -> ScriptMetricsMeta {
    ScriptMetricsMeta {
        x_size: metrics.x_size,
        y_size: metrics.y_size,
        x_offset: metrics.x_offset,
        y_offset: metrics.y_offset,
    }
}

fn tag_to_string(tag: Tag) -> String {
    let bytes = tag.to_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn platform_to_string(platform: PlatformId) -> String {
    match platform {
        PlatformId::Unicode => "Unicode",
        PlatformId::Macintosh => "Macintosh",
        PlatformId::Iso => "ISO",
        PlatformId::Windows => "Windows",
        PlatformId::Custom => "Custom",
    }
    .to_string()
}

fn permissions_to_string(permissions: ttf_parser::Permissions) -> String {
    match permissions {
        ttf_parser::Permissions::Installable => "installable",
        ttf_parser::Permissions::Restricted => "restricted",
        ttf_parser::Permissions::PreviewAndPrint => "preview-and-print",
        ttf_parser::Permissions::Editable => "editable",
    }
    .to_string()
}

fn raster_format_to_string(format: ttf_parser::RasterImageFormat) -> String {
    match format {
        ttf_parser::RasterImageFormat::PNG => "png",
        ttf_parser::RasterImageFormat::BitmapMono => "bitmap-mono",
        ttf_parser::RasterImageFormat::BitmapMonoPacked => "bitmap-mono-packed",
        ttf_parser::RasterImageFormat::BitmapGray2 => "bitmap-gray-2",
        ttf_parser::RasterImageFormat::BitmapGray2Packed => "bitmap-gray-2-packed",
        ttf_parser::RasterImageFormat::BitmapGray4 => "bitmap-gray-4",
        ttf_parser::RasterImageFormat::BitmapGray4Packed => "bitmap-gray-4-packed",
        ttf_parser::RasterImageFormat::BitmapGray8 => "bitmap-gray-8",
        ttf_parser::RasterImageFormat::BitmapPremulBgra32 => "bitmap-premul-bgra32",
    }
    .to_string()
}

fn name_id_label(id: u16) -> String {
    match id {
        name_id::COPYRIGHT_NOTICE => "copyright-notice",
        name_id::FAMILY => "family",
        name_id::SUBFAMILY => "subfamily",
        name_id::UNIQUE_ID => "unique-id",
        name_id::FULL_NAME => "full-name",
        name_id::VERSION => "version",
        name_id::POST_SCRIPT_NAME => "postscript-name",
        name_id::TRADEMARK => "trademark",
        name_id::MANUFACTURER => "manufacturer",
        name_id::DESIGNER => "designer",
        name_id::DESCRIPTION => "description",
        name_id::VENDOR_URL => "vendor-url",
        name_id::DESIGNER_URL => "designer-url",
        name_id::LICENSE => "license",
        name_id::LICENSE_URL => "license-url",
        name_id::TYPOGRAPHIC_FAMILY => "typographic-family",
        name_id::TYPOGRAPHIC_SUBFAMILY => "typographic-subfamily",
        name_id::COMPATIBLE_FULL => "compatible-full",
        name_id::SAMPLE_TEXT => "sample-text",
        name_id::POST_SCRIPT_CID => "postscript-cid",
        name_id::WWS_FAMILY => "wws-family",
        name_id::WWS_SUBFAMILY => "wws-subfamily",
        name_id::LIGHT_BACKGROUND_PALETTE => "light-background-palette",
        name_id::DARK_BACKGROUND_PALETTE => "dark-background-palette",
        name_id::VARIATIONS_POST_SCRIPT_NAME_PREFIX => "variations-postscript-name-prefix",
        _ => return format!("name-id-{}", id),
    }
    .to_string()
}

fn is_svgz(data: &[u8]) -> bool {
    data.len() >= 2 && data[0] == 0x1f && data[1] == 0x8b
}

fn clean_optional_string(value: Option<String>) -> Option<String> {
    value.and_then(|v| {
        let trimmed = v.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn normalize_paragraph_string(value: Option<String>) -> Option<String> {
    value.and_then(|v| {
        let normalized = v
            .split_whitespace()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    })
}

fn collect_names(face: &Face) -> NameTableMeta {
    let mut records = Vec::new();
    let mut selected = BTreeMap::new();

    for name in face.names() {
        let value = clean_optional_string(name.to_string());
        let label = name_id_label(name.name_id);
        if let Some(ref decoded) = value {
            selected.entry(label.clone()).or_insert_with(|| decoded.clone());
        }

        records.push(NameRecordMeta {
            platform_id: platform_to_string(name.platform_id),
            encoding_id: name.encoding_id,
            language_id: name.language_id,
            language: format!("{:?}", name.language()),
            name_id: name.name_id,
            name_id_label: label,
            is_unicode: name.is_unicode(),
            value,
            raw_length: name.name.len(),
        });
    }

    NameTableMeta { records, selected }
}

fn preferred_name(face: &Face, id: u16) -> Option<String> {
    let mut fallback_name = None;
    for name in face.names() {
        if name.name_id == id && name.is_unicode() {
            if let Some(decoded) = clean_optional_string(name.to_string()) {
                if name.platform_id == PlatformId::Windows
                    && name.encoding_id == 1
                    && name.language_id == 1033
                {
                    return Some(decoded);
                }
                if fallback_name.is_none() {
                    fallback_name = Some(decoded);
                }
            }
        }
    }
    fallback_name
}

fn all_name_fields(face: &Face) -> BTreeMap<&'static str, Option<String>> {
    let mut fields = BTreeMap::new();
    fields.insert("family", preferred_name(face, name_id::FAMILY));
    fields.insert("subfamily", preferred_name(face, name_id::SUBFAMILY));
    fields.insert("unique_id", preferred_name(face, name_id::UNIQUE_ID));
    fields.insert("full_name", preferred_name(face, name_id::FULL_NAME));
    fields.insert("version", preferred_name(face, name_id::VERSION));
    fields.insert("postscript_name", preferred_name(face, name_id::POST_SCRIPT_NAME));
    fields.insert("trademark", preferred_name(face, name_id::TRADEMARK));
    fields.insert("manufacturer", preferred_name(face, name_id::MANUFACTURER));
    fields.insert("author", preferred_name(face, name_id::DESIGNER));
    fields.insert(
        "description",
        normalize_paragraph_string(preferred_name(face, name_id::DESCRIPTION)),
    );
    fields.insert("vendor_url", preferred_name(face, name_id::VENDOR_URL));
    fields.insert("designer_url", preferred_name(face, name_id::DESIGNER_URL));
    fields.insert("license", preferred_name(face, name_id::LICENSE));
    fields.insert("license_url", preferred_name(face, name_id::LICENSE_URL));
    fields.insert(
        "typographic_family",
        preferred_name(face, name_id::TYPOGRAPHIC_FAMILY),
    );
    fields.insert(
        "typographic_subfamily",
        preferred_name(face, name_id::TYPOGRAPHIC_SUBFAMILY),
    );
    fields.insert(
        "compatible_full_name",
        preferred_name(face, name_id::COMPATIBLE_FULL),
    );
    fields.insert("sample_text", preferred_name(face, name_id::SAMPLE_TEXT));
    fields.insert("wws_family", preferred_name(face, name_id::WWS_FAMILY));
    fields.insert("wws_subfamily", preferred_name(face, name_id::WWS_SUBFAMILY));
    fields.insert(
        "light_background_palette_name",
        preferred_name(face, name_id::LIGHT_BACKGROUND_PALETTE),
    );
    fields.insert(
        "dark_background_palette_name",
        preferred_name(face, name_id::DARK_BACKGROUND_PALETTE),
    );
    fields.insert(
        "postscript_name_prefix",
        preferred_name(face, name_id::VARIATIONS_POST_SCRIPT_NAME_PREFIX),
    );
    fields.insert("copyright", preferred_name(face, name_id::COPYRIGHT_NOTICE));
    fields
}

fn collect_table_map(face: &Face) -> BTreeMap<String, usize> {
    let candidate_tags = [
        b"avar", b"ankr", b"bdat", b"bloc", b"CBDT", b"CBLC", b"CFF ", b"CFF2", b"cmap",
        b"COLR", b"CPAL", b"EBDT", b"EBLC", b"feat", b"fvar", b"gdef", b"glyf", b"GPOS",
        b"GSUB", b"gvar", b"head", b"hhea", b"hmtx", b"HVAR", b"kern", b"kerx", b"loca",
        b"math", b"maxp", b"morx", b"MVAR", b"name", b"OS/2", b"post", b"sbix", b"SVG ",
        b"trak", b"vhea", b"vmtx", b"VORG", b"VVAR",
    ];

    let raw_face = face.raw_face();
    let mut table_map = BTreeMap::new();
    for bytes in candidate_tags {
        let tag = Tag::from_bytes(bytes);
        if let Some(data) = raw_face.table(tag) {
            table_map.insert(tag_to_string(tag), data.len());
        }
    }
    table_map
}

fn collect_tables_present(face: &Face) -> Vec<String> {
    collect_table_map(face).into_keys().collect()
}

fn collect_table_sizes(face: &Face) -> BTreeMap<String, usize> {
    collect_table_map(face)
}

fn collect_variations(face: &mut Face) -> VariationMeta {
    let axes: Vec<VariationAxisMeta> = face
        .variation_axes()
        .into_iter()
        .map(|axis| {
            let name = preferred_name(face, axis.name_id);
            let _ = face.set_variation(axis.tag, axis.def_value);
            VariationAxisMeta {
                tag: tag_to_string(axis.tag),
                name,
                name_id: axis.name_id,
                min_value: axis.min_value,
                default_value: axis.def_value,
                max_value: axis.max_value,
                hidden: axis.hidden,
            }
        })
        .collect();

    VariationMeta {
        axes,
        coordinates: face.variation_coordinates().iter().map(|c| c.get()).collect(),
        has_non_default_coordinates: face.has_non_default_variation_coordinates(),
    }
}

fn collect_permissions(face: &Face) -> PermissionsMeta {
    PermissionsMeta {
        level: face.permissions().map(permissions_to_string),
        is_subsetting_allowed: face.is_subsetting_allowed(),
        is_bitmap_embedding_allowed: face.is_bitmap_embedding_allowed(),
    }
}

fn collect_glyph_categories(face: &Face) -> Option<Vec<GlyphCategory>> {
    let categories = [
        ("Numbers", 0x0030..=0x0039),
        ("Latin Uppercase", 0x0041..=0x005A),
        ("Latin Lowercase", 0x0061..=0x007A),
        ("Cyrillic", 0x0400..=0x04FF),
        ("Hiragana", 0x3040..=0x309F),
        ("Katakana", 0x30A0..=0x30FF),
        ("Han Ideograms", 0x4E00..=0x9FFF),
    ];

    let mut extracted_glyphs = Vec::with_capacity(categories.len());
    for (cat_name, range) in categories {
        let mut chars = String::with_capacity(200);
        let mut count = 0;
        for cp in range {
            if let Some(c) = char::from_u32(cp) {
                if face.glyph_index(c).is_some() {
                    chars.push(c);
                    chars.push(' ');
                    count += 1;
                    if count >= 100 {
                        break;
                    }
                }
            }
        }
        if count > 0 {
            chars.pop();
            extracted_glyphs.push(GlyphCategory {
                name: cat_name.to_string(),
                chars,
            });
        }
    }

    if extracted_glyphs.is_empty() {
        None
    } else {
        Some(extracted_glyphs)
    }
}

fn sample_variation_selectors() -> [char; 4] {
    ['\u{FE00}', '\u{FE0E}', '\u{FE0F}', '\u{E0100}']
}

fn collect_codepoint_samples(face: &Face) -> Vec<CodepointGlyphMeta> {
    let categories = [
        (0x0030..=0x0039),
        (0x0041..=0x005A),
        (0x0061..=0x007A),
        (0x00A0..=0x00FF),
        (0x0100..=0x017F),
        (0x0370..=0x03FF),
        (0x0400..=0x04FF),
        (0x2010..=0x2040),
        (0x2190..=0x21FF),
        (0x2460..=0x24FF),
        (0x3040..=0x309F),
        (0x30A0..=0x30FF),
        (0x4E00..=0x9FFF),
    ];

    let variation_selectors = sample_variation_selectors();
    let mut result = Vec::new();
    for range in categories {
        for cp in range {
            if let Some(ch) = char::from_u32(cp) {
                if let Some(glyph_id) = face.glyph_index(ch) {
                    let mut variation_selector = None;
                    let mut variation_glyph_id = None;
                    for selector in variation_selectors {
                        if let Some(var_gid) = face.glyph_variation_index(ch, selector) {
                            variation_selector = Some(selector.to_string());
                            variation_glyph_id = Some(var_gid.0);
                            break;
                        }
                    }
                    result.push(CodepointGlyphMeta {
                        char: ch.to_string(),
                        codepoint: cp,
                        glyph_id: glyph_id.0,
                        glyph_name: face.glyph_name(glyph_id).map(str::to_string),
                        variation_selector,
                        variation_glyph_id,
                    });
                }
            }
            if result.len() >= 256 {
                return result;
            }
        }
    }
    result
}

fn pick_raster_image(face: &Face, glyph_id: GlyphId) -> Option<RasterImageMeta> {
    for ppem in SAMPLE_RASTER_PPEM {
        if let Some(image) = face.glyph_raster_image(glyph_id, ppem) {
            return Some(RasterImageMeta {
                pixels_per_em: image.pixels_per_em,
                format: raster_format_to_string(image.format),
                x: image.x,
                y: image.y,
                width: image.width,
                height: image.height,
                data_length: image.data.len(),
            });
        }
    }
    None
}

fn collect_glyph_name_index(face: &Face) -> BTreeMap<String, u16> {
    let glyph_count = face.number_of_glyphs();
    let mut index = BTreeMap::new();
    for id in 0..glyph_count {
        let glyph_id = GlyphId(id);
        if let Some(name) = face.glyph_name(glyph_id) {
            index.insert(name.to_string(), id);
        }
    }
    index
}

fn collect_glyph_details(face: &Face) -> Vec<GlyphMeta> {
    let glyph_count = face.number_of_glyphs();
    let mut details = Vec::with_capacity(glyph_count as usize);
    for id in 0..glyph_count {
        let glyph_id = GlyphId(id);
        let svg_image = face.glyph_svg_image(glyph_id).map(|svg| SvgImageMeta {
            start_glyph_id: svg.start_glyph_id.0,
            end_glyph_id: svg.end_glyph_id.0,
            data_length: svg.data.len(),
            compressed: is_svgz(svg.data),
        });
        details.push(GlyphMeta {
            id,
            name: face.glyph_name(glyph_id).map(str::to_string),
            horizontal_advance: face.glyph_hor_advance(glyph_id),
            vertical_advance: face.glyph_ver_advance(glyph_id),
            horizontal_side_bearing: face.glyph_hor_side_bearing(glyph_id),
            vertical_side_bearing: face.glyph_ver_side_bearing(glyph_id),
            y_origin: face.glyph_y_origin(glyph_id),
            bounding_box: face.glyph_bounding_box(glyph_id).map(rect_meta),
            raster_image: pick_raster_image(face, glyph_id),
            svg_image,
            is_color_glyph: face.is_color_glyph(glyph_id),
        });
    }
    details
}

fn collect_collection_faces(font_data: &[u8], styles_count: u32) -> Option<Vec<CollectionFaceMeta>> {
    if styles_count <= 1 {
        return None;
    }

    let mut faces = Vec::with_capacity(styles_count as usize);
    for index in 0..styles_count {
        let face = match Face::parse(font_data, index) {
            Ok(face) => face,
            Err(_) => continue,
        };
        let name = preferred_name(&face, name_id::TYPOGRAPHIC_FAMILY)
            .or_else(|| preferred_name(&face, name_id::FAMILY))
            .unwrap_or_else(|| "Unknown Font".to_string());
        let postscript_name = preferred_name(&face, name_id::POST_SCRIPT_NAME);
        let style = if face.is_italic() {
            "italic"
        } else if face.is_oblique() {
            "oblique"
        } else {
            "normal"
        }
        .to_string();

        faces.push(CollectionFaceMeta {
            index,
            name,
            postscript_name,
            style,
            weight: face.weight().to_number(),
            width: face.width().to_number(),
            number_of_glyphs: face.number_of_glyphs(),
            is_variable: face.is_variable(),
        });
    }

    if faces.is_empty() {
        None
    } else {
        Some(faces)
    }
}

fn collect_capabilities(face: &Face, glyph_details: &[GlyphMeta]) -> CapabilitiesMeta {
    CapabilitiesMeta {
        is_regular: face.is_regular(),
        is_italic: face.is_italic(),
        is_oblique: face.is_oblique(),
        is_bold: face.is_bold(),
        is_monospaced: face.is_monospaced(),
        is_variable: face.is_variable(),
        has_raster_images: glyph_details.iter().any(|glyph| glyph.raster_image.is_some()),
        has_svg_images: glyph_details.iter().any(|glyph| glyph.svg_image.is_some()),
        has_color_glyphs: glyph_details.iter().any(|glyph| glyph.is_color_glyph),
        color_palettes: face.color_palettes().map(|count| count.get()),
    }
}

fn build_font_meta(font_data: &[u8], index: u32, styles_count: u32) -> Option<FontMeta> {
    let mut face = Face::parse(font_data, index).ok()?;
    let all_names = all_name_fields(&face);
    let name = all_names
        .get("typographic_family")
        .cloned()
        .flatten()
        .or_else(|| all_names.get("family").cloned().flatten())
        .unwrap_or_else(|| "Unknown Font".to_string());
    let postscript_name = all_names.get("postscript_name").cloned().flatten();
    let weight = face.weight().to_number();
    let width = face.width().to_number();
    let style = if face.is_italic() {
        "italic"
    } else if face.is_oblique() {
        "oblique"
    } else {
        "normal"
    }
    .to_string();
    let font_type = if face.is_variable() {
        "Variable"
    } else {
        "Static"
    }
    .to_string();

    let bbox = face.global_bounding_box();
    let metrics = FontMetrics {
        units_per_em: face.units_per_em(),
        ascender: face.ascender(),
        descender: face.descender(),
        height: face.height(),
        line_gap: face.line_gap(),
        typographic_ascender: face.typographic_ascender(),
        typographic_descender: face.typographic_descender(),
        typographic_line_gap: face.typographic_line_gap(),
        vertical_ascender: face.vertical_ascender(),
        vertical_descender: face.vertical_descender(),
        vertical_height: face.vertical_height(),
        vertical_line_gap: face.vertical_line_gap(),
        x_height: face.x_height(),
        cap_height: face.capital_height(),
        bbox: rect_meta(bbox),
        italic_angle: face.italic_angle(),
        underline: face.underline_metrics().map(line_metrics_meta),
        strikeout: face.strikeout_metrics().map(line_metrics_meta),
        subscript: face.subscript_metrics().map(script_metrics_meta),
        superscript: face.superscript_metrics().map(script_metrics_meta),
    };

    let glyphs = collect_glyph_categories(&face);
    let codepoint_samples = collect_codepoint_samples(&face);
    let glyph_name_index = collect_glyph_name_index(&face);
    let glyph_details = collect_glyph_details(&face);
    let capabilities = collect_capabilities(&face, &glyph_details);
    let variations = collect_variations(&mut face);
    let tables_present = collect_tables_present(&face);
    let table_sizes = collect_table_sizes(&face);
    let name_table = collect_names(&face);
    let permissions = collect_permissions(&face);
    let collection_faces = collect_collection_faces(font_data, styles_count);

    Some(FontMeta {
        name,
        family: all_names.get("family").cloned().flatten(),
        typographic_family: all_names.get("typographic_family").cloned().flatten(),
        subfamily: all_names.get("subfamily").cloned().flatten(),
        typographic_subfamily: all_names.get("typographic_subfamily").cloned().flatten(),
        full_name: all_names.get("full_name").cloned().flatten(),
        unique_id: all_names.get("unique_id").cloned().flatten(),
        postscript_name,
        postscript_name_prefix: all_names.get("postscript_name_prefix").cloned().flatten(),
        compatible_full_name: all_names.get("compatible_full_name").cloned().flatten(),
        wws_family: all_names.get("wws_family").cloned().flatten(),
        wws_subfamily: all_names.get("wws_subfamily").cloned().flatten(),
        light_background_palette_name: all_names
            .get("light_background_palette_name")
            .cloned()
            .flatten(),
        dark_background_palette_name: all_names
            .get("dark_background_palette_name")
            .cloned()
            .flatten(),
        font_type,
        style,
        weight,
        width,
        author: all_names.get("author").cloned().flatten(),
        designer_url: all_names.get("designer_url").cloned().flatten(),
        manufacturer: all_names.get("manufacturer").cloned().flatten(),
        vendor_url: all_names.get("vendor_url").cloned().flatten(),
        version: all_names.get("version").cloned().flatten(),
        description: all_names.get("description").cloned().flatten(),
        sample_text: all_names.get("sample_text").cloned().flatten(),
        trademark: all_names.get("trademark").cloned().flatten(),
        license: all_names.get("license").cloned().flatten(),
        copyright: all_names.get("copyright").cloned().flatten(),
        license_url: all_names.get("license_url").cloned().flatten(),
        permissions,
        styles_count,
        collection_faces,
        number_of_glyphs: face.number_of_glyphs(),
        metrics,
        capabilities,
        variations,
        tables_present,
        table_sizes,
        name_table,
        glyphs,
        codepoint_samples,
        glyph_name_index,
        glyph_details,
    })
}

#[wasm_func]
pub fn extract_metadata(font_data: &[u8]) -> Vec<u8> {
    let styles_count = ttf_parser::fonts_in_collection(font_data).unwrap_or(1);
    match build_font_meta(font_data, 0, styles_count) {
        Some(meta) => serde_json::to_vec(&meta).unwrap_or_else(|_| b"{}".to_vec()),
        None => b"{}".to_vec(),
    }
}
