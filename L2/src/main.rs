#[derive(Debug, Clone, Default)]
struct NumberWithUnit {
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        NumberWithUnit {
            unit: String::new(),
            value
        }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        NumberWithUnit {
            unit,
            value
        }
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        NumberWithUnit {
            unit: other.unit,
            value
        }
    }
    
    fn add(self, other: Self) -> Self {
        if self.unit == other.unit {
            NumberWithUnit::with_unit(self.value + other.value, self.unit)
        } else { 
            panic!()
        }
    }
    
    fn mul(self, other: Self) -> Self {
        NumberWithUnit::with_unit(self.value * other.value, format!("{}*{}", self.unit, other.unit))
    }
    
    fn div(self, other: Self) -> Self {
        NumberWithUnit::with_unit(self.value / other.value, format!("{}/{}", self.unit, other.unit))
    }
    
    fn add_in_place(&mut self, other: &Self) {
        if self.unit == other.unit {
            self.value += other.value;
        }
    }
    
    fn mul_in_place(&mut self, other: &Self) {
        self.value *= other.value;
        self.unit = format!("{}*{}", self.unit, other.unit);
    }
    
    fn div_in_place(&mut self, other: &Self) {
        self.value /= other.value;
        self.unit = format!("{}/{}", self.unit, other.unit);
    }
}

fn mul_vals(arr: &[NumberWithUnit]) -> NumberWithUnit {
    let mut ret:NumberWithUnit = arr[0].clone();

    for nu in arr.iter().skip(1) {
        ret.mul_in_place(nu);
    };

    ret
}

fn mul_vals_vec(arr: Vec<NumberWithUnit>) -> NumberWithUnit {
    let mut ret = arr[0].clone();
    for nu in arr.iter().skip(1) {
        ret.mul_in_place(nu);
    };
    ret
}

struct DoubleString(String, String);

impl DoubleString {
    fn from_strs(str1: &str, str2: &str) -> Self {
        DoubleString(str1.to_string(), str2.to_string())
    }
    
    fn from_strings(str1: &String, str2: &String) -> Self {
        DoubleString(str1.clone(), str2.clone())
    }
    
    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}

fn main() {
    let a = NumberWithUnit::unitless(3.0);
    println!("a = {} {}", a.value, a.unit);
    let b = NumberWithUnit::with_unit(4.0, String::from("kg"));
    println!("b = {} {}", b.value, b.unit);
    let c = NumberWithUnit::with_unit_from(b.clone(), 5.0);
    println!("c = {} {}", c.value, c.unit);
    
    let odleglosc = NumberWithUnit::with_unit(5.0, String::from("m"));
    let czas = NumberWithUnit::with_unit(1.0, String::from("s"));
    let predkosc = odleglosc.div(czas);
    println!("{predkosc:?}");
    
    let kg1 = NumberWithUnit::with_unit(1.0, String::from("kg"));
    let kg2 = NumberWithUnit::with_unit(2.0, String::from("kg"));
    let kg3 = kg1.add(kg2);
    let kg4 = NumberWithUnit::with_unit(5.0, String::from("kg"));
    let mut kg5 = kg3.mul(kg4);
    let kg6 = NumberWithUnit::with_unit(1.0, String::from("kg"));
    kg5.add_in_place(&kg6);
    let kg7 = NumberWithUnit::with_unit_from(kg5.clone(), 5.0);
    kg5.div_in_place(&kg7);
    
    let vec: Vec<NumberWithUnit> = vec![b, c, predkosc];
    mul_vals(&vec);
    mul_vals(&vec);
    mul_vals_vec(vec.clone());
    mul_vals_vec(vec.clone());
    
    
    let string: String = String::from("elo");
    let str_slice: &str = "pa";
    
    let x = DoubleString::from_strs(&string, str_slice);
    x.show();
    let y = DoubleString::from_strings(&string, &str_slice.to_owned());
    y.show();
}
