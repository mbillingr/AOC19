pub trait BackTracking {
    type PartialCandidate: std::fmt::Debug;

    fn output(&mut self, c: &Self::PartialCandidate);

    fn root(&self) -> Self::PartialCandidate;

    fn reject(&self, c: &Self::PartialCandidate) -> bool;
    fn accept(&self, c: &Self::PartialCandidate) -> bool;
    fn extend(
        &self,
        c: &Self::PartialCandidate,
    ) -> Box<dyn Iterator<Item = Self::PartialCandidate>>;

    fn backtrack(&mut self) {
        self.backtrack_recursive(&self.root())
    }

    fn backtrack_recursive(&mut self, c: &Self::PartialCandidate) {
        if self.reject(c) {
            return;
        }

        if self.accept(c) {
            self.output(c);
        }

        for s in self.extend(c) {
            self.backtrack_recursive(&s);
        }
    }
}
