use gtk4::glib;
use rand::{distributions::Distribution, random, thread_rng, Rng};

fn random_by_value(max_value: usize) -> usize {
	let mut rng = thread_rng();
	rng.gen_range(0..=max_value)
}

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "FormulaOp")]
pub enum FormulaOp {
	Add,
	Minus,
}

impl Default for FormulaOp {
	fn default() -> Self {
		Self::Add
	}
}

impl FormulaOp {
	pub const fn to_str(&self) -> &'static str {
		match self {
			Self::Add => "+",
			Self::Minus => "-",
		}
	}
}

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "Term")]
pub enum Term {
	Value(usize),
	Placeholder(usize),
}

impl Default for Term {
	fn default() -> Self {
		Self::Value(Default::default())
	}
}

#[derive(Debug)]
pub struct Formula {
	pub lhs: Term,
	pub op: FormulaOp,
	pub rhs: Term,
	pub result: Term,
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

impl Distribution<(Term, Term, Term)> for RndPlaceholder {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (Term, Term, Term) {
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

impl Formula {
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
