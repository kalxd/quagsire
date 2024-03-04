use rand::{random, thread_rng, Rng};

fn random_by_value(max_value: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(1..=max_value)
}

#[derive(Debug)]
enum Value {
    Raw(usize),
    Placeholder,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Raw(v) => write!(f, "{:1$}", v, 2),
            Value::Placeholder => write!(f, "[]"),
        }
    }
}

impl Value {
    fn mark(value: usize, is_mark: bool) -> Self {
        if is_mark {
            Self::Placeholder
        } else {
            Self::Raw(value)
        }
    }
}

#[derive(Debug)]
struct Expr((Value, String, Value, Value));

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} = {}",
            self.0 .0, self.0 .1, self.0 .2, self.0 .3
        )
    }
}

struct GenExpr(usize);

impl GenExpr {
    fn gen_random(&self) -> Expr {
        let total: usize = random_by_value(self.0);
        let a: usize = random_by_value(total);
        let b: usize = total - a;

        let n: usize = random_by_value(2); // 遮蔽哪个元素

        let r = if random() {
            // 加法
            (
                Value::mark(a, n == 0),
                "+".into(),
                Value::mark(b, n == 1),
                Value::mark(total, n == 2),
            )
        } else {
            (
                Value::mark(total, n == 0),
                "-".into(),
                Value::mark(a, n == 1),
                Value::mark(b, n == 2),
            )
        };

        Expr(r)
    }
}

fn read_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().expect("请输入一个正整数")
}

fn main() {
    let n = read_usize();

    for _ in 0..n {
        let gen = GenExpr(100);
        let r = gen.gen_random();
        println!("{r}");
    }
}
