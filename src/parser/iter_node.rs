use super::super::error::Error;

use tree_sitter::{Node, TreeCursor};

pub struct IterNode<'a, T, U> {
    cursor: TreeCursor<'a>,
    acc: T,
    res: Result<U, Error>,
}

impl<'a, T> IterNode<'a, T, ()> {}

impl<'a, T, U1> IterNode<'a, T, U1> {
    /// new iterator
    pub fn new(node: &Node<'a>, acc: T, init: U1) -> Self {
        Self {
            cursor: node.walk(),
            acc,
            res: Ok(init),
        }
    }

    /// first child
    pub fn first_child(mut self) -> Self {
        let node = self.cursor.node();
        if self.cursor.goto_first_child() {
            self
        } else {
            panic!("No child for node {node}")
        }
    }

    /// next sibling
    pub fn next(mut self) -> Self {
        let node = self.cursor.node();
        if self.cursor.goto_next_sibling() {
            self
        } else {
            panic!("No sibling for node {node}")
        }
    }

    /// apply [f1] to the current node and [f2] to combine result
    pub fn apply<F1, F2, U2, U3>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T, U3>
    where
        F1: FnMut(T, &Node<'a>) -> (T, Result<U2, Error>),
        F2: FnMut(U1, U2) -> U3,
    {
        match self.res {
            Err(errors) => IterNode {
                cursor: self.cursor,
                acc: self.acc,
                res: Err(errors),
            },
            Ok(x) => {
                let node = self.cursor.node();
                let (acc, res) = f1(self.acc, &node);
                IterNode {
                    cursor: self.cursor,
                    acc,
                    res: res.map(|y| f2(x, y)),
                }
            }
        }
    }

    /// apply on next
    pub fn apply_next<F1, F2, U2, U3>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T, U3>
    where
        F1: FnMut(T, &Node<'a>) -> (T, Result<U2, Error>),
        F2: FnMut(U1, U2) -> U3,
    {
        let self2 = self.apply(f1, f2);
        if self2.res.is_ok() {
            self2.next()
        } else {
            self2
        }
    }

    // apply [f1] to the current node and [f2] to combine result
    // if error as occured in [f1] cusor not go to next
    pub fn apply_opt<F1, F2, U2, U3>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T, U3>
    where
        F1: FnMut(T, &Node<'a>) -> (T, Result<U2, Error>),
        F2: FnMut(U1, Option<U2>) -> U3,
    {
        match self.res {
            Err(errors) => IterNode {
                cursor: self.cursor,
                acc: self.acc,
                res: Err(errors),
            },
            Ok(x) => {
                let node = self.cursor.node();
                let (acc, res) = f1(self.acc, &node);
                let res = match res {
                    Ok(y) => Ok(f2(x, Some(y))),
                    Err(_) => Ok(f2(x, None)),
                };
                IterNode {
                    cursor: self.cursor,
                    acc,
                    res,
                }
            }
        }
    }

    pub fn apply_opt_next<F1, F2, U2, U3>(self, f1: &mut F1, f2: &mut F2) -> IterNode<'a, T, U3>
    where
        F1: FnMut(T, &Node<'a>) -> (T, Result<U2, Error>),
        F2: FnMut(U1, Option<U2>) -> U3,
    {
        let mut is_ok = false;
        let self2 = self.apply_opt(f1, &mut |x1, opt_x2| {
            is_ok = opt_x2.is_some();
            f2(x1, opt_x2)
        });
        if is_ok {
            self2.next()
        } else {
            self2
        }
    }

    /// map result
    pub fn map<F, U2>(self, f: F) -> IterNode<'a, T, U2>
    where
        F: FnOnce(U1) -> U2,
    {
        IterNode {
            cursor: self.cursor,
            acc: self.acc,
            res: self.res.map(f),
        }
    }

    /// map error
    pub fn map_error<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Error) -> Error,
    {
        self.res = self.res.map_err(f);
        self
    }

    // apply [f1] to the until nodes and [f2] to combine results
    pub fn repeat<F1, F2, U2>(mut self, f1: &mut F1, f2: &mut F2) -> Self
    where
        F1: FnMut(T, &Node<'a>) -> (T, Result<U2, Error>),
        F2: FnMut(U1, U2) -> U1,
    {
        loop {
            let node = self.cursor.node();
            let (acc, res2) = f1(self.acc, &node);
            self.acc = acc;
            let res3 = match (self.res, res2) {
                (Ok(x), Ok(y)) => Ok(f2(x, y)),
                (Err(e), Ok(_)) | (Ok(_), Err(e)) => Err(e),
                (Err(e1), Err(e2)) => Err(e1.error_add(e2)),
            };
            self.res = res3;
            if !self.cursor.goto_next_sibling() {
                break;
            }
        }
        self
    }

    // get result and accumulator
    pub fn acc_result(self) -> (T, Result<U1, Error>) {
        (self.acc, self.res)
    }
}

impl<'a, T, U> IterNode<'a, T, U> {}
