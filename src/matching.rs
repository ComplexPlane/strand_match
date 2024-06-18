use std::cmp::Ordering;
use std::collections::HashMap;

use crate::function::AsmFunction;
use crate::mapfile;

pub fn match_funcs(sdk_funcs: Vec<AsmFunction>, rel_funcs: Vec<AsmFunction>) -> anyhow::Result<()> {
    run_matching(&sdk_funcs, &rel_funcs)
}

/*
New simpler matching function:
for each sdk function:
    add sdk function to one list per matching rel function
        rel_sdk_map.entry(rel_ref).or_insert(Vec::new()).push(sdk_ref)

print out rels with single matching sdks, then doubles
*/
fn run_matching(sdk_funcs: &[AsmFunction], rel_funcs: &[AsmFunction]) -> anyhow::Result<()> {
    let mut rel_sdk_map = HashMap::new();
    for sdk in sdk_funcs {
        for (rel_idx, rel) in rel_funcs.iter().enumerate() {
            if compare_simple(sdk, rel) != 0 {
                continue;
            }

            let matches_of_rel = rel_sdk_map.entry(rel_idx).or_insert(Vec::new());
            match matches_of_rel.first() {
                None => matches_of_rel.push(sdk),

                Some(curr_sdk_match) => {
                    let curr_exact_matches = exact_matching_insns(curr_sdk_match, rel);
                    let new_exact_matches = exact_matching_insns(sdk, rel);

                    match new_exact_matches.cmp(&curr_exact_matches) {
                        Ordering::Greater => {
                            matches_of_rel.clear();
                            matches_of_rel.push(sdk);
                        }
                        Ordering::Equal => {
                            matches_of_rel.push(sdk);
                        }
                        Ordering::Less => {
                            // Don't count this sdk function if there's less exact matches than what's
                            // already matched with the rel
                        }
                    }
                }
            }
        }
    }

    let mut rel_sdk_matches: Vec<_> = rel_sdk_map
        .iter()
        .filter(|(_, sdk_matches)| sdk_matches.len() == 1)
        .map(|(rel_i, sdk_matches)| (sdk_matches[0], &rel_funcs[*rel_i]))
        .collect();
    rel_sdk_matches.sort_unstable();

    println!("======================== strand_match Summary ==========================\n");

    println!("REL functions matched by a single SDK function:");
    for (sdk, rel) in &rel_sdk_matches {
        println!("{} -> {}", sdk.full_name(), rel.name);
    }

    println!("\n\nREL functions matched by more than one SDK function:");
    for (rel_idx, sdk_list) in rel_sdk_map {
        if sdk_list.len() < 2 || sdk_list[0].len <= 5 {
            continue;
        }
        let decent_sdk_matches: Vec<_> = sdk_list
            .iter()
            .filter(|sdk| exact_matching_insns(&rel_funcs[rel_idx], sdk) > 3)
            .map(|sdk| sdk.full_name())
            .collect();
        if !decent_sdk_matches.is_empty() {
            let lst_str = decent_sdk_matches.join(", ");
            println!("[{}] -> {}", lst_str, rel_funcs[rel_idx].name);
        }
    }

    mapfile::export_mapfile(&rel_sdk_matches)
}

// If lengths are equal, return how many instructions differ, not counting relocated instructions.
// If lengths differ, return std::usize::MAX
fn compare_simple(a: &AsmFunction, b: &AsmFunction) -> usize {
    if a.code.len() == b.code.len() {
        a.code
            .iter()
            .zip(b.code.iter())
            .filter(|pair| *pair.0 != 0 && *pair.1 != 0 && pair.0 != pair.1)
            .count()
    } else {
        std::usize::MAX
    }
}

// How many instructions match between the functions, where relocated instructions don't count?
fn exact_matching_insns(a: &AsmFunction, b: &AsmFunction) -> usize {
    if a.code.len() == b.code.len() {
        a.code
            .iter()
            .zip(b.code.iter())
            .filter(|pair| pair.0 == pair.1 && *pair.0 != 0 && *pair.1 != 0)
            .count()
    } else {
        0
    }
}
