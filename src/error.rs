use crate::rqlite;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};
use std::{borrow::Cow /*, str::from_utf8_unchecked*/};
/*
use libsqlite3_sys::{
    sqlite3, sqlite3_errmsg, sqlite3_extended_errcode, SQLITE_CONSTRAINT_CHECK,
    SQLITE_CONSTRAINT_FOREIGNKEY, SQLITE_CONSTRAINT_NOTNULL, SQLITE_CONSTRAINT_PRIMARYKEY,
    SQLITE_CONSTRAINT_UNIQUE,
};
*/
pub(crate) use sqlx_core::error::*;

// Error Codes And Messages
// https://www.sqlite.org/c3ref/errcode.html

#[derive(Debug)]
pub struct RqliteError {
    pub inner: Box<rqlite::RqliteError>,
}

impl RqliteError {
    /*
    pub(crate) fn new(inner: Box<rqlite::RqliteError>) -> Self {
        Self {
            inner,
        }
    }
    */
}

impl Display for RqliteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // We include the code as some produce ambiguous messages:
        // SQLITE_BUSY: "database is locked"
        // SQLITE_LOCKED: "database table is locked"
        // Sadly there's no function to get the string label back from an error code.
        write!(f, "{}", self.inner)
    }
}

impl StdError for RqliteError {}

impl DatabaseError for RqliteError {
    #[inline]
    fn message(&self) -> &str {
        "Rqlite database error"
    }

    /// The extended result code.
    #[inline]
    fn code(&self) -> Option<Cow<'_, str>> {
        //Some(format!("{}", self.code).into())
        None
    }

    #[doc(hidden)]
    fn as_error(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn as_error_mut(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn into_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> ErrorKind {
        match self.inner {
            /*
            SQLITE_CONSTRAINT_UNIQUE | SQLITE_CONSTRAINT_PRIMARYKEY => ErrorKind::UniqueViolation,
            SQLITE_CONSTRAINT_FOREIGNKEY => ErrorKind::ForeignKeyViolation,
            SQLITE_CONSTRAINT_NOTNULL => ErrorKind::NotNullViolation,
            SQLITE_CONSTRAINT_CHECK => ErrorKind::CheckViolation,
            */
            _ => ErrorKind::Other,
        }
    }
}
