/// A register named `r0` to `r{N-1}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register<const N: usize>(pub usize);

impl<const N: usize> TryFrom<&str> for Register<N> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let index = value
            .strip_prefix('r')
            .and_then(|x| x.parse::<usize>().ok())
            .ok_or(())?;
        if index < N {
            Ok(Self(index))
        } else {
            Err(())
        }
    }
}
