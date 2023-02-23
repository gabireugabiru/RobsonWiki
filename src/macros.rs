macro_rules! ele {
  ($arg:literal) => {
    ($arg, $arg)
  };
  ($arg:literal, $arg1:literal) => {
    ($arg, $arg1)
  };
}
pub(crate) use ele;
