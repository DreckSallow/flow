pub struct After<C: FnMut()>(pub C);

impl<C: FnMut()> Drop for After<C> {
    fn drop(&mut self) {
        (self.0)()
    }
}
