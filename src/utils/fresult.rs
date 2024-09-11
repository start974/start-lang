use super::super::error::Error;

pub struct FResult<T, U> {
    acc: T,
    res: Result<U, Error>,
}

impl<T, U> FResult<T, U> {
    pub fn ok(acc: T, res: U) -> Self {
        Self { acc, res: Ok(res) }
    }

    pub fn error<U2>(acc: T, err: Error) -> FResult<T, U2> {
        FResult { acc, res: Err(err) }
    }

    pub fn get_res(self) -> Result<U, Error> {
        self.res
    }

    //pub fn get_acc(self) -> T {
    //self.acc
    //}
}

impl<T, U1> FResult<T, U1> {
    pub fn and_then<F, U2>(self, f: F) -> FResult<T, U2>
    where
        F: FnOnce(T, U1) -> FResult<T, U2>,
    {
        match self.res {
            Ok(val) => f(self.acc, val),
            Err(e) => Self::error(self.acc, e),
        }
    }

    pub fn combine<F1, F2, U2, U3>(self, f1: F1, f2: F2) -> FResult<T, U3>
    where
        F1: FnOnce(T) -> FResult<T, U2>,
        F2: FnOnce(U1, U2) -> U3,
    {
        let r = f1(self.acc);
        let res = match (self.res, r.res) {
            (Ok(x), Ok(y)) => Ok(f2(x, y)),
            (Err(e), Ok(_)) | (Ok(_), Err(e)) => Err(e),
            (Err(e1), Err(e2)) => Err(e1.error_add(e2)),
        };
        FResult { acc: r.acc, res }
    }

    pub fn map_res<U2, F>(self, f: F) -> FResult<T, U2>
    where
        F: FnOnce(U1) -> U2,
    {
        FResult {
            acc: self.acc,
            res: self.res.map(f),
        }
    }

    pub fn map_res2<U2, F>(self, f: F) -> FResult<T, U2>
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

impl<T1, U> FResult<T1, U> {
    //pub fn map_acc<T2, F>(self, f: F) -> FResult<T2, U>
    //where
        //F: FnOnce(T1) -> T2,
    //{
        //FResult {
            //acc: f(self.acc),
            //res: self.res,
        //}
    //}

    pub fn map_acc2<F>(self, f: F) -> FResult<T1, U>
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
}
