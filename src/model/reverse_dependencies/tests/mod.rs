use super::*;

mod content;

#[test]
fn test_centerline_parse_content_for_reverse_dependencies() {
    // Given
    let content = content::CENTERLINE.clone();

    // When
    let reverse_dependencies = parse_content_for_reverse_dependencies(content);

    // Then
    insta::assert_debug_snapshot!(
        "test_centerline_parse_content_for_reverse_dependencies",
        reverse_dependencies
    );
}

#[test]
fn test_tesseract_sys_parse_content_for_reverse_dependencies() {
    // Given
    let content = content::TESSERACT_SYS.clone();

    // When
    let reverse_dependencies = parse_content_for_reverse_dependencies(content);

    // Then
    insta::assert_debug_snapshot!(
        "test_tesseract_sys_parse_content_for_reverse_dependencies",
        reverse_dependencies
    );
}
