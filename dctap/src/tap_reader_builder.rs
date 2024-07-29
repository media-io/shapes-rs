use crate::{tap_error::Result, tap_headers::TapHeaders};
use crate::{TapConfig, TapReader, TapReaderState};
use csv::{ReaderBuilder, Terminator, Trim};
use std::fs::File;
// use indexmap::IndexSet;
use std::io::{self};
use std::path::Path;

#[derive(Default)]
pub(crate) struct TapReaderBuilder {
    reader_builder: ReaderBuilder,
}

impl TapReaderBuilder {
    pub fn new() -> TapReaderBuilder {
        TapReaderBuilder::default()
    }

    // Most of these options are copied from CSV Rust
    pub fn flexible(mut self, yes: bool) -> Self {
        self.reader_builder.flexible(yes);
        self
    }

    pub fn _trim(&mut self, trim: Trim) -> &mut TapReaderBuilder {
        self.reader_builder.trim(trim);
        self
    }

    pub fn _terminator(&mut self, term: Terminator) -> &mut TapReaderBuilder {
        self.reader_builder.terminator(term);
        self
    }

    pub fn _quote(&mut self, quote: u8) -> &mut TapReaderBuilder {
        self.reader_builder.quote(quote);
        self
    }

    pub fn _delimiter(&mut self, delimiter: u8) -> &mut TapReaderBuilder {
        self.reader_builder.delimiter(delimiter);
        self
    }

    /// Build a TapReader from a path and a `TapConfig`
    ///
    /// # Example
    /// ```no_run
    /// use dctap::TapReaderBuilder;
    /// use std::error::Error;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn Error>> {
    ///     let mut tap = TapReaderBuilder::new().from_path("foo.csv")?;
    ///     for result in tap.shapes() {
    ///         let shape = result?;
    ///         println!("{:?}", shape);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_path<P: AsRef<Path>>(&self, path: P, config: TapConfig) -> Result<TapReader<File>> {
        let mut reader = self.reader_builder.from_path(path)?;
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new(reader, state, config))
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_reader<R: io::Read>(&mut self, rdr: R, config: TapConfig) -> Result<TapReader<R>> {
        let mut reader = self.reader_builder.from_reader(rdr);
        let rcd_headers = reader.headers()?;
        let headers = TapHeaders::from_record(rcd_headers)?;
        let state = TapReaderState::new().with_headers(headers);
        Ok(TapReader::new(reader, state, config))
    }
}