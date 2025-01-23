use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks, TypeKind};

#[derive(Debug)]
struct NMCallbacks {}

impl ParseCallbacks for NMCallbacks {
    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        if info.kind == TypeKind::Enum {
            ["TryFromPrimitive".to_owned()].to_vec()
        } else {
            Vec::new()
        }
    }

    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        // Dont include the macros as they are just dbus paths
        match name.starts_with("NM_") {
            true => MacroParsingBehavior::Ignore,
            false => MacroParsingBehavior::Default,
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        // Try to give more sensible enum variant names by stripping common prefixes
        let mut enum_name = enum_name.unwrap();
        let name_prefix = match enum_name {
            "NMConnectivityState" => "NM_CONNECTIVITY_".to_owned(),
            "NMDeviceCapabilities" => "NM_DEVICE_CAP_".to_owned(),
            "NMDeviceWifiCapabilities" => "NM_WIFI_DEVICE_CAP_".to_owned(),
            "NM80211ApFlags" => "NM_802_11_AP_FLAGS_".to_owned(),
            "NM80211ApSecurityFlags" => "NM_802_11_AP_SEC_".to_owned(),
            "NM80211Mode" => "NM_802_11_MODE_".to_owned(),
            "NMBluetoothCapabilities" => "NM_BT_CAPABILITY_".to_owned(),
            "NMDeviceModemCapabilities" => "NM_DEVICE_MODEM_CAPABILITY_".to_owned(),
            "NMSecretAgentCapabilities" => "NM_SECRET_AGENT_CAPABILITY_".to_owned(),
            "NMIPTunnelMode" => "NM_IP_TUNNEL_MODE_".to_owned(),
            _ => {
                // Most of the time the common prefix can be found by using the CamelCaseness of the names
                let mut name_parts = Vec::new();

                if enum_name.starts_with("NM") {
                    enum_name = &enum_name[2..];
                    name_parts.push("NM");
                }

                let mut start_idx = 0;
                for (idx, char) in enum_name.char_indices() {
                    if char.is_uppercase() && idx != start_idx {
                        name_parts.push(&enum_name[start_idx..idx]);
                        start_idx = idx;
                    }
                }
                let end_part = &enum_name[start_idx..];

                // Flags tend to have prefixes that end with FLAG not FLAGS
                if end_part.ends_with("Flags") {
                    name_parts.push(&end_part[..end_part.len() - 1]);
                } else {
                    name_parts.push(end_part);
                }

                name_parts
                    .into_iter()
                    .map(|name_part| name_part.to_uppercase())
                    .collect::<Vec<String>>()
                    .join("_")
                    + "_"
            }
        };

        if let Some(varient_name) = original_variant_name.strip_prefix(&name_prefix) {
            if varient_name.chars().next().unwrap().is_numeric() {
                Some("NM_".to_owned() + varient_name)
            } else {
                Some(varient_name.to_string())
            }
        } else {
            None
        }
    }
    fn read_env_var(&self, key: &str) {
        println!("cargo:rerun-if-env-changed={key}");
    }
}

fn main() {
    const INTERFACE_HEADER_NAME: &str = "nm-dbus-interface.h";
    println!("cargo:rerun-if-changed={INTERFACE_HEADER_NAME}");

    let processed_output_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join(INTERFACE_HEADER_NAME);

    fix_docstrings(
        &PathBuf::from(INTERFACE_HEADER_NAME),
        &processed_output_path,
    );

    let bindings = bindgen::Builder::default()
        .header(processed_output_path.to_str().unwrap())
        .parse_callbacks(Box::new(NMCallbacks {}))
        .rustified_enum("NM.*")
        .prepend_enum_name(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

struct VariantComment {
    variant: String,
    comment: String,
}
fn process_variant_comment(variant_comment: String) -> VariantComment {
    let (variant, comment) = variant_comment.split_once(':').unwrap();
    let variant = variant.split_once('@').unwrap().1;
    VariantComment {
        variant: variant.to_owned(),
        comment: comment.to_owned(),
    }
}

fn fix_docstrings(original: &Path, output: &Path) {
    // This function inlines the doc string so bindgen generates them correctly.
    // The basic process is as follows:
    // 1. Split the file line by line
    // 2. Collect the variant doc strings
    //   a. find lines containing @NM_, these are the start of the doc strings.
    //   b. collect multi line doc strings
    //   c. All other lines are consider not part of a multi line doc string
    // 3. Iterate over the collected variant doc strings and insert them
    //   a. find the first line that contains the variant name that is not part of a comment
    //   b. insert the doc string above it.
    const VARIANT_COMMENT_START: &str = "* @NM_";

    let header_file = std::fs::read_to_string(original).unwrap();

    let lines = header_file.split('\n').collect::<Vec<&str>>();
    let mut lines = VecDeque::from(lines);

    let mut varient_comments = Vec::new();
    let mut without_varient_comments = Vec::new();

    'eof: loop {
        let Some(line) = lines.pop_front() else {
            break 'eof;
        };
        if line.contains(VARIANT_COMMENT_START) {
            let mut variant = line.to_owned();
            'variant_comment_lines: loop {
                let Some(first_line) = lines.front() else {
                    break 'eof;
                };
                match *first_line {
                    " *" | " */" | " **/" => break 'variant_comment_lines,
                    line if line.contains(VARIANT_COMMENT_START) => break 'variant_comment_lines,
                    _ => {
                        let comment_line = lines.pop_front().unwrap();
                        variant.push('\n');
                        variant.push_str(comment_line);
                    }
                }
            }
            varient_comments.push(process_variant_comment(variant));
        } else {
            without_varient_comments.push(line);
        }
    }

    let mut processed_file = without_varient_comments
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();
    for vc in varient_comments.into_iter() {
        let line_idx = processed_file
            .iter()
            .enumerate()
            .find_map(|(idx, line)| {
                (line.contains(&vc.variant) && !(line.starts_with(" *") || line.starts_with("/**")))
                    .then_some(idx)
            })
            .unwrap();
        processed_file.insert(line_idx, format!("/**\n{}\n**/", vc.comment));
    }

    let processed_file = processed_file.join("\n");

    std::fs::write(output, processed_file).unwrap();
}
