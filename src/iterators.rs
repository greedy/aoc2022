pub trait AocItertools: Iterator {
    fn map_err<E, O, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(E) -> O,
    {
        MapErr::new(self, f)
    }

    fn parse<O, E, P>(self, p: P) -> Parse<Self, O, E, P>
    where
        Self: Sized,
        P: nom::Parser<Self::Item, O, E>
    {
        Parse::new(self, p)
    }

    fn into_eyre<T, E>(self) -> IntoEyre<Self>
        where
            Self: Iterator<Item = Result<T, E>> + Sized,
            E: Into<color_eyre::eyre::Report>
    {
        IntoEyre::new(self)
    }

    fn map_and_then<F, T, O, E>(self, f: F) -> MapAndThen<Self, F>
        where
            Self: Iterator<Item = Result<T, E>> + Sized,
            F: FnMut(T) -> Result<O, E>
    {
        MapAndThen::new(self, f)
    }
}

impl<I:Iterator> AocItertools for I { }

pub struct MapAndThen<I, F> {
    iter: I,
    f: F
}

impl<I, F> MapAndThen<I, F> {
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<T, E, O, I, F> Iterator for MapAndThen<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> Result<O, E>
{
    type Item = Result<O, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.and_then(&mut self.f))
    }
}

pub struct IntoEyre<I> {
    iter: I
}

impl<I> IntoEyre<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<T, E, I: Iterator<Item = Result<T, E>>> Iterator for IntoEyre<I>
where
    E: Into<color_eyre::eyre::Report>
{
    type Item = Result<T, color_eyre::eyre::Report>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map_err(Into::into))
    }
}

pub struct MapErr<I, F> {
    iter: I,
    f: F
}

impl<I, F> MapErr<I, F> {
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<T, E, O, I: Iterator<Item = Result<T, E>>, F> Iterator for MapErr<I, F>
where
    F: FnMut(E) -> O
{
    type Item = Result<T, O>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| r.map_err(&mut self.f))
    }
}

pub struct Parse<I, O, E, P> {
    iter: I,
    parser: P,
    phantom: std::marker::PhantomData<(O, E)>
}

impl<I: Iterator, O, E, P: nom::Parser<I::Item, O, E>> Parse<I, O, E, P> {
    fn new(iter: I, parser: P) -> Self {
        Self { iter, parser, phantom: std::marker::PhantomData::default() }
    }
}

impl<O, E, I: Iterator, P> Iterator for Parse<I, O, E, P>
where
    P: nom::Parser<I::Item, O, E>
{
    type Item = nom::IResult<I::Item, O, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use nom::Finish;
        self.iter.next().map(|i| self.parser.parse(i))
    }
}
