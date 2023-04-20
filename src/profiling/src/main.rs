use math::error::Error;
use math::{Number, Result};
use std::io::{self, BufReader, Read as _};

fn main() -> Result<()> {
    let mut nums = Vec::new();
    let mut sum = Number::zero();

    let bytes_stream = BufReader::new(io::stdin())
        .bytes()
        .map(|v| v.map_err(|_| Error::Message(String::from("Cannot read stdin"))));

    let mut num = None;
    for byte in bytes_stream {
        let byte = byte?;

        if byte == b' ' || byte == b'\n' || byte == b'\t' {
            if let Some(num) = num.take() {
                sum = sum.add(&num)?;
                nums.push(num);
            }

            continue;
        }

        if !byte.is_ascii_digit() {
            return Err(Error::Message(format!("{} is not a digit", byte as char)));
        }

        num = Some(num.get_or_insert(Number::zero()).mul(10)?.add(byte - 48)?);
    }

    if let Some(num) = num {
        sum = sum.add(&num)?;
        nums.push(num);
    }

    let n = nums.len();

    let avg = sum.div(n)?;
    let sum_squared = nums.into_iter().fold(Result::Ok(Number::zero()), |v, x| {
        v?.add(x.sub(&avg)?.power(2)?)
    })?;

    let standard_deviation = sum_squared.div(n - 1)?.sqrt()?;

    println!("{}", standard_deviation.to_string(Default::default(), 12));

    Ok(())
}
