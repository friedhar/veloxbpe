pub struct SmallString {
    inner: [char; 8],
    length: usize,
}
