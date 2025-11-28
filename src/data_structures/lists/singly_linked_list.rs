pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let Node { elem, next } = *boxed_node;
            self.head = next;
            elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        let x = self.head.as_deref();
        if let Some(val) = x {
            return Some(&val.elem);
        } else {
            return None;
        }
        // match &self.head {
        //     None => None,
        //     Some(val) => {
        //         let Node { elem, .. } = val.deref();
        //         return Some(elem);
        //     }
        // }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();

        while let Some(mut boxed_node) = cur {
            // here take the next pointer out before dropping boxed_node.
            cur = boxed_node.next.take();
            // loop continues: next node will be dropped in next iteration
        }
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(val) => {
                self.next = val.next.as_deref();
                return Some(&val.elem);
            }
        }
    }
}

