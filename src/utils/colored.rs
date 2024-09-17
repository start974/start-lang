use crate::args::Args;
pub use color_print::cformat;

pub trait Colored {
    /// string with color
    fn colored(&self) -> String;

    /// string colored if argument is set
    fn colored_string(&self, args: &Args) -> String
    where
        Self: std::fmt::Display + std::marker::Sized,
    {
        if args.no_color {
            self.to_string()
        } else {
            self.colored()
        }
    }

    // println with colored
    fn colored_println(&self, args: &Args)
    where
        Self: std::fmt::Display + std::marker::Sized,
    {
        println!("{}", self.colored_string(args));
    }

    // eprintln with colored
    fn colored_eprintln(&self, args: &Args)
    where
        Self: std::fmt::Display + std::marker::Sized,
    {
        eprintln!("{}", self.colored_string(args));
    }
}
