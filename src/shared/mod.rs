pub trait Day {
    fn part_1(&self) -> PartSolution;
    fn part_2(&self) -> PartSolution;
}

#[derive(PartialEq, Eq, Debug)]
pub enum PartSolution {
    I32(i32),
    U32(u32),
    U64(u64),
    USize(usize),
    String(String),
    Vec(Vec<String>),
    #[allow(dead_code)]
    None,
}

impl From<i32> for PartSolution {
    fn from(v: i32) -> Self {
        PartSolution::I32(v)
    }
}

impl From<u32> for PartSolution {
    fn from(v: u32) -> Self {
        PartSolution::U32(v)
    }
}

impl From<u64> for PartSolution {
    fn from(v: u64) -> Self {
        PartSolution::U64(v)
    }
}

impl From<usize> for PartSolution {
    fn from(v: usize) -> Self {
        PartSolution::USize(v)
    }
}

impl From<Vec<String>> for PartSolution {
    fn from(v: Vec<String>) -> Self {
        PartSolution::Vec(v)
    }
}

impl From<String> for PartSolution {
    fn from(v: String) -> Self {
        PartSolution::String(v)
    }
}

impl std::fmt::Display for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            PartSolution::I32(x) => x.to_string(),
            PartSolution::U32(x) => x.to_string(),
            PartSolution::U64(x) => x.to_string(),
            PartSolution::USize(x) => x.to_string(),
            PartSolution::String(x) => x.to_string(),
            PartSolution::Vec(x) => format!("\n{}", x.join("\n")),
            PartSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}

impl std::cmp::PartialEq<PartSolution> for i32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => self == x,
            PartSolution::U32(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            PartSolution::U64(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: i32| &o == self),
            PartSolution::String(_) | PartSolution::Vec(_) | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: u32| &o == self),
            PartSolution::U32(x) => self == x,
            PartSolution::U64(x) => (x) == &u64::from(*self),
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: u32| &o == self),
            PartSolution::String(_) | PartSolution::Vec(_) | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u64 {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: u64| &o == self),
            PartSolution::U32(x) => &u64::from(*x) == self,
            PartSolution::U64(x) => x == self,
            PartSolution::USize(x) => (*x).try_into().is_ok_and(|o: u64| &o == self),
            PartSolution::String(_) | PartSolution::Vec(_) | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for usize {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::U32(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::U64(x) => (*x).try_into().is_ok_and(|o: usize| &o == self),
            PartSolution::USize(x) => x == self,
            PartSolution::String(_) | PartSolution::Vec(_) | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for String {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(_)
            | PartSolution::U32(_)
            | PartSolution::U64(_)
            | PartSolution::Vec(_)
            | PartSolution::None
            | PartSolution::USize(_) => false,
            PartSolution::String(s) => s == self,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for Vec<String> {
    fn eq(&self, other: &PartSolution) -> bool {
        match other {
            PartSolution::I32(_)
            | PartSolution::U32(_)
            | PartSolution::U64(_)
            | PartSolution::None
            | PartSolution::USize(_)
            | PartSolution::String(_) => false,

            PartSolution::Vec(v) => {
                if v.len() != self.len() {
                    return false;
                }

                for (l, r) in self.iter().zip(v) {
                    if l != r {
                        return false;
                    }
                }

                true
            },
        }
    }
}
