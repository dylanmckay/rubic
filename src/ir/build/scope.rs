use {ir, ast};

use std::collections::{VecDeque, HashMap};

/// A single scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope
{
    /// The name of the scope.
    /// Empth string means global namespace.
    pub name: String,
    pub classes: HashMap<String, ir::ClassId>,
}

/// A tree of scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeContext
{
    stack: VecDeque<Scope>,
}

impl Scope
{
    /// Creates a new named scope.
    pub fn new(name: String) -> Self {
        Scope {
            name: name,
            classes: HashMap::new(),
        }
    }

    /// Creates the global scope.
    pub fn global() -> Self {
        Scope::new("".to_owned())
    }

    pub fn is_global(&self) -> bool { self.name.is_empty() }
}

impl ScopeContext
{
    /// Creates a new scope context.
    pub fn new() -> Self {
        let global_scope = Scope::global();

        ScopeContext {
            stack: vec![global_scope].into_iter().collect(),
        }
    }

    /// Creates a new nested scope.
    pub fn begin(&mut self, name: String) {
        self.stack.push_front(Scope::new(name));
    }

    /// Ends a scope.
    pub fn end(&mut self) {
        assert!(self.stack.front().is_some(), "no scope to end");
        assert!(!self.stack.front().unwrap().is_global(), "cannot end the global scope");
        self.stack.pop_front();
    }

    /// Resolves a scope by name.
    pub fn resolve_scope(&self, path: &ast::Path) -> Option<&Scope> {
        assert!(path.is_constant());
        let textual_parts = path.textual_parts();

        if let Some(first_idx) = self.stack.iter().rev().position(|scope| scope.name == textual_parts[0]) {
            let scope_names: Vec<&str> = self.stack.iter().rev().map(|stack| &stack.name[..]).collect();

            if &scope_names[first_idx..] == &textual_parts[..] {
                let idx = first_idx + textual_parts.len() - 1;
                Some(self.stack.iter().rev().nth(idx).unwrap())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn can_resolve_basic_scopes() {
        let mut context = ScopeContext::new();
        context.stack.push_front(Scope::new("Foo".to_owned()));
        context.stack.push_front(Scope::new("Bar".to_owned()));

        let path = ast::Path {
            parts: vec![ast::PathSegment {
                kind: ast::PathSegmentKind::Constant(ast::Constant("Foo".to_owned())),
                separator: ast::PathSeparator::Root},
            ast::PathSegment {
                kind: ast::PathSegmentKind::Constant(ast::Constant("Bar".to_owned())),
                separator: ast::PathSeparator::DoubleColon,
            }],
        };
        assert_eq!(context.resolve_scope(&path), Some(&Scope::new("Bar".to_owned())));
    }

    #[test]
    fn can_resolve_multiple_nested_scopes() {
        let mut context = ScopeContext::new();
        context.stack.push_front(Scope::new("Foo".to_owned()));
        context.stack.push_front(Scope::new("Bar".to_owned()));
        context.stack.push_front(Scope::new("Baz".to_owned()));

        let path = ast::Path {
            parts: vec![ast::PathSegment {
                kind: ast::PathSegmentKind::Constant(ast::Constant("Foo".to_owned())),
                separator: ast::PathSeparator::Root},
            ast::PathSegment {
                kind: ast::PathSegmentKind::Constant(ast::Constant("Bar".to_owned())),
                separator: ast::PathSeparator::DoubleColon,
            },
            ast::PathSegment {
                kind: ast::PathSegmentKind::Constant(ast::Constant("Baz".to_owned())),
                separator: ast::PathSeparator::DoubleColon,
            }],
        };
        assert_eq!(context.resolve_scope(&path), Some(&Scope::new("Baz".to_owned())));
    }
}
