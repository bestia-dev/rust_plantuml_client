// rust_plantuml_client/src/plantuml_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

// use crate::LibraryError;

pub fn get_svg(plant_uml_code: &str) -> String {
    let base_url = "https://plantuml.com/plantuml";
    let url_parameter = compress_plant_uml_code(plant_uml_code);
    let url = format!("{}/svg/{}", base_url, url_parameter);
    // use reqwest to GET from plantuml.com server
    let resp = reqwest::blocking::get(&url)
        .unwrap()
        .text_with_charset("utf-8")
        .unwrap();
    // return
    resp
}

/// deflate and strange base64, that is Url_safe
pub fn compress_plant_uml_code(plant_uml_code: &str) -> String {
    // first deflate
    let data = plant_uml_code.as_bytes();
    let compressed = deflate::deflate_bytes(data);
    // then the strange base64
    // https://plantuml.com/text-encoding
    let my_cfg = radix64::CustomConfig::with_alphabet(
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_",
    )
    .no_padding()
    .build()
    .unwrap();
    let b64 = my_cfg.encode(&compressed);
    // return
    b64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_compress_plant_uml_code() {
        // http://www.plantuml.com/plantuml/uml/SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0
        assert_eq!(
            compress_plant_uml_code(
                r#"@startuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response
@enduml"#
            ),
            "SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0"
        );
    }
}
