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
		#[property(get, set, type = FormulaOp)]
		op: RefCell<FormulaOp>,
		#[property(get, set, type= Term)]
		rhs: RefCell<Term>,
		#[property(get, set, type = Term)]
		result: RefCell<Term>,
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

use crate::formula::{Formula, Term};
use gtk4::{
	glib::{self, Object},
	subclass::prelude::ObjectSubclassIsExt,
};

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
	fn new(formula: Formula) -> Self {
		Object::builder()
			.property("lhs", formula.lhs)
			.property("op", formula.op)
			.property("rhs", formula.rhs)
			.property("result", formula.result)
			.build()
	}
}

glib::wrapper! {
	pub struct FormulaModel(ObjectSubclass<model_imp::InnerModel>)
		@implements gtk4::gio::ListModel;
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
}
