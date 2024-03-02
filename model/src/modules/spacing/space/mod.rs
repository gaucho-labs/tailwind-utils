use crate::Axis2D;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSpace {
    axis: Axis2D,
    negative: Negative,
    size: SpacingSize,
}

crate::macros::axis2d_collision!(TailwindSpace => "space");

impl TailwindSpace {
    /// https://tailwindcss.com/docs/space
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            ["x", rest @ ..] => Self::parse_axis(rest, arbitrary, Axis2D::X, negative),
            ["y", rest @ ..] => Self::parse_axis(rest, arbitrary, Axis2D::Y, negative),
            _ => syntax_error!("Unknown space instructions: {}", pattern.join("-")),
        }
    }
    fn parse_axis(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        axis: Axis2D,
        negative: Negative,
    ) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            [] => Ok(Self {
                axis,
                negative,
                size: SpacingSize::Arbitrary,
            }
            .boxed()),
            ["reverse"] => Ok(TailwindSpaceReverse { axis }.boxed()),
            _ => {
                let size = SpacingSize::parse(pattern, arbitrary, &Self::check_valid)?;
                Ok(Self {
                    axis,
                    negative,
                    size,
                }
                .boxed())
            }
        }
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/margin#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
