use crate::*;

pub struct TagAllocator<'a> {
  pub unavailable: &'a [Range<i32>],
  pub next_tag: i32,
}

impl<'a> TagAllocator<'a> {
  pub fn new(unavailable: &'a [Range<i32>]) -> Self {
    Self {
      unavailable,
      next_tag: 1,
    }
  }

  pub fn tag_is_unavailable(&self, number: &i32) -> bool {
    let result = self.unavailable.binary_search_by(|range| {
      if range.contains(number) {
        Ordering::Equal
      } else if *number < range.start {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    });

    result.is_ok()
  }

  pub fn next_tag(&mut self) -> i32 {
    while self.tag_is_unavailable(&self.next_tag) {
      self.next_tag += 1;
    }

    let tag = self.next_tag;
    self.next_tag += 1;
    tag
  }

  pub fn get_or_next(&mut self, manual_tag: Option<i32>) -> i32 {
    if let Some(tag) = manual_tag {
      return tag;
    }
    self.next_tag()
  }
}
