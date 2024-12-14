use rand::{distributions::Distribution, random, thread_rng, Rng};

fn random_by_value(max_value: usize) -> usize {
	let mut rng = thread_rng();
	rng.gen_range(0..=max_value)
}

#[derive(Debug)]
pub enum FormulaOp {
	Add,
	Minus,
}

impl FormulaOp {
	pub const fn to_str(&self) -> &'static str {
		match self {
			Self::Add => "+",
			Self::Minus => "-",
		}
	}
}

#[derive(Debug)]
pub enum Term<T> {
	Value(usize),
	Placeholder(T),
}

#[derive(Debug)]
pub struct Formula<T> {
	pub lhs: Term<T>,
	pub op: FormulaOp,
	pub rhs: Term<T>,
	pub result: Term<T>,
}

struct RndPlaceholder {
	a: usize,
	b: usize,
	c: usize,
}

impl RndPlaceholder {
	const fn new(a: usize, b: usize, c: usize) -> Self {
		Self { a, b, c }
	}
}

impl Distribution<(Term<usize>, Term<usize>, Term<usize>)> for RndPlaceholder {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (Term<usize>, Term<usize>, Term<usize>) {
		match rng.gen_range(1..=3) {
			1 => (
				Term::Placeholder(self.a),
				Term::Value(self.b),
				Term::Value(self.c),
			),
			2 => (
				Term::Value(self.a),
				Term::Placeholder(self.b),
				Term::Value(self.c),
			),
			_ => (
				Term::Value(self.a),
				Term::Value(self.b),
				Term::Placeholder(self.c),
			),
		}
	}
}

impl Formula<usize> {
	pub fn new(max_value: usize) -> Self {
		let a = random_by_value(max_value);
		let b = random_by_value(max_value - a);
		let c = a + b;
		let mut rng = thread_rng();

		if random() {
			// 加法
			let (x, y, z) = RndPlaceholder::new(a, b, c).sample(&mut rng);
			Self {
				lhs: x,
				op: FormulaOp::Add,
				rhs: y,
				result: z,
			}
		} else {
			let (x, y, z) = RndPlaceholder::new(c, a, b).sample(&mut rng);
			Self {
				lhs: x,
				op: FormulaOp::Minus,
				rhs: y,
				result: z,
			}
		}
	}
}
