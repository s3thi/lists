use std::ptr;

pub struct List<T> {
	head: Link<T>,
	tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None, tail: ptr::null_mut() }
	}

	pub fn push(&mut self, elem: T) {
		let mut new_tail = Box::new(Node {
			elem,
			next: None
		});

		let raw_tail: *mut _ = &mut *new_tail;

		if !self.tail.is_null() {
			unsafe {
				(*self.tail).next = Some(new_tail);
			}
		} else {
			self.head = Some(new_tail);
		}

		self.tail = raw_tail;
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|head| {
			let head = *head;
			self.head = head.next;

			if self.head.is_none() {
				self.tail = ptr::null_mut();
			}

			head.elem
		})
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| {
			&mut node.elem
		})
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter(&self) -> Iter<T> {
		Iter { next: self.head.as_ref().map(|node| &**node) }
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut { next: self.head.as_mut().map(|node| &mut **node) }
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut cur_link = self.head.take();
		while let Some(mut boxed_node) = cur_link {
			cur_link = boxed_node.next.take();
		}
	}
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
	next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
