use super::*;

mod content;

#[test]
fn test_centerline_parse_content_for_reverse_dependencies() {
    // Given
    let content = content::CENTERLINE;

    // When
    let reverse_dependencies = ReverseDependency::from(content);

    // Then
    insta::assert_debug_snapshot!(
        "test_centerline_parse_content_for_reverse_dependencies",
        reverse_dependencies
    );
}

#[test]
fn test_tesseract_sys_parse_content_for_reverse_dependencies() {
    // Given
    let content = content::TESSERACT_SYS;

    // When
    let reverse_dependencies = ReverseDependency::from(content);

    // Then
    insta::assert_debug_snapshot!(
        "test_tesseract_sys_parse_content_for_reverse_dependencies",
        reverse_dependencies
    );
}
