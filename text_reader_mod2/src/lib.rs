use aviutl2::{AnyResult, module::ScriptModuleFunctions};

#[aviutl2::plugin(ScriptModule)]
struct TextReader;

impl aviutl2::module::ScriptModule for TextReader {
    fn new(_info: aviutl2::AviUtl2Info) -> AnyResult<Self> {
        Ok(Self)
    }

    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: "TextReader v1.0.0 by Suzuke".into(),
            functions: Self::functions(),
        }
    }
}

#[aviutl2::module::functions]
impl TextReader {
    fn get_text(
        &self,
        layer: usize,
        frame: usize,
        section: &aviutl2::generic::ReadSection,
    ) -> AnyResult<String> {
        let info = read_text_object(layer, frame, section);
        Ok(info.text)
    }

    fn get_text_info(
        &self,
        layer: usize,
        frame: usize,
        section: &aviutl2::generic::ReadSection,
    ) -> AnyResult<(String, String, f64)> {
        let info = read_text_object(layer, frame, section);
        Ok((info.text, info.font, info.size))
    }
}

struct TextInfo {
    text: String,
    font: String,
    size: f64,
}

fn read_text_object(
    layer: usize,
    frame: usize,
    section: &aviutl2::generic::ReadSection,
) -> TextInfo {
    let obj = match section.find_object_after(layer, frame) {
        Ok(Some(o)) => o,
        _ => return TextInfo { text: String::new(), font: String::new(), size: 0.0 },
    };

    let text = read_effect_item(obj, section, &["文字列", "テキスト", "text", "Text"]);
    let font = read_effect_item(obj, section, &["フォント", "font", "Font"]);
    let size_str = read_effect_item(obj, section, &["サイズ", "size", "Size"]);
    let size: f64 = size_str.parse().unwrap_or(0.0);

    TextInfo { text, font, size }
}

fn read_effect_item(
    obj: aviutl2::generic::ObjectHandle,
    section: &aviutl2::generic::ReadSection,
    keys: &[&str],
) -> String {
    for key in keys {
        for ename in ["テキスト", "Text"] {
            if let Ok(val) = section.get_object_effect_item(obj, ename, 0, key) {
                let v = val.trim().to_string();
                if !v.is_empty() {
                    return v;
                }
            }
        }
    }
    String::new()
}

aviutl2::register_script_module!(TextReader);
