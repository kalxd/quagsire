use rand::{random, thread_rng, Rng};

fn random_by_value(max_value: usize) -> usize {
	let mut rng = thread_rng();
	rng.gen_range(1..=max_value)
}

#[derive(Debug)]
enum FormulaOp {
	Add,
	Minus,
}

impl FormulaOp {
	const fn to_str(&self) -> &'static str {
		match self {
			Self::Add => "+",
			Self::Minus => "-",
		}
	}
}

#[derive(Debug)]
enum Term {
	Value(usize),
	Placeholder,
}

#[derive(Debug)]
struct Formula {
	lhs: Term,
	op: FormulaOp,
	rhs: Term,
	result: Term,
}

impl Formula {
	pub fn new(max_value: usize) -> Self {
		let a = random_by_value(max_value);
		let b = random_by_value(max_value - a);
		let c = a + b;
		todo!()
	}
}
