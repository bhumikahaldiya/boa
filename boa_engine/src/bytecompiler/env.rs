use std::rc::Rc;

use super::ByteCompiler;
use crate::environments::{BindingLocator, BindingLocatorError, CompileTimeEnvironment};
use boa_ast::expression::Identifier;

impl ByteCompiler<'_, '_> {
    /// Push either a new declarative or function environment on the compile time environment stack.
    #[must_use]
    pub(crate) fn push_compile_environment(&mut self, function_scope: bool) -> u32 {
        self.current_open_environments_count += 1;

        self.current_environment = Rc::new(CompileTimeEnvironment::new(
            self.current_environment.clone(),
            function_scope,
        ));

        let index = self.compile_environments.len() as u32;
        self.compile_environments
            .push(self.current_environment.clone());

        index
    }

    /// Pops the top compile time environment and returns its index in the compile time environments array.
    #[track_caller]
    pub(crate) fn pop_compile_environment(&mut self) {
        self.current_open_environments_count -= 1;

        let outer = self
            .current_environment
            .outer()
            .expect("cannot pop the global environment");
        self.current_environment = outer;
    }

    /// Get the binding locator of the binding at bytecode compile time.
    pub(crate) fn get_binding_value(&self, name: Identifier) -> BindingLocator {
        self.current_environment.get_binding_recursive(name)
    }

    /// Return if a declarative binding exists at bytecode compile time.
    /// This does not include bindings on the global object.
    pub(crate) fn has_binding(&self, name: Identifier) -> bool {
        self.current_environment.has_binding_recursive(name)
    }

    /// Check if a binding name exists in a environment.
    /// If strict is `false` check until a function scope is reached.
    pub(crate) fn has_binding_eval(&self, name: Identifier, strict: bool) -> bool {
        self.current_environment.has_binding_eval(name, strict)
    }

    #[cfg(feature = "annex-b")]
    /// Check if a binding name exists in a environment.
    /// Stop when a function scope is reached.
    pub(crate) fn has_binding_until_var(&self, name: Identifier) -> bool {
        self.current_environment.has_binding_until_var(name)
    }

    /// Create a mutable binding at bytecode compile time.
    /// This function returns a syntax error, if the binding is a redeclaration.
    ///
    /// # Panics
    ///
    /// Panics if the global environment is not function scoped.
    pub(crate) fn create_mutable_binding(&mut self, name: Identifier, function_scope: bool) {
        assert!(self
            .current_environment
            .create_mutable_binding(name, function_scope));
    }

    /// Initialize a mutable binding at bytecode compile time and return its binding locator.
    pub(crate) fn initialize_mutable_binding(
        &self,
        name: Identifier,
        function_scope: bool,
    ) -> BindingLocator {
        self.current_environment
            .initialize_mutable_binding(name, function_scope)
    }

    /// Create an immutable binding at bytecode compile time.
    /// This function returns a syntax error, if the binding is a redeclaration.
    ///
    /// # Panics
    ///
    /// Panics if the global environment does not exist.
    pub(crate) fn create_immutable_binding(&mut self, name: Identifier, strict: bool) {
        self.current_environment
            .create_immutable_binding(name, strict);
    }

    /// Initialize an immutable binding at bytecode compile time and return it's binding locator.
    ///
    /// # Panics
    ///
    /// Panics if the global environment does not exist or a the binding was not created on the current environment.
    pub(crate) fn initialize_immutable_binding(&self, name: Identifier) -> BindingLocator {
        self.current_environment.initialize_immutable_binding(name)
    }

    /// Return the binding locator for a set operation on an existing binding.
    pub(crate) fn set_mutable_binding(
        &self,
        name: Identifier,
    ) -> Result<BindingLocator, BindingLocatorError> {
        self.current_environment.set_mutable_binding_recursive(name)
    }

    #[cfg(feature = "annex-b")]
    /// Return the binding locator for a set operation on an existing var binding.
    pub(crate) fn set_mutable_binding_var(
        &self,
        name: Identifier,
    ) -> Result<BindingLocator, BindingLocatorError> {
        self.current_environment
            .set_mutable_binding_var_recursive(name)
    }
}
