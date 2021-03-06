#[allow(unused_imports)]
use standback::prelude::*;
use std::cmp::Ordering;
use time::{error, prelude::*, util, Date, Result, Weekday};

macro_rules! julian {
    ($julian:literal) => {
        Date::from_julian_day($julian)
    };
}

#[test]
fn weeks_in_year_exhaustive() {
    let years_with_53 = &[
        4, 9, 15, 20, 26, 32, 37, 43, 48, 54, 60, 65, 71, 76, 82, 88, 93, 99, 105, 111, 116, 122,
        128, 133, 139, 144, 150, 156, 161, 167, 172, 178, 184, 189, 195, 201, 207, 212, 218, 224,
        229, 235, 240, 246, 252, 257, 263, 268, 274, 280, 285, 291, 296, 303, 308, 314, 320, 325,
        331, 336, 342, 348, 353, 359, 364, 370, 376, 381, 387, 392, 398,
    ];

    for year in 0..400 {
        assert_eq!(
            util::weeks_in_year(year),
            if years_with_53.contains(&year) {
                53
            } else {
                52
            }
        );
    }
}

// Test all dominical letters. For leap years, check the dates
// immediately preceding and after the leap day.

#[test]
fn test_monday_based_week() -> crate::Result<()> {
    // A
    assert_eq!(date!(2023 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2023 - 01 - 02).monday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 07).monday_based_week(), 1);

    // B
    assert_eq!(date!(2022 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2022 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2022 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 07).monday_based_week(), 1);

    // C
    assert_eq!(date!(2021 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2021 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2021 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2021 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 07).monday_based_week(), 1);

    // D
    assert_eq!(date!(2026 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2026 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2026 - 01 - 07).monday_based_week(), 1);

    // E
    assert_eq!(date!(2025 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 05).monday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2025 - 01 - 07).monday_based_week(), 1);

    // F
    assert_eq!(date!(2019 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 05).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 06).monday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 07).monday_based_week(), 1);

    // G
    assert_eq!(date!(2018 - 01 - 01).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 02).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2018 - 01 - 07).monday_based_week(), 1);

    // AG
    assert_eq!(date!(2012 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2012 - 01 - 02).monday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2012 - 02 - 28).monday_based_week(), 9);
    assert_eq!(date!(2012 - 02 - 29).monday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 01).monday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 05).monday_based_week(), 10);
    assert_eq!(date!(2012 - 03 - 06).monday_based_week(), 10);
    assert_eq!(date!(2012 - 03 - 07).monday_based_week(), 10);

    // BA
    assert_eq!(date!(2028 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2028 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2028 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2028 - 02 - 28).monday_based_week(), 9);
    assert_eq!(date!(2028 - 02 - 29).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 01).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 05).monday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 06).monday_based_week(), 10);
    assert_eq!(date!(2028 - 03 - 07).monday_based_week(), 10);

    // CB
    assert_eq!(date!(2016 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2016 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2016 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2016 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2016 - 02 - 28).monday_based_week(), 8);
    assert_eq!(date!(2016 - 02 - 29).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 01).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 05).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 06).monday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 07).monday_based_week(), 10);

    // DC
    assert_eq!(date!(2032 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2032 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2032 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2032 - 02 - 28).monday_based_week(), 8);
    assert_eq!(date!(2032 - 02 - 29).monday_based_week(), 8);
    assert_eq!(date!(2032 - 03 - 01).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 05).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 06).monday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 07).monday_based_week(), 9);

    // ED
    assert_eq!(date!(2020 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 05).monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2020 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2020 - 02 - 28).monday_based_week(), 8);
    assert_eq!(date!(2020 - 02 - 29).monday_based_week(), 8);
    assert_eq!(date!(2020 - 03 - 01).monday_based_week(), 8);
    assert_eq!(date!(2020 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 05).monday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 06).monday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 07).monday_based_week(), 9);

    // FE
    assert_eq!(date!(2036 - 01 - 01).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 02).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 03).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 04).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 05).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 06).monday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2036 - 02 - 28).monday_based_week(), 8);
    assert_eq!(date!(2036 - 02 - 29).monday_based_week(), 8);
    assert_eq!(date!(2036 - 03 - 01).monday_based_week(), 8);
    assert_eq!(date!(2036 - 03 - 02).monday_based_week(), 8);
    assert_eq!(date!(2036 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 04).monday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 05).monday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 06).monday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 07).monday_based_week(), 9);

    // GF
    assert_eq!(date!(2024 - 01 - 01).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 02).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 03).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 04).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 05).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 06).monday_based_week(), 1);
    assert_eq!(date!(2024 - 01 - 07).monday_based_week(), 1);
    assert_eq!(date!(2024 - 02 - 28).monday_based_week(), 9);
    assert_eq!(date!(2024 - 02 - 29).monday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 01).monday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 02).monday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 03).monday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 04).monday_based_week(), 10);
    assert_eq!(date!(2024 - 03 - 05).monday_based_week(), 10);
    assert_eq!(date!(2024 - 03 - 06).monday_based_week(), 10);
    assert_eq!(date!(2024 - 03 - 07).monday_based_week(), 10);
    Ok(())
}

#[test]
fn test_sunday_based_week() -> crate::Result<()> {
    // A
    assert_eq!(date!(2023 - 01 - 01).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 02).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2023 - 01 - 07).sunday_based_week(), 1);

    // B
    assert_eq!(date!(2022 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2022 - 01 - 02).sunday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2022 - 01 - 07).sunday_based_week(), 1);

    // C
    assert_eq!(date!(2021 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2021 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2021 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2021 - 01 - 07).sunday_based_week(), 1);

    // D
    assert_eq!(date!(2026 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2026 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2026 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2026 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2026 - 01 - 07).sunday_based_week(), 1);

    // E
    assert_eq!(date!(2025 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2025 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2025 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2025 - 01 - 07).sunday_based_week(), 1);

    // F
    assert_eq!(date!(2019 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 05).sunday_based_week(), 0);
    assert_eq!(date!(2019 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2019 - 01 - 07).sunday_based_week(), 1);

    // G
    assert_eq!(date!(2018 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 05).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 06).sunday_based_week(), 0);
    assert_eq!(date!(2018 - 01 - 07).sunday_based_week(), 1);

    // AG
    assert_eq!(date!(2012 - 01 - 01).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 02).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2012 - 02 - 28).sunday_based_week(), 9);
    assert_eq!(date!(2012 - 02 - 29).sunday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 01).sunday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2012 - 03 - 04).sunday_based_week(), 10);
    assert_eq!(date!(2012 - 03 - 05).sunday_based_week(), 10);
    assert_eq!(date!(2012 - 03 - 06).sunday_based_week(), 10);
    assert_eq!(date!(2012 - 03 - 07).sunday_based_week(), 10);

    // BA
    assert_eq!(date!(2028 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2028 - 01 - 02).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2028 - 02 - 28).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 02 - 29).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 01).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2028 - 03 - 05).sunday_based_week(), 10);
    assert_eq!(date!(2028 - 03 - 06).sunday_based_week(), 10);
    assert_eq!(date!(2028 - 03 - 07).sunday_based_week(), 10);

    // CB
    assert_eq!(date!(2016 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2016 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2016 - 01 - 03).sunday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2016 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2016 - 02 - 28).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 02 - 29).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 01).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 05).sunday_based_week(), 9);
    assert_eq!(date!(2016 - 03 - 06).sunday_based_week(), 10);
    assert_eq!(date!(2016 - 03 - 07).sunday_based_week(), 10);

    // DC
    assert_eq!(date!(2032 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2032 - 01 - 04).sunday_based_week(), 1);
    assert_eq!(date!(2032 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2032 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2032 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2032 - 02 - 28).sunday_based_week(), 8);
    assert_eq!(date!(2032 - 02 - 29).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 01).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 05).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 06).sunday_based_week(), 9);
    assert_eq!(date!(2032 - 03 - 07).sunday_based_week(), 10);

    // ED
    assert_eq!(date!(2020 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 05).sunday_based_week(), 1);
    assert_eq!(date!(2020 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2020 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2020 - 02 - 28).sunday_based_week(), 8);
    assert_eq!(date!(2020 - 02 - 29).sunday_based_week(), 8);
    assert_eq!(date!(2020 - 03 - 01).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 05).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 06).sunday_based_week(), 9);
    assert_eq!(date!(2020 - 03 - 07).sunday_based_week(), 9);

    // FE
    assert_eq!(date!(2036 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 05).sunday_based_week(), 0);
    assert_eq!(date!(2036 - 01 - 06).sunday_based_week(), 1);
    assert_eq!(date!(2036 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2036 - 02 - 28).sunday_based_week(), 8);
    assert_eq!(date!(2036 - 02 - 29).sunday_based_week(), 8);
    assert_eq!(date!(2036 - 03 - 01).sunday_based_week(), 8);
    assert_eq!(date!(2036 - 03 - 02).sunday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 05).sunday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 06).sunday_based_week(), 9);
    assert_eq!(date!(2036 - 03 - 07).sunday_based_week(), 9);

    // GF
    assert_eq!(date!(2024 - 01 - 01).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 02).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 03).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 04).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 05).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 06).sunday_based_week(), 0);
    assert_eq!(date!(2024 - 01 - 07).sunday_based_week(), 1);
    assert_eq!(date!(2024 - 02 - 28).sunday_based_week(), 8);
    assert_eq!(date!(2024 - 02 - 29).sunday_based_week(), 8);
    assert_eq!(date!(2024 - 03 - 01).sunday_based_week(), 8);
    assert_eq!(date!(2024 - 03 - 02).sunday_based_week(), 8);
    assert_eq!(date!(2024 - 03 - 03).sunday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 04).sunday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 05).sunday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 06).sunday_based_week(), 9);
    assert_eq!(date!(2024 - 03 - 07).sunday_based_week(), 9);
    Ok(())
}

#[test]
fn test_parse_monday_based_week() -> crate::Result<()> {
    macro_rules! parse {
        ($s:literal) => {
            Date::parse($s, "%a %W %Y")?
        };
    }

    // A
    assert_eq!(parse!("Sun 00 2023"), date!(2023 - 001));
    assert_eq!(parse!("Mon 01 2023"), date!(2023 - 002));
    assert_eq!(parse!("Tue 01 2023"), date!(2023 - 003));
    assert_eq!(parse!("Wed 01 2023"), date!(2023 - 004));
    assert_eq!(parse!("Thu 01 2023"), date!(2023 - 005));
    assert_eq!(parse!("Fri 01 2023"), date!(2023 - 006));
    assert_eq!(parse!("Sat 01 2023"), date!(2023 - 007));

    // B
    assert_eq!(parse!("Sat 00 2022"), date!(2022 - 001));
    assert_eq!(parse!("Sun 00 2022"), date!(2022 - 002));
    assert_eq!(parse!("Mon 01 2022"), date!(2022 - 003));
    assert_eq!(parse!("Tue 01 2022"), date!(2022 - 004));
    assert_eq!(parse!("Wed 01 2022"), date!(2022 - 005));
    assert_eq!(parse!("Thu 01 2022"), date!(2022 - 006));
    assert_eq!(parse!("Fri 01 2022"), date!(2022 - 007));

    // C
    assert_eq!(parse!("Fri 00 2021"), date!(2021 - 001));
    assert_eq!(parse!("Sat 00 2021"), date!(2021 - 002));
    assert_eq!(parse!("Sun 00 2021"), date!(2021 - 003));
    assert_eq!(parse!("Mon 01 2021"), date!(2021 - 004));
    assert_eq!(parse!("Tue 01 2021"), date!(2021 - 005));
    assert_eq!(parse!("Wed 01 2021"), date!(2021 - 006));
    assert_eq!(parse!("Thu 01 2021"), date!(2021 - 007));

    // D
    assert_eq!(parse!("Thu 00 2026"), date!(2026 - 001));
    assert_eq!(parse!("Fri 00 2026"), date!(2026 - 002));
    assert_eq!(parse!("Sat 00 2026"), date!(2026 - 003));
    assert_eq!(parse!("Sun 00 2026"), date!(2026 - 004));
    assert_eq!(parse!("Mon 01 2026"), date!(2026 - 005));
    assert_eq!(parse!("Tue 01 2026"), date!(2026 - 006));
    assert_eq!(parse!("Wed 01 2026"), date!(2026 - 007));

    // E
    assert_eq!(parse!("Wed 00 2025"), date!(2025 - 001));
    assert_eq!(parse!("Thu 00 2025"), date!(2025 - 002));
    assert_eq!(parse!("Fri 00 2025"), date!(2025 - 003));
    assert_eq!(parse!("Sat 00 2025"), date!(2025 - 004));
    assert_eq!(parse!("Sun 00 2025"), date!(2025 - 005));
    assert_eq!(parse!("Mon 01 2025"), date!(2025 - 006));
    assert_eq!(parse!("Tue 01 2025"), date!(2025 - 007));

    // F
    assert_eq!(parse!("Tue 00 2019"), date!(2019 - 001));
    assert_eq!(parse!("Wed 00 2019"), date!(2019 - 002));
    assert_eq!(parse!("Thu 00 2019"), date!(2019 - 003));
    assert_eq!(parse!("Fri 00 2019"), date!(2019 - 004));
    assert_eq!(parse!("Sat 00 2019"), date!(2019 - 005));
    assert_eq!(parse!("Sun 00 2019"), date!(2019 - 006));
    assert_eq!(parse!("Mon 01 2019"), date!(2019 - 007));

    // G
    assert_eq!(parse!("Mon 01 2018"), date!(2018 - 001));
    assert_eq!(parse!("Tue 01 2018"), date!(2018 - 002));
    assert_eq!(parse!("Wed 01 2018"), date!(2018 - 003));
    assert_eq!(parse!("Thu 01 2018"), date!(2018 - 004));
    assert_eq!(parse!("Fri 01 2018"), date!(2018 - 005));
    assert_eq!(parse!("Sat 01 2018"), date!(2018 - 006));
    assert_eq!(parse!("Sun 01 2018"), date!(2018 - 007));

    // AG
    assert_eq!(parse!("Sun 00 2012"), date!(2012 - 001));
    assert_eq!(parse!("Mon 01 2012"), date!(2012 - 002));
    assert_eq!(parse!("Tue 01 2012"), date!(2012 - 003));
    assert_eq!(parse!("Wed 01 2012"), date!(2012 - 004));
    assert_eq!(parse!("Thu 01 2012"), date!(2012 - 005));
    assert_eq!(parse!("Fri 01 2012"), date!(2012 - 006));
    assert_eq!(parse!("Sat 01 2012"), date!(2012 - 007));
    assert_eq!(parse!("Tue 09 2012"), date!(2012 - 059));
    assert_eq!(parse!("Wed 09 2012"), date!(2012 - 060));
    assert_eq!(parse!("Thu 09 2012"), date!(2012 - 061));
    assert_eq!(parse!("Fri 09 2012"), date!(2012 - 062));
    assert_eq!(parse!("Sat 09 2012"), date!(2012 - 063));
    assert_eq!(parse!("Sun 09 2012"), date!(2012 - 064));
    assert_eq!(parse!("Mon 10 2012"), date!(2012 - 065));
    assert_eq!(parse!("Tue 10 2012"), date!(2012 - 066));
    assert_eq!(parse!("Wed 10 2012"), date!(2012 - 067));

    // BA
    assert_eq!(parse!("Sat 00 2028"), date!(2028 - 001));
    assert_eq!(parse!("Sun 00 2028"), date!(2028 - 002));
    assert_eq!(parse!("Mon 01 2028"), date!(2028 - 003));
    assert_eq!(parse!("Tue 01 2028"), date!(2028 - 004));
    assert_eq!(parse!("Wed 01 2028"), date!(2028 - 005));
    assert_eq!(parse!("Thu 01 2028"), date!(2028 - 006));
    assert_eq!(parse!("Fri 01 2028"), date!(2028 - 007));
    assert_eq!(parse!("Mon 09 2028"), date!(2028 - 059));
    assert_eq!(parse!("Tue 09 2028"), date!(2028 - 060));
    assert_eq!(parse!("Wed 09 2028"), date!(2028 - 061));
    assert_eq!(parse!("Thu 09 2028"), date!(2028 - 062));
    assert_eq!(parse!("Fri 09 2028"), date!(2028 - 063));
    assert_eq!(parse!("Sat 09 2028"), date!(2028 - 064));
    assert_eq!(parse!("Sun 09 2028"), date!(2028 - 065));
    assert_eq!(parse!("Mon 10 2028"), date!(2028 - 066));
    assert_eq!(parse!("Tue 10 2028"), date!(2028 - 067));

    // CB
    assert_eq!(parse!("Fri 00 2016"), date!(2016 - 001));
    assert_eq!(parse!("Sat 00 2016"), date!(2016 - 002));
    assert_eq!(parse!("Sun 00 2016"), date!(2016 - 003));
    assert_eq!(parse!("Mon 01 2016"), date!(2016 - 004));
    assert_eq!(parse!("Tue 01 2016"), date!(2016 - 005));
    assert_eq!(parse!("Wed 01 2016"), date!(2016 - 006));
    assert_eq!(parse!("Thu 01 2016"), date!(2016 - 007));
    assert_eq!(parse!("Sun 08 2016"), date!(2016 - 059));
    assert_eq!(parse!("Mon 09 2016"), date!(2016 - 060));
    assert_eq!(parse!("Tue 09 2016"), date!(2016 - 061));
    assert_eq!(parse!("Wed 09 2016"), date!(2016 - 062));
    assert_eq!(parse!("Thu 09 2016"), date!(2016 - 063));
    assert_eq!(parse!("Fri 09 2016"), date!(2016 - 064));
    assert_eq!(parse!("Sat 09 2016"), date!(2016 - 065));
    assert_eq!(parse!("Sun 09 2016"), date!(2016 - 066));
    assert_eq!(parse!("Mon 10 2016"), date!(2016 - 067));

    // DC
    assert_eq!(parse!("Thu 00 2032"), date!(2032 - 001));
    assert_eq!(parse!("Fri 00 2032"), date!(2032 - 002));
    assert_eq!(parse!("Sat 00 2032"), date!(2032 - 003));
    assert_eq!(parse!("Sun 00 2032"), date!(2032 - 004));
    assert_eq!(parse!("Mon 01 2032"), date!(2032 - 005));
    assert_eq!(parse!("Tue 01 2032"), date!(2032 - 006));
    assert_eq!(parse!("Wed 01 2032"), date!(2032 - 007));
    assert_eq!(parse!("Sat 08 2032"), date!(2032 - 059));
    assert_eq!(parse!("Sun 08 2032"), date!(2032 - 060));
    assert_eq!(parse!("Mon 09 2032"), date!(2032 - 061));
    assert_eq!(parse!("Tue 09 2032"), date!(2032 - 062));
    assert_eq!(parse!("Wed 09 2032"), date!(2032 - 063));
    assert_eq!(parse!("Thu 09 2032"), date!(2032 - 064));
    assert_eq!(parse!("Fri 09 2032"), date!(2032 - 065));
    assert_eq!(parse!("Sat 09 2032"), date!(2032 - 066));
    assert_eq!(parse!("Sun 09 2032"), date!(2032 - 067));

    // ED
    assert_eq!(parse!("Wed 00 2020"), date!(2020 - 001));
    assert_eq!(parse!("Thu 00 2020"), date!(2020 - 002));
    assert_eq!(parse!("Fri 00 2020"), date!(2020 - 003));
    assert_eq!(parse!("Sat 00 2020"), date!(2020 - 004));
    assert_eq!(parse!("Sun 00 2020"), date!(2020 - 005));
    assert_eq!(parse!("Mon 01 2020"), date!(2020 - 006));
    assert_eq!(parse!("Tue 01 2020"), date!(2020 - 007));
    assert_eq!(parse!("Fri 08 2020"), date!(2020 - 059));
    assert_eq!(parse!("Sat 08 2020"), date!(2020 - 060));
    assert_eq!(parse!("Sun 08 2020"), date!(2020 - 061));
    assert_eq!(parse!("Mon 09 2020"), date!(2020 - 062));
    assert_eq!(parse!("Tue 09 2020"), date!(2020 - 063));
    assert_eq!(parse!("Wed 09 2020"), date!(2020 - 064));
    assert_eq!(parse!("Thu 09 2020"), date!(2020 - 065));
    assert_eq!(parse!("Fri 09 2020"), date!(2020 - 066));
    assert_eq!(parse!("Sat 09 2020"), date!(2020 - 067));

    // FE
    assert_eq!(parse!("Tue 00 2036"), date!(2036 - 001));
    assert_eq!(parse!("Wed 00 2036"), date!(2036 - 002));
    assert_eq!(parse!("Thu 00 2036"), date!(2036 - 003));
    assert_eq!(parse!("Fri 00 2036"), date!(2036 - 004));
    assert_eq!(parse!("Sat 00 2036"), date!(2036 - 005));
    assert_eq!(parse!("Sun 00 2036"), date!(2036 - 006));
    assert_eq!(parse!("Mon 01 2036"), date!(2036 - 007));
    assert_eq!(parse!("Thu 08 2036"), date!(2036 - 059));
    assert_eq!(parse!("Fri 08 2036"), date!(2036 - 060));
    assert_eq!(parse!("Sat 08 2036"), date!(2036 - 061));
    assert_eq!(parse!("Sun 08 2036"), date!(2036 - 062));
    assert_eq!(parse!("Mon 09 2036"), date!(2036 - 063));
    assert_eq!(parse!("Tue 09 2036"), date!(2036 - 064));
    assert_eq!(parse!("Wed 09 2036"), date!(2036 - 065));
    assert_eq!(parse!("Thu 09 2036"), date!(2036 - 066));
    assert_eq!(parse!("Fri 09 2036"), date!(2036 - 067));

    // GF
    assert_eq!(parse!("Mon 01 2024"), date!(2024 - 001));
    assert_eq!(parse!("Tue 01 2024"), date!(2024 - 002));
    assert_eq!(parse!("Wed 01 2024"), date!(2024 - 003));
    assert_eq!(parse!("Thu 01 2024"), date!(2024 - 004));
    assert_eq!(parse!("Fri 01 2024"), date!(2024 - 005));
    assert_eq!(parse!("Sat 01 2024"), date!(2024 - 006));
    assert_eq!(parse!("Sun 01 2024"), date!(2024 - 007));
    assert_eq!(parse!("Wed 09 2024"), date!(2024 - 059));
    assert_eq!(parse!("Thu 09 2024"), date!(2024 - 060));
    assert_eq!(parse!("Fri 09 2024"), date!(2024 - 061));
    assert_eq!(parse!("Sat 09 2024"), date!(2024 - 062));
    assert_eq!(parse!("Sun 09 2024"), date!(2024 - 063));
    assert_eq!(parse!("Mon 10 2024"), date!(2024 - 064));
    assert_eq!(parse!("Tue 10 2024"), date!(2024 - 065));
    assert_eq!(parse!("Wed 10 2024"), date!(2024 - 066));
    assert_eq!(parse!("Thu 10 2024"), date!(2024 - 067));

    Ok(())
}

#[test]
fn test_parse_sunday_based_week() -> crate::Result<()> {
    macro_rules! parse {
        ($s:literal) => {
            Date::parse($s, "%a %U %Y")?
        };
    }

    // A
    assert_eq!(parse!("Sun 01 2018"), date!(2018 - 001));
    assert_eq!(parse!("Mon 01 2018"), date!(2018 - 002));
    assert_eq!(parse!("Tue 01 2018"), date!(2018 - 003));
    assert_eq!(parse!("Wed 01 2018"), date!(2018 - 004));
    assert_eq!(parse!("Thu 01 2018"), date!(2018 - 005));
    assert_eq!(parse!("Fri 01 2018"), date!(2018 - 006));
    assert_eq!(parse!("Sat 01 2018"), date!(2018 - 007));

    // B
    assert_eq!(parse!("Sat 00 2023"), date!(2023 - 001));
    assert_eq!(parse!("Sun 01 2023"), date!(2023 - 002));
    assert_eq!(parse!("Mon 01 2023"), date!(2023 - 003));
    assert_eq!(parse!("Tue 01 2023"), date!(2023 - 004));
    assert_eq!(parse!("Wed 01 2023"), date!(2023 - 005));
    assert_eq!(parse!("Thu 01 2023"), date!(2023 - 006));
    assert_eq!(parse!("Fri 01 2023"), date!(2023 - 007));

    // C
    assert_eq!(parse!("Fri 00 2022"), date!(2022 - 001));
    assert_eq!(parse!("Sat 00 2022"), date!(2022 - 002));
    assert_eq!(parse!("Sun 01 2022"), date!(2022 - 003));
    assert_eq!(parse!("Mon 01 2022"), date!(2022 - 004));
    assert_eq!(parse!("Tue 01 2022"), date!(2022 - 005));
    assert_eq!(parse!("Wed 01 2022"), date!(2022 - 006));
    assert_eq!(parse!("Thu 01 2022"), date!(2022 - 007));

    // D
    assert_eq!(parse!("Thu 00 2021"), date!(2021 - 001));
    assert_eq!(parse!("Fri 00 2021"), date!(2021 - 002));
    assert_eq!(parse!("Sat 00 2021"), date!(2021 - 003));
    assert_eq!(parse!("Sun 01 2021"), date!(2021 - 004));
    assert_eq!(parse!("Mon 01 2021"), date!(2021 - 005));
    assert_eq!(parse!("Tue 01 2021"), date!(2021 - 006));
    assert_eq!(parse!("Wed 01 2021"), date!(2021 - 007));

    // E
    assert_eq!(parse!("Wed 00 2026"), date!(2026 - 001));
    assert_eq!(parse!("Thu 00 2026"), date!(2026 - 002));
    assert_eq!(parse!("Fri 00 2026"), date!(2026 - 003));
    assert_eq!(parse!("Sat 00 2026"), date!(2026 - 004));
    assert_eq!(parse!("Sun 01 2026"), date!(2026 - 005));
    assert_eq!(parse!("Mon 01 2026"), date!(2026 - 006));
    assert_eq!(parse!("Tue 01 2026"), date!(2026 - 007));

    // F
    assert_eq!(parse!("Tue 00 2025"), date!(2025 - 001));
    assert_eq!(parse!("Wed 00 2025"), date!(2025 - 002));
    assert_eq!(parse!("Thu 00 2025"), date!(2025 - 003));
    assert_eq!(parse!("Fri 00 2025"), date!(2025 - 004));
    assert_eq!(parse!("Sat 00 2025"), date!(2025 - 005));
    assert_eq!(parse!("Sun 01 2025"), date!(2025 - 006));
    assert_eq!(parse!("Mon 01 2025"), date!(2025 - 007));

    // G
    assert_eq!(parse!("Mon 00 2019"), date!(2019 - 001));
    assert_eq!(parse!("Tue 00 2019"), date!(2019 - 002));
    assert_eq!(parse!("Wed 00 2019"), date!(2019 - 003));
    assert_eq!(parse!("Thu 00 2019"), date!(2019 - 004));
    assert_eq!(parse!("Fri 00 2019"), date!(2019 - 005));
    assert_eq!(parse!("Sat 00 2019"), date!(2019 - 006));
    assert_eq!(parse!("Sun 01 2019"), date!(2019 - 007));

    // AG
    assert_eq!(parse!("Sun 01 2024"), date!(2024 - 001));
    assert_eq!(parse!("Mon 01 2024"), date!(2024 - 002));
    assert_eq!(parse!("Tue 01 2024"), date!(2024 - 003));
    assert_eq!(parse!("Wed 01 2024"), date!(2024 - 004));
    assert_eq!(parse!("Thu 01 2024"), date!(2024 - 005));
    assert_eq!(parse!("Fri 01 2024"), date!(2024 - 006));
    assert_eq!(parse!("Sat 01 2024"), date!(2024 - 007));
    assert_eq!(parse!("Tue 09 2024"), date!(2024 - 059));
    assert_eq!(parse!("Wed 09 2024"), date!(2024 - 060));
    assert_eq!(parse!("Thu 09 2024"), date!(2024 - 061));
    assert_eq!(parse!("Fri 09 2024"), date!(2024 - 062));
    assert_eq!(parse!("Sat 09 2024"), date!(2024 - 063));
    assert_eq!(parse!("Sun 10 2024"), date!(2024 - 064));
    assert_eq!(parse!("Mon 10 2024"), date!(2024 - 065));
    assert_eq!(parse!("Tue 10 2024"), date!(2024 - 066));
    assert_eq!(parse!("Wed 10 2024"), date!(2024 - 067));

    // BA
    assert_eq!(parse!("Sat 00 2012"), date!(2012 - 001));
    assert_eq!(parse!("Sun 01 2012"), date!(2012 - 002));
    assert_eq!(parse!("Mon 01 2012"), date!(2012 - 003));
    assert_eq!(parse!("Tue 01 2012"), date!(2012 - 004));
    assert_eq!(parse!("Wed 01 2012"), date!(2012 - 005));
    assert_eq!(parse!("Thu 01 2012"), date!(2012 - 006));
    assert_eq!(parse!("Fri 01 2012"), date!(2012 - 007));
    assert_eq!(parse!("Mon 09 2012"), date!(2012 - 059));
    assert_eq!(parse!("Tue 09 2012"), date!(2012 - 060));
    assert_eq!(parse!("Wed 09 2012"), date!(2012 - 061));
    assert_eq!(parse!("Thu 09 2012"), date!(2012 - 062));
    assert_eq!(parse!("Fri 09 2012"), date!(2012 - 063));
    assert_eq!(parse!("Sat 09 2012"), date!(2012 - 064));
    assert_eq!(parse!("Sun 10 2012"), date!(2012 - 065));
    assert_eq!(parse!("Mon 10 2012"), date!(2012 - 066));
    assert_eq!(parse!("Tue 10 2012"), date!(2012 - 067));

    // CB
    assert_eq!(parse!("Fri 00 2028"), date!(2028 - 001));
    assert_eq!(parse!("Sat 00 2028"), date!(2028 - 002));
    assert_eq!(parse!("Sun 01 2028"), date!(2028 - 003));
    assert_eq!(parse!("Mon 01 2028"), date!(2028 - 004));
    assert_eq!(parse!("Tue 01 2028"), date!(2028 - 005));
    assert_eq!(parse!("Wed 01 2028"), date!(2028 - 006));
    assert_eq!(parse!("Thu 01 2028"), date!(2028 - 007));
    assert_eq!(parse!("Sun 09 2028"), date!(2028 - 059));
    assert_eq!(parse!("Mon 09 2028"), date!(2028 - 060));
    assert_eq!(parse!("Tue 09 2028"), date!(2028 - 061));
    assert_eq!(parse!("Wed 09 2028"), date!(2028 - 062));
    assert_eq!(parse!("Thu 09 2028"), date!(2028 - 063));
    assert_eq!(parse!("Fri 09 2028"), date!(2028 - 064));
    assert_eq!(parse!("Sat 09 2028"), date!(2028 - 065));
    assert_eq!(parse!("Sun 10 2028"), date!(2028 - 066));
    assert_eq!(parse!("Mon 10 2028"), date!(2028 - 067));

    // DC
    assert_eq!(parse!("Thu 00 2016"), date!(2016 - 001));
    assert_eq!(parse!("Fri 00 2016"), date!(2016 - 002));
    assert_eq!(parse!("Sat 00 2016"), date!(2016 - 003));
    assert_eq!(parse!("Sun 01 2016"), date!(2016 - 004));
    assert_eq!(parse!("Mon 01 2016"), date!(2016 - 005));
    assert_eq!(parse!("Tue 01 2016"), date!(2016 - 006));
    assert_eq!(parse!("Wed 01 2016"), date!(2016 - 007));
    assert_eq!(parse!("Sat 08 2016"), date!(2016 - 059));
    assert_eq!(parse!("Sun 09 2016"), date!(2016 - 060));
    assert_eq!(parse!("Mon 09 2016"), date!(2016 - 061));
    assert_eq!(parse!("Tue 09 2016"), date!(2016 - 062));
    assert_eq!(parse!("Wed 09 2016"), date!(2016 - 063));
    assert_eq!(parse!("Thu 09 2016"), date!(2016 - 064));
    assert_eq!(parse!("Fri 09 2016"), date!(2016 - 065));
    assert_eq!(parse!("Sat 09 2016"), date!(2016 - 066));
    assert_eq!(parse!("Sun 10 2016"), date!(2016 - 067));

    // ED
    assert_eq!(parse!("Wed 00 2032"), date!(2032 - 001));
    assert_eq!(parse!("Thu 00 2032"), date!(2032 - 002));
    assert_eq!(parse!("Fri 00 2032"), date!(2032 - 003));
    assert_eq!(parse!("Sat 00 2032"), date!(2032 - 004));
    assert_eq!(parse!("Sun 01 2032"), date!(2032 - 005));
    assert_eq!(parse!("Mon 01 2032"), date!(2032 - 006));
    assert_eq!(parse!("Tue 01 2032"), date!(2032 - 007));
    assert_eq!(parse!("Fri 08 2032"), date!(2032 - 059));
    assert_eq!(parse!("Sat 08 2032"), date!(2032 - 060));
    assert_eq!(parse!("Sun 09 2032"), date!(2032 - 061));
    assert_eq!(parse!("Mon 09 2032"), date!(2032 - 062));
    assert_eq!(parse!("Tue 09 2032"), date!(2032 - 063));
    assert_eq!(parse!("Wed 09 2032"), date!(2032 - 064));
    assert_eq!(parse!("Thu 09 2032"), date!(2032 - 065));
    assert_eq!(parse!("Fri 09 2032"), date!(2032 - 066));
    assert_eq!(parse!("Sat 09 2032"), date!(2032 - 067));

    // FE
    assert_eq!(parse!("Tue 00 2020"), date!(2020 - 001));
    assert_eq!(parse!("Wed 00 2020"), date!(2020 - 002));
    assert_eq!(parse!("Thu 00 2020"), date!(2020 - 003));
    assert_eq!(parse!("Fri 00 2020"), date!(2020 - 004));
    assert_eq!(parse!("Sat 00 2020"), date!(2020 - 005));
    assert_eq!(parse!("Sun 01 2020"), date!(2020 - 006));
    assert_eq!(parse!("Mon 01 2020"), date!(2020 - 007));
    assert_eq!(parse!("Thu 08 2020"), date!(2020 - 059));
    assert_eq!(parse!("Fri 08 2020"), date!(2020 - 060));
    assert_eq!(parse!("Sat 08 2020"), date!(2020 - 061));
    assert_eq!(parse!("Sun 09 2020"), date!(2020 - 062));
    assert_eq!(parse!("Mon 09 2020"), date!(2020 - 063));
    assert_eq!(parse!("Tue 09 2020"), date!(2020 - 064));
    assert_eq!(parse!("Wed 09 2020"), date!(2020 - 065));
    assert_eq!(parse!("Thu 09 2020"), date!(2020 - 066));
    assert_eq!(parse!("Fri 09 2020"), date!(2020 - 067));

    // GF
    assert_eq!(parse!("Mon 00 2036"), date!(2036 - 001));
    assert_eq!(parse!("Tue 00 2036"), date!(2036 - 002));
    assert_eq!(parse!("Wed 00 2036"), date!(2036 - 003));
    assert_eq!(parse!("Thu 00 2036"), date!(2036 - 004));
    assert_eq!(parse!("Fri 00 2036"), date!(2036 - 005));
    assert_eq!(parse!("Sat 00 2036"), date!(2036 - 006));
    assert_eq!(parse!("Sun 01 2036"), date!(2036 - 007));
    assert_eq!(parse!("Wed 08 2036"), date!(2036 - 059));
    assert_eq!(parse!("Thu 08 2036"), date!(2036 - 060));
    assert_eq!(parse!("Fri 08 2036"), date!(2036 - 061));
    assert_eq!(parse!("Sat 08 2036"), date!(2036 - 062));
    assert_eq!(parse!("Sun 09 2036"), date!(2036 - 063));
    assert_eq!(parse!("Mon 09 2036"), date!(2036 - 064));
    assert_eq!(parse!("Tue 09 2036"), date!(2036 - 065));
    assert_eq!(parse!("Wed 09 2036"), date!(2036 - 066));
    assert_eq!(parse!("Thu 09 2036"), date!(2036 - 067));

    Ok(())
}

#[test]
fn is_leap_year() -> crate::Result<()> {
    use time::is_leap_year;
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2004));
    assert!(!is_leap_year(2005));
    assert!(!is_leap_year(2100));
    Ok(())
}

#[test]
fn days_in_year() -> crate::Result<()> {
    use time::days_in_year;
    assert_eq!(days_in_year(1900), 365);
    assert_eq!(days_in_year(2000), 366);
    assert_eq!(days_in_year(2004), 366);
    assert_eq!(days_in_year(2005), 365);
    assert_eq!(days_in_year(2100), 365);
    Ok(())
}

#[test]
fn weeks_in_year() -> crate::Result<()> {
    use time::weeks_in_year;
    assert_eq!(weeks_in_year(2019), 52);
    assert_eq!(weeks_in_year(2020), 53);
    Ok(())
}

#[test]
#[cfg(feature = "std")]
#[allow(deprecated)]
fn today() {
    let _ = Date::today();
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn from_ymd() -> crate::Result<()> {
    assert_eq!(Date::from_ymd(2019, 1, 1), date!(2019 - 001));
    assert_eq!(Date::from_ymd(2019, 12, 31), date!(2019 - 365));
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn from_yo() -> crate::Result<()> {
    assert_eq!(Date::from_yo(2019, 1), date!(2019 - 01 - 01));
    assert_eq!(Date::from_yo(2019, 365), date!(2019 - 12 - 31));
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn from_iso_ywd() -> crate::Result<()> {
    use Weekday::*;
    assert_eq!(Date::from_iso_ywd(2019, 1, Monday), date!(2018 - 12 - 31));
    assert_eq!(Date::from_iso_ywd(2019, 1, Tuesday), date!(2019 - 01 - 01));
    assert_eq!(Date::from_iso_ywd(2020, 53, Friday), date!(2021 - 01 - 01));
    Ok(())
}

#[test]
fn try_from_iso_ywd() -> crate::Result<()> {
    use Weekday::*;
    assert!(Date::try_from_iso_ywd(2019, 1, Monday).is_ok());
    assert!(Date::try_from_iso_ywd(2019, 1, Tuesday).is_ok());
    assert!(Date::try_from_iso_ywd(2020, 53, Friday).is_ok());
    assert!(Date::try_from_iso_ywd(2019, 53, Monday).is_err()); // 2019 doesn't have 53 weeks.
    Ok(())
}

#[test]
fn year() -> crate::Result<()> {
    assert_eq!(date!(2019 - 002).year(), 2019);
    assert_eq!(date!(2020 - 002).year(), 2020);
    Ok(())
}

#[test]
fn month() -> crate::Result<()> {
    assert_eq!(date!(2019 - 002).month(), 1);
    assert_eq!(date!(2020 - 002).month(), 1);
    assert_eq!(date!(2019 - 060).month(), 3);
    assert_eq!(date!(2020 - 060).month(), 2);
    Ok(())
}

#[test]
fn day() -> crate::Result<()> {
    assert_eq!(date!(2019 - 002).day(), 2);
    assert_eq!(date!(2020 - 002).day(), 2);
    assert_eq!(date!(2019 - 060).day(), 1);
    assert_eq!(date!(2020 - 060).day(), 29);
    Ok(())
}

#[test]
fn iso_year_week() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01).iso_year_week(), (2019, 1));
    assert_eq!(date!(2019 - 10 - 04).iso_year_week(), (2019, 40));
    assert_eq!(date!(2020 - 01 - 01).iso_year_week(), (2020, 1));
    assert_eq!(date!(2020 - 12 - 31).iso_year_week(), (2020, 53));
    assert_eq!(date!(2021 - 01 - 01).iso_year_week(), (2020, 53));
    Ok(())
}

#[test]
fn week() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01).week(), 1);
    assert_eq!(date!(2019 - 10 - 04).week(), 40);
    assert_eq!(date!(2020 - 01 - 01).week(), 1);
    assert_eq!(date!(2020 - 12 - 31).week(), 53);
    assert_eq!(date!(2021 - 01 - 01).week(), 53);
    Ok(())
}

#[test]
fn as_ymd() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 02).as_ymd(), (2019, 1, 2));
    Ok(())
}

#[test]
fn as_yo() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01).as_yo(), (2019, 1));
    Ok(())
}

#[test]
fn next_day() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01).next_day(), date!(2019 - 01 - 02));
    assert_eq!(date!(2019 - 01 - 31).next_day(), date!(2019 - 02 - 01));
    assert_eq!(date!(2019 - 12 - 31).next_day(), date!(2020 - 01 - 01));
    Ok(())
}

#[test]
fn previous_day() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 02).previous_day(), date!(2019 - 01 - 01));
    assert_eq!(date!(2019 - 02 - 01).previous_day(), date!(2019 - 01 - 31));
    assert_eq!(date!(2020 - 01 - 01).previous_day(), date!(2019 - 12 - 31));
    Ok(())
}

#[test]
fn julian_day() -> crate::Result<()> {
    assert_eq!(date!(-4713 - 11 - 24).julian_day(), 0);
    assert_eq!(date!(2000 - 01 - 01).julian_day(), 2_451_545);
    assert_eq!(date!(2019 - 01 - 01).julian_day(), 2_458_485);
    assert_eq!(date!(2019 - 12 - 31).julian_day(), 2_458_849);
    Ok(())
}

#[test]
fn from_julian_day() -> crate::Result<()> {
    assert_eq!(julian!(0), date!(-4713 - 11 - 24));
    assert_eq!(julian!(2_451_545), date!(2000 - 01 - 01));
    assert_eq!(julian!(2_458_485), date!(2019 - 01 - 01));
    assert_eq!(julian!(2_458_849), date!(2019 - 12 - 31));
    Ok(())
}

#[test]
#[should_panic]
fn from_julian_day_large() {
    Date::from_julian_day(i64::MAX);
}

#[test]
fn midnight() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).midnight(),
        date!(1970 - 01 - 01).with_time(time!(0:00)),
    );
    Ok(())
}

#[test]
fn with_time() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).with_time(time!(0:00)),
        date!(1970 - 01 - 01).midnight(),
    );
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn with_hms() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).with_hms(0, 0, 0),
        date!(1970 - 01 - 01).midnight(),
    );
    Ok(())
}

#[test]
fn try_with_hms() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).try_with_hms(0, 0, 0),
        Ok(date!(1970 - 01 - 01).midnight()),
    );
    assert!(date!(1970 - 01 - 01).try_with_hms(24, 0, 0).is_err());
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn with_hms_milli() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).with_hms_milli(0, 0, 0, 0),
        date!(1970 - 01 - 01).midnight(),
    );
    Ok(())
}

#[test]
fn try_with_hms_milli() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).try_with_hms_milli(0, 0, 0, 0),
        Ok(date!(1970 - 01 - 01).midnight()),
    );
    assert!(date!(1970 - 01 - 01)
        .try_with_hms_milli(24, 0, 0, 0)
        .is_err());
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn with_hms_micro() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).with_hms_micro(0, 0, 0, 0),
        date!(1970 - 01 - 01).midnight(),
    );
    Ok(())
}

#[test]
fn try_with_hms_micro() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).try_with_hms_micro(0, 0, 0, 0),
        Ok(date!(1970 - 01 - 01).midnight()),
    );
    assert!(date!(1970 - 01 - 01)
        .try_with_hms_micro(24, 0, 0, 0)
        .is_err());
    Ok(())
}

#[test]
#[cfg(feature = "panicking-api")]
#[allow(deprecated)]
fn with_hms_nano() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).with_hms_nano(0, 0, 0, 0),
        date!(1970 - 01 - 01).midnight(),
    );
    Ok(())
}

#[test]
fn try_with_hms_nano() -> crate::Result<()> {
    assert_eq!(
        date!(1970 - 01 - 01).try_with_hms_nano(0, 0, 0, 0),
        Ok(date!(1970 - 01 - 01).midnight()),
    );
    assert!(date!(1970 - 01 - 01)
        .try_with_hms_nano(24, 0, 0, 0)
        .is_err());
    Ok(())
}

#[test]
fn format() -> crate::Result<()> {
    // Check all specifiers for date objects.
    let date = date!(2019 - 01 - 02);
    assert_eq!(date.format("%a"), "Wed");
    assert_eq!(date.format("%A"), "Wednesday");
    assert_eq!(date.format("%b"), "Jan");
    assert_eq!(date.format("%B"), "January");
    assert_eq!(date.format("%C"), "20");
    assert_eq!(date.format("%d"), "02");
    assert_eq!(date.format("%D"), "1/02/19");
    assert_eq!(date.format("%F"), "2019-01-02");
    assert_eq!(date.format("%g"), "19");
    assert_eq!(date.format("%G"), "2019");
    assert_eq!(date.format("%j"), "002");
    assert_eq!(date.format("%m"), "01");
    assert_eq!(date.format("%u"), "3");
    assert_eq!(date.format("%U"), "00");
    assert_eq!(date.format("%V"), "01");
    assert_eq!(date.format("%w"), "3");
    assert_eq!(date.format("%W"), "00");
    assert_eq!(date.format("%y"), "19");
    assert_eq!(date.format("%Y"), "2019");

    // Ensure the sign is emitted correctly for all year specifiers.
    let date = date!(10_000 - 01 - 03);
    assert_eq!(date.format("%G"), "+10000");
    assert_eq!(date.format("%Y"), "+10000");

    Ok(())
}

#[test]
fn parse() -> crate::Result<()> {
    // Check all specifiers for date objects. To ensure that the date parses
    // successfully otherwise, additional data is provided.
    let date = date!(2019 - 01 - 02);
    assert_eq!(Date::parse("2019-01-02 Wed", "%F %a"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 Wednesday", "%F %A"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 Jan", "%F %b"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 January", "%F %B"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 20", "%F %C"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 02", "%F %d"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 1/02/19", "%F %D"), Ok(date));
    assert_eq!(Date::parse("2019-01-02", "%F"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 19", "%F %g"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 2019", "%F %G"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 002", "%F %j"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 01", "%F %m"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 3", "%F %u"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 00", "%F %U"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 01", "%F %V"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 3", "%F %w"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 00", "%F %W"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 19", "%F %y"), Ok(date));
    assert_eq!(Date::parse("2019-01-02 2019", "%F %Y"), Ok(date));

    // Additional coverage
    assert_eq!(
        Date::parse("", ""),
        Err(error::Parse::InsufficientInformation)
    );

    Ok(())
}

// See #221.
#[test]
fn parse_regression() -> crate::Result<()> {
    assert_eq!(
        Date::parse("0000-01-01", "%Y-%m-%d"),
        Ok(date!(0000 - 01 - 01))
    );
    Ok(())
}

#[test]
fn display() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01).to_string(), "2019-01-01");
    assert_eq!(date!(2019 - 12 - 31).to_string(), "2019-12-31");
    assert_eq!(date!(-4713 - 11 - 24).to_string(), "-4713-11-24");
    assert_eq!(date!(10_000 - 01 - 01).to_string(), "+10000-01-01");
    Ok(())
}

#[test]
fn add() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01) + 5.days(), date!(2019 - 01 - 06));
    assert_eq!(date!(2019 - 12 - 31) + 1.days(), date!(2020 - 01 - 01));
    Ok(())
}

#[test]
fn add_std() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 01) + 5.std_days(), date!(2019 - 01 - 06));
    assert_eq!(date!(2019 - 12 - 31) + 1.std_days(), date!(2020 - 01 - 01));
    Ok(())
}

#[test]
fn add_assign() -> crate::Result<()> {
    let mut date = date!(2019 - 12 - 31);
    date += 1.days();
    assert_eq!(date, date!(2020 - 01 - 01));
    Ok(())
}

#[test]
fn add_assign_std() -> crate::Result<()> {
    let mut date = date!(2019 - 12 - 31);
    date += 1.std_days();
    assert_eq!(date, date!(2020 - 01 - 01));
    Ok(())
}

#[test]
fn sub() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 06) - 5.days(), date!(2019 - 01 - 01));
    assert_eq!(date!(2020 - 01 - 01) - 1.days(), date!(2019 - 12 - 31));
    Ok(())
}

#[test]
fn sub_std() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 06) - 5.std_days(), date!(2019 - 01 - 01));
    assert_eq!(date!(2020 - 01 - 01) - 1.std_days(), date!(2019 - 12 - 31));
    Ok(())
}

#[test]
fn sub_assign() -> crate::Result<()> {
    let mut date = date!(2020 - 01 - 01);
    date -= 1.days();
    assert_eq!(date, date!(2019 - 12 - 31));
    Ok(())
}

#[test]
fn sub_assign_std() -> crate::Result<()> {
    let mut date = date!(2020 - 01 - 01);
    date -= 1.std_days();
    assert_eq!(date, date!(2019 - 12 - 31));
    Ok(())
}

#[test]
fn sub_self() -> crate::Result<()> {
    assert_eq!(date!(2019 - 01 - 06) - date!(2019 - 01 - 01), 5.days());
    assert_eq!(date!(2020 - 01 - 01) - date!(2019 - 12 - 31), 1.days());
    Ok(())
}

#[test]
fn partial_ord() -> crate::Result<()> {
    let first = date!(2019 - 01 - 01);
    let second = date!(2019 - 01 - 02);

    assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
    assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
    Ok(())
}

#[test]
fn ord() -> crate::Result<()> {
    let first = date!(2019 - 01 - 01);
    let second = date!(2019 - 01 - 02);

    assert_eq!(first.cmp(&first), Ordering::Equal);
    assert_eq!(first.cmp(&second), Ordering::Less);
    assert_eq!(second.cmp(&first), Ordering::Greater);
    Ok(())
}

#[test]
#[should_panic]
fn next_day_panics() {
    date!(+100_000-12-31).next_day();
}

#[test]
#[should_panic]
fn previous_day_panics() {
    date!(-100_000 - 01 - 01).previous_day();
}

#[test]
#[should_panic]
fn julian_day_panics() {
    Date::from_julian_day(i64::MAX);
}
