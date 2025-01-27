use super::Type;
use ruff_python_ast::name::Name;

/// Typed arguments for a single call, in source order.
#[derive(Clone, Debug, Default)]
pub(crate) struct CallArguments<'db>(Vec<Argument<'db>>);

impl<'db> CallArguments<'db> {
    /// Create a [`CallArguments`] from an iterator over non-variadic positional argument types.
    pub(crate) fn positional(positional_tys: impl IntoIterator<Item = Type<'db>>) -> Self {
        positional_tys
            .into_iter()
            .map(Argument::Positional)
            .collect()
    }

    /// Prepend an extra positional argument.
    pub(crate) fn with_self(&self, self_ty: Type<'db>) -> Self {
        let mut arguments = Vec::with_capacity(self.0.len() + 1);
        arguments.push(Argument::Positional(self_ty));
        arguments.extend_from_slice(&self.0);
        Self(arguments)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Argument<'db>> {
        self.0.iter()
    }

    // TODO this should be eliminated in favor of [`bind_call`]
    pub(crate) fn first_argument(&self) -> Option<Type<'db>> {
        self.0.first().map(Argument::ty)
    }
}

impl<'db, 'a> IntoIterator for &'a CallArguments<'db> {
    type Item = &'a Argument<'db>;
    type IntoIter = std::slice::Iter<'a, Argument<'db>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'db> FromIterator<Argument<'db>> for CallArguments<'db> {
    fn from_iter<T: IntoIterator<Item = Argument<'db>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Argument<'db> {
    /// A positional argument.
    Positional(Type<'db>),
    /// A starred positional argument (e.g. `*args`).
    Variadic(Type<'db>),
    /// A keyword argument (e.g. `a=1`).
    Keyword { name: Name, ty: Type<'db> },
    /// The double-starred keywords argument (e.g. `**kwargs`).
    Keywords(Type<'db>),
}

impl<'db> Argument<'db> {
    fn ty(&self) -> Type<'db> {
        match self {
            Self::Positional(ty) => *ty,
            Self::Variadic(ty) => *ty,
            Self::Keyword { name: _, ty } => *ty,
            Self::Keywords(ty) => *ty,
        }
    }
}
