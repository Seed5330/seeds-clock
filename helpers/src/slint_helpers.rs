use std::{rc::Rc};
use slint::{Model, ModelRc, VecModel};

pub fn new_slint_vector<T>(vector: Vec<T>) -> Rc<VecModel<T>> {
	Rc::new(VecModel::from(vector))
}

pub fn model_to_vec<T>(model: ModelRc<T>) -> Vec<T> {
	model.iter().collect()
}

pub trait VecToModel<T> {
	fn to_model(self) -> ModelRc<T>;
}

impl<T> VecToModel<T> for Vec<T>
where
	T: Clone + 'static,
{
	fn to_model(self) -> ModelRc<T> {
		Rc::new(VecModel::from(self)).into()
	}	
}

pub trait ModelToVec<T> {
	fn to_vec(self) -> Vec<T>;
}

impl<T> ModelToVec<T> for ModelRc<T>
where
	T: Clone + 'static,
{
	fn to_vec(self) -> Vec<T> {
		self.iter().collect()
	}
}



