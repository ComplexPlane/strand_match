use std::collections::HashMap;

use crate::function::AsmFunction;

const BL_OPCODE: u32 = 0b01001000_00000000_00000000_00000001;
const BL_OPCODE_MASK: u32 = 0b11111100_00000000_00000000_00000001;

pub fn match_funcs(mut sdk_funcs: Vec<AsmFunction>, mut rel_funcs: Vec<AsmFunction>) {
    run_matching(&sdk_funcs, &rel_funcs);
}

/*
New simpler matching function:
for each sdk function:
    add sdk function to one list per matching rel function
        rel_sdk_map.entry(rel_ref).or_insert(Vec::new()).push(sdk_ref)

print out rels with single matching sdks, then doubles
*/
fn run_matching(sdk_funcs: &[AsmFunction], rel_funcs: &[AsmFunction]) {
    let mut rel_sdk_map = HashMap::new();
    for sdk in sdk_funcs {
        for (rel_idx, rel) in rel_funcs.iter().enumerate() {

            if compare_simple(sdk, rel) != 0 {
                continue;
            }

            let matches_of_rel = rel_sdk_map.entry(rel_idx).or_insert(Vec::new());
            match matches_of_rel.get(0) {
                None => matches_of_rel.push(sdk),

                Some(curr_sdk_match) => {
                    let curr_exact_matches = exact_matching_insns(curr_sdk_match, rel);
                    let new_exact_matches = exact_matching_insns(sdk, rel);

                    if new_exact_matches > curr_exact_matches {
                        matches_of_rel.clear();
                        matches_of_rel.push(sdk);

                    } else if new_exact_matches == curr_exact_matches {
                        matches_of_rel.push(sdk);
                    }
                    // Don't count this sdk function if there's less exact matches than what's
                    // already matched with the rel
                }
            }
        }
    }

    let mut rel_sdk_matches: Vec<_> = rel_sdk_map.iter()
        .filter(|(_, sdk_matches)| sdk_matches.len() == 1)
        .map(|(rel_i, sdk_matches)| (&sdk_matches[0], &rel_funcs[*rel_i]))
        .collect();
    rel_sdk_matches.sort_unstable();

    println!("======================== strand_match Summary ==========================\n");

    println!("REL functions matched by a single SDK function:");
    for (sdk, rel) in rel_sdk_matches {
        println!("{} -> {}", sdk.full_name(), rel.name);
    }

    println!("\n\nREL functions matched by more than one SDK function:");
    for (rel_idx, sdk_list) in rel_sdk_map {
        if sdk_list.len() < 2 || sdk_list[0].len <= 5 {
            continue;
        }
        let decent_sdk_matches: Vec<_> = sdk_list.iter()
            .filter(|sdk| {
                exact_matching_insns(&rel_funcs[rel_idx], sdk) > 3
            })
            .map(|sdk| sdk.full_name())
            .collect();
        if decent_sdk_matches.len() > 0 {
            let lst_str = decent_sdk_matches.join(", ");
            println!("[{}] -> {}", lst_str, rel_funcs[rel_idx].name);
        }
    }
}

// Zero the arguments to all `bl` instructions, as these are often modified during linking or
// runtime relocation
fn equalize_bl(funcs: &mut [AsmFunction]) {
    for fun in funcs {
        for inst in &mut fun.code {
            if *inst & BL_OPCODE_MASK == BL_OPCODE {
                *inst = BL_OPCODE; // Zero all other bits
            }
        }
    }
}

// If lengths are equal, return how many instructions differ, not counting relocated instructions.
// If lengths differ, return std::usize::MAX
fn compare_simple(a: &AsmFunction, b: &AsmFunction) -> usize {
    if a.code.len() == b.code.len() {
        a.code.iter()
            .zip(b.code.iter())
            .filter(|&(&x, &y)| x != 0 && y != 0 && x != y)
            .count()
    } else {
        std::usize::MAX
    }
}

// How many instructions match between the functions, where relocated instructions don't count?
fn exact_matching_insns(a: &AsmFunction, b: &AsmFunction) -> usize {
    if a.code.len() == b.code.len() {
        a.code.iter()
            .zip(b.code.iter())
            .filter(|&(&x, &y)| x == y && x != 0 && y != 0)
            .count()
    } else {
        0
    }
}
