use crate::alloc::Layout;
use crate::marker::PointeeSized;

/// This trait is used to get the layout of a value-sized type.
///
/// # Safety
///
/// The layout must be the actual layout of the value.
#[unstable(feature = "value_sized", issue = "none")]
pub unsafe trait ValueLayout: PointeeSized {
    /// Returns the layout of the value.
    #[unstable(feature = "value_sized", issue = "none")]
    fn layout(&self) -> Layout;
}
