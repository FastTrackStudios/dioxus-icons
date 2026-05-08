# Effect of `rsx!` on Build Timing

The generated `rsx!` icon bodies were the dominant source of compile-time cost in
`dioxus-icons`, mostly because they produced a large amount of Rust for the
compiler to expand, resolve, and type-check for every generated icon.

Clean package-check timing for the `dioxus-icons` crate unit:

```text
rsx! generated icons:          23.8s
manual VDOM templates:          4.1s
manual VDOM + const builders:   2.3s
```

That is roughly a 10x reduction from the original generated `rsx!` version to
the current helper-based generated output.

Macro expansion itself was not the entire cost. In the current version, rustc
reports:

```text
total:              2.520s
macro_expand_crate: 0.212s
type_check_crate:   1.657s
generate_metadata:  0.129s
```

The bigger win came from avoiding the large expanded `rsx!` expression tree and
emitting compact release-style VDOM templates directly. That shrank the amount
of generated Rust that rustc has to resolve and type-check, which is why the
crate unit fell from about 23.8s to 2.3s.


## Trace with rsx:

```
time:   0.001; rss:   41MB ->   41MB (   +0MB)  parse_crate
time:   0.001; rss:   42MB ->   42MB (   +0MB)  incr_comp_prepare_session_directory
time:   0.000; rss:   42MB ->   42MB (   +0MB)  incr_comp_garbage_collect_session_directories
time:   0.000; rss:   46MB ->   46MB (   +0MB)  crate_injection
time:  10.357; rss:   46MB -> 1490MB (+1443MB)  expand_crate
time:  10.357; rss:   46MB -> 1490MB (+1443MB)  macro_expand_crate
time:   0.168; rss: 1490MB -> 1490MB (   +0MB)  AST_validation
time:   0.041; rss: 1490MB -> 1572MB (  +82MB)  finalize_imports
time:   0.037; rss: 1572MB -> 1574MB (   +3MB)  compute_effective_visibilities
time:   0.009; rss: 1574MB -> 1574MB (   +0MB)  lint_reexports
time:   0.089; rss: 1574MB -> 1576MB (   +1MB)  finalize_macro_resolutions
time:   1.753; rss: 1576MB -> 1800MB ( +224MB)  late_resolve_crate
time:   0.060; rss: 1800MB -> 1800MB (   +0MB)  resolve_check_unused
time:   0.117; rss: 1800MB -> 1800MB (   +0MB)  resolve_postprocess
time:   2.106; rss: 1490MB -> 1800MB ( +310MB)  resolve_crate
time:   0.061; rss: 1611MB -> 1611MB (   +0MB)  write_dep_info
time:   0.065; rss: 1611MB -> 1612MB (   +0MB)  complete_gated_feature_checking
time:   0.253; rss: 2183MB -> 2183MB (   +0MB)  drop_ast
time:   0.000; rss: 2017MB -> 2017MB (   +0MB)  looking_for_entry_point
time:   0.000; rss: 2017MB -> 2017MB (   +0MB)  check_externally_implementable_items
time:   0.022; rss: 2017MB -> 2013MB (   -3MB)  looking_for_derive_registrar
time:   0.000; rss: 2014MB -> 2014MB (   +0MB)  unused_lib_feature_checking
time:   0.544; rss: 2017MB -> 2037MB (  +20MB)  misc_checking_1
time:   6.608; rss: 2037MB -> 2384MB ( +347MB)  coherence_checking
time:  18.751; rss: 2037MB -> 2682MB ( +645MB)    type_check_crate
time:  17.017; rss: 2682MB -> 2348MB ( -334MB)    MIR_borrow_checking
time:   0.684; rss: 2190MB -> 2211MB (  +21MB)    module_lints
time:   0.684; rss: 2190MB -> 2211MB (  +21MB)    lint_checking
time:   0.489; rss: 2211MB -> 2217MB (   +6MB)  privacy_checking_modules
time:   0.004; rss: 2217MB -> 2220MB (   +3MB)  check_lint_expectations
time:   1.792; rss: 2348MB -> 2220MB ( -128MB)  misc_checking_3
time:   0.011; rss: 3103MB -> 3102MB (   -2MB)  monomorphization_collector_root_collections
time:   1.568; rss: 3102MB -> 3385MB ( +283MB)  monomorphization_collector_graph_walk
time:   0.325; rss: 3385MB -> 3402MB (  +17MB)  partition_and_assert_distinct_symbols
time:   4.139; rss: 2220MB -> 3272MB (+1051MB)  generate_crate_metadata
time:   0.000; rss: 3272MB -> 3272MB (   +0MB)  codegen_crate
time:   0.000; rss: 3272MB -> 3272MB (   +0MB)  assert_dep_graph
time:   0.648; rss: 3272MB -> 3414MB ( +142MB)  encode_query_results
time:   0.744; rss: 3272MB -> 3358MB (  +86MB)  incr_comp_serialize_result_cache
time:   0.744; rss: 3272MB -> 3358MB (  +86MB)  incr_comp_persist_result_cache
time:   0.744; rss: 3272MB -> 3358MB (  +86MB)  serialize_dep_graph
time:   0.000; rss: 1217MB -> 1217MB (   +0MB)  finish_ongoing_codegen
time:   0.000; rss: 1217MB -> 1217MB (   +0MB)  serialize_work_products
time:   0.237; rss: 1217MB -> 1370MB ( +153MB)  link_rlib
time:   0.240; rss: 1217MB -> 1370MB ( +153MB)  link_binary
time:   0.241; rss: 1217MB -> 1218MB (   +1MB)  link_crate
time:   0.241; rss: 1217MB -> 1218MB (   +1MB)  link
time:  59.437; rss:   25MB ->   87MB (  +62MB)  total
```

## Trace without rsx:

```text
time:   0.000; rss:   41MB ->   41MB (   +0MB)  parse_crate
time:   0.000; rss:   42MB ->   42MB (   +0MB)  incr_comp_prepare_session_directory
time:   0.000; rss:   42MB ->   42MB (   +0MB)  incr_comp_garbage_collect_session_directories
time:   0.000; rss:   46MB ->   47MB (   +0MB)  crate_injection
time:   0.225; rss:   47MB ->  136MB (  +89MB)  expand_crate
time:   0.225; rss:   47MB ->  136MB (  +89MB)  macro_expand_crate
time:   0.006; rss:  136MB ->  136MB (   +0MB)  AST_validation
time:   0.017; rss:  136MB ->  141MB (   +5MB)  finalize_imports
time:   0.012; rss:  141MB ->  142MB (   +1MB)  compute_effective_visibilities
time:   0.001; rss:  142MB ->  142MB (   +0MB)  lint_reexports
time:   0.001; rss:  142MB ->  143MB (   +1MB)  finalize_macro_resolutions
time:   0.026; rss:  143MB ->  159MB (  +17MB)  late_resolve_crate
time:   0.007; rss:  159MB ->  159MB (   +0MB)  resolve_check_unused
time:   0.002; rss:  159MB ->  160MB (   +0MB)  resolve_postprocess
time:   0.065; rss:  136MB ->  160MB (  +24MB)  resolve_crate
time:   0.002; rss:  160MB ->  160MB (   +0MB)  write_dep_info
time:   0.001; rss:  160MB ->  160MB (   +0MB)  complete_gated_feature_checking
time:   0.012; rss:  173MB ->  173MB (   +0MB)  drop_ast
time:   0.000; rss:  173MB ->  173MB (   +0MB)  looking_for_entry_point
time:   0.000; rss:  173MB ->  173MB (   +0MB)  check_externally_implementable_items
time:   0.004; rss:  173MB ->  173MB (   +0MB)  looking_for_derive_registrar
time:   0.000; rss:  173MB ->  173MB (   +0MB)  unused_lib_feature_checking
time:   0.031; rss:  173MB ->  173MB (   +0MB)  misc_checking_1
time:   0.116; rss:  173MB ->  191MB (  +18MB)  coherence_checking
time:   1.634; rss:  173MB ->  281MB ( +108MB)  type_check_crate
time:   0.129; rss:  281MB ->  294MB (  +14MB)  MIR_borrow_checking
time:   0.055; rss:  298MB ->  299MB (   +0MB)  module_lints
time:   0.055; rss:  298MB ->  299MB (   +0MB)  lint_checking
time:   0.020; rss:  299MB ->  299MB (   +0MB)  privacy_checking_modules
time:   0.106; rss:  294MB ->  299MB (   +5MB)  misc_checking_3
time:   0.002; rss:  321MB ->  322MB (   +1MB)  monomorphization_collector_root_collections
time:   0.034; rss:  322MB ->  331MB (   +9MB)  monomorphization_collector_graph_walk
time:   0.031; rss:  331MB ->  333MB (   +2MB)  partition_and_assert_distinct_symbols
time:   0.129; rss:  299MB ->  327MB (  +29MB)  generate_crate_metadata
time:   0.000; rss:  327MB ->  328MB (   +0MB)  codegen_crate
time:   0.000; rss:  328MB ->  328MB (   +0MB)  assert_dep_graph
time:   0.027; rss:  328MB ->  335MB (   +8MB)  encode_query_results
time:   0.033; rss:  328MB ->  331MB (   +3MB)  incr_comp_serialize_result_cache
time:   0.033; rss:  328MB ->  331MB (   +3MB)  incr_comp_persist_result_cache
time:   0.034; rss:  328MB ->  331MB (   +3MB)  serialize_dep_graph
time:   0.000; rss:  272MB ->  272MB (   +0MB)  finish_ongoing_codegen
time:   0.000; rss:  272MB ->  272MB (   +0MB)  serialize_work_products
time:   0.008; rss:  272MB ->  280MB (   +8MB)  link_rlib
time:   0.008; rss:  272MB ->  280MB (   +8MB)  link_binary
time:   0.008; rss:  272MB ->  273MB (   +0MB)  link_crate
time:   0.009; rss:  272MB ->  273MB (   +0MB)  link
time:   2.499; rss:   25MB ->   78MB (  +53MB)  total
```