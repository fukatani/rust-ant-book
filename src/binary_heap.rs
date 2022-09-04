
impl<T, F> BinaryHeap<T, F>
where
   F: Fn(&T, &T) -> bool,
{
   pub fn new(cond: F) -> Self {
       BinaryHeap {
           data: vec![],
           cond: cond,
       }
   }
   pub fn push(&mut self, v: T) {
       let pos = self.data.len();
       self.data.push(v);
       self.up_heap(pos);
   }
   pub fn pop(&mut self) -> Option<T> {
       self.data.pop().map(|mut item| {
           if !self.is_empty() {
               std::mem::swap(&mut item, &mut self.data[0]);
               self.down_heap();
           }
           item
       })
   }
   fn up_heap(&mut self, mut pos: usize) {
       let data = &mut self.data;
       while pos > 0 {
           let parent = (pos - 1) / 2;
           if !(self.cond)(&data[pos], &data[parent]) {
               break;
           }
           data.swap(pos, parent);
           pos = parent;
       }
   }
   fn down_heap(&mut self) {
       let data = &mut self.data;
       let mut pos = 0;
       let mut child = 1;
       while child + 1 < data.len() {
           if (self.cond)(&data[child + 1], &data[child]) {
               child += 1;
           }
           data.swap(child, pos);
           pos = child;
           child = 2 * child + 1;
       }
       if child < data.len() {
           data.swap(pos, child);
           pos = child;
       }
       self.up_heap(pos);
   }
}
