pub mod utils;
use crate::logging::create_log_dirs;
use crate::logging::log_masterhelp_output;
use serde_json::{json, map::Map, Value, Value::Array};
use std::path::Path;
use utils::logging;

pub fn ingest_commands() -> Vec<String> {
    create_log_dirs();
    let cli_help_output = get_command_help("");
    check_success(&cli_help_output.status);
    let raw_help = std::string::String::from_utf8(cli_help_output.stdout)
        .expect("Invalid, not UTF-8. Error!");
    log_masterhelp_output(&raw_help);
    let help_lines_iter = raw_help.lines();
    let mut help_lines = Vec::new();
    for li in help_lines_iter {
        if li != "" && !li.starts_with("=") {
            help_lines.push(li);
        }
    }
    let mut commands_str = Vec::new();
    for line in help_lines {
        let mut temp_iter = line.split_ascii_whitespace();
        match temp_iter.next() {
            Some(x) => commands_str.push(x),
            None => panic!("error during command parsing"),
        }
    }
    let mut commands = Vec::new();
    for c in commands_str {
        commands.push(c.to_string());
    }
    commands
}

pub fn get_command_help(cmd: &str) -> std::process::Output {
    let command_help = std::process::Command::new(Path::new("zcash-cli"))
        .arg("help")
        .arg(&cmd)
        .output()
        .expect("failed to execute command help");
    command_help
}

pub fn check_success(output: &std::process::ExitStatus) {
    // simple boolean that output succeeded by spawning
    // and monitoring child process, if false: panic
    assert!(output.success());
    // then match output exit code
    match output.code() {
        Some(0) => (),
        Some(_) => panic!("exit code not 0"),
        None => panic!("error! no exit code"),
    }
}

pub fn interpret_raw_output(raw_command_help: &str) -> String {
    let (cmd_name, result_data) = extract_name_and_result(raw_command_help);
    // TODO remove these kind of special cases or consolidate
    // OR use tests to demonstrate?
    /*
    if cmd_name == "getblockchaininfo".to_string() {
        // TODO this token does appear, but it is read?
        result_data = result_data.replace("[0..1]", "ZZZZZZ");
        // TODO this token seems to be meaningful, therefore should
        // be used or incorporated elsewhere
        result_data = result_data.replace("}, ...", "}");
        // TODO consider also, "reject" (same fields as "enforce")
        // special case? `{ ... }` on line
    }
    */
    let result_chars = &mut result_data.chars();
    let last_char = result_chars.next().expect("Missing first char!");
    let context = &mut Context {
        cmd_name,
        last_char,
    };
    let annotated_json_text =
        annotate_result(context, result_chars).to_string();
    annotated_json_text
}

fn extract_name_and_result(raw_command_help: &str) -> (String, String) {
    let result_sections =
        raw_command_help.split("Result:\n").collect::<Vec<&str>>();
    // TODO? instead of panicing, failed check break to next command
    // related to `blessed` commands, defined slightly differently,
    // these checks could be folded into or serve to augment blessed.
    assert_eq!(result_sections.len(), 2, "Wrong number of Results!");
    let cmd_name = result_sections[0]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()[0];
    let end_section = result_sections[1];
    let example_sections =
        end_section.split("Examples:\n").collect::<Vec<&str>>();
    // TODO same as last comment.
    assert_eq!(example_sections.len(), 2, "Wrong number of Examples!");
    (cmd_name.to_string(), example_sections[0].trim().to_string())
}

// TODO consider need for struct; related to 'special cases' revisions
// also, last_char changes within parsing individual command but
// cmd_name doesn't, why bind a variable and a 'constant' in a struct?
struct Context {
    cmd_name: String,
    last_char: char,
}

fn annotate_result(
    mut context: &mut Context,
    mut result_chars: &mut std::str::Chars,
) -> serde_json::Value {
    let mut viewed = String::new();
    match context.last_char {
        '{' => {
            let mut ident_label_bindings = Map::new();
            let mut partial_ident_label_bindings = Map::new();
            loop {
                match result_chars.next().unwrap() {
                    '}' => {
                        dbg!("end brace");
                        dbg!(&viewed);
                        if viewed.trim().is_empty(){
                            break;
                        }
                        partial_ident_label_bindings = bind_idents_labels(
                            viewed.clone(),
                            context.cmd_name.clone(),
                            None,
                        );
                        viewed.clear();
                        dbg!(&partial_ident_label_bindings);
                        ident_label_bindings.append(&mut partial_ident_label_bindings);

                        dbg!(&ident_label_bindings);
                        break;
                    }
                    last_viewed if last_viewed == '[' || last_viewed == '{' => {
                        dbg!("recursing");
                        let inner_value = recurse(
                            last_viewed,
                            &mut context,
                            &mut result_chars,
                        );
                        dbg!(&inner_value);
                        // needs a different funtion to construct
                        // intermediate Map.
                        // bind_ident_labels returns a Map.
                        partial_ident_label_bindings = bind_idents_labels(
                            viewed.clone(),
                            context.cmd_name.clone(),
                            Some(inner_value),
                        );
                        viewed.clear();

                        ident_label_bindings.append(&mut partial_ident_label_bindings);
                          
                        //break;
                    }
                    // TODO: Handle unbalanced braces
                    x if x.is_ascii() => viewed.push(x),
                    _ => panic!("character is UTF-8 but not ASCII!"),
                }
            }
            dbg!(&ident_label_bindings);
            return Value::Object(ident_label_bindings);
        }
        //TODO bring arrays up to speed
        '[' => {
            let mut ordered_results: Vec<Value> = vec![];
            loop {
                match result_chars.next().unwrap() {
                    ']' => {
                        ordered_results = label_by_position(viewed.clone());
                        break;
                    }
                    last_viewed if last_viewed == '[' || last_viewed == '{' => {
                        let inner_value = recurse(
                            last_viewed,
                            &mut context,
                            &mut result_chars,
                        );
                        bind_idents_labels(
                            viewed.clone(),
                            context.cmd_name.clone(),
                            Some(inner_value),
                        );
                        //dbg!(&inner_value);
                        /*if inner_value.is_object() {
                            //inner = serde_json::Value::Object(temp);
                            inner_object = inner_value.as_object().unwrap().clone();
                            dbg!(&inner_object);
                        } else if inner_value.is_array() {
                            //inner = serde_json::Value::Array(temp);
                            inner_array = Array(inner_value.as_array().unwrap().clone());
                            dbg!(&inner_array);
                        }*/
                    }
                    // TODO: Handle unbalanced braces
                    x if x.is_ascii() => viewed.push(x),
                    _ => panic!("character is UTF-8 but not ASCII!"),
                }
            }
            return Value::Array(ordered_results);
        }
        _ => todo!(),
    }
}

// could be cleaned up, and/or broken into cases 
// as opposed to internal conditional logic.
fn bind_idents_labels(
    viewed: String,
    cmd_name: String,
    inner_value: Option<Value>,
) -> Map<String, Value> {
    dbg!("bind_idents_labels called");
    let cleaned = clean_viewed(viewed);
    //cleaned is now a Vec of strings (that were lines in viewed).
    /*
    // consolodate special cases
    if cleaned[0] == "...".to_string()
        && cmd_name == "getblockchaininfo".to_string()
    {
        special_cases::getblockchaininfo_reject::create_bindings()
    } else { ...
     }
    */
    //dbg!(&cleaned);
    //dbg!(&inner_value);
    if inner_value != None { // possible if/let
        let mut cleaned_mutable = cleaned.clone();
        let last_ident_untrimmed = cleaned_mutable.pop().unwrap();
        let last_ident = last_ident_untrimmed
            .trim()
            .splitn(2, ':')
            .collect::<Vec<&str>>()[0]
            .trim_matches('"');
        let mut begin_map = Map::new();
        if cleaned_mutable.len() > 0 {
            begin_map = cleaned_mutable
                .iter()
                .map(|ident_rawlabel| {
                    label_identifier(
                        ident_rawlabel.to_string(),
                        cmd_name.as_str(),
                    )
                })
                .map(|(a, b)| (a.to_string(), json!(b.to_string())))
                .collect::<Map<String, Value>>();
        }
        //dbg!(&begin_map);
        // TODO create return from begin_map and following;
        // currently set to `return`
        // && make acceptable to outer Value
        return [(last_ident, inner_value.unwrap())]
            .iter()
            .cloned()
            .map(|(a, b)| (a.to_string(), b))
            .collect::<Map<String, Value>>();
    } else {
        return cleaned
            .iter() // back into iter, could streamline?
            .map(|ident_rawlabel| {
                label_identifier(ident_rawlabel.to_string(), cmd_name.as_str())
            })
            .map(|(ident, annotation)| (ident.to_string(), json!(annotation.to_string())))
            .collect::<Map<String, Value>>();
    }
}

// consolodate with other preparation?
fn clean_viewed(raw_viewed: String) -> Vec<String> {
    let mut ident_labels = raw_viewed
        .trim_end()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    match ident_labels.remove(0).trim() { //TODO these are special cases
        empty if empty.is_empty() => (),
        description if description.contains("(object)") => (),
        i if i == "...".to_string() => ident_labels = vec![String::from(i)],
        catchall @ _ => {
            dbg!(catchall);
        }
    }
    ident_labels
}

// TODO consolidate special cases
/*mod special_cases {
    pub(crate) mod getblockchaininfo_reject {
        pub const TRAILING_TRASH: &str = "      (object)";
        use serde_json::{json, Map, Value};
        pub const BINDINGS: [(&str, &str); 4] = [
            ("found", "Decimal"),
            ("required", "Decimal"),
            ("status", "bool"),
            ("window", "Decimal"),
        ];
        pub fn create_bindings() -> Map<String, Value> {
            BINDINGS
                .iter()
                .map(|(a, b)| (a.to_string(), json!(b)))
                .collect()
        }
    }
}
*/

// assumes well-formed `ident_with_metadata`
fn label_identifier(
    ident_with_metadata: String,
    cmd_name: &str,
) -> (String, String) {
    let ident_and_metadata = ident_with_metadata
        .trim()
        .splitn(2, ':')
        .collect::<Vec<&str>>();
    let ident = ident_and_metadata[0].trim_matches('"');
    let meta_data = ident_and_metadata[1].trim();
    //dbg!(&meta_data);
    let mut annotation = String::new();
    /*
    // TODO special case
    // consolodate
    if meta_data
        .contains(special_cases::getblockchaininfo_reject::TRAILING_TRASH)
        && cmd_name == "getblockchaininfo".to_string()
    {
        meta_data = meta_data
            .split(special_cases::getblockchaininfo_reject::TRAILING_TRASH)
            .collect::<Vec<&str>>()[0]
            .trim();
    }
    if meta_data.starts_with('{') || meta_data.starts_with('[') {
        annotation = meta_data.to_string(); 
    } else {*/
        let raw_label: &str = meta_data
            .split(|c| c == '(' || c == ')')
            .collect::<Vec<&str>>()[1];
        annotation = make_label(raw_label);
    //}
    (ident.to_string(), annotation)
}

fn make_label(raw_label: &str) -> String {
    let mut annotation = String::new();
    if raw_label.starts_with("numeric") {
        annotation.push_str("Decimal");
    } else if raw_label.starts_with("string") {
        annotation.push_str("String");
    } else if raw_label.starts_with("boolean") {
        annotation.push_str("bool");
    } else {
        panic!("annotation should have a value at this point.");
    }
    if raw_label.contains(", optional") {
        return format!("Option<{}>", annotation);
    }
    annotation
}

fn recurse(
    last_viewed: char,
    mut context: &mut Context,
    mut result_chars: &mut std::str::Chars,
) -> serde_json::value::Value {
    context.last_char = last_viewed;
    annotate_result(&mut context, &mut result_chars)
}

fn label_by_position(raw_observed: String) -> Vec<Value> {
    let trimmed = raw_observed
        .trim_end_matches(|c| c != '}')
        .trim_start_matches(|c| c != '{');
    vec![Value::Object(
        serde_json::from_str::<Map<String, Value>>(trimmed)
            .expect("Couldn't map into a Map<String, Value>!"),
    )]
}

// ------------------- tests ----------------------------------------

#[cfg(test)]
mod unit {
    use super::*;
    use crate::utils::test;
    use serde_json::json;

    // ----------------label_identifier---------------

    #[test]
    fn label_identifier_with_expected_input_valid() {
        let raw_version =
            r#""version": xxxxx,           (numeric) the server version"#;
        let valid_annotation = ("version".to_string(), "Decimal".to_string());
        assert_eq!(
            valid_annotation,
            label_identifier(raw_version.to_string(), "")
        );
    }

    // ----------------annotate_result---------------

    #[test]
    fn annotate_result_simple_unnested_generate() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let last_char = simple_unnested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_unnested,
        );
        let expected_result = test::simple_unnested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_to_string() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let last_char = simple_unnested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_unnested,
        );
        let expected_annotation = test::SIMPLE_UNNESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_unnested() {
        let mut simple_unnested = &mut test::SIMPLE_UNNESTED.chars();
        let last_char = simple_unnested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_unnested,
        );
        let expected_annotation: Value = serde_json::de::from_str(test::SIMPLE_UNNESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_nested_to_string() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let last_char = simple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_nested,
        );
        //TODO rename this const in test?
        let expected_annotation = test::SIMPLE_NESTED_RESULT;
        assert_eq!(expected_annotation, annotated.to_string());
    }

    #[test]
    fn annotate_result_simple_nested() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let last_char = simple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_nested,
        );
        let expected_annotation: Value = serde_json::de::from_str(test::SIMPLE_NESTED_RESULT).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED.chars();
        let last_char = multiple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut multiple_nested,
        );
        let expected_annotation: Value = serde_json::de::from_str(test::MULTIPLE_NESTED_ANNOTATION).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    //TODO 
    // make saner sanity checks
    #[test]
    fn annotate_result_multiple_nested_2() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_2.chars();
        let last_char = multiple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut multiple_nested,
        );
        let expected_annotation: Value = serde_json::de::from_str(test::MULTIPLE_NESTED_2_ANNOTATION).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_multiple_nested_3() {
        let mut multiple_nested = &mut test::MULTIPLE_NESTED_3.chars();
        let last_char = multiple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut multiple_nested,
        );
        let expected_annotation: Value = serde_json::de::from_str(test::MULTIPLE_NESTED_ANNOTATION).unwrap();
        assert_eq!(expected_annotation, annotated);
    }

    #[test]
    fn annotate_result_simple_unnested_getblockchaininfo() {
        let mut simple_unnested_blockchaininfo =
            &mut test::SIMPLE_UNNESTED_GETBLOCKCHAININFO.chars();
        let last_char = simple_unnested_blockchaininfo
            .next()
            .expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_unnested_blockchaininfo,
        );
        let expected_result = test::SIMPLE_UNNESTED_GETBLOCKCHAININFO_RESULT;
        assert_eq!(expected_result, annotated.to_string());
    }

    #[test]
    fn annotate_result_from_getinfo_expected() {
        let expected_testdata_annotated = test::valid_getinfo_annotation();
        let (cmd_name, section_data) =
            extract_name_and_result(test::HELP_GETINFO);
        let data_stream = &mut section_data.chars();
        let last_char = data_stream.next().unwrap();
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name,
            },
            data_stream,
        );
        assert_eq!(annotated, expected_testdata_annotated);
    }

    #[test]
    fn annotate_result_enforce_as_input() {
        use std::collections::HashMap;
        let testmap = json!(test::INTERMEDIATE_REPR_ENFORCE
            .iter()
            .map(|(a, b)| (a.to_string(), json!(b.to_string())))
            .collect::<HashMap<String, Value>>());
        assert_eq!(
            testmap,
            annotate_result(
                &mut Context {
                    last_char: '{',
                    cmd_name: "getblockchaininfo".to_string()
                },
                &mut test::ENFORCE_EXTRACTED.chars(),
            )
        );
    }

    // ------------------ annotate_result : ignored --------

    //TODO generators may be inherently flawed 
    #[ignore]
    #[test]
    fn annotate_result_simple_nested_generate() {
        let mut simple_nested = &mut test::SIMPLE_NESTED.chars();
        let last_char = simple_nested.next().expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut simple_nested,
        );
        let expected_result = test::simple_nested_json_generator();
        assert_eq!(expected_result, annotated);
    }

    //rename function for clarity?
    #[ignore]
    #[test]
    fn annotate_result_help_getblockchain_reject_fragment() {
        let expected_data = test::GETBLOCKCHAININFO_REJECT_FRAGMENT;
        let (cmd_name, _) = extract_name_and_result(expected_data);
        let fake_ident_label = "...".to_string();
        let bound = bind_idents_labels(fake_ident_label, cmd_name, None);
        for (k, v) in test::INTERMEDIATE_REPR_ENFORCE.iter() {
            assert_eq!(&bound.get(k.clone()).unwrap().as_str().unwrap(), v);
        }
    }

    #[ignore]
    #[test]
    fn annotate_result_special_nested_blockchaininfo() {
        let mut special_nested_blockchaininfo =
            &mut test::SPECIAL_NESTED_GETBLOCKCHAININFO.chars();
        let last_char = special_nested_blockchaininfo
            .next()
            .expect("Missing first char!");
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut special_nested_blockchaininfo,
        );
        let expected_result = test::SPECIAL_NESTED_GETBLOCKCHAININFO_RESULT;
        assert_eq!(expected_result, annotated.to_string());
    }

    #[ignore]
    #[test]
    fn annotate_result_nested_obj_extracted_from_softfork() {
        let mut expected_nested = test::SIMPLIFIED_SOFTFORK.chars();
        let last_char = expected_nested.nth(0).unwrap();
        let annotated = annotate_result(
            &mut Context {
                last_char,
                cmd_name: "getblockchaininfo".to_string(),
            },
            &mut expected_nested,
        );
        let expected_enforce: Map<String, Value> =
            serde_json::from_str(test::SOFTFORK_EXTRACT_JSON).unwrap();
        assert_eq!(Value::Object(expected_enforce), annotated);
    }

    // ----------------sanity_check---------------

    #[test]
    fn sanity_check_simple_unnested() {
        let simple_unnested_result = test::SIMPLE_UNNESTED_RESULT.to_string();
        let simple_unnested_json =
            test::simple_unnested_json_generator().to_string();
        assert_eq!(simple_unnested_result, simple_unnested_json);
    }

    #[test]
    fn sanity_check_simple_nested() {
        let simple_nested_result = test::SIMPLE_NESTED_RESULT.to_string();
        let simple_nested_json =
            test::simple_nested_json_generator().to_string();
        assert_eq!(simple_nested_result, simple_nested_json);
    }

    #[test]
    fn sanity_check_multiple_nested() {
        let multiple_nested_annotation = test::MULTIPLE_NESTED_ANNOTATION.to_string();
        let multiple_nested_json =
            test::multiple_nested_json_generator().to_string();
        assert_eq!(multiple_nested_annotation, multiple_nested_json);
    }

    // this test returns a non-equivalence (failure) due to the macro
    // in `multiple_nested_2_json_generator().to_string()` 
    // serializing key-value pairs in a different order than is 
    // provided as the input to the macro.
    // One possible solution is to compare actual JSON values as opposed
    // to testing for the equivalence of the serialized JSON strings.
    #[ignore]
    #[test]
    fn sanity_check_multiple_nested_2() {
        let multiple_nested_2_annotation = test::MULTIPLE_NESTED_2_ANNOTATION.to_string();
        let multiple_nested_2_json =
            test::multiple_nested_2_json_generator().to_string();
        assert_eq!(multiple_nested_2_annotation, multiple_nested_2_json);
    }
    // ----------------interpret_raw_output---------------

    #[test]
    fn interpret_raw_output_simple_unnested_full() {
        let simple_unnested_full = test::SIMPLE_UNNESTED_FULL;
        let interpreted = interpret_raw_output(simple_unnested_full);
        let expected_result = test::SIMPLE_UNNESTED_RESULT;
        assert_eq!(interpreted, expected_result);
    }

    #[test]
    fn interpret_raw_output_simple_nested_full() {
        let simple_nested_full = test::SIMPLE_NESTED_FULL;
        let interpreted = interpret_raw_output(simple_nested_full);
        let expected_result = test::SIMPLE_NESTED_RESULT;
        assert_eq!(interpreted, expected_result);
    }


    #[test]
    #[should_panic]
    fn interpret_raw_output_extrabrackets_within_input_lines() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS3_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[test]
    #[should_panic]
    fn interpret_raw_output_more_than_one_set_of_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::MORE_BRACKET_PAIRS_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_two_starting_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRA_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_two_ending_brackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRA_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_results_input() {
        let valid_help_in = interpret_raw_output(test::NO_RESULT_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_end_bracket_input() {
        let valid_help_in =
            interpret_raw_output(test::NO_END_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }
    #[test]
    #[should_panic]
    fn interpret_raw_output_no_start_bracket_input() {
        let valid_help_in =
            interpret_raw_output(test::NO_START_BRACKET_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    // ----------------interpret_raw_output : ignored---------------

    // TODO look at these first few
    #[ignore]
    #[test]
    fn interpret_raw_output_upgrades_in_obj_extracted() {
        dbg!(interpret_raw_output(test::UPGRADES_IN_OBJ_EXTRACTED));
    }
    // what is test::valid_getinfo_annotation()
    #[ignore]
    #[test]
    fn interpret_raw_output_expected_input_valid() {
        let valid_help_in = interpret_raw_output(test::HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_lbracket_input() {
        let valid_help_in = interpret_raw_output(test::LBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_rbracket_input() {
        let valid_help_in = interpret_raw_output(test::RBRACKETY_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_early_extrabrackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS1_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_late_extrabrackets_input() {
        let valid_help_in =
            interpret_raw_output(test::EXTRABRACKETS2_HELP_GETINFO);
        assert_eq!(valid_help_in, test::valid_getinfo_annotation());
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_softforks_fragment() {
        let expected_incoming = test::GETBLOCKCHAININFO_SOFTFORK_FRAGMENT;
        let expected_results = r#"{"softforks":"[{\"enforce\":\"{\\\"found\\\":\\\"Decimal\\\",\\\"required\\\":\\\"Decimal\\\",\\\"status\\\":\\\"bool\\\",\\\"window\\\":\\\"Decimal\\\"},\",\"id\":\"String\",\"reject\":\"{\\\"found\\\":\\\"Decimal\\\",\\\"required\\\":\\\"Decimal\\\",\\\"status\\\":\\\"bool\\\",\\\"window\\\":\\\"Decimal\\\"}\",\"version\":\"Decimal\"}],"}"#;
        assert_eq!(
            format!("{}", interpret_raw_output(expected_incoming)),
            expected_results
        );
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_enforce_and_reject_fragment() {
        let expected_incoming =
            test::GETBLOCKCHAININFO_ENFORCE_AND_REJECT_FRAGMENT;
        let expected_results = r#"{"enforce":"{\"found\":\"Decimal\",\"required\":\"Decimal\",\"status\":\"bool\",\"window\":\"Decimal\"},","id":"String","reject":"{\"found\":\"Decimal\",\"required\":\"Decimal\",\"status\":\"bool\",\"window\":\"Decimal\"}","version":"Decimal"}"#;
        let interpreted =
            format!("{}", interpret_raw_output(expected_incoming));
        assert_eq!(interpreted, expected_results);
    }

    #[ignore]
    #[test]
    fn interpret_raw_output_getblockchaininfo_complete() {
        dbg!(interpret_raw_output(test::HELP_GETBLOCKCHAININFO_COMPLETE));
    }

    // ----------------serde_json_value----------------
    // need to be retooled or deprecated
    #[test]
    fn serde_json_value_help_getinfo() {
        let getinfo_serde_json_value = test::getinfo_export();
        let help_getinfo = interpret_raw_output(test::HELP_GETINFO);
        assert_eq!(getinfo_serde_json_value.to_string(), help_getinfo);
    }

    // ----------------serde_json_value : ignored---------------
    // need to be retooled or deprecated
    #[ignore]
    #[test]
    fn serde_json_value_help_getblockchaininfo() {
        let getblockchaininfo_serde_json_value =
            test::getblockchaininfo_export();
        let help_getblockchaininfo =
            interpret_raw_output(test::HELP_GETBLOCKCHAININFO_COMPLETE);
        assert_eq!(
            getblockchaininfo_serde_json_value.to_string(),
            help_getblockchaininfo
        );
    }
}
