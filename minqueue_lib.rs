#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    main: VecDeque<T>,
    min: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            main: VecDeque::new(),
            min: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.main.push_back(val.clone());
        while let Some(back) = self.min.back() {
            if *back > val {
                self.min.pop_back();
            } else {
                break;
            }
        }
        self.min.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(front) = self.main.pop_front() {
            if Some(&front) == self.min.front() {
                self.min.pop_front();
            }
            Some(front)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.main.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.min.front()
    }

    pub fn len(&self) -> usize {
        self.main.len()
    }

    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }
}
