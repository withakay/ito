use super::strip_ito_internal_comment_blocks;

#[test]
fn strip_ito_internal_comment_blocks_removes_internal_template_guidance() {
    let contents = r#"
Keep this.
<!-- ITO:INTERNAL:START -->
## Your Guidance
(placeholder)
<!-- ITO:INTERNAL:END -->
Keep this too.
"#;

    let stripped = strip_ito_internal_comment_blocks(contents);
    assert!(stripped.contains("Keep this."));
    assert!(stripped.contains("Keep this too."));
    assert!(!stripped.contains("## Your Guidance"));
    assert!(!stripped.contains("(placeholder)"));
}
