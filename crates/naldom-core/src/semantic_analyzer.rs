// crates/naldom-core/src/semantic_analyzer.rs

use naldom_ir::{CreateArrayParams, Intent, SortArrayParams, WaitParams};
use std::collections::HashMap;

/// Represents the types known to our type system.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Array,
}

/// Represents a declared symbol (e.g., a variable) in the program.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
}

/// The Symbol Table stores all symbols declared in a given scope.
#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new symbol to the table.
    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    /// Retrieves a symbol by name.
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

/// The Semantic Analyzer walks the IntentGraph and validates it.
#[derive(Default)]
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    variable_counter: u32,
    last_created_variable: Option<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a new, unique variable name for internal tracking.
    fn new_variable_name(&mut self) -> String {
        let name = format!("var_{}", self.variable_counter);
        self.variable_counter += 1;
        name
    }

    /// The main entry point for semantic analysis.
    pub fn analyze(&mut self, intent_graph: &[Intent]) -> Result<Vec<Intent>, String> {
        let validated_graph = intent_graph.to_vec();

        for intent in intent_graph {
            self.analyze_intent(intent)?;
        }

        Ok(validated_graph)
    }

    /// Analyzes a single intent.
    fn analyze_intent(&mut self, intent: &Intent) -> Result<(), String> {
        match intent {
            Intent::CreateArray(params) => self.analyze_create_array(params),
            Intent::SortArray(params) => self.analyze_sort_array(params),
            Intent::PrintArray => self.analyze_print_array(),
            Intent::Wait(params) => self.analyze_wait(params),
        }
    }

    fn analyze_create_array(&mut self, _params: &CreateArrayParams) -> Result<(), String> {
        let new_var_name = self.new_variable_name();
        let symbol = Symbol {
            name: new_var_name.clone(),
            symbol_type: SymbolType::Array,
        };
        self.symbol_table.insert(symbol);
        self.last_created_variable = Some(new_var_name);
        Ok(())
    }

    fn analyze_sort_array(&mut self, _params: &SortArrayParams) -> Result<(), String> {
        let var_name = self.last_created_variable.as_ref().ok_or_else(|| {
            "Semantic Error: Attempted to sort, but no array has been created yet.".to_string()
        })?;

        let symbol = self.symbol_table.get(var_name).unwrap();
        if symbol.symbol_type != SymbolType::Array {
            return Err(format!(
                "Semantic Error: Attempted to sort '{}', which is not an Array. It has type {:?}.",
                var_name, symbol.symbol_type
            ));
        }

        Ok(())
    }

    fn analyze_print_array(&mut self) -> Result<(), String> {
        let var_name = self.last_created_variable.as_ref().ok_or_else(|| {
            "Semantic Error: Attempted to print, but nothing has been created yet.".to_string()
        })?;

        let symbol = self.symbol_table.get(var_name).unwrap();
        if symbol.symbol_type != SymbolType::Array {
            return Err(format!(
                "Semantic Error: Attempted to print '{}', which is not an Array. It has type {:?}.",
                var_name, symbol.symbol_type
            ));
        }

        Ok(())
    }

    fn analyze_wait(&mut self, _params: &WaitParams) -> Result<(), String> {
        Ok(())
    }
}

// Unit tests for the semantic analyzer.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_valid_sequence() {
        // Arrange
        let intent_graph = vec![
            Intent::CreateArray(CreateArrayParams {
                size: 5,
                // The `source` field is removed here
            }),
            Intent::SortArray(SortArrayParams {
                order: "ascending".to_string(),
            }),
            Intent::PrintArray,
        ];
        let mut analyzer = SemanticAnalyzer::new();

        // Act
        let result = analyzer.analyze(&intent_graph);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_sort_before_create() {
        // Arrange
        let intent_graph = vec![
            Intent::SortArray(SortArrayParams {
                order: "ascending".to_string(),
            }),
            Intent::CreateArray(CreateArrayParams {
                size: 5,
                // The `source` field is removed here
            }),
        ];
        let mut analyzer = SemanticAnalyzer::new();

        // Act
        let result = analyzer.analyze(&intent_graph);

        // Assert
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Attempted to sort, but no array has been created yet."));
    }

    #[test]
    fn test_analyze_print_before_create() {
        // Arrange
        let intent_graph = vec![Intent::PrintArray];
        let mut analyzer = SemanticAnalyzer::new();

        // Act
        let result = analyzer.analyze(&intent_graph);

        // Assert
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Attempted to print, but nothing has been created yet."));
    }
}
