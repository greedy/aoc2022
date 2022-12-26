use aoc2022::prelude::*;
use itertools::{process_results, Itertools};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<25>
}

struct SNAFU(Vec<i8>);

impl SNAFU {
}

impl TryFrom<i64> for SNAFU {
    type Error = Report;
    fn try_from(mut num: i64) -> Result<Self, Self::Error> {
        if num < 0 { return Err(eyre!("Cannot convert {num} < 0 into SNAFU")) }
        // (15 + 2) % 5 - 2 => 0
        // (15 - 0) / 5 => 3
        // (3 + 2) % 5 - 2 => -2
        // (3 - -2) / 5 => 1
        let mut fugits = Vec::new();
        while num != 0 {
            let fugit = (num as i64 + 2) % 5 - 2;
            num = (num - fugit) / 5;
            fugits.push(fugit.try_into().unwrap());
        }
        Ok(Self(fugits))
    }
}

impl std::fmt::Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fugit in self.0.iter().rev() {
            let fugit_char = match fugit {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("unexepcted fugit value {}", fugit)
            };
            write!(f, "{}", fugit_char)?;
        }
        Ok(())
    }
}

impl TryFrom<SNAFU> for i64 {
    type Error = Report;

    fn try_from(snafu: SNAFU) -> Result<Self, Self::Error> {
        let mut result = 0;
        let mut pow5 = 1;
        for fugit in snafu.0 {
            let fugit_value = i64::checked_mul(pow5, fugit.into()).ok_or_else(|| eyre!("Overflow if fugit_value"))?;
            result = i64::checked_add(result, fugit_value).ok_or_else(|| eyre!("Overflow in result"))?;
            pow5 = i64::checked_mul(pow5, 5).ok_or_else(|| eyre!("Overflow in pow5"))?;
        }
        Ok(result)
    }
}

impl std::str::FromStr for SNAFU {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fugits = s.chars().rev().map(|c| {
            Ok(match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => bail!("Bad fugit")
            })
        }).try_collect()?;
        Ok(Self(fugits))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(3, "1=")]
    #[test_case(4, "1-")]
    #[test_case(5, "10")]
    #[test_case(6, "11")]
    #[test_case(7, "12")]
    #[test_case(8, "2=")]
    #[test_case(9, "2-")]
    #[test_case(10, "20")]
    #[test_case(15, "1=0")]
    #[test_case(20, "1-0")]
    #[test_case(2022, "1=11-2")]
    #[test_case(12345, "1-0---0")]
    #[test_case(314159265, "1121-1110-1=0")]
    fn test_num_to_snafu(num: i64, snafu_str: &str) {
        assert_eq!(super::SNAFU::try_from(num).unwrap().to_string(), snafu_str);
    }

    #[test_case("1=-0-2", 1747)]
    #[test_case("12111", 906)]
    #[test_case("2=0=", 198)]
    #[test_case("21", 11)]
    #[test_case("2=01", 201)]
    #[test_case("111", 31)]
    #[test_case("20012", 1257)]
    #[test_case("112", 32)]
    #[test_case("1=-1=", 353)]
    #[test_case("1-12", 107)]
    #[test_case("12", 7)]
    #[test_case("1=", 3)]
    #[test_case("122", 37)]
    fn test_snafo_to_num(snafu_str: &str, num: i64) {
        let snafu = snafu_str.parse::<super::SNAFU>().unwrap();
        let snafu_num: i64 = snafu.try_into().unwrap();
        assert_eq!(snafu_num, num)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let total: i64 = process_results(cli.input.get_input()?.lines().into_eyre().map_and_then(|s| s.parse::<SNAFU>().and_then(<SNAFU as TryInto<i64>>::try_into)), |iter| iter.sum())?;

    println!("The total is {} aka {}", total, <i64 as TryInto<SNAFU>>::try_into(total).expect("total to be a snafu"));

    Ok(())
}
