use {IntrusiveIterator, FromIntrusiveIterator};

/// Extension methods for Intrusive Iterators
pub trait IntrusiveIteratorExt<T> : IntrusiveIterator<T> {
    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<Self, F> {
        Map { iter: self, closure: f }
    }

    fn filter<F: FnMut(&T) -> bool>(self, pred: F) -> Filter<Self, F> {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, pred: F) -> FilterMap<Self, F> {
        FilterMap { iter: self, predicate: pred }
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate(self)
    }

    fn skip(self, n: uint) -> Skip<Self> {
        Skip { iter: self, n: n }
    }

    fn take(self, n: uint) -> Take<Self> {
        Take { iter: self, n: n }
    }

    fn skip_while<F: FnMut(T) -> bool>(self, pred: F) -> SkipWhile<Self, F> {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F: FnMut(T) -> bool>(self, pred: F) -> TakeWhile<Self, F> {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F: FnMut(&T)>(self, f: F) -> Inspect<Self, F> {
        Inspect { iter: self, closure: f }
    }

    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, f: F) -> FlatMap<Self, F> {
        FlatMap { iter: self, producer: f }
    }

    fn chain<O: IntrusiveIterator<T>>(self, other: O) -> Chain<Self, O> {
        Chain { one: self, two: other }
    }

    fn count(self) -> uint {
        let mut count = 0;
        self.iterate(|_| { count += 1; });
        count
    }

    fn collect<D: FromIntrusiveIterator<T>>(self) -> D {
        FromIntrusiveIterator::collect(self)
    }
}

impl<T, I: IntrusiveIterator<T>> IntrusiveIteratorExt<T> for I {}

/// An IntrusiveIterator that maps over the contents of
/// another IntrusiveIterator.
pub struct Map<I, F> {
    iter: I,
    closure: F
}

impl<T, O, I: IntrusiveIterator<T>, F: FnMut(T) -> O> IntrusiveIterator<O> for Map<I, F> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            f(closure(t))
        });
    }
}

pub struct Filter<I, F> {
    iter: I,
    predicate: F
}

impl<T, I, F> IntrusiveIterator<T> for Filter<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            if predicate(&t) { f(t) } else { false }
        });
    }
}

pub struct FilterMap<I, F> {
    iter: I,
    predicate: F
}

impl<T, O, I, F> IntrusiveIterator<O> for FilterMap<I, F>
where I: IntrusiveIterator<T>, F: FnMut(T) -> Option<O> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            match predicate(t) {
                Some(o) => f(o),
                None => false
            }
        });
    }
}

pub struct Enumerate<I>(I);

impl<T, I> IntrusiveIterator<(uint, T)> for Enumerate<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut((uint, T)) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        self.0.traverse(|t: T| {
            let res = f((counter, t));
            counter += 1;
            res
        })
    }
}

pub struct Skip<I> {
    iter: I,
    n: uint
}

impl<T, I> IntrusiveIterator<T> for Skip<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.traverse(|t: T| {
            if counter != n {
                counter += 1;
                true
            } else {
                f(t)
            }
        })
    }
}

pub struct Take<I> {
    iter: I,
    n: uint
}

impl<T, I> IntrusiveIterator<T> for Take<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.traverse(|t: T| {
            if counter != n {
                counter += 1;
                f(t)
            } else {
                true
            }
        })
    }
}

pub struct SkipWhile<I, F> {
    iter: I,
    predicate: F
}

impl<T, I, F> IntrusiveIterator<T> for SkipWhile<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        let mut flag = false;
        self.iter.traverse(move |t: T| {
            // Done skipping
            if flag {
                if !predicate(&t) {
                    flag = true;
                }
                false
            } else {
                f(t)
            }
        });
    }
}

pub struct TakeWhile<I, F> {
    iter: I,
    predicate: F
}

impl<T, I, F> IntrusiveIterator<T> for TakeWhile<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            if predicate(&t) { f(t) } else { true }
        });
    }
}

pub struct Inspect<I, F> {
    iter: I,
    closure: F
}

impl<T, I, F> IntrusiveIterator<T> for Inspect<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            closure(&t);
            f(t)
        });
    }
}

pub struct Chain<I, O> {
    one: I,
    two: O
}

impl<T, I, O> IntrusiveIterator<T> for Chain<I, O>
where I: IntrusiveIterator<T>, O: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut flag = false;
        self.one.traverse(|t: T| {
            flag = f(t); flag
        });

        if !flag {
            self.two.traverse(|t: T| {
                f(t)
            });
        }
    }
}

pub struct FlatMap<I, F> {
    iter: I,
    producer: F
}

impl<T, O, U, I, F> IntrusiveIterator<O> for FlatMap<I, F>
where I: IntrusiveIterator<T>,
      F: FnMut(T) -> U,
      U: IntrusiveIterator<O> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut producer = self.producer;
        let mut flag = false;
        self.iter.traverse(|t: T| {
            producer(t).traverse(|o: O| {
                flag = f(o); flag
            });
            flag
        });
    }
}

