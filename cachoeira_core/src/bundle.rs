use specs::prelude::DispatcherBuilder;

error_chain!();

pub trait SystemBundle<'a, 'b> {
	fn build(self, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<()>;
}