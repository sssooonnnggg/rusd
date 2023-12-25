use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "usd.pest"]
pub struct UsdParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    fn print_pairs(pair: pest::iterators::Pair<Rule>, depth: usize) {
        let indent = depth * 4;
        println!(
            "{:indent$}rule: {:?}, str: {}",
            "",
            pair.as_rule(),
            pair.as_str(),
            indent = indent
        );
        for inner_pair in pair.into_inner() {
            print_pairs(inner_pair, depth + 1);
        }
    }

    fn print_and_test(rule: Rule, input: &str) {
        let result = UsdParser::parse(rule, input);
        match &result {
            Ok(pairs) => {
                for pair in pairs.clone() {
                    print_pairs(pair, 0);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_line_comment() {
        print_and_test(Rule::single_line_comment, "// this is a comment");
    }

    #[test]
    fn test_single_line_comment_with_newline() {
        print_and_test(Rule::single_line_comment, "// this is a comment\n");
    }

    #[test]
    fn test_multi_line_comment() {
        print_and_test(
            Rule::multi_line_comment,
            r#"/* this is a comment 
        line1,
        line2,
        line3
        */"#,
        );
    }

    #[test]
    fn test_identifier() {
        print_and_test(Rule::identifier, "abc");
        print_and_test(Rule::identifier, "abc0123");
        print_and_test(Rule::identifier, "abc_0123");
        print_and_test(Rule::identifier, "abc_0123_");
        print_and_test(Rule::identifier, "abc_0123_abc");
        print_and_test(Rule::identifier, "_abc_0123");
        print_and_test(Rule::identifier, "camelCase");
        print_and_test(Rule::identifier, "PascalCase");
        print_and_test(Rule::identifier, "snake_case");
        print_and_test(Rule::identifier, "SCREAMING_SNAKE_CASE");
    }

    #[test]
    #[should_panic]
    fn test_identifier_invalid() {
        print_and_test(Rule::identifier, "0123abc");
    }

    #[test]
    #[should_panic]
    fn test_identifier_invalid_1() {
        print_and_test(Rule::identifier_for_test, "kebab-case");
    }

    #[test]
    #[should_panic]
    fn test_identifier_invalid_2() {
        print_and_test(Rule::identifier_for_test, "Train-Case");
    }

    #[test]
    #[should_panic]
    fn test_identifier_invalid_3() {
        print_and_test(Rule::identifier_for_test, "ab c");
    }

    #[test]
    #[should_panic]
    fn test_identifier_invalid_4() {
        print_and_test(Rule::identifier_for_test, "ab\nc");
    }

    #[test]
    fn test_namespace_identifier() {
        print_and_test(Rule::namespace_identifier, "transform:position");
    }

    #[test]
    #[should_panic]
    fn test_invalid_namespace() {
        print_and_test(Rule::namespace_identifier, "position");
    }

    #[test]
    fn test_scene_path() {
        print_and_test(Rule::scene_path, "<a/b/c>");
        print_and_test(Rule::scene_path, "<a/b/c.usd>");
        print_and_test(Rule::scene_path, "<a/b/c.usda>");
        print_and_test(Rule::scene_path, "<a/b/c/d.usdc>");
        print_and_test(Rule::scene_path, "<../../a/b/c/d.usdc>");
    }

    #[test]
    fn test_asset_reference() {
        print_and_test(Rule::asset_reference, "@a/b/c@");
    }

    #[test]
    fn test_single_quoted_string() {
        print_and_test(Rule::single_quoted_string, "''");
        print_and_test(Rule::single_quoted_string, "'abc'");
        print_and_test(Rule::single_quoted_string, r#"'"abc"'"#);
        print_and_test(Rule::single_quoted_string, r"'456\'abc\'123'");
    }

    #[test]
    fn test_double_quoted_string() {
        print_and_test(Rule::double_quoted_string, r#""""#);
        print_and_test(Rule::double_quoted_string, r#""abc""#);
        print_and_test(Rule::double_quoted_string, r#""a'bc'd""#);
        print_and_test(Rule::double_quoted_string, r#""456\"abc\"123""#);
        print_and_test(Rule::double_quoted_string, r#""\"abcdefg\"""#);
    }

    #[test]
    #[should_panic]
    fn test_escaped_invalid() {
        print_and_test(Rule::escaped_char_for_test, r"\ n");
    }

    #[test]
    fn test_triple_quoted_string() {
        print_and_test(Rule::triple_quoted_string, r#""""""""#);
        print_and_test(Rule::triple_quoted_string, r#""""abc""""#);
        print_and_test(
            Rule::triple_quoted_string,
            r#""""abc
        def
        ghi""""#,
        );
        print_and_test(
            Rule::triple_quoted_string,
            r#""""abc
        "def"
        ghi""""#,
        );
        print_and_test(Rule::triple_quoted_string, r#"''''''"#);
        print_and_test(Rule::triple_quoted_string, r#"'''abc'''"#);
        print_and_test(
            Rule::triple_quoted_string,
            r#"'''abc
        'def'"def"
        ghi'''"#,
        );
    }

    #[test]
    #[should_panic]
    fn test_triple_quoted_string_invalid() {
        print_and_test(
            Rule::triple_quoted_string,
            r#""""abc
        "def"
        '''"#,
        );
    }

    #[test]
    fn test_int() {
        print_and_test(Rule::int, "0");
        print_and_test(Rule::int, "1");
        print_and_test(Rule::int, "123");
        print_and_test(Rule::int, "-123");
        print_and_test(Rule::int, "-2");
    }

    #[test]
    #[should_panic]
    fn test_int_invalid() {
        print_and_test(Rule::int_for_test, "0123");
    }

    #[test]
    #[should_panic]
    fn test_int_invalid_1() {
        print_and_test(Rule::int_for_test, "123.456");
    }

    #[test]
    #[should_panic]
    fn test_int_invalid_2() {
        print_and_test(Rule::int_for_test, "123 456");
    }

    #[test]
    fn test_float() {
        print_and_test(Rule::float_for_test, "0.0");
        print_and_test(Rule::float_for_test, "0.");
        print_and_test(Rule::float_for_test, "1.");
        print_and_test(Rule::float_for_test, "123.456");
        print_and_test(Rule::float_for_test, "-123.456");
        print_and_test(Rule::float_for_test, "5.9604641222676946e-8")
    }

    #[test]
    #[should_panic]
    fn test_float_invalid() {
        print_and_test(Rule::float_for_test, "0123.456");
    }

    #[test]
    #[should_panic]
    fn test_float_invalid_1() {
        print_and_test(Rule::float_for_test, "123");
    }

    #[test]
    #[should_panic]
    fn test_float_invalid_2() {
        print_and_test(Rule::float_for_test, "123 456");
    }

    #[test]
    fn test_namespaced_name() {
        print_and_test(Rule::namespaced_name_for_test, "abc:efg");
        print_and_test(Rule::namespaced_name_for_test, "efg");
        print_and_test(Rule::namespaced_name_for_test, "abc:efg:xyz");
    }

    #[test]
    fn test_relationship() {
        print_and_test(Rule::relationship_for_test, "rel abc:_def");
        print_and_test(Rule::relationship_for_test, "rel abc");
        print_and_test(Rule::relationship_for_test, "rel abc = None");
        print_and_test(Rule::relationship_for_test, "delete custom varying rel abc");
        print_and_test(Rule::relationship_for_test, "delete custom rel abc");
        print_and_test(Rule::relationship_for_test, "rel abc:_def = <abc/def.usd>");
        print_and_test(Rule::relationship_for_test, "rel abc:_def = []");
        print_and_test(
            Rule::relationship_for_test,
            "rel abc:_def = [<abc/def.usd>]",
        );
        print_and_test(
            Rule::relationship_for_test,
            "rel abc:_def = [<abc/def.usd>, <xyz/uvw.usd>]",
        );
        print_and_test(
            Rule::relationship_for_test,
            r#"rel abc:_def = [
            <abc/def.usd>, 
            <xyz/uvw.usd>,
            <just/be/your/self.jpg>
        ]"#,
        );

        print_and_test(
            Rule::relationship_for_test,
            r#"
            rel audioSetting:bankArray = [
                </Workspace/Logic/Identity/CPLConfig/Audio/Bank/_c1a8401135fdd494a80056c0885ddd70>,
                </Workspace/Logic/Identity/CPLConfig/Audio/Bank/_3b3631c9cbd2bdd41a436b6cd8b7d1c7>,
                </Workspace/Logic/Identity/CPLConfig/Audio/Bank/_ac8a0aa15da9c344e9ba50f655877ae5>,
            ]"#,
        );
    }

    #[test]
    fn test_metadata_list() {
        print_and_test(Rule::metadata_for_test, "()");
        print_and_test(
            Rule::metadata_for_test,
            r#"(
            n = None;
            f = 1.234;
            a = 1;
            b = "some thing";
            doc = """this is a doc string""";
            tuple = (1, 2, 3);
            nested_tuple = ((1, 3, 3), (4, 5, 6));
            list = [1, 2, 3];
            nested_list = [[1, 2, 3], [4, 5, 6]];
            list_of_tuple = [(1, 2, 3), (4, 5, 6)];
        )"#,
        );
        print_and_test(
            Rule::metadata_for_test,
            r#"(
            n = None
            f = 1.234
            a = 1
            b = "some thing"
            doc = """this is a doc string"""
            tuple = (1, 2, 3)
            nested_tuple = ((1, 3, 3), (4, 5, 6))
            list = [1, 2, 3]
            nested_list = [[1, 2, 3], [4, 5, 6]]
            list_of_tuple = [(1, 2, 3), (4, 5, 6)]
        )"#,
        );
    }

    #[test]
    fn test_custom_data() {
        print_and_test(
            Rule::metadata_for_test,
            r#"
        (
            customData = {
                string a = "123"
                int b = 456
                int[] bs = [456, 789]
                float c = 1.234
                float cs = [1.234, 5.678]
                token d = "Enum"
                token[] allowedTokens = ["Enum1", "Enum2", "Enum3"]
            }
        )
        "#,
        );
    }

    #[test]
    fn test_prim_attribute() {
        print_and_test(
            Rule::prim_attribute_for_test,
            "float2 xformOp:translate = (1.0, 2.0)",
        );
        print_and_test(
            Rule::prim_attribute_for_test,
            "float2[] xformOp:translate = [(1.0, 2.0), (3.0, 4.0)]",
        );
        print_and_test(Rule::prim_attribute_for_test, "int a = 1");
        print_and_test(Rule::prim_attribute_for_test, "int[] a = [1, 2, 3]");
        print_and_test(Rule::prim_attribute_for_test, "float a = 1.234");
        print_and_test(Rule::prim_attribute_for_test, "float[] a = [1.234, 5.678]");
        print_and_test(Rule::prim_attribute_for_test, "string a = \"123\"");
        print_and_test(
            Rule::prim_attribute_for_test,
            "string[] a = [\"123\", \"456\"]",
        );
        print_and_test(Rule::prim_attribute_for_test, "custom int a = 1");
        print_and_test(Rule::prim_attribute_for_test, "custom int a");
        print_and_test(
            Rule::prim_attribute_for_test,
            "uniform token a = 'WorldSpace'",
        );
    }

    #[test]
    fn test_variant_set() {
        print_and_test(
            Rule::variant_set_stmt_for_test,
            r#"
        variantSet "shadingVariant" = {
            "blue" {
                over "world"
                {
                    color3f[] primvars:displayColor = [(0, 0, 1)]
                }
            }
    
            "green" {
                over "world"
                {
                    color3f[] primvars:displayColor = [(0, 1, 0)]
                }
            }
    
            "red" {
                over "world"
                {
                    color3f[] primvars:displayColor = [(1, 0, 0)]
                }
            }
        }
        "#,
        );
    }

    #[test]
    fn test_prim() {
        print_and_test(
            Rule::prim_stmt_for_test,
            r#"
            def Xform "hello" (
                variants = {
                    string shadingVariant = "green"
                }
                prepend variantSets = "shadingVariant"
            )
            {
                custom double3 xformOp:translate = (4, 5, 6)
                uniform token[] xformOpOrder = ["xformOp:translate"]
            
                def Sphere "world"
                {
                    float3[] extent = [(-2, -2, -2), (2, 2, 2)]
                    color3f[] primvars:displayColor
                    double radius = 2
                }
            
                variantSet "shadingVariant" = {
                    "blue" {
                        over "world"
                        {
                            color3f[] primvars:displayColor = [(0, 0, 1)]
                        }
                    }
            
                    "green" {
                        over "world"
                        {
                            color3f[] primvars:displayColor = [(0, 1, 0)]
                        }
                    }
            
                    "red" {
                        over "world"
                        {
                            color3f[] primvars:displayColor = [(1, 0, 0)]
                        }
                    }
                }
            }
        "#,
        );
    }

    #[test]
    fn test_layer_ref() {
        print_and_test(
            Rule::prim_stmt_for_test,
            r#"    
            def "_04575deb781a141a28d3f9e4270abc7c" (
                prepend references = @./Workspace.resources.usdc@</_04575deb781a141a28d3f9e4270abc7c>
            )
            {
            }
            "#,
        );
    }

    #[test]
    fn test_layer() {
        print_and_test(
            Rule::usd,
            r#"
                #usda 1.0
                (
                    "DO NOT modify: this file is auto-generated."
                    subLayers = [
                        @./surfacing.usda@
                    ]
                    doc = """Generated from Composed Stage of root layer RefExample.usda
                """
                )

                def Xform "refSphere"
                {
                    double3 xformOp:translate = (4, 5, 6)
                    uniform token[] xformOpOrder = []

                    def Sphere "world"
                    {
                        float3[] extent = [(-2, -2, -2), (2, 2, 2)]
                        color3f[] primvars:displayColor = [(0, 0, 1)]
                        double radius = 2
                    }
                }

                def Xform "refSphere2"
                {
                    double3 xformOp:translate = (4, 5, 6)
                    uniform token[] xformOpOrder = ["xformOp:translate"]

                    def Sphere "world"
                    {
                        float3[] extent = [(-2, -2, -2), (2, 2, 2)]
                        color3f[] primvars:displayColor = [(1, 0, 0)]
                        double radius = 2
                    }
                }
        "#,
        );
    }
}
