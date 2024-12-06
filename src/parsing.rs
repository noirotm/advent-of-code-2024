use std::io::BufRead;
use std::str::FromStr;

pub struct WhitespaceSeparatedList<T>(Vec<T>);

impl<T> AsRef<[T]> for WhitespaceSeparatedList<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> From<WhitespaceSeparatedList<T>> for Vec<T> {
    fn from(value: WhitespaceSeparatedList<T>) -> Self {
        value.0
    }
}

impl<T> FromStr for WhitespaceSeparatedList<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(T::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v))
    }
}

pub trait BufReadExt<T> {
    fn split_by<B: FromIterator<T>>(self, separator: u8) -> B;
    fn split_commas<B: FromIterator<T>>(self) -> B;
    fn split_lines<B: FromIterator<T>>(self) -> B;
    fn split_groups<B: FromIterator<T>>(self) -> B;
}

impl<R, T> BufReadExt<T> for R
where
    R: BufRead,
    T: FromStr,
{
    fn split_by<B: FromIterator<T>>(self, separator: u8) -> B {
        self.split(separator)
            .map_while(Result::ok)
            .flat_map(String::from_utf8)
            .flat_map(|s| s.parse())
            .collect()
    }

    fn split_commas<B: FromIterator<T>>(self) -> B {
        self.split_by(b',')
    }

    fn split_lines<B: FromIterator<T>>(self) -> B {
        self.lines()
            .map_while(Result::ok)
            .flat_map(|l| l.parse())
            .collect()
    }

    fn split_groups<B: FromIterator<T>>(self) -> B {
        self.lines()
            .map_while(Result::ok)
            .collect::<Vec<_>>()
            .split(|l| l.is_empty())
            .flat_map(|e| e.join("\n").parse())
            .collect()
    }
}

pub trait ReadAll {
    fn read_all(self) -> String;
}

impl<R> ReadAll for R
where
    R: BufRead,
{
    fn read_all(mut self) -> String {
        let mut s = String::new();
        let _ = self.read_to_string(&mut s).unwrap_or_default();
        s
    }
}
