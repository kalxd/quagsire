mod term_imp {
	use crate::formula::Term;
	use gtk4::glib::{self, prelude::*, subclass::prelude::*};
	use std::cell::RefCell;

	#[derive(Default, glib::Properties)]
	#[properties(wrapper_type = super::TermObj)]
	pub struct InnerTerm {
		#[property(get, set, type = Term)]
		term: RefCell<Term>,
	}

	#[glib::derived_properties]
	impl ObjectImpl for InnerTerm {}

	#[glib::object_subclass]
	impl ObjectSubclass for InnerTerm {
		const NAME: &str = "term";
		type Type = super::TermObj;
	}
}

mod formula_imp {
	use crate::formula::{FormulaOp, Term};
	use gtk4::glib::{self, prelude::*, subclass::prelude::*};
	use std::cell::RefCell;

	#[derive(Default, glib::Properties)]
	#[properties(wrapper_type = super::FormulaObj)]
	pub struct InnerFormula {
		#[property(get, set, type = Term)]
		lhs: RefCell<Term>,
		#[property(get, type = FormulaOp)]
		op: FormulaOp,
		#[property(get, set, type= Term)]
		rhs: RefCell<Term>,
		#[property(get, set, type = Term)]
		reslt: RefCell<Term>,
	}

	#[glib::derived_properties]
	impl ObjectImpl for InnerFormula {}

	#[glib::object_subclass]
	impl ObjectSubclass for InnerFormula {
		const NAME: &str = "formula";
		type Type = super::FormulaObj;
	}
}

use crate::formula::{Formula, Term};
use gtk4::glib::{self, Object};

glib::wrapper! {
	pub struct TermObj(ObjectSubclass<term_imp::InnerTerm>);
}

impl TermObj {
	fn new(term: Term) -> Self {
		Object::builder().property("term", term).build()
	}
}

glib::wrapper! {
	pub struct FormulaObj(ObjectSubclass<formula_imp::InnerFormula>);
}

impl FormulaObj {
	pub fn new(formula: Formula) -> Self {
		Object::builder()
			.property("lhs", formula.lhs)
			.property("op", formula.op)
			.property("rhs", formula.rhs)
			.property("result", formula.result)
			.build()
	}
}
