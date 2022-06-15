// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Expression, Identifier};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A circuit member access expression `inner.name` to some structure with *named members*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberAccess {
    /// The structure that the member `name` is being extracted from.
    pub inner: Box<Expression>,
    /// The name of the member to extract in `inner`.
    pub name: Identifier,
    /// The span covering all of `inner.name`.
    pub span: Span,
}

impl fmt::Display for MemberAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.inner, self.name)
    }
}

