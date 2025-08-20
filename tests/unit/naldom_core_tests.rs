// tests/unit/naldom_core_tests.rs

// This file contains integration-style unit tests for the `naldom-core` crate.
// By being in the `tests/` directory, it tests the public API of the crate,
// ensuring that the core components work together as expected.

mod parser_tests {
    use naldom_core::parser::parse_to_intent_graph;
    use naldom_ir::{CreateArrayParams, Intent, SortArrayParams};

    #[test]
    fn test_parse_valid_json() {
        // Arrange: Define a mock LLM output string.
        let llm_output = r#"
            <think>Some conversational text...</think>
            [
              {"intent": "CreateArray", "parameters": {"size": 10, "source": "random"}},
              {"intent": "SortArray", "parameters": {"order": "ascending"}},
              {"intent": "PrintArray"}
            ]
        "#;

        // Act: Call the parser function.
        let result = parse_to_intent_graph(llm_output);

        // Assert: Check if the parsing was successful and the structure is correct.
        assert!(result.is_ok());
        let intent_graph = result.unwrap();

        assert_eq!(intent_graph.len(), 3);

        match &intent_graph[0] {
            Intent::CreateArray(params) => {
                assert_eq!(params.size, 10);
                assert_eq!(params.source, "random");
            }
            _ => panic!("Expected first intent to be CreateArray"),
        }

        match &intent_graph[1] {
            Intent::SortArray(params) => {
                assert_eq!(params.order, "ascending");
            }
            _ => panic!("Expected second intent to be SortArray"),
        }

        assert!(matches!(intent_graph[2], Intent::PrintArray));
    }

    #[test]
    fn test_parse_invalid_json() {
        // Arrange: Define a non-JSON string.
        let llm_output = "This is not valid JSON.";

        // Act: Call the parser function.
        let result = parse_to_intent_graph(llm_output);

        // Assert: Check that the function correctly returns an error.
        assert!(result.is_err());
    }
}

mod lowering_tests {
    use naldom_core::lowering::LoweringContext;
    use naldom_ir::{
        CreateArrayParams, HLExpression, HLProgram, HLStatement, HLValue, Intent, SortArrayParams,
    };

    #[test]
    fn test_lowering_full_sequence() {
        // Arrange: Manually create an IntentGraph.
        let intent_graph = vec![
            Intent::CreateArray(CreateArrayParams {
                size: 10,
                source: "random".to_string(),
            }),
            Intent::SortArray(SortArrayParams {
                order: "ascending".to_string(),
            }),
            Intent::PrintArray,
        ];

        let mut context = LoweringContext::default();

        // Act: Call the lowering function.
        let hl_program = context.lower(&intent_graph);

        // Assert: Check if the generated IR-HL is correct.
        let expected_program = HLProgram {
            statements: vec![
                HLStatement::Assign {
                    variable: "var_0".to_string(),
                    expression: HLExpression::FunctionCall {
                        function: "create_random_array".to_string(),
                        arguments: vec![HLExpression::Literal(HLValue::Integer(10))],
                    },
                },
                HLStatement::Call {
                    function: "sort_array".to_string(),
                    arguments: vec![
                        HLExpression::Variable("var_0".to_string()),
                        HLExpression::Literal(HLValue::String("ascending".to_string())),
                    ],
                },
                HLStatement::Call {
                    function: "print_array".to_string(),
                    arguments: vec![HLExpression::Variable("var_0".to_string())],
                },
            ],
        };

        assert_eq!(hl_program, expected_program);
    }
}

mod codegen_tests {
    use naldom_core::codegen_python::PythonCodeGenerator;
    use naldom_ir::{HLProgram, HLStatement, HLExpression, HLValue};

    #[test]
    fn test_generate_python_code() {
        // Arrange: Manually create an IR-HL program.
        let hl_program = HLProgram {
            statements: vec![
                HLStatement::Assign {
                    variable: "var_0".to_string(),
                    expression: HLExpression::FunctionCall {
                        function: "create_random_array".to_string(),
                        arguments: vec![HLExpression::Literal(HLValue::Integer(10))],
                    },
                },
                HLStatement::Call {
                    function: "sort_array".to_string(),
                    arguments: vec![
                        HLExpression::Variable("var_0".to_string()),
                        HLExpression::Literal(HLValue::String("ascending".to_string())),
                    ],
                },
                HLStatement::Call {
                    function: "print_array".to_string(),
                    arguments: vec![HLExpression::Variable("var_0".to_string())],
                },
            ],
        };

        let generator = PythonCodeGenerator;

        // Act: Call the code generation function.
        let python_code = generator.generate(&hl_program);

        // Assert: Check if the generated string is correct.
        let expected_code = [
            "var_0 = create_random_array(10)",
            "sort_array(var_0, 'ascending')",
            "print_array(var_0)",
        ]
        .join("\n");

        assert_eq!(python_code, expected_code);
    }
}