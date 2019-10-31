use crate::function::AsmFunction;

const BL_OPCODE: u32 = 0b01001000_00000000_00000000_00000001;
const BL_OPCODE_MASK: u32 = 0b11111100_00000000_00000000_00000001;

pub fn match_funcs(mut sdk_funcs: Vec<AsmFunction>, mut rel_funcs: Vec<AsmFunction>) {
//    let target_idx = rel_funcs
//        .iter()
//        .enumerate()
//        .find(|(i, f)| &f.name.to_lowercase() == "fun_80251f38")
//        .unwrap()
//        .0;
//
//    debug_export_function(&rel_funcs[target_idx]);
//
    equalize_bl(&mut sdk_funcs);
    equalize_bl(&mut rel_funcs);

    match_levenshtein(&sdk_funcs, &rel_funcs);
}

/*
naive O(n^2) approach:
TODO try just testing on one known SDK function

non-naive-O(n^2) approach (todo):
let's just try this first to see what happens

using levenshtein for everything for now, including same-length
for function in sdk funcs:
    for each rel func of same length:
        find all funcs that meet normalized levenshtein similarity threshold
    if similarity greater than threshold (say 0.9), AND it's unique, declare it a match
        mark with Some(&func) in array
    if not:
        todo
*/
fn match_levenshtein(sdk_funcs: &[AsmFunction], rel_funcs: &[AsmFunction]) {
//    let mut rel_matches = vec![None; rel_funcs.len()];
    let mut matches_in_rel = Vec::new();

    let mut comparisons = 0u64;

//    let filtered_sdk_funcs: Vec<_> = sdk_funcs.iter()
//        .filter(|f| &f.namespace == "gx")
//        .collect();
    let filtered_sdk_funcs= sdk_funcs;

    for (j, sdk_func) in filtered_sdk_funcs.iter().enumerate() {
        let mut min_diff = std::usize::MAX;
        let mut closest_rel_func = &rel_funcs[0];
            for (i, rel_func) in rel_funcs.iter().enumerate() {
            let leven = strsim::generic_levenshtein(&rel_func.code, &sdk_func.code);
            if leven < min_diff {
                min_diff = leven;
                closest_rel_func = rel_func;
            }

            comparisons += 1;
            if comparisons % 10_000 == 0 {
                let percent = comparisons as f64 / ((filtered_sdk_funcs.len()) * rel_funcs.len())
                    as f64 * 100.0;
                println!("{} comparisons ({:.1}%)", comparisons, percent);
            }
        }
        matches_in_rel.push((closest_rel_func, min_diff));
    }

    for (i, mtch) in matches_in_rel.iter().enumerate() {
        println!("============ SCORE: {} ============", mtch.1);
        println!("------------ SDK FUNC -------------");
        println!("{}", filtered_sdk_funcs[i]);
        println!("------------ MATCHED REL FUNC -------------");
        println!("{}\n\n\n", mtch.0);
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
