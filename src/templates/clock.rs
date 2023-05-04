use super::*;

#[derive(Boilerplate)]
pub(crate) struct ClockSvg {
  height: Height,
  hour: f64,
  minute: f64,
  second: f64,
}

impl ClockSvg {
  pub(crate) fn new(height: Height) -> Self {
    let min = height.min(Epoch::FIRST_POST_SUBSIDY.starting_height());

    Self {
      height,
      hour: (min.n() % Epoch::FIRST_POST_SUBSIDY.starting_height().n()) as f64
        / Epoch::FIRST_POST_SUBSIDY.starting_height().n() as f64
        * 360.0,
      minute: (min.n() % SUBSIDY_HALVING_INTERVAL) as f64 / SUBSIDY_HALVING_INTERVAL as f64 * 360.0,
      second: height.period_offset() as f64 / DIFFCHANGE_INTERVAL as f64 * 360.0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn second() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).second, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(504)).second, 90.0);
    pretty_assert_eq!(ClockSvg::new(Height(1008)).second, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(1512)).second, 270.0);
    pretty_assert_eq!(ClockSvg::new(Height(2016)).second, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930000)).second, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930504)).second, 270.0);
  }

  #[test]
  fn minute() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(210000)).minute, 90.0);
    pretty_assert_eq!(ClockSvg::new(Height(420000)).minute, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(630000)).minute, 270.0);
    pretty_assert_eq!(ClockSvg::new(Height(840000)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720000)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720001)).minute, 0.0);
  }

  #[test]
  fn hour() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).hour, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930000)).hour, 90.0);
    pretty_assert_eq!(ClockSvg::new(Height(13860000)).hour, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(20790000)).hour, 270.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720000)).hour, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720001)).hour, 0.0);
  }

  #[test]
  fn final_subsidy_height() {
    pretty_assert_eq!(
      ClockSvg::new(Height(27719999)).second,
      2015.0 / 2016.0 * 360.0
    );
    pretty_assert_eq!(
      ClockSvg::new(Height(27719999)).minute,
      839_999.0 / 840_000.0 * 360.0
    );
    pretty_assert_eq!(
      ClockSvg::new(Height(27719999)).hour,
      27719999.0 / 27720000.0 * 360.0
    );
  }

  #[test]
  fn first_post_subsidy_height() {
    pretty_assert_eq!(ClockSvg::new(Height(27720000)).second, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720000)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(27720000)).hour, 0.0);
  }

  #[test]
  fn clock_svg() {
    assert_regex_match!(
      ClockSvg::new(Height(27719999)).to_string(),
      r##"<\?xml version="1.0" encoding="UTF-8"\?>
<svg.*>.*
  <text.*>27719999</text>.*
  <line y2="-9" transform="rotate\(359.999987012987\)"><title>Subsidy</title></line>.*
  <line y2="-13" stroke-width="0.6" transform="rotate\(359.9995714285714\)"><title>Epoch</title></line>.*
  <line y2="-16" stroke="#d00505" stroke-width="0.2" transform="rotate\(359.82142857142856\)"><title>Period</title></line>.*
  <circle r="0.7" stroke="#d00505" stroke-width="0.3"/>.*
</svg>
"##,
    );
  }
}
