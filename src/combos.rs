/// A selected combination of a Pipeline, a BindGroupLayout, and a BindGroup
/// using the mapping labels to identify each one.
pub struct RenderCombo {
    name: &str,
    pipeline: &str,
    bglayout: &str,
    bindgroup: &str,
}