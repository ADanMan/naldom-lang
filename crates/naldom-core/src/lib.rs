// crates/naldom-core/src/lib.rs

//! The core compiler components for the Naldom language.

pub mod codegen_llvm;
pub mod codegen_python;
pub mod llm_inference;
pub mod lowering;
pub mod lowering_hl_to_ll;
pub mod parser;
pub mod semantic_analyzer;

// --- Integration Tests for the Compiler Pipeline ---
#[cfg(test)]
mod pipeline_tests {
    use crate::codegen_llvm::generate_llvm_ir;
    use crate::lowering::LoweringContext;
    use crate::lowering_hl_to_ll::lower_hl_to_ll;
    use crate::parser::parse_to_intent_graph;
    use crate::semantic_analyzer::SemanticAnalyzer;
    use naldom_ir::Intent;

    /// This test simulates the entire compiler pipeline from a mocked LLM response
    /// down to the final LLVM IR, without any external dependencies.
    #[test]
    fn test_full_pipeline_to_llvm_ir_for_wait_program() {
        // Arrange:
        // 1. Mock the JSON response we expect from the LLM for our wait_program.md
        let mocked_llm_response = r#"
        [
            {
                "intent": "CreateArray",
                "parameters": { "size": 5 }
            },
            {
                "intent": "PrintArray"
            },
            {
                "intent": "Wait",
                "parameters": { "durationMs": 100 }
            },
            {
                "intent": "PrintArray"
            }
        ]
        "#;

        // Act & Assert (step-by-step)

        // 2. Parse to IntentGraph
        let intent_graph = parse_to_intent_graph(mocked_llm_response).expect("Parsing failed");
        assert_eq!(intent_graph.len(), 4);
        assert!(matches!(intent_graph[2], Intent::Wait(_)));

        // 3. Analyze
        let mut analyzer = SemanticAnalyzer::new();
        let validated_graph = analyzer.analyze(&intent_graph).expect("Analysis failed");

        // 4. Lower to IR-HL
        let mut hl_context = LoweringContext::new();
        let hl_program = hl_context.lower(&validated_graph);
        assert_eq!(hl_program.statements.len(), 4);

        // 5. Lower to IR-LL
        let ll_program = lower_hl_to_ll(&hl_program);
        assert_eq!(
            ll_program.functions[0].basic_blocks[0].instructions.len(),
            4
        );

        // 6. Generate LLVM IR
        let target_triple = "arm64-apple-darwin"; // Example target
        let llvm_ir_result = generate_llvm_ir(&ll_program, target_triple);
        assert!(llvm_ir_result.is_ok());
        let llvm_ir = llvm_ir_result.unwrap();

        // 7. Assert that the final IR contains the call to our sleep function
        assert!(llvm_ir.contains("declare void @naldom_async_sleep(i64)"));
        assert!(llvm_ir.contains("call void @naldom_async_sleep(i64 100)"));
        assert!(llvm_ir.contains("call void @print_array"));
    }
}
