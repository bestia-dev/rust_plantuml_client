// tests/integration_test.rs

#[test]
fn integration_test_01() {
    let svg_start = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" contentStyleType="text/css" height="120px" preserveAspectRatio="none" style="width:113px;height:120px;background:#FFFFFF;" version="1.1" viewBox="0 0 113 120" width="113px" zoomAndPan="magnify"><defs/><g><line style="stroke:#181818;stroke-width:0.5;stroke-dasharray:5.0,5.0;" x1="26" x2="26" y1="36.2969" y2="85.4297"/><line style="stroke:#181818;stroke-width:0.5;stroke-dasharray:5.0,5.0;" x1="82" x2="82" y1="36.2969" y2="85.4297"/><rect fill="#E2E2F0" h"##;
    assert_eq!(
        rust_plantuml_client::get_svg(
            "@startuml
    Bob -> Alice : hello
    @enduml"
        )[..50],
        svg_start[..50]
    );
}
