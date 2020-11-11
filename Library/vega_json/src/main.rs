#[macro_use]
extern crate json;
extern crate peroxide;
use json::*;
use peroxide::*;
use std::fs::File;
use std::io::Error;

fn main() -> std::io::Result<()> {
    let mut a = DataFrame::with_header(vec!["x", "y"]);
    a["x"] = seq(1, 10, 1);
    a["y"] = a["x"].fmap(|x| x.powi(2));

    let mut x = Encoding::new();
    x.Field = "x".to_string();
    x.Type = Type::Quantitative;
    x.Axis = Axis { Title: "X Axis".to_string() };

    let mut y = Encoding::new();
    y.Field = "y".to_string();
    y.Type = Type::Quantitative;
    y.Axis = Axis { Title: "Y Axis".to_string() };

    let json = vega_generator(&a, x, y, Mark::Line);
    println!("{:#}", json);

    let buffer = File::create("data/test.vg.json")?;
    let mut buf_writer = std::io::BufWriter::new(buffer);

    json.write(&mut buf_writer)?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Mark {
    Point,
    Bar,
    Line,
    Tick,
    Rect,
}

#[derive(Debug, Clone)]
struct Encoding {
    pub Aggregate: Option<Aggregate>,
    pub Field: String,
    pub Time_unit: Option<TimeUnit>,
    pub Type: Type,
    pub Axis: Axis
}

#[derive(Debug, Clone)]
struct Axis {
    pub Title: String,
    // Ticks: Ticks,
}

impl Axis {
    fn new() -> Self {
        Axis {
            Title: String::default()
        }
    }

    fn to_json_values(&self) -> JsonValue {
        object! {
            "title" => self.Title.as_str()
        }
    }
}

// enum Ticks {
//     Ticks(bool),
//     TickBand(String),
//     TickColor(String),
//     TickCound(usize),
//     TickExtra()
// }

impl Encoding {
    fn new() -> Self {
        Encoding {
            Aggregate: None,
            Field: String::default(),
            Time_unit: None,
            Type: Type::Nominal,
            Axis: Axis::new(),
        }
    }

    fn to_json_values(&self) -> JsonValue {
        let mut data = object! {
            "type" => match self.Type {
                Type::Quantitative => "quantitative",
                Type::Nominal => "nominal",
                Type::Ordinal => "ordinal",
                Type::Temporal => "temporal",
                Type::GeoJson => "geojson"
            },
            "field" => self.Field.as_str(),
            "axis" => self.Axis.to_json_values(),
        };

        match self.Aggregate {
            None => (),
            Some(aggr) => {
                data.insert(
                    "aggregate", 
                    match aggr {
                        Aggregate::Count => "count",
                        Aggregate::Valid => "valid",
                        Aggregate::Missing => "missing",
                        Aggregate::Distinct => "distinct",
                        Aggregate::Sum => "sum",
                        Aggregate::Mean => "mean",
                        Aggregate::Average => "average",
                        Aggregate::Variance => "variance",
                        Aggregate::Variancep => "variancep",
                        Aggregate::Stdev => "stdev",
                        Aggregate::Stdevp => "stdevp",
                        Aggregate::Stderr => "stderr",
                        Aggregate::Median => "median",
                        Aggregate::Q1 => "q1",
                        Aggregate::Q3 => "q3",
                        Aggregate::Ci0 => "ci0",
                        Aggregate::Ci1 => "ci1",
                        Aggregate::Min => "min",
                        Aggregate::Max => "max",
                        Aggregate::Argmin => "argmin",
                        Aggregate::Argmax => "argmax"
                    }
                ).expect("Can't insert aggregate");
            }
        }

        match self.Time_unit {
            None => (),
            Some(time) => {
                data.insert(
                    "time",
                    match time {
                        TimeUnit::Year => "year",
                        TimeUnit::YearQuarter => "yearquarter",
                        TimeUnit::YearQuarterMonth => "yearquartermonth",
                        TimeUnit::YearMonth => "yearmonth",
                        TimeUnit::YearMonthDate => "yearmonthdate",
                        TimeUnit::YearMonthDateHours => "yearmonthdatehours",
                        TimeUnit::YearMonthDateHoursMinutes => "yearmonthdatehoursminutes",
                        TimeUnit::YearMonthDateHoursMinutesSeconds => "yearmonthdatehoursminutesseconds",
                        TimeUnit::Quarter => "quarter",
                        TimeUnit::QuarterMonth => "quartermonth",
                        TimeUnit::Month => "month",
                        TimeUnit::MonthDate => "monthdate",
                        TimeUnit::Date => "date",
                        TimeUnit::Day => "day",
                        TimeUnit::Hours => "hours",
                        TimeUnit::HoursMinutes => "hoursminutes",
                        TimeUnit::HoursMinutesSeconds => "hoursminutesseconds",
                        TimeUnit::Minutes => "minutes",
                        TimeUnit::MinutesSeconds => "minutesseconds",
                        TimeUnit::Seconds => "seconds",
                        TimeUnit::SecondsMilliSeconds => "secondsmilliseconds",
                        TimeUnit::MilliSeconds => "milliseconds",
                    }
                ).expect("Can't insert timeunit");
            }
        }

        data
    }
}

#[derive(Debug, Copy, Clone)]
enum Aggregate {
    Count,
    Valid,
    Missing,
    Distinct,
    Sum,
    Mean,
    Average,
    Variance,
    Variancep,
    Stdev,
    Stdevp,
    Stderr,
    Median,
    Q1,
    Q3,
    Ci0,
    Ci1,
    Min,
    Max,
    Argmin,
    Argmax,
}

#[derive(Debug, Copy, Clone)]
enum TimeUnit {
    Year,
    YearQuarter,
    YearQuarterMonth,
    YearMonth,
    YearMonthDate,
    YearMonthDateHours,
    YearMonthDateHoursMinutes,
    YearMonthDateHoursMinutesSeconds,
    Quarter,
    QuarterMonth,
    Month,
    MonthDate,
    Date,
    Day,
    Hours,
    HoursMinutes,
    HoursMinutesSeconds,
    Minutes,
    MinutesSeconds,
    Seconds,
    SecondsMilliSeconds,
    MilliSeconds,
}

#[derive(Debug, Copy, Clone)]
enum Type {
    Quantitative,
    Temporal,
    Ordinal,
    Nominal,
    GeoJson,
}

fn vega_generator(data: &DataFrame, x: Encoding, y: Encoding, mark: Mark) -> JsonValue {
    let mut values = Vec::<JsonValue>::new();
    for i in 0 .. data[x.Field.as_str()].len() {
        let mut row_object = JsonValue::new_object();
        for head in data.headers() {
            row_object.insert(head, data[head][i]).expect("Can't insert row object");
        }
        values.push(row_object);
    }
    let array = Array::from(values);

    object! {
        "data" => object! {
            "values" => array,
        },
        "mark" => match mark {
            Mark::Point => "point",
            Mark::Line => "line",
            Mark::Bar => "bar",
            Mark::Tick => "tick",
            Mark::Rect => "rect"
        },
        "encoding" => object! {
            "x" => x.to_json_values(),
            "y" => y.to_json_values(),
        }
    }
}