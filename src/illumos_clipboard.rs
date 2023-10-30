use crate::common::*;
use crate::x11_clipboard::{Clipboard, X11ClipboardContext};
use crate::Result;

pub struct IllumosClipboardContext(X11ClipboardContext);

impl ClipboardProvider for IllumosClipboardContext {
    fn new() -> Result<IllumosClipboardContext> {
        let context = X11ClipboardContext::<Clipboard>::new()?;
        Ok(Self(context))
    }

    fn get_contents(&mut self) -> Result<String> {
        self.0.get_contents()
    }

    fn set_contents(&mut self, content: String) -> Result<()> {
        self.0.set_contents(content)
    }

    fn clear(&mut self) -> Result<()> {
        self.0.clear()
    }
}
