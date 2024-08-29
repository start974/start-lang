use super::error::ErrorsResult;

use tree_sitter::{Node, TreeCursor};

pub struct IterNode<'a, T, U> {
    cursor: Option<TreeCursor<'a>>,
    acc: T,
    res: ErrorsResult<U>,
}

impl<'a, T> IterNode<'a, T, ()> {}

impl<'a, T1, U1> IterNode<'a, T1, U1> {
    // apply [f1] to the current node and [f2] to combine result
    pub fn next<F1, F2, T2, U2, U3>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T2, U3>
    where
        F1: FnMut(T1, &Node<'a>) -> (T2, ErrorsResult<U2>),
        F2: FnMut(U1, U2) -> U3,
    {
        match self.cursor {
            Some(mut actual_cursor) => {
                let node = actual_cursor.node();
                let (acc, res) = f1(self.acc, &node);
                let cursor = if actual_cursor.goto_first_child() {
                    Some(actual_cursor)
                } else {
                    None
                };
                match (self.res, res) {
                    (Ok(x), Ok(y)) => IterNode {
                        cursor,
                        acc,
                        res: Ok(f2(x, y)),
                    },
                    (Ok(_), Err(errors)) | (Err(errors), Ok(_)) => IterNode {
                        cursor: None,
                        acc,
                        res: Err(errors),
                    },
                    (Err(errors1), Err(errors2)) => IterNode {
                        cursor: None,
                        acc,
                        res: Err(errors1.concat(errors2)),
                    },
                }
            }
            None => panic!("Node has no children"),
        }
    }
}

impl<'a, T, U> IterNode<'a, T, U> {
    pub fn new(node: &Node<'a>, acc: T, init: U) -> Self {
        let mut cursor = node.walk();
        let opt_cursor = if cursor.goto_first_child() {
            Some(cursor)
        } else {
            None
        };

        Self {
            cursor: opt_cursor,
            acc,
            res: Ok(init),
        }
    }
    // apply [f1] to the until nodes and [f2] to combine results
    pub fn repeat<F1, F2, U2>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T, U>
    where
        F1: FnMut(T, &Node<'a>) -> (T, ErrorsResult<U2>),
        F2: FnMut(U, U2) -> U,
    {
        if self.cursor.is_none() {
            self
        } else {
            self.next(f1, f2).repeat(f1, f2)
        }
    }

    /// map result
    pub fn map_result<U2>(self, f: impl FnOnce(U) -> U2) -> IterNode<'a, T, U2> {
        IterNode {
            cursor: self.cursor,
            acc: self.acc,
            res: self.res.map(f),
        }
    }

    /// map accumulator
    pub fn map_acc<T2>(self, f: impl FnOnce(T) -> T2) -> IterNode<'a, T2, U> {
        IterNode {
            cursor: self.cursor,
            acc: f(self.acc),
            res: self.res,
        }
    }

    // get the result
    pub fn result(self) -> ErrorsResult<U> {
        self.res
    }

    // get the accumulator
    pub fn acc(self) -> T {
        self.acc
    }

    // get result and accumulator
    pub fn acc_result(self) -> (T, ErrorsResult<U>) {
        (self.acc, self.res)
    }
}
