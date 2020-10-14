use std::io;


pub struct RawLineSplitter<R> {
	buffer: Vec<u8>,
	reader: R
}

impl<R> RawLineSplitter<R>
where R: io::BufRead
{
	pub fn new(reader: R) -> Self {
		Self {
			buffer: Vec::new(),
			reader
		}
	}


	pub fn read_line<'a>(&'a mut self) -> io::Result<Option<&'a [u8]>> {
		self.buffer.clear();

		let count = self.reader.read_until(b'\n', &mut self.buffer)?;

		Ok(
			if count == 0 {
				None
			}
			else {
				self.buffer.pop();
				Some(&self.buffer[..])
			}
		)
	}
}
