use rand::{distributions::Distribution, random, thread_rng, Rng};

fn random_by_value(max_value: usize) -> usize {
	let mut rng = thread_rng();

	dbg!(max_value);

	rng.gen_range(0..=max_value)
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
pub struct Formula {
	lhs: Term,
	op: FormulaOp,
	rhs: Term,
	result: Term,
	answer: usize,
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

impl Distribution<(Term, Term, Term, usize)> for RndPlaceholder {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (Term, Term, Term, usize) {
		match rng.gen_range(1..=3) {
			1 => (
				Term::Placeholder,
				Term::Value(self.b),
				Term::Value(self.c),
				self.a,
			),
			2 => (
				Term::Value(self.a),
				Term::Placeholder,
				Term::Value(self.c),
				self.b,
			),
			_ => (
				Term::Value(self.a),
				Term::Value(self.b),
				Term::Placeholder,
				self.c,
			),
		}
	}
}

impl Formula {
	pub fn new(max_value: usize) -> Self {
		dbg!("==== do begining ===");

		let a = random_by_value(max_value);
		let b = random_by_value(max_value - a);
		let c = a + b;
		let mut rng = thread_rng();

		dbg!("=== do this? ===");

		if random() {
			// 加法
			let (x, y, z, p) = RndPlaceholder::new(a, b, c).sample(&mut rng);
			Self {
				lhs: x,
				op: FormulaOp::Add,
				rhs: y,
				result: z,
				answer: p,
			}
		} else {
			let (x, y, z, p) = RndPlaceholder::new(c, a, b).sample(&mut rng);
			Self {
				lhs: x,
				op: FormulaOp::Minus,
				rhs: y,
				result: z,
				answer: p,
			}
		}
	}
}
