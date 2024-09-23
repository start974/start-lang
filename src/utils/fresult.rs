use crate::error::*;
use crate::location::*;

pub struct FResult<T, U, E> {
    acc: T,
    res: Result<U, E>,
}

impl<T, U, E> FResult<T, U, E> {
    pub fn make(acc: T, res: Result<U, E>) -> Self {
        Self { acc, res }
    }

    pub fn ok(acc: T, res: U) -> Self {
        Self::make(acc, Ok(res))
    }

    pub fn err(acc: T, err: E) -> Self {
        Self::make(acc, Err(err))
    }

    //pub fn get_res(self) -> Result<U, Error> {
    //self.res
    //}

    //pub fn get_acc(self) -> T {
    //self.acc
    //}

    pub fn get_pair(self) -> (T, Result<U, E>) {
        (self.acc, self.res)
    }

    //pub fn or_else<F>(self, f: F) -> FResult<T, U>
    //where
    //F: FnOnce(T) -> FResult<T, U>,
    //{
    //match self.res {
    //Ok(_) => self,
    //Err(_) => f(self.acc),
    //}
    //}

    pub fn inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&T, &U),
    {
        if let Ok(x) = &self.res {
            f(&self.acc, x);
        }
        self
    }

    //pub fn inspect_acc<F>(self, f: F) -> Self
    //where
    //F: FnOnce(&T),
    //{
    //f(&self.acc);
    //self
    //}

    /*    pub fn inspect_res<F>(self, f: F) -> Self*/
    /*where*/
    /*F: FnOnce(&U),*/
    /*{*/
    /*if let Ok(x) = &self.res {*/
    /*f(x);*/
    /*}*/
    /*self*/
    /*}*/

    pub fn get_result(self) -> Result<(T, U), E> {
        self.res.map(|x| (self.acc, x))
    }
}

impl<T, U1, E> FResult<T, U1, E> {
    pub fn and_then<F, U2>(self, f: F) -> FResult<T, U2, E>
    where
        F: FnOnce(T, U1) -> FResult<T, U2, E>,
    {
        match self.res {
            Ok(val) => f(self.acc, val),
            Err(e) => FResult::err(self.acc, e),
        }
    }

    pub fn map_res<U2, F>(self, f: F) -> FResult<T, U2, E>
    where
        F: FnOnce(U1) -> U2,
    {
        FResult {
            acc: self.acc,
            res: self.res.map(f),
        }
    }

    pub fn map_res2<U2, F>(self, f: F) -> FResult<T, U2, E>
    where
        F: FnOnce(&T, U1) -> U2,
    {
        let res2 = self.res.map(|x| f(&self.acc, x));
        FResult {
            acc: self.acc,
            res: res2,
        }
    }
}

impl<T1, U, E> FResult<T1, U, E> {
    //pub fn map_acc<T2, F>(self, f: F) -> FResult<T2, U>
    //where
    //F: FnOnce(T1) -> T2,
    //{
    //FResult {
    //acc: f(self.acc),
    //res: self.res,
    //}
    //}

    pub fn map_acc2<F>(self, f: F) -> FResult<T1, U, E>
    where
        F: FnOnce(T1, &U) -> T1,
    {
        match &self.res {
            Ok(x) => FResult {
                acc: f(self.acc, x),
                res: self.res,
            },
            Err(_) => self,
        }
    }

    //pub fn set_acc<T2>(self, acc: T2) -> FResult<T2, U>
    //{
    //FResult { acc, res: self.res }
    //}
}

impl<T, U1> FResult<T, U1, Errors> {
    /// combine result made by [f1] using [f2] and
    /// and make error list for many errors occured
    pub fn combine<F1, F2, U2, U3>(self, f1: F1, f2: F2) -> FResult<T, U3, Errors>
    where
        F1: FnOnce(T) -> FResult<T, U2, Error>,
        F2: FnOnce(U1, U2) -> U3,
    {
        let r = f1(self.acc);
        let res = match (self.res, r.res) {
            (Ok(x), Ok(y)) => Ok(f2(x, y)),
            (Ok(_), Err(e)) => Err(Errors::from(e)),
            (Err(errs), Ok(_)) => Err(errs),
            (Err(errs), Err(e)) => Err(errs.add_error(e)),
        };
        FResult { acc: r.acc, res }
    }

    /// cobmine result made by [f1] using [f2] and
    /// and make error list for many errors occured
    pub fn combine_box<F1, F2, U2, U3>(self, f1: F1, f2: F2) -> FResult<T, U3, Errors>
    where
        F1: FnOnce(T) -> FResult<T, U2, ErrorBox>,
        F2: FnOnce(U1, U2) -> U3,
    {
        let r = f1(self.acc);
        let res = match (self.res, r.res) {
            (Ok(x), Ok(y)) => Ok(f2(x, y)),
            (Ok(_), Err(e)) => Err(Errors::from(e)),
            (Err(errs), Ok(_)) => Err(errs),
            (Err(errs), Err(e)) => Err(errs.add_error(*e)),
        };
        FResult { acc: r.acc, res }
    }
}

impl<T, U, E1> FResult<T, U, E1> {
    /// map error from [E1] to [E2]
    pub fn map_err<E2, F>(self, f: F) -> FResult<T, U, E2>
    where
        F: FnOnce(E1) -> E2,
    {
        FResult {
            acc: self.acc,
            res: self.res.map_err(f),
        }
    }
}

impl<T, U> FResult<T, U, Error> {
    /// comvert error to errors
    pub fn into_errors(self) -> FResult<T, U, Errors> {
        self.map_err(Errors::from)
    }
    ///// convert error to error box
    //pub fn into_error_box(self) -> FResult<T, U, ErrorBox> {
        //self.map_err(Box::new)
    //}
}

impl<T, U> FResult<T, U, ErrorBox> {
    /// comvert error box to errors
    pub fn into_errors(self) -> FResult<T, U, Errors> {
        self.map_err(Errors::from)
    }
}

impl<T, U, E> Located for FResult<T, U, E>
where
    U: Located,
    E: Located,
{
    fn get_location(&self) -> &Option<Location> {
        match self.res {
            Ok(ref x) => x.get_location(),
            Err(ref e) => e.get_location(),
        }
    }
    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.res = match self.res {
            Ok(x) => Ok(x.set_opt_location(opt_location)),
            Err(e) => Err(e.set_opt_location(opt_location)),
        };
        self
    }
}
