pub(super) struct Line(usize, Vec<u8>);

impl Line {
    pub(super) const fn new() -> Self {
        Line(0, Vec::new())
    }

    pub(super) fn insert(&mut self, c: u8) {
        self.1.insert(self.0, c);
        self.0 += 1;
    }

    // pub(super) fn delete(&mut self) {
    //     self.1.remove(self.0 - 1);
    //     self.0 -= 1;
    // }

    pub(super) fn backspace(&mut self) {
        self.1.remove(self.0 - 1);
        self.0 -= 1;
    }

    pub(super) fn left(&mut self) {
        self.0 -= 1;
    }

    pub(super) fn right(&mut self) {
        self.0 += 1;
    }

    pub(super) fn clear(&mut self) {
        self.0 = 0;
        self.1.clear();
    }

    pub(super) fn len(&self) -> usize {
        self.1.len()
    }

    pub(super) fn position(&self) -> usize {
        self.0
    }

    // pub(super) fn chars(&self) -> Vec<u8> {
    //     self.1.clone()
    // }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.1).to_string()
    }
}


