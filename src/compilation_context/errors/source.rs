use std::path::{Path, PathBuf};
use std::sync::Arc;

use module_loader::{FileId, Modules, SourceError, SourceErrorKind};
use slynx_lexer::LexerError;

use crate::{
    ErrorPosition, SlynxContext,
    compilation_context::errors::{SlynxError, helpers::suggestions_from_source},
    helpers::suggestions_from_parser,
};

impl SlynxContext {
    ///Puts `path` in the form used as the key of the file map.
    ///
    ///`Modules` records the canonicalized path of every module while the
    ///context stores the paths it was handed, so the two only agree after
    ///canonicalization. Paths that do not exist on disk (in-memory sources) are
    ///compared as given.
    fn normalize_path(path: &Path) -> PathBuf {
        std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    }

    ///Finds the `Arc<PathBuf>` key for a given path in the context's file map.
    pub(crate) fn find_file_key(&self, path: &Path) -> Option<Arc<PathBuf>> {
        let wanted = Self::normalize_path(path);
        self.files.iter().find_map(|k| {
            let key = k.key().clone();
            (Self::normalize_path(&key) == wanted).then_some(key)
        })
    }

    ///Resolves `index` — an offset into the source of `file` — to a line, a
    ///column and the text of the line it lands on.
    ///
    ///Falls back to the entry point when the file is not in the context, so a
    ///diagnostic never loses its position entirely.
    pub(crate) fn position_of(
        &self,
        modules: &Modules,
        file: FileId,
        index: usize,
    ) -> (ErrorPosition, String) {
        let path = modules.path_of(file).unwrap_or(&self.entry_point);
        let key = self
            .find_file_key(path)
            .unwrap_or_else(|| self.entry_point.clone());
        let info = self.get_line_info(&key, index);
        (
            ErrorPosition {
                line: info.line,
                column: info.column_start,
                end_column: info.column_end.max(info.column_start),
            },
            info.src,
        )
    }

    ///Handles a source error. The `generator` is the path of the file that generated the provided `error`
    pub fn handle_source_error(&self, error: SourceError) -> SlynxError {
        let suggestion = suggestions_from_source(&error);
        let file_name = error.entry().display().to_string();
        let file_key = self.find_file_key(error.entry());

        match error.kind() {
            SourceErrorKind::InexistantSource(inner, span, generator) => {
                let file_key = self.find_file_key(generator);
                let (line, col_start, col_end, src) = if let Some(ref key) = file_key {
                    let info = self.get_line_info(key, span.start as usize);
                    (info.line, info.column_start, info.column_end, info.src)
                } else {
                    Default::default()
                };
                let msg = format!("Could not open file '{}': {inner}", error.entry().display());
                SlynxError::new_compiler(
                    ErrorPosition {
                        line,
                        column: col_start,
                        end_column: col_end,
                    },
                    msg,
                    generator.display().to_string(),
                    src,
                    suggestion,
                    error.backtrace(),
                )
            }
            SourceErrorKind::Lexing(lex_err) => {
                let (index, _end_index) = match lex_err {
                    LexerError::MalformedNumber { init, end, .. } => (*init, *end),
                    LexerError::UnrecognizedChar { index, .. } => (*index, *index),
                };
                let (position, src) = if let Some(ref key) = file_key {
                    let info = self.get_line_info(key, index);
                    (
                        ErrorPosition {
                            line: info.line,
                            column: info.column_start,
                            end_column: info.column_end.max(info.column_start),
                        },
                        info.src,
                    )
                } else {
                    (
                        ErrorPosition {
                            line: 0,
                            column: 0,
                            end_column: 0,
                        },
                        String::new(),
                    )
                };
                SlynxError::new_lexer(
                    position,
                    lex_err.to_string(),
                    file_name,
                    src,
                    suggestion,
                    error.backtrace(),
                )
            }
            SourceErrorKind::Parsing(parse_err) => {
                let suggestion = suggestions_from_parser(parse_err);
                // The span is an offset into the file that failed to parse, which
                // is `error.entry()` — not necessarily the entry point.
                let (position, src) = if let Some(ref key) = file_key {
                    let info = self.get_line_info(key, parse_err.span.start as usize);
                    (
                        ErrorPosition {
                            line: info.line,
                            column: info.column_start,
                            end_column: info.column_end.max(info.column_start),
                        },
                        info.src,
                    )
                } else {
                    (
                        ErrorPosition {
                            line: 0,
                            column: 0,
                            end_column: 0,
                        },
                        String::new(),
                    )
                };
                SlynxError::new_parser(
                    position,
                    parse_err.to_string(),
                    file_name,
                    src,
                    suggestion,
                    error.backtrace(),
                )
            }
        }
    }
}
