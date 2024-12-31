use crate::formula::{Formula, Term};
use gtk4::{
	glib::{self, Object},
	prelude::ListModelExt,
	subclass::prelude::ObjectSubclassIsExt,
};

#[derive(Clone, glib::Boxed)]
#[boxed_type(name = "TermAttr")]
pub enum TermAttr {
	Value(usize),
	Placeholder((usize, Option<usize>)),
}

impl Default for TermAttr {
	fn default() -> Self {
		Self::Value(Default::default())
	}
}

impl From<Term> for TermAttr {
	fn from(value: Term) -> Self {
		match value {
			Term::Value(x) => Self::Value(x),
			Term::Placeholder(x) => Self::Placeholder((x, None)),
		}
	}
}

mod term_imp {
	use super::TermAttr;
	use gtk4::glib::{self, prelude::*, subclass::prelude::*};
	use std::cell::RefCell;

	#[derive(Default, glib::Properties)]
	#[properties(wrapper_type = super::TermObj)]
	pub struct InnerTerm {
		#[property(get, set, type = TermAttr)]
		term: RefCell<TermAttr>,
	}

	#[glib::derived_properties]
	impl ObjectImpl for InnerTerm {}

	#[glib::object_subclass]
	impl ObjectSubclass for InnerTerm {
		const NAME: &str = "term";
		type Type = super::TermObj;
	}
}

glib::wrapper! {
	pub struct TermObj(ObjectSubclass<term_imp::InnerTerm>);
}

mod formula_imp {
	use super::TermAttr;
	use crate::formula::FormulaOp;
	use gtk4::glib::{self, prelude::*, subclass::prelude::*};
	use std::cell::RefCell;

	#[derive(Default, glib::Properties)]
	#[properties(wrapper_type = super::FormulaObj)]
	pub struct InnerFormula {
		#[property(get, set, type = TermAttr)]
		lhs: RefCell<TermAttr>,
		#[property(get, set, type = FormulaOp)]
		op: RefCell<FormulaOp>,
		#[property(get, set, type= TermAttr)]
		rhs: RefCell<TermAttr>,
		#[property(get, set, type = TermAttr)]
		result: RefCell<TermAttr>,
		#[property(get, set, type = super::FormulaAction)]
		action: RefCell<super::FormulaAction>,
	}

	#[glib::derived_properties]
	impl ObjectImpl for InnerFormula {}

	#[glib::object_subclass]
	impl ObjectSubclass for InnerFormula {
		const NAME: &str = "formula";
		type Type = super::FormulaObj;
	}
}

mod model_imp {
	use gtk4::{
		glib::{self, prelude::*, subclass::prelude::*},
		subclass::prelude::ListModelImpl,
	};
	use std::cell::RefCell;

	use super::FormulaObj;

	#[derive(Default)]
	pub struct InnerModel(pub(super) RefCell<Vec<FormulaObj>>);

	#[glib::object_subclass]
	impl ObjectSubclass for InnerModel {
		const NAME: &str = "model";
		type Type = super::FormulaModel;
		type Interfaces = (gtk4::gio::ListModel,);
	}

	impl ObjectImpl for InnerModel {}

	impl ListModelImpl for InnerModel {
		fn item_type(&self) -> glib::Type {
			FormulaObj::static_type()
		}

		fn n_items(&self) -> u32 {
			self.0.borrow().len() as u32
		}

		fn item(&self, pos: u32) -> Option<glib::Object> {
			self.0
				.borrow()
				.get(pos as usize)
				.map(|o| o.clone().upcast::<glib::Object>())
		}
	}
}

#[derive(Debug, Clone, Default, Copy, glib::Boxed)]
#[boxed_type(name = "FormulaResult")]
pub enum FormulaAction {
	#[default]
	Uncheck,
	Pass,
	Fail,
}

glib::wrapper! {
	pub struct FormulaObj(ObjectSubclass<formula_imp::InnerFormula>);
}

impl FormulaObj {
	fn new(formula: Formula) -> Self {
		Object::builder()
			.property("lhs", TermAttr::from(formula.lhs))
			.property("op", formula.op)
			.property("rhs", TermAttr::from(formula.rhs))
			.property("result", TermAttr::from(formula.result))
			.property("action", FormulaAction::default())
			.build()
	}
}

glib::wrapper! {
	pub struct FormulaModel(ObjectSubclass<model_imp::InnerModel>)
		@implements gtk4::gio::ListModel;
}

macro_rules! check_term {
	($e: expr) => {
		let b = match $e {
			TermAttr::Value(_) => true,
			TermAttr::Placeholder((a, b)) => Some(a) == b,
		};

		if !b {
			return false;
		}
	};
}

fn check_formula(f: &FormulaObj) -> bool {
	check_term!(f.lhs());
	check_term!(f.rhs());
	check_term!(f.result());

	true
}

impl FormulaModel {
	pub fn with_opt(amount: usize, max_value: usize) -> Self {
		let o = Object::new::<Self>();
		let imp = o.imp();

		let formula: Vec<FormulaObj> = (0..amount)
			.into_iter()
			.map(move |_| Formula::new(max_value))
			.map(FormulaObj::new)
			.collect();

		imp.0.replace(formula);

		o
	}

	pub fn refresh(&self) {
		for i in 0..self.n_items() {
			self.items_changed(i, 1, 1);
		}
	}

	pub fn check_result(&self) {
		for x in self.imp().0.borrow_mut().iter_mut() {
			if check_formula(x) {
				x.set_action(FormulaAction::Pass);
			} else {
				x.set_action(FormulaAction::Fail);
			}
		}

		self.refresh();
	}
}
