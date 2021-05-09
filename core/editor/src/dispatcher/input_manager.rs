use super::{
	events::{Event, Key, MouseState},
	Action,
};
use std::collections::HashMap;

pub struct KeyState {
	depressed: bool,
	// time of last press
	// mod keys held down while pressing
	// …
}

pub struct InputPreprocessor {
	mouse_keys: MouseState,
	keyboard: HashMap<Key, KeyState>,
	key_translation: HashMap<Key, VirtualInputAction>,
}

impl InputPreprocessor {
	pub fn handle_user_input(&mut self, event: Event) -> Option<Vec<Action>> {
		// clean user input and if possible reconstruct it
		// store the changes in the keyboard if it is a key event
		// translate the key events to VirtualKeyActions and return them
		// transform canvas coordinates to document coordinates
		// Last pressed key
		// respect text input mode
		Some(vec![event])
	}
}

pub enum VirtualInputAction {
	SelectSelectTool,
	SelectEllipseTool,
	Undo,
	Redo,
	IncreaseSize,
	DecreaseSize,
	// …
}