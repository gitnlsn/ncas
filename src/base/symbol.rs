/**
 * Generical Data Holder for single value data
 */
#[derive(Debug, Clone)]
pub struct Symbol<T> {
    pub data: T,
}
