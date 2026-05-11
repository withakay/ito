# Iteration 37: Validation Repair Verification

*2026-05-11T18:00:05Z by Showboat 0.6.1*
<!-- showboat-id: 8b541b79-6ae7-492b-9cb0-b0fa5c53c580 -->

Repaired three review findings: generated enhanced task templates now include Files/Verify/Done When for checkpoints, scenario grammar accepts ordered-list GIVEN/WHEN/THEN steps, and contract-ref parsing exposes short ref-like unknown schemes such as rpc:Call so validation can reject them.

```bash
cargo test -p ito-core --test validate_delta_rules scenario_grammar_rule_accepts_ordered_list_steps && cargo test -p ito-core --test validate_delta_rules contract_refs_rule_rejects_short_unknown_scheme_after_known_ref && cargo test -p ito-core --test show parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier && cargo test -p ito-domain --test tasks enhanced_template_parses_and_has_checkpoint_warning
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test scenario_grammar_rule_accepts_ordered_list_steps ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test contract_refs_rule_rejects_short_unknown_scheme_after_known_ref ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/show.rs (target/debug/deps/show-2dc99ecb9e0315f2)

running 1 test
test parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/tasks.rs (target/debug/deps/tasks-5cb039ab908fe47d)

running 1 test
test enhanced_template_parses_and_has_checkpoint_warning ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s

```

```bash
ito validate 001-33_enhance-spec-driven-workflow-validation --strict && ito validate repo
```

```output
Change '001-33_enhance-spec-driven-workflow-validation' is valid
Repository validation passed.
```

```bash
make check
```

```output
check for added large files..............................................Passed
check for merge conflicts................................................Passed
check toml...............................................................Passed
check yaml...............................................................Passed
check json...............................................................Passed
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
pretty format json.......................................................Passed
yamllint.................................................................Passed
markdownlint-cli2........................................................Passed
cargo fmt (ito-rs).......................................................Passed
forbid local version metadata in Cargo.toml..............................Passed
cargo clippy (ito-rs)....................................................Passed
cargo doc warnings as errors (ito-rs)....................................Passed
cargo test with coverage (ito-rs)........................................Failed
- hook id: cargo-test-coverage
- files were modified by this hook

  Coverage enforcement: hard min=80%, target=90%
    Below 80%: build FAILS (hard floor)
    Below 90%: WARNING (target)
    Excluded crates: ito-web (no tests yet)

  Filename                                              Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
  -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  ito-backend/src/api.rs                                    729               164    77.50%          66                15    77.27%         536               105    80.41%           0                 0         -
  ito-backend/src/auth.rs                                   333                 7    97.90%          24                 0   100.00%         172                 3    98.26%           0                 0         -
  ito-backend/src/error.rs                                  165                34    79.39%          20                 2    90.00%         152                21    86.18%           0                 0         -
  ito-backend/src/server.rs                                 143                55    61.54%          10                 5    50.00%          92                39    57.61%           0                 0         -
  ito-backend/src/state.rs                                  132                14    89.39%           8                 1    87.50%          81                 8    90.12%           0                 0         -
  ito-cli/src/app/archive.rs                                661               146    77.91%          23                 4    82.61%         429                84    80.42%           0                 0         -
  ito-cli/src/app/common.rs                                 205                16    92.20%          11                 0   100.00%         131                 8    93.89%           0                 0         -
  ito-cli/src/app/entrypoint.rs                              21                 0   100.00%           1                 0   100.00%          16                 0   100.00%           0                 0         -
  ito-cli/src/app/grep.rs                                   210                95    54.76%           4                 0   100.00%         116                48    58.62%           0                 0         -
  ito-cli/src/app/init.rs                                   615               191    68.94%          36                14    61.11%         403               140    65.26%           0                 0         -
  ito-cli/src/app/instructions.rs                          1377               396    71.24%          77                31    59.74%         811               194    76.08%           0                 0         -
  ito-cli/src/app/list.rs                                   569                88    84.53%          29                 7    75.86%         350                62    82.29%           0                 0         -
  ito-cli/src/app/memory_instructions.rs                    293                72    75.43%          17                 7    58.82%         203                61    69.95%           0                 0         -
  ito-cli/src/app/run.rs                                    384                48    87.50%          33                 4    87.88%         269                38    85.87%           0                 0         -
  ito-cli/src/app/show.rs                                   541               158    70.79%          27                 8    70.37%         304                92    69.74%           0                 0         -
  ito-cli/src/app/status.rs                                 171                45    73.68%           5                 1    80.00%          92                39    57.61%           0                 0         -
  ito-cli/src/app/trace.rs                                  109                22    79.82%           1                 0   100.00%          80                17    78.75%           0                 0         -
  ito-cli/src/app/update.rs                                 180                51    71.67%          10                 3    70.00%         117                32    72.65%           0                 0         -
  ito-cli/src/app/validate.rs                               735               111    84.90%          25                 3    88.00%         474                85    82.07%           0                 0         -
  ito-cli/src/app/worktree_wizard.rs                        203                73    64.04%          15                 5    66.67%         179                79    55.87%           0                 0         -
  ito-cli/src/cli.rs                                         17                 0   100.00%           1                 0   100.00%           7                 0   100.00%           0                 0         -
  ito-cli/src/cli/agent.rs                                  112                16    85.71%           1                 0   100.00%          55                 4    92.73%           0                 0         -
  ito-cli/src/cli/ralph.rs                                    8                 0   100.00%           1                 0   100.00%           8                 0   100.00%           0                 0         -
  ito-cli/src/cli_error.rs                                   25                 0   100.00%           7                 0   100.00%          27                 0   100.00%           0                 0         -
  ito-cli/src/commands/audit.rs                             369                65    82.38%           9                 5    44.44%         219                45    79.45%           0                 0         -
  ito-cli/src/commands/backend.rs                           376                64    82.98%          26                 5    80.77%         286                46    83.92%           0                 0         -
  ito-cli/src/commands/completions.rs                        15                 0   100.00%           1                 0   100.00%          10                 0   100.00%           0                 0         -
  ito-cli/src/commands/config.rs                            407                55    86.49%          16                 4    75.00%         168                22    86.90%           0                 0         -
  ito-cli/src/commands/create.rs                            777               155    80.05%          25                 3    88.00%         434                75    82.72%           0                 0         -
  ito-cli/src/commands/help.rs                              181                29    83.98%          11                 1    90.91%         117                14    88.03%           0                 0         -
  ito-cli/src/commands/path.rs                              194                16    91.75%          10                 0   100.00%          95                 6    93.68%           0                 0         -
  ito-cli/src/commands/plan.rs                               89                 8    91.01%           4                 1    75.00%          59                 7    88.14%           0                 0         -
  ito-cli/src/commands/ralph.rs                             744               122    83.60%          21                 7    66.67%         463                63    86.39%           0                 0         -
  ito-cli/src/commands/ralph/support.rs                    1015               303    70.15%          63                26    58.73%         710               171    75.92%           0                 0         -
  ito-cli/src/commands/serve.rs                              93                60    35.48%          13                 9    30.77%          65                41    36.92%           0                 0         -
  ito-cli/src/commands/serve_api.rs                         282                67    76.24%          22                 7    68.18%         176                33    81.25%           0                 0         -
  ito-cli/src/commands/stats.rs                              22                 3    86.36%           1                 0   100.00%          18                 2    88.89%           0                 0         -
  ito-cli/src/commands/sync.rs                               85                47    44.71%           4                 1    75.00%          57                36    36.84%           0                 0         -
  ito-cli/src/commands/tasks.rs                            1418               392    72.36%          38                 9    76.32%         882               265    69.95%           0                 0         -
  ito-cli/src/commands/tasks/backend.rs                     286               254    11.19%          12                10    16.67%         192               175     8.85%           0                 0         -
  ito-cli/src/commands/tasks/support.rs                     122                37    69.67%          10                 1    90.00%          87                22    74.71%           0                 0         -
  ito-cli/src/commands/templates.rs                          34                 4    88.24%           1                 0   100.00%          22                 2    90.91%           0                 0         -
  ito-cli/src/commands/util.rs                               79                79     0.00%           3                 3     0.00%          40                40     0.00%           0                 0         -
  ito-cli/src/commands/view.rs                              140                47    66.43%           7                 3    57.14%          75                27    64.00%           0                 0         -
  ito-cli/src/commands/worktree.rs                          141               141     0.00%          12                12     0.00%          73                73     0.00%           0                 0         -
  ito-cli/src/diagnostics.rs                                171                 2    98.83%          11                 0   100.00%         121                 2    98.35%           0                 0         -
  ito-cli/src/main.rs                                         3                 0   100.00%           1                 0   100.00%           3                 0   100.00%           0                 0         -
  ito-cli/src/runtime.rs                                    128                 9    92.97%          20                 3    85.00%          91                 6    93.41%           0                 0         -
  ito-cli/src/util.rs                                       589               143    75.72%          29                 6    79.31%         359               107    70.19%           0                 0         -
  ito-common/src/fs.rs                                       51                19    62.75%           9                 4    55.56%          32                12    62.50%           0                 0         -
  ito-common/src/git_url.rs                                 199                 4    97.99%          20                 0   100.00%          98                 2    97.96%           0                 0         -
  ito-common/src/id/change_id.rs                            360                44    87.78%          22                 5    77.27%         242                68    71.90%           0                 0         -
  ito-common/src/id/error.rs                                  8                 0   100.00%           2                 0   100.00%           5                 0   100.00%           0                 0         -
  ito-common/src/id/mod.rs                                  163                 1    99.39%          12                 0   100.00%          97                 1    98.97%           0                 0         -
  ito-common/src/id/module_id.rs                            127                17    86.61%           8                 1    87.50%         102                35    65.69%           0                 0         -
  ito-common/src/id/spec_id.rs                               51                10    80.39%           5                 1    80.00%          38                11    71.05%           0                 0         -
  ito-common/src/id/sub_module_id.rs                        230                16    93.04%          18                 2    88.89%         165                28    83.03%           0                 0         -
  ito-common/src/io.rs                                       95                28    70.53%          14                 5    64.29%          52                14    73.08%           0                 0         -
  ito-common/src/match_.rs                                  129                 1    99.22%           5                 0   100.00%          53                 1    98.11%           0                 0         -
  ito-common/src/paths.rs                                    90                 0   100.00%          12                 0   100.00%          58                 0   100.00%           0                 0         -
  ito-config/src/config/backend_types.rs                     49                 2    95.92%           8                 0   100.00%          47                 2    95.74%           0                 0         -
  ito-config/src/config/defaults.rs                           9                 0   100.00%           2                 0   100.00%           6                 0   100.00%           0                 0         -
  ito-config/src/config/mod.rs                             1001                62    93.81%          65                 7    89.23%         524                42    91.98%           0                 0         -
  ito-config/src/config/schema.rs                            62                12    80.65%           8                 4    50.00%          30                 8    73.33%           0                 0         -
  ito-config/src/config/types.rs                            271                54    80.07%          56                 8    85.71%         299                41    86.29%           0                 0         -
  ito-config/src/config/worktree_init_types.rs               23                 0   100.00%           3                 0   100.00%          12                 0   100.00%           0                 0         -
  ito-config/src/context.rs                                  94                 8    91.49%           5                 1    80.00%          48                 4    91.67%           0                 0         -
  ito-config/src/ito_dir/mod.rs                             246                20    91.87%          15                 1    93.33%         142                13    90.85%           0                 0         -
  ito-config/src/output/mod.rs                               88                 1    98.86%           8                 0   100.00%          71                 1    98.59%           0                 0         -
  ito-core/src/archive.rs                                   318                60    81.13%          20                 9    55.00%         184                28    84.78%           0                 0         -
  ito-core/src/audit/mirror.rs                              824               239    71.00%          55                19    65.45%         628               177    71.82%           0                 0         -
  ito-core/src/audit/reader.rs                               57                 0   100.00%           5                 0   100.00%          39                 0   100.00%           0                 0         -
  ito-core/src/audit/reconcile.rs                           419                13    96.90%          16                 0   100.00%         232                11    95.26%           0                 0         -
  ito-core/src/audit/store.rs                               596                68    88.59%          39                 4    89.74%         351                41    88.32%           0                 0         -
  ito-core/src/audit/stream.rs                              347                35    89.91%          12                 1    91.67%         193                23    88.08%           0                 0         -
  ito-core/src/audit/validate.rs                            325                 4    98.77%          14                 1    92.86%         258                 1    99.61%           0                 0         -
  ito-core/src/audit/worktree.rs                            333                27    91.89%          18                 0   100.00%         258                22    91.47%           0                 0         -
  ito-core/src/audit/writer.rs                              334                18    94.61%          20                 2    90.00%         171                10    94.15%           0                 0         -
  ito-core/src/backend_auth.rs                              217                60    72.35%          23                 9    60.87%         132                25    81.06%           0                 0         -
  ito-core/src/backend_change_repository.rs                 626                63    89.94%          37                 3    91.89%         355                45    87.32%           0                 0         -
  ito-core/src/backend_client.rs                            432                 5    98.84%          36                 2    94.44%         334                 3    99.10%           0                 0         -
  ito-core/src/backend_coordination.rs                      383                28    92.69%          33                 5    84.85%         286                28    90.21%           0                 0         -
  ito-core/src/backend_health.rs                            183                93    49.18%           7                 3    57.14%         169                93    44.97%           0                 0         -
  ito-core/src/backend_http.rs                              893               314    64.84%          55                20    63.64%         615               201    67.32%           0                 0         -
  ito-core/src/backend_import.rs                            274                28    89.78%          18                 3    83.33%         230                12    94.78%           0                 0         -
  ito-core/src/backend_module_repository.rs                 105                13    87.62%           9                 0   100.00%          62                 9    85.48%           0                 0         -
  ito-core/src/backend_spec_repository.rs                    12                 0   100.00%           3                 0   100.00%           9                 0   100.00%           0                 0         -
  ito-core/src/backend_sync.rs                              848               107    87.38%          54                22    59.26%         421                32    92.40%           0                 0         -
  ito-core/src/backend_task_repository.rs                    91                 1    98.90%          10                 0   100.00%          51                 0   100.00%           0                 0         -
  ito-core/src/change_meta.rs                                45                 4    91.11%           5                 1    80.00%          37                 3    91.89%           0                 0         -
  ito-core/src/change_repository.rs                        1278               145    88.65%          84                13    84.52%         722                87    87.95%           0                 0         -
  ito-core/src/config.rs                                   1183                96    91.89%          64                 5    92.19%         748                94    87.43%           0                 0         -
  ito-core/src/coordination.rs                              742               435    41.37%          53                37    30.19%         506               321    36.56%           0                 0         -
  ito-core/src/coordination_worktree.rs                    1072               277    74.16%          69                28    59.42%         791               208    73.70%           0                 0         -
  ito-core/src/create/mod.rs                               1310               198    84.89%          78                15    80.77%         806               142    82.38%           0                 0         -
  ito-core/src/distribution.rs                              566                55    90.28%          39                 8    79.49%         366                55    84.97%           0                 0         -
  ito-core/src/error_bridge.rs                                4                 0   100.00%           1                 0   100.00%           3                 0   100.00%           0                 0         -
  ito-core/src/errors.rs                                     75                 7    90.67%           8                 0   100.00%          59                 7    88.14%           0                 0         -
  ito-core/src/event_forwarder.rs                           650                48    92.62%          42                 3    92.86%         399                28    92.98%           0                 0         -
  ito-core/src/front_matter.rs                              530                 8    98.49%          42                 2    95.24%         312                 7    97.76%           0                 0         -
  ito-core/src/fs_project_store.rs                          408                40    90.20%          35                10    71.43%         229                17    92.58%           0                 0         -
  ito-core/src/git.rs                                       767               347    54.76%          44                16    63.64%         636               308    51.57%           0                 0         -
  ito-core/src/git_remote.rs                                255                28    89.02%          23                 3    86.96%         209                21    89.95%           0                 0         -
  ito-core/src/grep.rs                                      341                21    93.84%          18                 2    88.89%         190                14    92.63%           0                 0         -
  ito-core/src/harness/claude_code.rs                       111                 0   100.00%           9                 0   100.00%          64                 0   100.00%           0                 0         -
  ito-core/src/harness/codex.rs                              94                 0   100.00%           8                 0   100.00%          52                 0   100.00%           0                 0         -
  ito-core/src/harness/github_copilot.rs                     96                 0   100.00%           8                 0   100.00%          53                 0   100.00%           0                 0         -
  ito-core/src/harness/opencode.rs                           86                 1    98.84%           8                 0   100.00%          49                 1    97.96%           0                 0         -
  ito-core/src/harness/streaming_cli.rs                     253                43    83.00%          13                 2    84.62%         170                33    80.59%           0                 0         -
  ito-core/src/harness/stub.rs                              162                 2    98.77%          17                 1    94.12%         114                 2    98.25%           0                 0         -
  ito-core/src/harness/types.rs                             171                 0   100.00%          16                 0   100.00%         113                 0   100.00%           0                 0         -
  ito-core/src/harness_context.rs                           121                11    90.91%           6                 0   100.00%          85                 7    91.76%           0                 0         -
  ito-core/src/installers/markers.rs                        217                11    94.93%          11                 0   100.00%         155                 9    94.19%           0                 0         -
  ito-core/src/installers/mod.rs                           1638               295    81.99%          85                30    64.71%         912               145    84.10%           0                 0         -
  ito-core/src/list.rs                                      703               124    82.36%          38                 8    78.95%         449                76    83.07%           0                 0         -
  ito-core/src/memory/mod.rs                                 69                 0   100.00%           7                 0   100.00%          60                 0   100.00%           0                 0         -
  ito-core/src/memory/rendering.rs                          217                 7    96.77%          16                 1    93.75%         125                 5    96.00%           0                 0         -
  ito-core/src/module_repository.rs                         884                94    89.37%          59                12    79.66%         483                53    89.03%           0                 0         -
  ito-core/src/orchestrate/gates.rs                         152                 0   100.00%           7                 0   100.00%          79                 0   100.00%           0                 0         -
  ito-core/src/orchestrate/plan.rs                          280               102    63.57%          15                 6    60.00%         164                58    64.63%           0                 0         -
  ito-core/src/orchestrate/preset.rs                         42                27    35.71%           4                 3    25.00%          25                16    36.00%           0                 0         -
  ito-core/src/orchestrate/state.rs                         231                65    71.86%          26                14    46.15%         150                34    77.33%           0                 0         -
  ito-core/src/orchestrate/types.rs                          69                26    62.32%           7                 3    57.14%          65                29    55.38%           0                 0         -
  ito-core/src/orchestrate/user_prompt.rs                   142                25    82.39%           9                 2    77.78%          93                16    82.80%           0                 0         -
  ito-core/src/planning_init.rs                              51                 8    84.31%           3                 0   100.00%          28                 3    89.29%           0                 0         -
  ito-core/src/process.rs                                   462                60    87.01%          33                 6    81.82%         335                60    82.09%           0                 0         -
  ito-core/src/ralph/duration.rs                            238                23    90.34%          15                 4    73.33%         123                19    84.55%           0                 0         -
  ito-core/src/ralph/prompt.rs                              256                29    88.67%          16                 2    87.50%         189                15    92.06%           0                 0         -
  ito-core/src/ralph/runner.rs                             1444               299    79.29%          37                 9    75.68%        1005               230    77.11%           0                 0         -
  ito-core/src/ralph/state.rs                               451                56    87.58%          31                10    67.74%         232                14    93.97%           0                 0         -
  ito-core/src/ralph/task_sources.rs                        165                52    68.48%          12                 6    50.00%         118                39    66.95%           0                 0         -
  ito-core/src/ralph/validation.rs                          737                74    89.96%          37                 3    91.89%         400                44    89.00%           0                 0         -
  ito-core/src/remote_task_repository.rs                      8                 0   100.00%           2                 0   100.00%           6                 0   100.00%           0                 0         -
  ito-core/src/repo_index.rs                                 31                 4    87.10%           1                 0   100.00%          16                 0   100.00%           0                 0         -
  ito-core/src/repo_paths.rs                                360                71    80.28%          33                10    69.70%         215                37    82.79%           0                 0         -
  ito-core/src/repository_runtime.rs                        415                73    82.41%          55                12    78.18%         323                65    79.88%           0                 0         -
  ito-core/src/show/mod.rs                                  827                59    92.87%          43                 4    90.70%         512                34    93.36%           0                 0         -
  ito-core/src/spec_repository.rs                            75                 5    93.33%           6                 0   100.00%          46                 3    93.48%           0                 0         -
  ito-core/src/sqlite_project_store.rs                      431               125    71.00%          47                29    38.30%         318                37    88.36%           0                 0         -
  ito-core/src/sqlite_project_store_backend.rs              254                61    75.98%          29                10    65.52%         221                45    79.64%           0                 0         -
  ito-core/src/sqlite_project_store_mutations.rs            225                85    62.22%          27                12    55.56%         180                76    57.78%           0                 0         -
  ito-core/src/sqlite_project_store_repositories.rs        1238               212    82.88%          67                19    71.64%         738               126    82.93%           0                 0         -
  ito-core/src/stats.rs                                      93                 8    91.40%           5                 0   100.00%          97                 7    92.78%           0                 0         -
  ito-core/src/task_mutations.rs                            181                52    71.27%          13                 1    92.31%         134                19    85.82%           0                 0         -
  ito-core/src/task_repository.rs                           281                 9    96.80%          16                 2    87.50%         165                 4    97.58%           0                 0         -
  ito-core/src/tasks.rs                                    1078               187    82.65%          81                25    69.14%         735               122    83.40%           0                 0         -
  ito-core/src/templates/guidance.rs                        134                 4    97.01%           7                 0   100.00%          94                 1    98.94%           0                 0         -
  ito-core/src/templates/mod.rs                             969               161    83.38%          62                12    80.65%         624               110    82.37%           0                 0         -
  ito-core/src/templates/review.rs                          278                75    73.02%          11                 1    90.91%         194                56    71.13%           0                 0         -
  ito-core/src/templates/schema_assets.rs                   244                47    80.74%          19                 5    73.68%         164                38    76.83%           0                 0         -
  ito-core/src/templates/task_parsing.rs                    218                25    88.53%           9                 2    77.78%         144                18    87.50%           0                 0         -
  ito-core/src/templates/types.rs                           127                 1    99.21%           8                 0   100.00%          93                 1    98.92%           0                 0         -
  ito-core/src/time.rs                                       10                 5    50.00%           2                 1    50.00%           6                 3    50.00%           0                 0         -
  ito-core/src/token.rs                                      56                 0   100.00%           7                 0   100.00%          31                 0   100.00%           0                 0         -
  ito-core/src/trace.rs                                      88                 5    94.32%           1                 0   100.00%          55                 4    92.73%           0                 0         -
  ito-core/src/validate/delta_rules.rs                      830                49    94.10%          37                 0   100.00%         562                36    93.59%           0                 0         -
  ito-core/src/validate/issue.rs                            204                 2    99.02%          20                 0   100.00%         132                 2    98.48%           0                 0         -
  ito-core/src/validate/mod.rs                             1030               232    77.48%          31                 1    96.77%         764               181    76.31%           0                 0         -
  ito-core/src/validate/repo_integrity.rs                   190                25    86.84%          10                 3    70.00%          95                10    89.47%           0                 0         -
  ito-core/src/validate/report.rs                            97                 0   100.00%           9                 0   100.00%          60                 0   100.00%           0                 0         -
  ito-core/src/validate/rules_engine.rs                     136                27    80.15%          10                 0   100.00%         132                18    86.36%           0                 0         -
  ito-core/src/validate/tracking_rules.rs                   185                 5    97.30%           9                 0   100.00%         163                 4    97.55%           0                 0         -
  ito-core/src/viewer/bat.rs                                 16                13    18.75%           4                 3    25.00%          12                 9    25.00%           0                 0         -
  ito-core/src/viewer/collector.rs                          187                21    88.77%          10                 3    70.00%          96                 8    91.67%           0                 0         -
  ito-core/src/viewer/glow.rs                                16                13    18.75%           4                 3    25.00%          12                 9    25.00%           0                 0         -
  ito-core/src/viewer/html.rs                               138                44    68.12%          14                 4    71.43%         111                37    66.67%           0                 0         -
  ito-core/src/viewer/mod.rs                                139                 9    93.53%          16                 3    81.25%          75                 9    88.00%           0                 0         -
  ito-core/src/viewer/registry.rs                            48                 0   100.00%           6                 0   100.00%          41                 0   100.00%           0                 0         -
  ito-core/src/viewer/tmux_nvim.rs                           55                52     5.45%           7                 6    14.29%          45                42     6.67%           0                 0         -
  ito-core/src/viewer/util.rs                                47                15    68.09%           7                 3    57.14%          33                10    69.70%           0                 0         -
  ito-core/src/worktree_ensure.rs                           266                82    69.17%          14                 6    57.14%         202                55    72.77%           0                 0         -
  ito-core/src/worktree_init.rs                             259                85    67.18%          17                 9    47.06%         177                67    62.15%           0                 0         -
  ito-domain/src/audit/context.rs                           207                13    93.72%          15                 1    93.33%         113                 4    96.46%           0                 0         -
  ito-domain/src/audit/event.rs                             425                12    97.18%          32                 2    93.75%         299                 6    97.99%           0                 0         -
  ito-domain/src/audit/materialize.rs                       290                 1    99.66%          10                 0   100.00%         206                 0   100.00%           0                 0         -
  ito-domain/src/audit/reconcile.rs                         492                33    93.29%          17                 0   100.00%         249                16    93.57%           0                 0         -
  ito-domain/src/audit/writer.rs                             60                 0   100.00%           8                 0   100.00%          44                 0   100.00%           0                 0         -
  ito-domain/src/backend.rs                                 178                 0   100.00%          11                 0   100.00%         116                 0   100.00%           0                 0         -
  ito-domain/src/changes/mod.rs                             381                55    85.56%          26                 4    84.62%         264                42    84.09%           0                 0         -
  ito-domain/src/changes/repository.rs                       63                 4    93.65%          11                 0   100.00%          42                 3    92.86%           0                 0         -
  ito-domain/src/discovery.rs                               193                13    93.26%          15                 2    86.67%          91                 6    93.41%           0                 0         -
  ito-domain/src/errors.rs                                   72                 6    91.67%           6                 0   100.00%          49                 3    93.88%           0                 0         -
  ito-domain/src/modules/mod.rs                              93                 0   100.00%           6                 0   100.00%          83                 0   100.00%           0                 0         -
  ito-domain/src/modules/repository.rs                        6                 6     0.00%           2                 2     0.00%           6                 6     0.00%           0                 0         -
  ito-domain/src/planning.rs                                 99                 3    96.97%           9                 0   100.00%          68                 0   100.00%           0                 0         -
  ito-domain/src/schemas/workflow.rs                        142                14    90.14%           3                 0   100.00%         102                 9    91.18%           0                 0         -
  ito-domain/src/schemas/workflow_plan.rs                    67                 7    89.55%           3                 0   100.00%          63                 6    90.48%           0                 0         -
  ito-domain/src/schemas/workflow_state.rs                   79                 5    93.67%           3                 0   100.00%          65                 0   100.00%           0                 0         -
  ito-domain/src/tasks/checkbox.rs                           68                 0   100.00%           3                 0   100.00%          40                 0   100.00%           0                 0         -
  ito-domain/src/tasks/compute.rs                           520                18    96.54%          33                 0   100.00%         329                12    96.35%           0                 0         -
  ito-domain/src/tasks/cycle.rs                              61                 8    86.89%           3                 0   100.00%          42                 0   100.00%           0                 0         -
  ito-domain/src/tasks/mutations.rs                          13                 4    69.23%           4                 1    75.00%          15                 6    60.00%           0                 0         -
  ito-domain/src/tasks/parse.rs                             941                44    95.32%          50                 2    96.00%         660                43    93.48%           0                 0         -
  ito-domain/src/tasks/relational.rs                        282                70    75.18%           2                 0   100.00%         258                73    71.71%           0                 0         -
  ito-domain/src/tasks/repository.rs                         32                11    65.62%           4                 1    75.00%          16                 4    75.00%           0                 0         -
  ito-domain/src/tasks/update.rs                            238                 6    97.48%           3                 0   100.00%         140                 6    95.71%           0                 0         -
  ito-domain/src/traceability.rs                            136                 0   100.00%           1                 0   100.00%         107                 0   100.00%           0                 0         -
  ito-logging/src/lib.rs                                    431                43    90.02%          22                 2    90.91%         293                30    89.76%           0                 0         -
  ito-templates/src/agents.rs                               251                 9    96.41%          17                 1    94.12%         224                 7    96.88%           0                 0         -
  ito-templates/src/instructions.rs                         103                 6    94.17%          10                 0   100.00%          59                 1    98.31%           0                 0         -
  ito-templates/src/lib.rs                                 1266                52    95.89%          95                 1    98.95%         657                34    94.82%           0                 0         -
  ito-templates/src/project_templates.rs                    295                 0   100.00%          18                 0   100.00%         189                 0   100.00%           0                 0         -
  ito-test-support/src/lib.rs                               343               102    70.26%          20                 4    80.00%         200                71    64.50%           0                 0         -
  ito-test-support/src/mock_repos.rs                        304                59    80.59%          35                 6    82.86%         288                57    80.21%           0                 0         -
  ito-test-support/src/pty/mod.rs                           137                 1    99.27%           5                 0   100.00%          81                 1    98.77%           0                 0         -
  -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  TOTAL                                                   65161             11075    83.00%        4039               851    78.93%       42209              7452    82.34%           0                 0         -
  info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
     Compiling ito-common v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-common)
     Compiling ito-templates v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-templates)
     Compiling ito-cli v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-cli)
     Compiling ito-logging v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-logging)
     Compiling ito-domain v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-domain)
     Compiling ito-config v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-config)
     Compiling ito-test-support v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-test-support)
     Compiling ito-core v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-core)
     Compiling ito-backend v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-backend)
     Compiling ito-web v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-web)
      Finished `test` profile [optimized + debuginfo] target(s) in 28.86s
       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_backend-428768240a04e47d)

  running 27 tests
  test auth::tests::exempt_paths_are_health_and_ready ... ok
  test auth::tests::extract_org_repo_valid_path ... ok
  test auth::tests::extract_org_repo_no_trailing ... ok
  test error::tests::bad_request_response_has_400_status ... ok
  test auth::tests::extract_org_repo_non_project_path ... ok
  test auth::tests::token_scope_serializes_admin ... ok
  test auth::tests::validate_token_admin_matches ... ok
  test auth::tests::derive_project_token_is_64_hex_chars ... ok
  test auth::tests::derive_project_token_differs_by_seed ... ok
  test auth::tests::derive_project_token_is_deterministic ... ok
  test auth::tests::derive_project_token_differs_by_project ... ok
  test auth::tests::token_scope_serializes_project ... ok
  test error::tests::api_error_serializes_to_json_with_error_and_code ... ok
  test auth::tests::validate_token_project_matches ... ok
  test auth::tests::validate_token_wrong_project_fails ... ok
  test auth::tests::validate_token_invalid_fails ... ok
  test error::tests::forbidden_response_has_403_status ... ok
  test error::tests::core_not_found_maps_to_404 ... ok
  test error::tests::core_validation_maps_to_400 ... ok
  test error::tests::internal_response_has_500_status ... ok
  test error::tests::not_found_response_has_404_status ... ok
  test error::tests::service_unavailable_response_has_503_status ... ok
  test error::tests::unauthorized_response_has_401_status ... ok
  test error::tests::into_response_produces_json_content_type ... ok
  test state::tests::ito_path_for_rejects_path_traversal ... ok
  test state::tests::ito_path_for_resolves_to_expected_path ... ok
  test state::tests::ensure_project_dir_creates_directories ... ok

  test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/archive_sync.rs (target/llvm-cov-target/debug/deps/archive_sync-d28fb080c172bd7a)

  running 3 tests
  test sync_pull_returns_artifact_bundle ... ok
  test sync_push_updates_backend_artifacts ... ok
  test archive_endpoint_promotes_specs_and_moves_change ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

       Running tests/bootstrap_endpoints.rs (target/llvm-cov-target/debug/deps/bootstrap_endpoints-b4d46d517e6b63bc)

  running 9 tests
  test health_endpoint_does_not_require_auth ... ok
  test ready_endpoint_does_not_require_auth ... ok
  test project_route_rejects_missing_token ... ok
  test project_route_rejects_invalid_token ... ok
  test project_route_accepts_derived_project_token ... ok
  test project_route_accepts_admin_token ... ok
  test health_endpoint_returns_status_and_version ... ok
  test ready_endpoint_returns_ready_when_data_dir_exists ... ok
  test project_route_rejects_non_allowlisted_org ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/event_ingest.rs (target/llvm-cov-target/debug/deps/event_ingest-4de220a654807a67)

  running 6 tests
  test ingest_requires_authentication ... ok
  test ingest_missing_idempotency_key_rejected ... ok
  test ingest_empty_batch_accepted ... ok
  test ingest_accepts_event_batch ... ok
  test ingest_idempotent_retry_returns_duplicates ... ok
  test list_events_returns_backend_managed_audit_log ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/multi_tenant.rs (target/llvm-cov-target/debug/deps/multi_tenant-b724c32bbc4858db)

  running 13 tests
  test get_change_tasks_returns_task_list ... ok
  test derived_token_for_project_b_cannot_access_project_a ... ok
  test non_allowlisted_repo_in_allowed_org_is_rejected ... ok
  test derived_token_for_project_a_accesses_project_a ... ok
  test derived_token_for_project_a_cannot_access_project_b ... ok
  test modules_are_isolated_between_projects ... ok
  test admin_token_lists_changes_for_project_b ... ok
  test get_nonexistent_module_returns_404 ... ok
  test admin_token_lists_changes_for_project_a ... ok
  test get_single_change_returns_detail ... ok
  test get_single_module_returns_detail ... ok
  test get_nonexistent_change_returns_404 ... ok
  test events_are_isolated_between_projects ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

       Running tests/specs.rs (target/llvm-cov-target/debug/deps/specs-a4edbaed4a34915e)

  running 2 tests
  test list_specs_returns_promoted_specs ... ok
  test get_spec_returns_markdown ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

       Running tests/task_mutations.rs (target/llvm-cov-target/debug/deps/task_mutations-32be2b19c311d15c)

  running 5 tests
  test start_task_endpoint_reports_missing_tasks_as_not_found ... ok
  test tasks_markdown_endpoint_returns_none_for_missing_artifact ... ok
  test shelve_task_endpoint_accepts_reason_payload ... ok
  test complete_task_endpoint_accepts_note_payload ... ok
  test start_task_endpoint_updates_remote_tasks ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running unittests src/main.rs (target/llvm-cov-target/debug/deps/ito-605bd9f06ee82361)

  running 66 tests
  test app::archive::tests::only_filesystem_mode_requires_local_changes_dir ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_none_input ... ok
  test app::archive::tests::archive_follow_up_messages_cover_all_modes ... ok
  test app::instructions::tests::json_get_empty_keys_returns_root ... ok
  test app::instructions::tests::worktree_config_defaults_when_no_worktrees_key ... ok
  test app::instructions::tests::worktree_config_ignores_empty_strings ... ok
  test app::instructions::tests::worktree_config_checkout_siblings_sets_project_root ... ok
  test app::instructions::tests::json_get_traverses_nested_keys ... ok
  test app::instructions::tests::worktree_config_parses_all_fields ... ok
  test app::instructions::tests::worktree_config_checkout_subdir_sets_project_root ... ok
  test app::instructions::tests::json_get_returns_none_for_missing_key ... ok
  test app::instructions::tests::json_get_returns_none_for_non_object_intermediate ... ok
  test app::instructions::tests::worktree_config_no_project_root_when_none_passed ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_empty_slice ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_mixed_levels ... ok
  test app::instructions::tests::collect_context_files_preserves_order ... ok
  test app::instructions::tests::worktree_config_parses_bare_control_siblings_strategy ... ok
  test app::list::tests::parse_sort_order_supports_separate_and_equals_forms ... ok
  test cli::ralph::ralph_tests::harness_arg_converts_to_core_harness_name ... ok
  test app::list::tests::format_task_status_handles_various_states ... ok
  test app::list::tests::format_relative_time_covers_major_buckets ... ok
  test commands::backend::tests::resolve_project_root_returns_parent_directory ... ok
  test commands::backend::tests::resolve_project_root_rejects_parentless_paths ... ok
  test app::run::tests::removed_serve_api_replacement_preserves_flags_and_args ... ok
  test app::instructions::tests::backend_instruction_is_cli_first_for_remote_mode ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_empty_ip ... ok
  test commands::config::config_tests::json_render_value_renders_common_json_types ... ok
  test app::worktree_wizard::worktree_wizard_tests::load_worktree_result_from_config_returns_expected_defaults_and_values ... ok
  test app::worktree_wizard::worktree_wizard_tests::is_worktree_configured_detects_strategy_key ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_non_zero_exit ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_when_command_missing ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_success ... ok
  test commands::serve_api::serve_api_tests::builds_config_with_defaults ... ok
  test commands::serve_api::serve_api_tests::builds_allowlist_from_allow_org_args ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_errors_when_enabled_missing_fields ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_missing ... ok
  test commands::config::config_tests::config_schema_includes_coordination_sync_interval_default ... ok
  test commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default ... ok
  test cli::cli_tests::parses_top_level_sync_force_flag ... ok
  test cli::cli_tests::parses_top_level_sync_command ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_disabled_and_preserves_other_keys ... ok
  test diagnostics::tests::blocking_task_error_message_includes_rendered_errors ... ok
  test diagnostics::tests::blocking_task_error_message_returns_none_when_no_errors ... ok
  test commands::serve_api::serve_api_tests::merge_allow_orgs_preserves_existing_repo_rules ... ok
  test diagnostics::tests::format_path_line_includes_optional_line_number ... ok
  test diagnostics::tests::render_task_diagnostics_filters_by_level_and_renders_task_id_when_present ... ok
  test diagnostics::tests::render_validation_issues_renders_level_path_and_message ... ok
  test diagnostics::tests::render_validation_issues_renders_rule_id_when_present ... ok
  test util::tests::command_id_maps_gr_to_grep ... ok
  test util::tests::command_id_maps_x_templates_to_templates ... ok
  test util::tests::command_id_uses_positional_args_and_normalizes_hyphens ... ok
  test util::tests::sanitize_args_redacts_equals_form ... ok
  test util::tests::sanitize_args_redacts_sensitive_flags ... ok
  test util::tests::sanitize_args_replaces_paths ... ok
  test util::tests::split_csv_trims_parts ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_ok_when_present ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_path_is_file ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_enabled_settings ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_accepts_full_ito_json_config ... ok
  test commands::config::config_tests::handle_config_schema_writes_file_when_output_is_set ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_trailing_json_content ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_unknown_json_fields ... ok
  test app::worktree_wizard::worktree_wizard_tests::save_worktree_config_writes_config_and_runs_print_paths ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_reads_toml ... ok
  test app::list::tests::progress_filter_flags_are_mutually_exclusive ... ok
  test app::instructions::tests::worktree_config_bare_control_siblings_calls_resolve ... ok

  test result: ok. 66 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/agent_instruction_bootstrap.rs (target/llvm-cov-target/debug/deps/agent_instruction_bootstrap-10eeef52f3345004)

  running 9 tests
  test bootstrap_codex_success ... ok
  test bootstrap_rejects_invalid_tool ... ok
  test bootstrap_contains_artifact_pointers ... ok
  test bootstrap_opencode_success ... ok
  test bootstrap_requires_tool_flag ... ok
  test bootstrap_github_copilot_success ... ok
  test bootstrap_output_is_short ... ok
  test bootstrap_json_output ... ok
  test bootstrap_claude_success ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.67s

       Running tests/agent_instruction_context.rs (target/llvm-cov-target/debug/deps/agent_instruction_context-2fe9329cc9dfb902)

  running 2 tests
  Switched to a new branch '023-07_harness-context-inference'
  test agent_instruction_context_prefers_path_inference_in_text_output ... ok
  test agent_instruction_context_supports_json_output ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

       Running tests/agent_instruction_memory.rs (target/llvm-cov-target/debug/deps/agent_instruction_memory-6886037c52c0ef07)

  running 14 tests
  test agent_instruction_help_lists_memory_artifacts ... ok
  test memory_search_not_configured_branch_renders_setup_guidance ... ok
  test memory_search_command_branch_overrides_limit_when_supplied ... ok
  test memory_capture_skill_branch_emits_structured_inputs ... ok
  test memory_query_skill_branch_emits_structured_inputs ... ok
  test memory_search_command_branch_substitutes_query_and_default_limit ... ok
  test memory_query_renders_not_configured_when_only_capture_set ... ok
  test memory_query_command_branch_substitutes_query ... ok
  test memory_capture_renders_skill_when_only_capture_configured ... ok
  test memory_query_not_configured_branch_renders_setup_guidance ... ok
  test memory_search_requires_query_flag ... ok
  test memory_search_skill_branch_emits_structured_inputs ... ok
  test memory_capture_command_branch_renders_executable_command_line ... ok
  test memory_capture_not_configured_branch_renders_setup_guidance ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.99s

       Running tests/agent_instruction_orchestrate.rs (target/llvm-cov-target/debug/deps/agent_instruction_orchestrate-77f8c3befc070640)

  running 5 tests
  test orchestrate_requires_orchestrate_md ... ok
  test orchestrate_succeeds_when_orchestrate_md_exists ... ok
  test orchestrate_json_output_has_correct_artifact_id ... ok
  test orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter ... ok
  test orchestrate_surfaces_recommended_skills_from_preset ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

       Running tests/agent_instruction_repo_sweep.rs (target/llvm-cov-target/debug/deps/agent_instruction_repo_sweep-e4567a7dee9c6ed9)

  running 3 tests
  test repo_sweep_succeeds_without_change_flag ... ok
  test repo_sweep_json_output_has_correct_artifact_id ... ok
  test repo_sweep_output_contains_key_phrases ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/agent_instruction_worktrees.rs (target/llvm-cov-target/debug/deps/agent_instruction_worktrees-636d0a3c24032b96)

  running 2 tests
  test worktrees_instruction_does_not_require_change ... ok
  test worktrees_instruction_json_output ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/aliases.rs (target/llvm-cov-target/debug/deps/aliases-84c6f21fd1e880e8)

  running 4 tests
  test subcommand_aliases_work ... ok
  test main_command_aliases_work ... ok
  test main_command_aliases_execute ... ok
  test short_flags_work ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/archive_completed.rs (target/llvm-cov-target/debug/deps/archive_completed-ae288e3e4983af13)

  running 7 tests
  test archive_completed_conflict_with_positional ... ok
  test archive_completed_no_completed_changes ... ok
  test archive_completed_decline_confirmation_cancels ... ok
  test archive_completed_empty_confirmation_cancels ... ok
  test archive_completed_accept_yes_confirmation_archives ... ok
  test archive_completed_archives_all_completed ... ok
  test archive_completed_skip_specs ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s

       Running tests/archive_remote_mode.rs (target/llvm-cov-target/debug/deps/archive_remote_mode-37e0edf3bc779889)

  running 1 test
  test remote_archive_succeeds_without_local_active_change_markdown ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

       Running tests/archive_smoke.rs (target/llvm-cov-target/debug/deps/archive_smoke-fce63e9901ecc00e)

  running 1 test
  test archive_with_specs_and_validation_smoke ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

       Running tests/audit_more.rs (target/llvm-cov-target/debug/deps/audit_more-95dee92b89b2f09a)

  running 6 tests
  test audit_more_local_audit_writes_warn_and_fallback_without_worktree_log_when_branch_storage_is_unavailable ... ok
  test audit_log_stats_and_validate_json_outputs_are_well_formed ... ok
  test audit_more_local_audit_writes_use_internal_branch_without_worktree_log_churn ... ok
  test audit_subcommands_cover_text_output_limit_reconcile_and_stream ... ok
  test audit_stream_all_worktrees_dedupes_shared_routed_storage ... ok
  test audit_commands_migrate_legacy_worktree_log_into_routed_storage ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.45s

       Running tests/audit_remote_mode.rs (target/llvm-cov-target/debug/deps/audit_remote_mode-fdaee7d3aa67c7f8)

  running 2 tests
  test audit_commands_in_backend_mode_use_server_only_storage ... ok
  test validate_single_change_in_backend_mode_skips_local_audit_reconcile ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.26s

       Running tests/backend_import.rs (target/llvm-cov-target/debug/deps/backend_import-cbdc964b12f3b4f3)

  running 4 tests
  test backend_import_rejects_local_mode ... ok
  test backend_import_dry_run_reports_scope_without_writing_backend ... ok
  test backend_import_writes_active_and_archived_changes_to_backend ... ok
  test backend_import_is_idempotent_and_remote_reads_match_imported_changes ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.37s

       Running tests/backend_qa_walkthrough.rs (target/llvm-cov-target/debug/deps/backend_qa_walkthrough-3c32be800f8f69f9)

  running 1 test
  test backend_qa_script_verify_runs_end_to_end ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s

       Running tests/backend_serve.rs (target/llvm-cov-target/debug/deps/backend_serve-43a1b3b178aedcd0)

  running 5 tests
  test backend_serve_reports_unknown_fields_in_explicit_config_file ... ok
  test backend_serve_init_prints_backend_command_guidance ... ok
  test backend_serve_service_mode_reports_malformed_backend_config ... ok
  test backend_serve_service_mode_reuses_existing_auth_without_printing_init_output ... ok
  test backend_serve_service_mode_bootstraps_missing_auth_silently ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/backend_status_more.rs (target/llvm-cov-target/debug/deps/backend_status_more-918ba38be831672f)

  running 20 tests
  test generate_token_no_seed_fails ... ok
  test generate_token_missing_org_fails ... ok
  test backend_status_json_includes_config_details ... ok
  test backend_status_with_valid_config_but_no_server ... ok
  test generate_token_with_all_sources_prefers_env ... ok
  test backend_status_disabled_json_output ... ok
  test generate_token_seed_from_env_takes_precedence ... ok
  test backend_status_unreachable_server_json_output ... ok
  test generate_token_derives_deterministic_token ... ok
  test backend_status_unreachable_server_fails ... ok
  test backend_status_with_env_token_no_warning ... ok
  test backend_status_incomplete_config_fails ... ok
  test generate_token_missing_repo_fails ... ok
  test generate_token_flag_overrides_for_org_repo ... ok
  test backend_status_disabled_shows_informational_output ... ok
  test backend_status_token_security_warning ... ok
  test silent_fallback_grep_warns_on_bad_config ... ok
  test silent_fallback_with_valid_backend_no_warnings ... ok
  test silent_fallback_tasks_warns_on_bad_config ... ok
  test silent_fallback_event_forwarding_warns_on_bad_config ... ok

  test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.39s

       Running tests/cli_smoke.rs (target/llvm-cov-target/debug/deps/cli_smoke-ecb1cbda302b3b30)

  running 6 tests
  test cli_help_hides_top_level_serve_api_entrypoint ... ok
  test cli_top_level_serve_api_help_shows_backend_migration_guidance ... ok
  test cli_top_level_serve_api_shows_backend_migration_guidance ... ok
  test agent_instruction_status_archive_smoke ... ok
  test list_show_validate_smoke ... ok
  test create_workflow_plan_state_config_smoke ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.87s

       Running tests/cli_snapshots.rs (target/llvm-cov-target/debug/deps/cli_snapshots-7bebb474ff6facdb)

  running 14 tests
  test snapshot_agent_instruction_help ... ok
  test snapshot_validate_help ... ok
  test snapshot_agent_help ... ok
  test snapshot_ralph_help ... ok
  test snapshot_help ... ok
  test snapshot_version ... ok
  test snapshot_list_help ... ok
  test snapshot_init_help ... ok
  test snapshot_backend_serve_help ... ok
  test snapshot_backend_help ... ok
  test snapshot_create_help ... ok
  test snapshot_tasks_help ... ok
  test snapshot_help_all_subcommand ... ok
  test snapshot_help_all_global_flag ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/config_more.rs (target/llvm-cov-target/debug/deps/config_more-b1868609abe193d8)

  running 5 tests
  test config_unknown_subcommand_errors ... ok
  test config_set_rejects_invalid_audit_mirror_branch_name ... ok
  test config_set_rejects_invalid_coordination_branch_name ... ok
  test config_help_path_list_unset_and_schema_smoke ... ok
  test config_set_get_supports_coordination_and_audit_mirror_keys ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s

       Running tests/coverage_smoke.rs (target/llvm-cov-target/debug/deps/coverage_smoke-5af3dfea479278d9)

  running 3 tests
  test serve_errors_when_no_ito_dir_exists ... ok
  test completions_command_runs_for_all_shells ... ok
  test audit_validate_and_log_work_with_empty_event_log ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

       Running tests/create_more.rs (target/llvm-cov-target/debug/deps/create_more-044a219aef567401)

  running 4 tests
  test create_change_sub_module_and_module_are_mutually_exclusive ... ok
  test create_change_sub_module_rejects_remote_persistence_mode ... ok
  test create_change_with_sub_module_flag_creates_composite_id_change ... ok
  test create_module_and_change_error_paths_and_outputs ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.56s

       Running tests/grep_more.rs (target/llvm-cov-target/debug/deps/grep_more-e97bf78d6733320c)

  running 5 tests
  test grep_change_scope_rejects_too_many_positional_args ... ok
  test grep_change_scope_prints_matches_with_locations ... ok
  test grep_module_scope_searches_all_changes_in_module ... ok
  test grep_limit_caps_output_and_prints_warning ... ok
  test grep_all_scope_searches_all_changes ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/help.rs (target/llvm-cov-target/debug/deps/help-ca98306e444902cd)

  running 7 tests
  test help_shows_navigation_footer ... ok
  test agent_instruction_help_shows_instruction_details ... ok
  test help_prints_usage ... ok
  test help_all_global_flag_works ... ok
  test dash_h_help_matches_dash_dash_help ... ok
  test help_all_shows_complete_reference ... ok
  test help_all_json_outputs_valid_json ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/init_coordination.rs (target/llvm-cov-target/debug/deps/init_coordination-80bd4f14f0a0dcef)

  running 4 tests
  test init_no_coordination_worktree_writes_embedded_storage ... ok
  test init_without_git_remote_falls_back_gracefully ... ok
  test init_upgrade_does_not_touch_coordination_storage ... ok
  test init_with_git_remote_creates_coordination_worktree ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.47s

       Running tests/init_gitignore_session_json.rs (target/llvm-cov-target/debug/deps/init_gitignore_session_json-8f1cd4540320b3f9)

  running 1 test
  test init_writes_gitignore_session_json_and_is_idempotent ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

       Running tests/init_more.rs (target/llvm-cov-target/debug/deps/init_more-cfd4a221b9db6a3d)

  running 28 tests
  test init_interactive_detects_tools_and_installs_adapter_files ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
  test init_help_prints_usage ... ok
  test init_requires_tools_when_non_interactive ... ok
  test init_refuses_to_overwrite_existing_file_without_markers_when_not_forced ... ok
  test init_renders_agents_md_without_raw_jinja2_syntax ... ok
  test init_prints_project_setup_nudge_when_marker_incomplete ... ok
  test init_opencode_installs_audit_hook_plugin ... ok
  test init_github_copilot_installs_audit_preflight_assets ... ok
  test init_renders_skill_files_without_raw_jinja2_syntax ... ok
  test init_codex_installs_audit_instruction_assets ... ok
  test init_tools_csv_ignores_empty_segments ... ok
  test init_force_overwrites_existing_user_prompt_stubs ... ok
  test init_does_not_print_project_setup_nudge_when_marker_complete ... ok
  test init_does_not_print_project_setup_nudge_when_marker_absent ... ok
  test init_with_tools_none_installs_ito_skeleton ... ok
  test init_update_without_prior_init_creates_all_files ... ok
  test init_update_does_not_overwrite_existing_user_prompt_stubs ... ok
  test init_with_tools_opencode_installs_orchestrator_agent_template ... ok
  test init_update_preserves_user_files_and_creates_missing ... ok
  test init_update_renders_agents_md_without_raw_jinja2 ... ok
  test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
  test init_writes_config_with_release_tag_schema_reference ... ok
  test init_with_tools_csv_installs_selected_adapters ... ok
  test init_tools_parser_covers_all_and_invalid_id ... ok
  test init_setup_coordination_branch_fails_without_origin_remote ... ok
  test init_setup_coordination_branch_reports_ready_when_already_present ... ok
  test init_setup_coordination_branch_creates_branch_on_origin ... ok
  test init_setup_coordination_branch_uses_configured_branch_name ... ok

  test result: ok. 27 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.96s

       Running tests/init_tmux.rs (target/llvm-cov-target/debug/deps/init_tmux-c1a88496f96309a1)

  running 5 tests
  test init_interactive_can_disable_tmux_preference ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
  test init_with_no_tmux_writes_tmux_enabled_false ... ok
  test init_uses_cascading_tmux_preference_from_global_config ... ok
  test init_writes_tmux_enabled_true_by_default ... ok
  test init_update_preserves_existing_tmux_preference ... ok

  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/init_upgrade_more.rs (target/llvm-cov-target/debug/deps/init_upgrade_more-1546a5e5128d904f)

  running 5 tests
  test init_upgrade_flag_is_accepted ... ok
  test init_upgrade_skips_and_warns_when_markers_missing ... ok
  test init_update_does_not_error_on_existing_agents_md_without_markers ... ok
  test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
  test init_update_preserves_user_owned_files ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

       Running tests/instructions_more.rs (target/llvm-cov-target/debug/deps/instructions_more-7dbb6d860e57a63d)

  running 14 tests
  test agent_instruction_archive_without_change_prints_generic_guidance ... ok
  test agent_instruction_review_requires_change_flag ... ok
  test agent_instruction_finish_with_change_prompts_for_archive ... ok
  test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
  test agent_instruction_change_flag_reports_ambiguous_target ... ok
  test agent_instruction_change_flag_supports_slug_query ... ok
  test agent_instruction_archive_with_invalid_change_fails ... ok
  test agent_instruction_change_flag_supports_shorthand ... ok
  test agent_instruction_proposal_honors_testing_policy_override ... ok
  test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
  test agent_instruction_text_output_renders_artifact_envelope ... ok
  test agent_instruction_proposal_without_change_supports_json_output ... ok
  test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
  test agent_instruction_review_renders_review_template ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s

       Running tests/list_archive.rs (target/llvm-cov-target/debug/deps/list_archive-b3d7869d2a9687bd)

  running 3 tests
  test list_archive_reports_empty_archives ... ok
  test list_archive_json_lists_archived_changes_only ... ok
  test list_archive_lists_archived_changes_only ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/list_regression.rs (target/llvm-cov-target/debug/deps/list_regression-f638587e978d6190)

  running 3 tests
  test list_sort_regression ... ok
  test list_default_text_and_json_shape_regression ... ok
  test list_filters_regression ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

       Running tests/misc_more.rs (target/llvm-cov-target/debug/deps/misc_more-615159f9de72c2d1)

  running 16 tests
  test archive_prompts_on_incomplete_tasks_and_proceeds_when_confirmed ... ignored, PTY interactive test — can hang in CI; run with --ignored locally
  test list_errors_when_ito_changes_dir_missing ... ok
  test plan_status_errors_when_roadmap_missing ... ok
  test list_modules_empty_prints_hint ... ok
  test status_schema_not_found_includes_available_schemas ... ok
  test status_change_flag_not_found_shows_suggestions ... ok
  test status_change_flag_reports_ambiguous_target ... ok
  test show_unknown_item_offers_suggestions ... ok
  test status_change_flag_supports_module_scoped_slug_query ... ok
  test status_missing_change_flag_lists_available_changes ... ok
  test list_specs_empty_prints_sentence_even_for_json ... ok
  test git_env_vars_do_not_override_runtime_root_detection ... ok
  test commands_run_from_nested_dir_use_git_worktree_root ... ok
  test status_change_flag_supports_shorthand_and_partial_match ... ok
  test show_module_errors_and_json_not_implemented ... ok
  test show_spec_json_filters_and_requirement_index_errors ... ok

  test result: ok. 15 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running tests/new_more.rs (target/llvm-cov-target/debug/deps/new_more-ece3d61c7244f877)

  running 1 test
  test new_change_covers_happy_and_error_paths ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

       Running tests/parity_help_version.rs (target/llvm-cov-target/debug/deps/parity_help_version-8b6ce79e54b6e4b0)

  running 2 tests
  test version_prints_workspace_version ... ok
  test help_prints_usage ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/parity_tasks.rs (target/llvm-cov-target/debug/deps/parity_tasks-ecf758152c0899e0)

  running 2 tests
  test parity_tasks_init_writes_same_file ... ok
  test parity_tasks_status_next_start_complete_match_oracle ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

       Running tests/path_more.rs (target/llvm-cov-target/debug/deps/path_more-38477eb8dcdb7796)

  running 8 tests
  test path_missing_subcommand_errors ... ok
  test path_errors_in_bare_repo ... ok
  test path_worktree_requires_a_selector_flag ... ok
  test path_worktrees_root_requires_worktrees_enabled ... ok
  test path_roots_json_includes_worktree_fields_when_enabled ... ok
  test path_roots_text_renders_worktree_fields_when_available ... ok
  test path_worktrees_root_and_change_worktree_resolve_from_config ... ok
  test path_roots_are_absolute_in_initialized_repo ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

       Running tests/plan_state_more.rs (target/llvm-cov-target/debug/deps/plan_state_more-8a4e1303506799b4)

  running 3 tests
  test plan_status_fails_without_roadmap ... ok
  test plan_init_creates_structure ... ok
  test plan_status_succeeds_after_init ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

       Running tests/ralph_smoke.rs (target/llvm-cov-target/debug/deps/ralph_smoke-566db59e5eb532cd)

  running 26 tests
  test ralph_change_flag_supports_shorthand_resolution ... ok
  test ralph_change_flag_supports_slug_query_resolution ... ok
  test ralph_file_flag_requires_readable_file ... ok
  test ralph_file_flag_runs_without_change_or_module ... ok
  test ralph_file_flag_allowed_without_change_or_module ... ok
  test ralph_continue_ready_exits_successfully_when_all_changes_complete ... ok
  test ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
  test ralph_interactive_prompts_and_runs_selected_changes_sequentially ... ok
  test ralph_no_interactive_without_target_returns_clear_error ... ok
  test ralph_markdown_prd_source_marks_first_pending_task_complete ... ok
  test ralph_interactive_status_prompts_for_exactly_one_change ... ok
  test ralph_unknown_harness_returns_clear_error ... ok
  test ralph_yaml_source_marks_first_pending_task_complete ... ok
  test ralph_accepts_new_harness_names_for_status_flow ... ok
  [main (root-commit) eaf039b] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  [main (root-commit) eaf039b] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  [main (root-commit) 0619915] init
   6 files changed, 44 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 README.md
   create mode 100644 tasks.yaml
  [main (root-commit) 4b9e139] init
   6 files changed, 38 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 README.md
   create mode 100644 tasks.yaml
  [main (root-commit) 4ccf055] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  test ralph_stub_harness_writes_state_and_status_works ... ok
  test ralph_branch_per_task_requires_clean_worktree ... ok
  test ralph_github_source_closes_issue_on_success ... ok
  test ralph_branch_per_task_creates_task_branch_for_prd_source ... ok
  To /var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpQWaAgp
   * [new branch]      main -> main
  branch 'main' set up to track 'origin/main'.
  test ralph_sync_issue_updates_prd_back_to_github_issue ... ok
  test ralph_parallel_yaml_source_completes_grouped_tasks ... ok
  test ralph_notify_emits_operator_notification_on_success ... ok
  test ralph_interactive_options_wizard_exit_on_error_stops_on_nonzero_harness_exit ... ok
  test ralph_browser_flag_injects_agent_browser_guidance_for_opencode ... ok
  test ralph_create_pr_uses_base_branch_and_fake_gh ... ok
  test ralph_interactive_options_wizard_prompts_for_missing_values_and_applies_them ... ok
  test ralph_parallel_preserves_worker_code_changes ... ok

  test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.63s

       Running tests/serve_more.rs (target/llvm-cov-target/debug/deps/serve_more-3bd815967863768c)

  running 1 test
  test serve_errors_when_not_initialized ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/show_specs_bundle.rs (target/llvm-cov-target/debug/deps/show_specs_bundle-96328f5b5eb079f9)

  running 2 tests
  test show_specs_bundles_truth_specs_as_json_with_absolute_paths ... ok
  test show_specs_bundles_truth_specs_as_markdown_with_metadata ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/show_specs_remote_mode.rs (target/llvm-cov-target/debug/deps/show_specs_remote_mode-d258f04ea50cea56)

  running 1 test
  test show_specs_reads_backend_specs_without_local_markdown ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

       Running tests/source_file_size.rs (target/llvm-cov-target/debug/deps/source_file_size-59d5e87e25e22ae9)

  running 1 test
  test ito_cli_source_files_are_reasonably_sized ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/stats.rs (target/llvm-cov-target/debug/deps/stats-da3379e11a7e4199)

  running 1 test
  test stats_counts_command_end_events ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/tasks_more.rs (target/llvm-cov-target/debug/deps/tasks_more-b20efe15d68127c3)

  running 11 tests
  test tasks_status_rejects_free_form_with_more_than_two_numbers ... ok
  test tasks_status_resolves_short_change_id ... ok
  test tasks_status_resolves_free_form_two_numbers ... ok
  test tasks_commands_use_apply_tracks_filename_when_set ... ok
  test tasks_json_lists_are_sorted_by_task_id ... ok
  test tasks_complete_supports_checkbox_compat_mode ... ok
  test tasks_error_paths_cover_more_branches ... ok
  test tasks_start_supports_checkbox_compat_mode_and_enforces_single_in_progress ... ok
  test tasks_next_supports_checkbox_compat_mode_and_shows_current_or_next ... ok
  test tasks_add_shelve_unshelve_show_cover_more_paths ... ok
  test tasks_commands_support_json_output ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

       Running tests/tasks_remote_mode.rs (target/llvm-cov-target/debug/deps/tasks_remote_mode-3c71828aeafa7c3e)

  running 2 tests
  test remote_missing_tasks_commands_do_not_hard_fail ... ok
  test remote_task_start_updates_backend_without_local_tasks_file ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s

       Running tests/templates_schemas_export.rs (target/llvm-cov-target/debug/deps/templates_schemas_export-fad80cb61bff6359)

  running 3 tests
  test templates_help_includes_schemas_export ... ok
  test templates_schemas_export_writes_embedded_files ... ok
  test templates_schemas_export_skips_without_force_then_overwrites_with_force ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

       Running tests/trace_more.rs (target/llvm-cov-target/debug/deps/trace_more-14c56ac2f8b63fb5)

  running 8 tests
  test trace_missing_change_exits_nonzero ... ok
  test trace_unresolved_reference_shows_unresolved_in_output ... ok
  test trace_partial_ids_json_shows_invalid_status ... ok
  test trace_fully_covered_exits_zero ... ok
  test trace_uncovered_requirement_shows_uncovered_in_output ... ok
  test trace_legacy_checkbox_change_shows_unavailable ... ok
  test trace_fully_covered_json_has_ready_status ... ok
  test trace_uncovered_requirement_json_shows_uncovered_list ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

       Running tests/update_marker_scoped.rs (target/llvm-cov-target/debug/deps/update_marker_scoped-47b1612f7adc8b16)

  running 5 tests
  test update_refuses_to_overwrite_partial_marker_pair ... ok
  test update_preserves_user_edits_after_end_marker_in_harness_command ... ok
  test update_still_refreshes_non_markdown_manifest_assets ... ok
  test update_preserves_user_edits_after_end_marker_in_harness_skill ... ok
  test second_update_is_a_noop_for_harness_skills ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

       Running tests/update_smoke.rs (target/llvm-cov-target/debug/deps/update_smoke-4d6415c68e1dd5f0)

  running 8 tests
  test update_installs_adapter_files_from_local_ito_skills ... ok
  test update_preserves_project_config_and_project_md ... ok
  test update_preserves_user_guidance_and_user_prompt_files ... ok
  test update_merges_claude_settings_without_clobbering_user_keys ... ok
  test update_refreshes_codex_audit_instruction_assets ... ok
  test update_renders_agents_md_without_jinja2_syntax ... ok
  test update_refreshes_opencode_plugin_and_preserves_user_config ... ok
  test update_refreshes_github_copilot_audit_assets ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s

       Running tests/user_guidance_injection.rs (target/llvm-cov-target/debug/deps/user_guidance_injection-d372bbdcd5062a7e)

  running 3 tests
  test agent_instruction_includes_user_guidance_when_present ... ok
  test agent_instruction_includes_scoped_user_prompt_for_artifact ... ok
  test agent_instruction_prefers_user_prompts_shared_guidance_file ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

       Running tests/validate_more.rs (target/llvm-cov-target/debug/deps/validate_more-3b35b345e402ec74)

  running 10 tests
  test validate_type_module_special_cases_to_spec_by_id ... ok
  test validate_unknown_spec_offers_suggestions ... ok
  test validate_ambiguous_item_is_an_error ... ok
  test validate_all_json_success_has_summary_and_by_type ... ok
  test validate_change_runs_schema_rules_for_custom_schema ... ok
  test validate_change_and_bulk_do_not_duplicate_schema_tracking_issues ... ok
  test validate_module_routes_and_error_paths ... ok
  test validate_all_prints_failure_report_in_text_mode ... ok
  test validate_change_reports_audit_drift_against_routed_storage ... ok
  test validate_single_change_audit_flag_reports_only_audit_issues ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

       Running tests/view_proposal.rs (target/llvm-cov-target/debug/deps/view_proposal-9f9590a9936550ce)

  running 8 tests
  test view_proposal_help_shows_viewer_flag ... ok
  test view_proposal_html_viewer_errors_when_pandoc_missing ... ok
  test view_proposal_unknown_change_fails ... ok
  test view_proposal_unknown_viewer_is_rejected ... ok
  test view_proposal_disabled_tmux_is_rejected ... ok
  test view_proposal_json_outputs_bundle ... ok
  test view_proposal_html_viewer_is_recognized ... ok
  test view_proposal_html_viewer_succeeds_with_stub_pandoc ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_common-c989c8b715a9503f)

  running 59 tests
  test git_url::tests::handles_ssh_url_without_user ... ok
  test git_url::tests::parses_git_protocol_url ... ok
  test git_url::tests::handles_trailing_slash_in_https_url ... ok
  test git_url::tests::parses_https_url_without_git_suffix ... ok
  test git_url::tests::parses_http_scheme ... ok
  test git_url::tests::parses_gitlab_style_subgroup_takes_last_two_segments ... ok
  test git_url::tests::parses_scp_ssh_url ... ok
  test git_url::tests::parses_ssh_with_explicit_port ... ok
  test git_url::tests::parses_https_url_with_git_suffix ... ok
  test git_url::tests::returns_none_for_empty_string ... ok
  test git_url::tests::returns_none_for_bare_string_without_separator ... ok
  test git_url::tests::returns_none_for_no_path_after_host ... ok
  test git_url::tests::returns_none_for_scp_url_with_single_component ... ok
  test git_url::tests::returns_none_for_single_path_component ... ok
  test git_url::tests::returns_none_for_whitespace_only ... ok
  test git_url::tests::strips_git_suffix_only_once ... ok
  test id::change_id::tests::parse_change_id_allows_large_change_numbers ... ok
  test id::change_id::tests::parse_change_id_allows_three_digit_change_numbers ... ok
  test id::change_id::tests::parse_change_id_normalizes_excessive_padding_for_large_change_numbers ... ok
  test id::change_id::tests::parse_change_id_pads_both_parts ... ok
  test id::change_id::tests::parse_change_id_rejects_overlong_input ... ok
  test id::change_id::tests::parse_change_id_missing_name_has_specific_error ... ok
  test id::change_id::tests::parse_change_id_sub_module_format_canonical ... ok
  test id::change_id::tests::parse_change_id_sub_module_format_lowercases_name ... ok
  test id::change_id::tests::parse_change_id_sub_module_format_pads_all_parts ... ok
  test id::change_id::tests::parse_change_id_sub_module_missing_name_is_error ... ok
  test id::change_id::tests::parse_change_id_sub_module_rejects_module_overflow ... ok
  test id::change_id::tests::parse_change_id_sub_module_rejects_sub_overflow ... ok
  test id::change_id::tests::parse_change_id_supports_extra_leading_zeros_for_change_num ... ok
  test id::change_id::tests::parse_change_id_uses_specific_hint_for_wrong_separator ... ok
  test id::module_id::tests::parse_module_id_pads_and_lowercases_name ... ok
  test id::module_id::tests::parse_module_id_rejects_overflow ... ok
  test id::module_id::tests::parse_module_id_rejects_overlong_input ... ok
  test id::spec_id::tests::parse_spec_id_preserves_value ... ok
  test id::spec_id::tests::parse_spec_id_rejects_path_traversal_sequences ... ok
  test id::sub_module_id::tests::parse_sub_module_id_canonical_form ... ok
  test id::sub_module_id::tests::parse_sub_module_id_lowercases_name ... ok
  test id::sub_module_id::tests::parse_sub_module_id_pads_both_parts ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_empty ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_missing_dot ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_module_overflow ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_non_digit_module ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_overlong_input ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_sub_overflow ... ok
  test id::sub_module_id::tests::parse_sub_module_id_strips_extra_leading_zeros ... ok
  test id::sub_module_id::tests::parse_sub_module_id_with_name_suffix ... ok
  test id::sub_module_id::tests::sub_module_id_display ... ok
  test id::tests::classify_id_hyphen_without_underscore_is_module_change_id ... ok
  test id::tests::classify_id_module_change_id ... ok
  test id::tests::classify_id_module_id ... ok
  test id::tests::classify_id_sub_module_change_id ... ok
  test id::tests::classify_id_sub_module_id ... ok
  test id::tests::looks_like_change_id_recognizes_sub_module_format ... ok
  test id::tests::looks_like_change_id_requires_digits_hyphen_and_underscore ... ok
  test id::tests::looks_like_module_id_is_digit_prefixed ... ok
  test match_::tests::levenshtein_matches_ts_examples ... ok
  test match_::tests::nearest_matches_is_stable_on_ties ... ok
  test paths::tests::builders_join_expected_paths ... ok
  test paths::tests::default_ito_root_is_dot_ito ... ok

  test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_config-e935c55953d256e8)

  running 68 tests
  test config::tests::ito_config_dir_prefers_xdg ... ok
  test config::tests::global_config_path_prefers_xdg ... ok
  test config::schema::tests::schema_contains_expected_sections ... ok
  test config::tests::audit_mirror_defaults_exist_in_cascading_config ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_missing_storage_defaults_to_worktree ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_missing_worktree_path_is_none ... ok
  test config::tests::load_global_ito_config_returns_defaults_when_no_file ... ok
  test config::tests::logging_invalid_commands_defaults_exist_in_cascading_config ... ok
  test config::tests::coordination_branch_defaults_exist_in_cascading_config ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_absent_not_serialized ... ok
  test config::tests::worktrees_config_has_defaults_in_cascading_config ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_round_trips ... ok
  test config::types::coordination_storage_tests::coordination_storage_default_is_worktree ... ok
  test config::types::coordination_storage_tests::coordination_storage_serializes_embedded_as_lowercase ... ok
  test config::types::coordination_storage_tests::coordination_storage_serializes_worktree_as_lowercase ... ok
  test config::types::coordination_storage_tests::coordination_storage_round_trips_embedded ... ok
  test config::types::coordination_storage_tests::coordination_storage_round_trips_worktree ... ok
  test config::types::memory_tests::memory_default_is_absent_on_ito_config ... ok
  test config::types::memory_tests::memory_op_config_skill_variant_requires_skill_field ... ok
  test config::types::memory_tests::memory_op_config_command_variant_requires_command_field ... ok
  test config::types::memory_tests::memory_op_config_unknown_kind_is_rejected ... ok
  test config::types::memory_tests::memory_section_omits_absent_ops_when_serialized ... ok
  test config::tests::tools_tmux_enabled_defaults_to_true_in_cascading_config ... ok
  test config::types::memory_tests::memory_section_accepts_capture_only ... ok
  test config::types::memory_tests::memory_section_accepts_skill_with_options ... ok
  test config::types::memory_tests::memory_section_round_trips_full_config ... ok
  test config::types::memory_tests::memory_section_unknown_op_key_is_rejected ... ok
  test config::types::memory_tests::memory_section_skill_options_are_optional ... ok
  test config::types::memory_tests::memory_section_round_trips_when_absent ... ok
  test config::types::memory_tests::memory_section_supports_mixed_per_op_shapes ... ok
  test config::types::worktree_init_tests::worktree_init_config_absent_deserializes_to_default ... ok
  test config::types::worktree_init_tests::full_ito_config_with_worktree_init_round_trips ... ok
  test config::types::worktree_init_tests::worktree_init_config_default_has_empty_include_and_no_setup ... ok
  test config::types::worktree_init_tests::worktree_init_config_deserializes_with_include_only ... ok
  test config::types::worktree_init_tests::worktree_init_config_with_multiple_setup_deserializes ... ok
  test config::types::worktree_init_tests::worktree_init_config_with_single_setup_deserializes ... ok
  test config::types::worktree_init_tests::worktree_setup_config_array_deserializes ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_empty_multiple_empty_vec ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_empty_single_empty_string ... ok
  test config::types::worktree_init_tests::worktree_setup_config_single_round_trips ... ok
  test config::types::worktree_init_tests::worktrees_config_init_does_not_break_existing_fields ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_not_empty_with_command ... ok
  test config::types::worktree_init_tests::worktree_setup_config_multiple_round_trips ... ok
  test config::types::worktree_init_tests::worktree_setup_config_single_string_deserializes ... ok
  test config::types::worktree_init_tests::worktrees_config_with_init_section_deserializes ... ok
  test config::types::worktree_init_tests::worktrees_config_without_init_section_uses_defaults ... ok
  test config::tests::cascading_project_config_ignores_schema_ref_key ... ok
  test context::tests::resolve_with_ctx_sets_none_when_ito_dir_is_missing ... ok
  test ito_dir::tests::get_ito_dir_name_defaults_to_dot_ito ... ok
  test config::tests::legacy_worktree_default_branch_key_migrates ... ok
  test ito_dir::tests::sanitize_rejects_path_separators_and_overlong_values ... ok
  test config::tests::new_worktree_keys_take_precedence_over_legacy ... ok
  test output::tests::no_color_env_set_matches_ts_values ... ok
  test output::tests::resolve_interactive_respects_cli_and_env ... ok
  test config::tests::coordination_branch_defaults_can_be_overridden ... ok
  test output::tests::resolve_ui_options_combines_sources ... ok
  test context::tests::resolve_with_ctx_uses_explicit_config_context_paths ... ok
  test config::tests::cascading_project_config_ignores_invalid_json_sources ... ok
  test config::tests::legacy_worktree_local_files_key_migrates ... ok
  test config::tests::logging_invalid_commands_can_be_enabled ... ok
  test config::tests::audit_mirror_defaults_can_be_overridden ... ok
  test context::tests::resolve_with_ctx_sets_ito_path_when_directory_exists ... ok
  test config::tests::load_global_ito_config_reads_backend_server_auth ... ok
  test ito_dir::tests::dot_repo_config_overrides_repo_config ... ok
  test ito_dir::tests::invalid_repo_project_path_falls_back_to_default ... ok
  test ito_dir::tests::get_ito_path_normalizes_dotdot_segments ... ok
  test config::tests::cascading_project_config_merges_sources_in_order_with_scalar_override ... ok
  test ito_dir::tests::repo_config_overrides_global_config ... ok

  test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_core-54cb6a4cc36f2073)

  running 583 tests
  test audit::mirror::tests::merge_jsonl_dedupes_and_appends_local_lines ... ok
  test audit::reader::reader_tests::reads_events_from_injected_store ... ok
  test audit::mirror::tests::merge_jsonl_ignores_blank_lines ... ok
  test audit::store::tests::internal_branch_location_keys_include_branch_identity ... ok
  test audit::reconcile::tests::build_file_state_from_default_tasks_md ... ok
  test audit::reconcile::tests::build_file_state_uses_apply_tracks_when_set ... ok
  test audit::stream::tests::default_config_has_sensible_values ... ok
  test audit::reconcile::tests::reconcile_empty_log ... ok
  test audit::reader::reader_tests::read_from_missing_file_returns_empty ... ok
  test audit::reader::reader_tests::filter_by_operation ... ok
  test audit::reader::reader_tests::combined_filters ... ok
  test audit::validate::tests::detect_duplicate_create ... ok
  test audit::validate::tests::detect_status_transition_mismatch ... ok
  test audit::validate::tests::detect_timestamp_ordering_violation ... ok
  test audit::validate::tests::different_scopes_are_independent ... ok
  test audit::reader::reader_tests::filter_by_scope ... ok
  test audit::validate::tests::empty_events_no_issues ... ok
  test audit::validate::tests::no_issues_for_valid_sequence ... ok
  test audit::worktree::tests::aggregate_empty_worktrees ... ok
  test audit::reader::reader_tests::skips_malformed_lines ... ok
  test audit::worktree::tests::find_worktree_bare_excluded ... ok
  test audit::worktree::tests::find_worktree_matching_branch ... ok
  test audit::worktree::tests::find_worktree_multiple_returns_first_match ... ok
  test audit::worktree::tests::find_worktree_no_match ... ok
  test audit::worktree::tests::parse_bare_worktree_excluded ... ok
  test audit::worktree::tests::parse_detached_head ... ok
  test audit::worktree::tests::parse_multiple_worktrees ... ok
  test audit::worktree::tests::parse_single_worktree ... ok
  test audit::worktree::tests::worktree_audit_log_path_resolves ... ok
  test audit::reader::reader_tests::read_parses_valid_events ... ok
  test audit::writer::tests::audit_log_path_resolves_correctly ... ok
  test audit::reader::reader_tests::filter_by_entity_type ... ok
  test audit::reader::reader_tests::skips_empty_lines ... ok
  test audit::writer::tests::best_effort_returns_ok_even_on_failure ... ok
  test audit::writer::tests::each_line_is_valid_json ... ok
  test backend_change_repository::tests::get_delegates_to_reader ... ok
  test backend_change_repository::tests::list_complete_filters_correctly ... ok
  test backend_change_repository::tests::list_incomplete_filters_correctly ... ok
  test backend_change_repository::tests::list_returns_all_changes ... ok
  test backend_change_repository::tests::resolve_target_ambiguous ... ok
  test backend_change_repository::tests::resolve_target_exact_match ... ok
  test audit::writer::tests::appends_events_to_existing_file ... ok
  test backend_change_repository::tests::resolve_target_prefix_match ... ok
  test backend_client::tests::custom_backup_dir_is_used ... ok
  test backend_change_repository::tests::resolve_target_not_found ... ok
  test backend_client::tests::default_backup_dir_uses_home ... ok
  test backend_client::tests::disabled_backend_returns_none ... ok
  test audit::writer::tests::creates_directory_and_file_on_first_write ... ok
  test audit::writer::tests::events_deserialize_back_correctly ... ok
  test backend_client::tests::enabled_backend_with_env_var_token_resolves ... ok
  test backend_client::tests::enabled_backend_missing_token_fails ... ok
  test backend_client::tests::enabled_backend_empty_token_fails ... ok
  test backend_client::tests::enabled_backend_with_explicit_token_resolves ... ok
  test backend_client::tests::env_var_token_takes_precedence_over_config_token ... ok
  test backend_client::tests::idempotency_key_includes_operation ... ok
  test backend_client::tests::is_retriable_status_checks ... ok
  test backend_client::tests::project_api_prefix_formats_correctly ... ok
  test backend_client::tests::project_namespace_empty_string_falls_through_to_env ... ok
  test backend_client::tests::project_namespace_env_takes_precedence_over_config ... ok
  test backend_client::tests::project_namespace_from_config ... ok
  test backend_client::tests::project_namespace_from_env_vars ... ok
  test backend_client::tests::project_namespace_missing_org_fails ... ok
  test backend_client::tests::project_namespace_missing_repo_fails ... ok
  test backend_coordination::tests::allocate_no_work ... ok
  test backend_coordination::tests::allocate_with_work ... ok
  test audit::writer::tests::preserves_existing_content ... ok
  test backend_coordination::tests::claim_success ... ok
  test backend_coordination::tests::claim_conflict ... ok
  test backend_coordination::tests::is_backend_unavailable_detects_process_error ... ok
  test backend_coordination::tests::release_success ... ok
  test backend_health::tests::backend_health_status_default_is_all_false ... ok
  test backend_health::tests::backend_health_status_serializes_error_state ... ok
  test backend_health::tests::backend_health_status_serializes_to_json ... ok
  test backend_http::backend_http_tests::archived_task_fallback_only_treats_not_found_as_missing ... ok
  test backend_http::backend_http_tests::audit_ingest_posts_can_opt_into_retries ... ok
  test backend_http::backend_http_tests::get_requests_are_retried_by_default ... ok
  test backend_http::backend_http_tests::optional_task_text_body_serializes_payload_when_present ... ok
  test backend_http::backend_http_tests::optional_task_text_body_uses_empty_object_when_absent ... ok
  test backend_http::backend_http_tests::post_requests_are_not_retried_by_default ... ok
  test backend_http::backend_http_tests::parse_timestamp_returns_error_for_invalid_rfc3339 ... ok
  test backend_sync::tests::backend_error_mapping_produces_correct_error_types ... ok
  test backend_sync::tests::path_traversal_in_change_id_rejected ... ok
  test backend_sync::tests::path_traversal_in_capability_rejected ... ok
  test backend_sync::tests::pull_creates_backup ... ok
  test backend_sync::tests::pull_writes_artifacts_locally ... ok
  test backend_sync::tests::push_missing_change_dir_fails ... ok
  test backend_coordination::tests::archive_with_backend_skip_specs ... ok
  test backend_coordination::tests::archive_with_backend_backend_unavailable ... ok
  test backend_coordination::tests::archive_with_backend_happy_path ... ok
  test backend_sync::tests::push_conflict_returns_actionable_error ... ok
  test backend_task_repository::tests::checkbox_tasks_parsed_correctly ... ok
  test backend_task_repository::tests::get_task_counts_from_backend ... ok
  test backend_task_repository::tests::has_tasks_empty_content ... ok
  test backend_task_repository::tests::has_tasks_detects_content ... ok
  test backend_task_repository::tests::missing_tasks_returns_empty ... ok
  test change_repository::tests::resolve_target_includes_archive_when_requested ... ok
  test backend_sync::tests::read_local_bundle_sorts_specs ... ok
  test change_repository::tests::exists_and_get_work ... ok
  test change_repository::tests::list_skips_archive_dir ... ok
  test config::tests::is_valid_integration_mode_checks_correctly ... ok
  test config::tests::is_valid_repository_mode_checks_correctly ... ok
  test config::tests::is_valid_worktree_strategy_checks_correctly ... ok
  test backend_sync::tests::push_sends_local_bundle ... ok
  test config::tests::resolve_worktree_template_defaults_uses_defaults_when_missing ... ok
  test config::tests::skill_id_resolves_returns_false_when_no_paths_exist ... ok
  test config::tests::validate_config_value_accepts_archive_main_integration_mode ... ok
  test config::tests::resolve_worktree_template_defaults_reads_overrides ... ok
  test config::tests::validate_config_value_accepts_positive_sync_interval ... ok
  test config::tests::validate_config_value_accepts_unknown_keys ... ok
  test config::tests::validate_config_value_accepts_valid_audit_mirror_branch_name ... ok
  test config::tests::validate_config_value_accepts_valid_coordination_branch_name ... ok
  test config::tests::validate_config_value_accepts_valid_integration_mode ... ok
  test config::tests::validate_config_value_accepts_valid_memory_kind ... ok
  test config::tests::validate_config_value_accepts_valid_repository_mode ... ok
  test config::tests::validate_config_value_accepts_valid_strategy ... ok
  test config::tests::validate_config_value_rejects_empty_memory_command_template ... ok
  test config::tests::validate_config_value_rejects_empty_memory_skill_id ... ok
  test config::tests::validate_config_value_rejects_invalid_archive_main_integration_mode ... ok
  test config::tests::validate_config_value_rejects_invalid_audit_mirror_branch_name ... ok
  test config::tests::validate_config_value_rejects_invalid_coordination_branch_name ... ok
  test config::tests::validate_config_value_rejects_invalid_integration_mode ... ok
  test config::tests::validate_config_value_rejects_invalid_repository_mode ... ok
  test config::tests::validate_config_value_rejects_invalid_strategy ... ok
  test config::tests::validate_config_value_rejects_lock_suffix_in_path_segment ... ok
  test change_repository::tests::resolve_target_reports_ambiguity ... ok
  test config::tests::validate_config_value_rejects_memory_op_missing_required_field ... ok
  test config::tests::validate_config_value_rejects_memory_op_unknown_kind ... ok
  test config::tests::validate_config_value_rejects_non_string_strategy ... ok
  test config::tests::validate_config_value_rejects_unknown_memory_kind ... ok
  test config::tests::validate_config_value_rejects_unknown_memory_op_key ... ok
  test config::tests::validate_config_value_rejects_zero_sync_interval ... ok
  test config::tests::validate_memory_config_passes_when_no_skill_provider ... ok
  test change_repository::tests::resolve_target_module_scoped_query ... ok
  test config::tests::validate_memory_config_rejects_missing_skill ... ok
  test coordination::tests::create_dir_link_creates_symlink ... ok
  test coordination::tests::format_message_broken_symlinks_contains_paths_and_hint ... ok
  test coordination::tests::format_message_embedded_is_none ... ok
  test coordination::tests::format_message_healthy_is_none ... ok
  test coordination::tests::format_message_not_wired_contains_dir_and_hint ... ok
  test coordination::tests::format_message_worktree_missing_contains_path_and_hint ... ok
  test coordination::tests::create_dir_link_fails_when_dst_exists ... ok
  test coordination::tests::format_message_wrong_target_contains_paths_and_hint ... ok
  test config::tests::validate_memory_config_passes_when_skill_resolves_in_flat_layout ... ok
  test config::tests::validate_memory_config_passes_when_skill_resolves_in_grouped_layout ... ok
  test coordination::tests::gitignore_created_when_absent ... ok
  test coordination::tests::gitignore_entries_added_when_missing ... ok
  test coordination::tests::gitignore_no_duplicates_on_second_call ... ok
  test coordination::tests::gitignore_preserves_existing_content ... ok
  test coordination::tests::gitignore_skips_already_present_entries ... ok
  test coordination::tests::health_embedded_returns_embedded ... ok
  test coordination::tests::health_missing_link_is_not_wired ... ok
  test change_repository::tests::suggest_targets_prioritizes_slug_matches ... ok
  test coordination::tests::health_worktree_missing_when_dir_absent ... ok
  test coordination::tests::health_broken_symlinks_when_target_missing ... ok
  test coordination::tests::health_not_wired_when_real_dirs_present ... ok
  test coordination::tests::health_healthy_when_all_symlinks_correct ... ok
  test coordination::tests::remove_is_noop_when_dirs_absent ... ok
  test coordination::tests::remove_is_noop_for_real_dirs ... ok
  test coordination::tests::health_wrong_target_when_symlink_points_elsewhere ... ok
  test coordination::tests::wire_creates_symlinks_for_all_dirs ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged ... ok
  test coordination::tests::wire_handles_empty_real_dir ... ok
  test coordination::tests::wire_is_idempotent ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails ... ok
  test coordination::tests::remove_restores_real_dirs_with_content ... ok
  test coordination::tests::wire_migrates_real_dir_content ... ok
  test coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded ... ok
  test coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch ... ok
  test coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails ... ok
  test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails ... ok
  test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails ... ok
  test coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target ... ok
  test create::create_sub_module_tests::create_sub_module_accepts_full_module_folder_name ... ok
  test create::create_sub_module_tests::create_sub_module_allocates_sequential_numbers ... ok
  test create::create_sub_module_tests::create_sub_module_creates_directory_and_module_md ... ok
  test create::create_sub_module_tests::create_sub_module_errors_on_duplicate_name ... ok
  test create::create_sub_module_tests::create_sub_module_errors_on_unknown_parent_module ... ok
  test create::create_sub_module_tests::create_sub_module_rejects_invalid_name ... ok
  test create::create_sub_module_tests::create_sub_module_with_description_writes_purpose ... ok
  test distribution::tests::ensure_manifest_script_is_executable_only_adds_execute_bits ... ok
  test distribution::tests::pi_adapter_asset_exists_in_embedded_templates ... ok
  test distribution::tests::pi_agent_templates_discoverable ... ok
  test distribution::tests::pi_manifests_commands_match_opencode_commands ... ok
  test distribution::tests::pi_manifests_includes_adapter_skills_and_commands ... ok
  test distribution::tests::pi_manifests_skills_match_opencode_skills ... ok
  test errors::tests::core_error_helpers_construct_expected_variants ... ok
  test event_forwarder::tests::checkpoint_missing_returns_zero ... ok
  test event_forwarder::tests::checkpoint_roundtrip ... ok
  test audit::reconcile::tests::reconcile_no_drift ... ok
  test audit::reconcile::tests::reconcile_missing_tasks_file ... ok
  test audit::reconcile::tests::reconcile_detects_drift ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist ... ok
  test event_forwarder::tests::forward_no_events_returns_zero ... ok
  test audit::worktree::tests::aggregate_worktree_with_events ... ok
  test event_forwarder::tests::forward_result_equality ... ok
  test audit::stream::tests::poll_detects_new_events ... ok
  test audit::stream::tests::poll_returns_empty_when_no_new_events ... ok
  test event_forwarder::tests::forward_reports_duplicates ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists ... ok
  test event_forwarder::tests::is_retriable_backend_error_checks ... ok
  test front_matter::tests::body_sha256_is_deterministic ... ok
  test front_matter::tests::created_at_dt_returns_none_for_invalid_timestamp ... ok
  test front_matter::tests::created_at_dt_returns_none_when_absent ... ok
  test front_matter::tests::format_timestamp_produces_rfc3339 ... ok
  test front_matter::tests::parse_delimiter_with_extra_text_on_first_line ... ok
  test front_matter::tests::parse_empty_front_matter ... ok
  test front_matter::tests::parse_invalid_yaml ... ok
  test front_matter::tests::parse_no_closing_delimiter ... ok
  test front_matter::tests::parse_no_front_matter ... ok
  test front_matter::tests::parse_preserves_extra_fields ... ok
  test front_matter::tests::parse_valid_front_matter ... ok
  test front_matter::tests::parse_with_integrity ... ok
  test front_matter::tests::roundtrip_write_parse ... ok
  test front_matter::tests::touch_creates_new_front_matter ... ok
  test front_matter::tests::touch_updates_existing ... ok
  test front_matter::tests::update_integrity_sets_checksum ... ok
  test front_matter::tests::validate_id_fails_on_mismatch ... ok
  test front_matter::tests::validate_id_passes_when_absent ... ok
  test front_matter::tests::validate_id_passes_when_matching ... ok
  test front_matter::tests::validate_integrity_fails_on_mismatch ... ok
  test front_matter::tests::validate_integrity_passes_when_matching ... ok
  test front_matter::tests::validate_integrity_passes_when_no_checksum ... ok
  test front_matter::tests::write_no_front_matter_returns_body ... ok
  test fs_project_store::tests::change_repository_returns_box_trait ... ok
  test fs_project_store::tests::ensure_project_creates_directory ... ok
  test fs_project_store::tests::ito_path_rejects_path_traversal ... ok
  test fs_project_store::tests::ito_path_resolves_correctly ... ok
  test fs_project_store::tests::module_repository_returns_box_trait ... ok
  test fs_project_store::tests::project_exists_returns_false_for_missing ... ok
  test fs_project_store::tests::store_is_send_sync ... ok
  test fs_project_store::tests::task_repository_returns_box_trait ... ok
  test git::tests::fetch_coordination_branch_classifies_missing_remote_branch ... ok
  test git::tests::fetch_coordination_branch_classifies_missing_remote_configuration ... ok
  test git::tests::fetch_coordination_branch_succeeds_on_clean_fetch ... ok
  test git::tests::push_coordination_branch_classifies_missing_remote_configuration ... ok
  test git::tests::push_coordination_branch_classifies_non_fast_forward_rejection ... ok
  test git::tests::push_coordination_branch_classifies_protection_rejection ... ok
  test event_forwarder::tests::forward_persists_checkpoint_per_batch ... ok
  test git::tests::setup_coordination_branch_creates_branch_when_remote_missing ... ok
  test git::tests::setup_coordination_branch_fails_when_not_git_worktree ... ok
  test git::tests::setup_coordination_branch_reports_missing_origin_when_create_push_fails ... ok
  test git::tests::setup_coordination_branch_returns_ready_when_remote_branch_exists ... ok
  test git_remote::tests::falls_back_to_remote_when_config_empty ... ok
  test git_remote::tests::falls_back_to_remote_when_config_org_missing ... ok
  test git_remote::tests::falls_back_to_remote_when_config_repo_missing ... ok
  test git_remote::tests::ignores_empty_config_strings_and_falls_back_to_remote ... ok
  test git_remote::tests::reexport_delegates_to_common_parser ... ok
  test git_remote::tests::returns_config_values_when_both_set ... ok
  test git_remote::tests::returns_none_when_remote_command_fails ... ok
  test git_remote::tests::returns_none_when_remote_output_is_empty ... ok
  test git_remote::tests::returns_none_when_remote_url_unrecognised ... ok
  test grep::tests::collect_change_artifact_files_finds_all_md_files ... ok
  test grep::tests::search_files_finds_matching_lines ... ok
  test grep::tests::search_files_includes_correct_line_numbers ... ok
  test grep::tests::search_files_rejects_invalid_regex ... ok
  test grep::tests::search_files_respects_limit ... ok
  test grep::tests::search_files_returns_empty_for_no_matches ... ok
  test harness::claude_code::tests::binary_is_claude ... ok
  test harness::claude_code::tests::build_args_with_allow_all ... ok
  test harness::claude_code::tests::build_args_without_allow_all ... ok
  test harness::claude_code::tests::build_args_without_model ... ok
  test harness::claude_code::tests::harness_name_is_claude ... ok
  test harness::codex::tests::binary_is_codex ... ok
  test harness::codex::tests::build_args_with_allow_all ... ok
  test harness::codex::tests::build_args_without_allow_all ... ok
  test harness::codex::tests::harness_name_is_codex ... ok
  test harness::github_copilot::tests::binary_is_copilot ... ok
  test harness::github_copilot::tests::build_args_with_allow_all ... ok
  test harness::github_copilot::tests::build_args_without_allow_all ... ok
  test harness::github_copilot::tests::harness_name_is_github_copilot ... ok
  test harness::opencode::tests::binary_is_opencode ... ok
  test harness::opencode::tests::build_args_with_model ... ok
  test harness::opencode::tests::build_args_without_model ... ok
  test harness::opencode::tests::harness_name_is_opencode ... ok
  test harness::stub::tests::from_env_or_default_with_explicit_path ... ok
  test harness::stub::tests::name_returns_stub ... ok
  test harness::stub::tests::run_sets_nonzero_duration ... ok
  test harness::stub::tests::run_sets_timed_out_false ... ok
  test harness::stub::tests::streams_output_returns_false ... ok
  test harness::types::tests::as_str_all_variants ... ok
  test harness::types::tests::display_matches_as_str ... ok
  test harness::types::tests::from_str_invalid_returns_error ... ok
  test harness::types::tests::from_str_valid_variants ... ok
  test harness::types::tests::harness_help_matches_user_facing ... ok
  test harness::types::tests::is_not_retriable_for_normal_codes ... ok
  test harness::types::tests::is_retriable_for_all_retriable_codes ... ok
  test harness::types::tests::parse_error_display ... ok
  test installers::json_tests::classify_project_file_ownership_handles_user_owned_paths ... ok
  test installers::json_tests::merge_json_objects_appends_and_deduplicates_array_entries ... ok
  test installers::json_tests::merge_json_objects_keeps_existing_and_adds_template_keys ... ok
  test installers::json_tests::write_claude_settings_merges_existing_file_on_update ... ok
  test installers::json_tests::write_claude_settings_preserves_invalid_json_on_update ... ok
  test installers::markers::tests::errors_when_only_one_marker_found ... ok
  test installers::markers::tests::idempotent_when_applying_same_content_twice ... ok
  test installers::markers::tests::inserts_block_when_missing ... ok
  test installers::markers::tests::marker_must_be_on_own_line ... ok
  test installers::markers::tests::replaces_existing_block_preserving_unmanaged_content ... ok
  test installers::markers::tests::updates_file_on_disk ... ok
  test installers::tests::gitignore_audit_session_added ... ok
  test installers::tests::gitignore_both_session_entries ... ok
  test installers::tests::gitignore_created_when_missing ... ok
  test event_forwarder::tests::forward_retries_transient_failure ... ok
  test installers::tests::gitignore_exact_line_matching_trims_whitespace ... ok
  test installers::tests::gitignore_does_not_duplicate_on_repeated_calls ... ok
  test installers::tests::gitignore_full_audit_setup ... ok
  test installers::tests::gitignore_ignores_local_configs ... ok
  test installers::tests::gitignore_legacy_audit_events_unignore_noop_when_absent ... ok
  test installers::tests::gitignore_legacy_audit_events_unignore_removed ... ok
  test installers::tests::gitignore_noop_when_already_present ... ok
  test installers::tests::release_tag_is_prefixed_with_v ... ok
  test installers::tests::should_install_project_rel_filters_by_tool_id ... ok
  test installers::tests::should_install_project_rel_filters_pi ... ok
  test installers::tests::gitignore_preserves_existing_content_and_adds_newline_if_missing ... ok
  test installers::tests::update_model_in_yaml_replaces_or_inserts ... ok
  test installers::tests::update_agent_model_field_updates_frontmatter_when_present ... ok
  test installers::tests::write_one_marker_managed_files_error_when_markers_missing_in_update_mode ... ok
  test installers::tests::write_one_marker_managed_files_refuse_overwrite_without_markers ... ok
  test installers::tests::write_one_marker_managed_files_update_existing_markers ... ok
  test installers::tests::write_one_non_marker_files_skip_on_init_update_mode ... ok
  test installers::tests::write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode ... ok
  test list::tests::counts_requirements_from_headings ... ok
  test installers::tests::write_one_non_marker_user_owned_files_preserve_on_update_mode ... ok
  test list::tests::iso_millis_matches_expected_shape ... ok
  test list::tests::list_changes_sorts_by_name_and_recent ... ok
  test list::tests::parse_modular_change_module_id_allows_overflow_change_numbers ... ok
  test memory::rendering_tests::capture_command_empty_lists_render_as_empty_strings ... ok
  test memory::rendering_tests::capture_command_expands_files_as_repeated_flags ... ok
  test memory::rendering_tests::capture_command_expands_folders_with_explicit_flag_name ... ok
  test memory::rendering_tests::capture_command_preserves_unknown_placeholders_literally ... ok
  test memory::rendering_tests::capture_command_quotes_shell_metacharacters ... ok
  test memory::rendering_tests::capture_command_substitutes_context_with_quoting ... ok
  test memory::rendering_tests::capture_command_substitutes_missing_context_with_empty_quoted_string ... ok
  test memory::rendering_tests::capture_not_configured_when_memory_section_absent ... ok
  test memory::rendering_tests::capture_not_configured_when_only_search_is_set ... ok
  test memory::rendering_tests::capture_skill_emits_structured_inputs_and_options ... ok
  test memory::rendering_tests::mixed_shapes_render_independently ... ok
  test memory::rendering_tests::query_command_substitutes_query ... ok
  test memory::rendering_tests::search_command_renders_scope_as_empty_quoted_token_when_absent ... ok
  test memory::rendering_tests::search_command_renders_scope_as_quoted_value ... ok
  test memory::rendering_tests::search_command_substitutes_query_and_default_limit ... ok
  test memory::rendering_tests::search_command_uses_supplied_limit_when_present ... ok
  test memory::rendering_tests::search_not_configured_when_only_capture_is_set ... ok
  test memory::rendering_tests::search_skill_includes_default_limit_in_structured_inputs ... ok
  test memory::rendering_tests::shell_quote_escapes_embedded_single_quotes ... ok
  test memory::rendering_tests::shell_quote_handles_empty_string ... ok
  test memory::rendering_tests::shell_quote_preserves_unicode_bytes ... ok
  test memory::rendering_tests::shell_quote_wraps_simple_strings_in_single_quotes ... ok
  test module_repository::tests::regression_change_repository_populates_sub_module_id ... ok
  test list::tests::list_changes_filters_by_progress_status ... ok
  test module_repository::tests::test_exists ... ok
  test module_repository::tests::test_get ... ok
  test module_repository::tests::regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes ... ok
  test module_repository::tests::test_get_not_found ... ok
  test module_repository::tests::test_get_uses_full_name_input ... ok
  test module_repository::tests::test_list ... ok
  test orchestrate::gates::tests::remediation_includes_failed_gate_and_downstream_run_gates ... ok
  test orchestrate::gates::tests::remediation_includes_failed_gate_even_when_policy_is_skip ... ok
  test orchestrate::gates::tests::remediation_returns_empty_when_failed_gate_not_found ... ok
  test orchestrate::gates::tests::remediation_skips_downstream_skip_gates ... ok
  test module_repository::tests::test_list_with_change_counts ... ok
  test git::tests::setup_coordination_branch_core_wraps_process_error ... ok
  test process::tests::missing_executable_is_spawn_failure ... ok
  test process::tests::rejects_current_dir_with_parent_component ... ok
  test process::tests::rejects_empty_program ... ok
  test process::tests::rejects_excessive_argument_bytes ... ok
  test process::tests::rejects_nul_in_argument ... ok
  test process::tests::rejects_nul_in_program ... ok
  test process::tests::rejects_relative_program_with_components ... ok
  test process::tests::run_returns_invalid_request_before_spawn ... ok
  test ralph::duration::tests::test_format_duration ... ok
  test ralph::duration::tests::test_parse_bare_number ... ok
  test ralph::duration::tests::test_parse_case_insensitive ... ok
  test ralph::duration::tests::test_parse_combined ... ok
  test ralph::duration::tests::test_parse_errors ... ok
  test ralph::duration::tests::test_parse_hours ... ok
  test ralph::duration::tests::test_parse_minutes ... ok
  test ralph::duration::tests::test_parse_seconds ... ok
  test ralph::duration::tests::test_parse_with_whitespace ... ok
  test ralph::prompt::tests::build_prompt_preamble_includes_completion_promise ... ok
  test ralph::prompt::tests::build_prompt_preamble_includes_context ... ok
  test ralph::prompt::tests::build_prompt_preamble_includes_iteration ... ok
  test ralph::prompt::tests::build_prompt_preamble_includes_validation_failure ... ok
  test ralph::prompt::tests::build_prompt_preamble_omits_context_when_none ... ok
  test ralph::prompt::tests::build_prompt_preamble_omits_validation_when_none ... ok
  test ralph::runner::runner_tests::commit_iteration_errors_on_git_add_failure ... ok
  test ralph::runner::runner_tests::commit_iteration_errors_when_failed_commit_still_has_staged_changes ... ok
  test ralph::runner::runner_tests::commit_iteration_noops_when_no_changes ... ok
  test ralph::runner::runner_tests::commit_iteration_succeeds_when_git_add_and_commit_succeed ... ok
  test ralph::runner::runner_tests::commit_iteration_treats_no_staged_changes_after_failed_commit_as_success ... ok
  test ralph::runner::runner_tests::count_git_changes_counts_non_empty_lines ... ok
  test ralph::runner::runner_tests::count_git_changes_returns_zero_on_git_failure ... ok
  test ralph::runner::runner_tests::filter_eligible ... ok
  test ralph::runner::runner_tests::filter_incomplete ... ok
  test ralph::runner::runner_tests::filter_module_incomplete ... ok
  test ralph::runner::runner_tests::filter_ready ... ok
  test ralph::runner::runner_tests::filter_unprocessed_changes ... ok
  test ralph::runner::runner_tests::finalize_queue_results_errors_with_failed_change_ids ... ok
  test ralph::runner::runner_tests::infer_module_no_hyphen ... ok
  test ralph::runner::runner_tests::infer_module_ok ... ok
  test ralph::runner::runner_tests::now_ms_returns_positive_value ... ok
  test ralph::runner::runner_tests::print_helpers ... ok
  test ralph::runner::runner_tests::promise_empty_stdout ... ok
  test ralph::runner::runner_tests::promise_empty_token ... ok
  test ralph::runner::runner_tests::promise_incomplete ... ok
  test ralph::runner::runner_tests::promise_nested ... ok
  test ralph::runner::runner_tests::promise_no_tags ... ok
  test ralph::runner::runner_tests::promise_second_match ... ok
  test ralph::runner::runner_tests::promise_single_match ... ok
  test ralph::runner::runner_tests::promise_whitespace_trimmed ... ok
  test ralph::runner::runner_tests::render_failure_both ... ok
  test ralph::runner::runner_tests::render_failure_empty ... ok
  test ralph::runner::runner_tests::render_validation_fail_with_output ... ok
  test ralph::runner::runner_tests::render_validation_pass ... ok
  test ralph::runner::runner_tests::render_validation_whitespace_output ... ok
  test ralph::runner::runner_tests::resolve_cwd_no_change_targeted_fallback ... ok
  test ralph::runner::runner_tests::resolve_cwd_no_worktree_found_fallback ... ok
  test ralph::runner::runner_tests::resolve_cwd_worktree_found ... ok
  test ralph::runner::runner_tests::resolve_cwd_worktrees_not_enabled_fallback ... ok
  test ralph::runner::runner_tests::worktree_task_validation_repo_selection ... ok
  test ralph::state::tests::append_context_no_op_on_whitespace ... ok
  test ralph::state::tests::is_safe_change_id_segment_accepts_valid ... ok
  test ralph::state::tests::is_safe_change_id_segment_rejects_backslash ... ok
  test ralph::state::tests::is_safe_change_id_segment_rejects_empty ... ok
  test ralph::state::tests::is_safe_change_id_segment_rejects_too_long ... ok
  test ralph::state::tests::load_context_returns_empty_when_missing ... ok
  test ralph::state::tests::load_state_backfills_missing_new_fields ... ok
  test ralph::state::tests::load_state_returns_none_when_missing ... ok
  test ralph::state::tests::ralph_context_path_correct ... ok
  test ralph::state::tests::ralph_state_dir_uses_safe_fallback_for_invalid_change_ids ... ok
  test ralph::state::tests::ralph_state_json_path_correct ... ok
  test ralph::state::tests::save_and_load_state_round_trip ... ok
  test ralph::validation::tests::discover_commands_falls_back_to_agents_md ... ok
  test ralph::validation::tests::discover_commands_falls_back_to_claude_md ... ok
  test ralph::validation::tests::discover_commands_ito_config_json ... ok
  test ralph::validation::tests::discover_commands_priority_ito_json_first ... ok
  test ralph::validation::tests::discover_commands_returns_empty_when_nothing_configured ... ok
  test ralph::validation::tests::extract_commands_from_json_multiple_paths ... ok
  test ralph::validation::tests::extract_commands_from_markdown_finds_make_check ... ok
  test ralph::validation::tests::extract_commands_from_markdown_finds_make_test ... ok
  test ralph::validation::tests::extract_commands_from_markdown_ignores_other_lines ... ok
  test ralph::validation::tests::normalize_commands_value_array ... ok
  test ralph::validation::tests::normalize_commands_value_non_string ... ok
  test ralph::validation::tests::normalize_commands_value_null ... ok
  test ralph::validation::tests::normalize_commands_value_string ... ok
  test ralph::validation::tests::project_validation_discovers_commands_from_repo_json ... ok
  test process::tests::captures_stdout_and_stderr ... ok
  test process::tests::captures_non_zero_exit ... ok
  test ralph::validation::tests::run_extra_validation_failure ... ok
  test ralph::validation::tests::shell_timeout_is_failure ... ok
  test ralph::validation::tests::task_completion_passes_when_no_tasks ... ok
  test ralph::validation::tests::truncate_for_context_long_truncated ... ok
  test ralph::validation::tests::truncate_for_context_multibyte_utf8 ... ok
  test ralph::validation::tests::truncate_for_context_short_unchanged ... ok
  test ralph::validation::tests::task_completion_fails_when_remaining ... ok
  test sqlite_project_store::repositories::tests::archive_change_rolls_back_when_spec_promotion_fails ... ok
  test audit::reconcile::tests::reconcile_fix_writes_compensating_events ... ok
  test sqlite_project_store::repositories::tests::ensure_project_is_idempotent ... ok
  test sqlite_project_store::repositories::tests::get_change_returns_full_data ... ok
  test sqlite_project_store::repositories::tests::ensure_project_creates_row ... ok
  test sqlite_project_store::repositories::tests::get_missing_change_returns_not_found ... ok
  test sqlite_project_store::repositories::tests::get_module_by_id ... ok
  test sqlite_project_store::repositories::tests::open_in_memory_creates_schema ... ok
  test sqlite_project_store::repositories::tests::store_is_send_sync ... ok
  test sqlite_project_store::repositories::tests::push_artifact_bundle_rolls_back_partial_writes_on_failure ... ok
  test sqlite_project_store::repositories::tests::task_repository_loads_tasks ... ok
  test sqlite_project_store::repositories::tests::task_mutation_service_reports_poisoned_connection_without_panicking ... ok
  test sqlite_project_store::repositories::tests::task_repository_missing_change_returns_empty ... ok
  test sqlite_project_store::repositories::tests::two_projects_are_isolated ... ok
  test sqlite_project_store::repositories::tests::upsert_and_list_changes ... ok
  test sqlite_project_store::repositories::tests::upsert_and_list_modules ... ok
  test task_repository::tests::test_get_task_counts_checkbox_format ... ok
  test sqlite_project_store::repositories::tests::on_disk_database_persists ... ok
  test task_repository::tests::load_tasks_uses_schema_apply_tracks_when_set ... ok
  test task_repository::tests::test_missing_tasks_file_returns_zero ... ok
  test task_repository::tests::test_get_task_counts_enhanced_format ... ok
  test tasks::tests::read_tasks_markdown_rejects_traversal_like_change_id ... ok
  test task_repository::tests::test_has_tasks ... ok
  test tasks::tests::read_tasks_markdown_returns_error_for_missing_file ... ok
  test tasks::tests::read_tasks_markdown_returns_contents_for_existing_file ... ok
  test templates::guidance::tests::strip_ito_internal_comment_blocks_removes_internal_template_guidance ... ok
  test templates::schema_assets::tests::safe_relative_path_validation_blocks_traversal_and_absolute_paths ... ok
  test templates::schema_assets::tests::safe_schema_name_rejects_dot_segments_and_periods ... ok
  test templates::task_parsing::tests::parse_enhanced_tasks_extracts_ids_status_and_done ... ok
  test templates::types::tests::schema_source_as_str_returns_expected_labels ... ok
  test templates::types::tests::validation_yaml_parses_minimal_config ... ok
  test templates::types::tests::validation_yaml_parses_proposal_entry_with_rules ... ok
  test templates::types::tests::validation_yaml_parses_rules_extension_without_breaking_existing_shape ... ok
  test token::tests::generated_token_has_expected_length ... ok
  test token::tests::generated_token_is_url_safe ... ok
  test token::tests::two_tokens_are_distinct ... ok
  test token::tests::url_safe_base64_encode_known_vector ... ok
  test token::tests::url_safe_base64_roundtrip_known_value ... ok
  test validate::issue::tests::constructors_set_expected_fields ... ok
  test validate::issue::tests::format_spec_is_idempotent_for_message_suffix ... ok
  test validate::issue::tests::format_spec_preserves_non_object_metadata ... ok
  test validate::issue::tests::location_helpers_set_line_and_column ... ok
  test validate::issue::tests::metadata_helper_attaches_json_context ... ok
  test validate::issue::tests::rule_id_helper_marks_issue_and_is_reflected_in_metadata ... ok
  test validate::report::tests::extend_collects_multiple_issues ... ok
  test validate::report::tests::finish_non_strict_only_fails_on_errors ... ok
  test validate::report::tests::finish_strict_fails_on_warnings ... ok
  test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
  test tasks::tests::returns_empty_when_no_ready_tasks_exist ... ok
  test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
  test tasks::tests::returns_ready_tasks_for_ready_changes ... ok
  test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok
  test viewer::html::tests::html_viewer_reports_expected_description ... ok
  test viewer::html::tests::html_viewer_reports_expected_name ... ok
  test viewer::tests::concrete_viewers_report_expected_names ... ok
  test viewer::tests::default_registry_includes_html_viewer ... ok
  test viewer::html::tests::html_viewer_open_errors_when_pandoc_missing ... ok
  test viewer::tests::viewer_backend_trait_exposes_required_methods ... ok
  test viewer::tests::viewer_registry_filters_and_finds_available_viewers ... ok
  test viewer::tests::viewer_registry_hides_tmux_when_disabled ... ok
  test viewer::html::tests::html_viewer_availability_depends_on_pandoc ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_creates_worktree_when_absent ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_existing_worktree_returns_path_without_creation ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_git_failure_returns_error ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_worktrees_disabled_returns_cwd ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_accepts_normal_ids ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_empty ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_leading_dash ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_nul ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_separators ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_traversal ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_with_include_files_copies_them ... ok
  test worktree_init::worktree_init_tests::copy_include_files_copies_to_dest ... ok
  test worktree_init::worktree_init_tests::copy_include_files_empty_config_and_no_file ... ok
  test worktree_init::worktree_init_tests::copy_include_files_skips_missing_source ... ok
  test worktree_init::worktree_init_tests::copy_include_files_skips_existing_destination ... ok
  test worktree_init::worktree_init_tests::init_worktree_copies_files_and_runs_setup ... ok
  test worktree_init::worktree_init_tests::init_worktree_no_setup_copies_files_only ... ok
  test worktree_init::worktree_init_tests::init_worktree_setup_failure_returns_error ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_comments_only ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_empty_content ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_strips_comments_and_blanks ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_trims_whitespace ... ok
  test event_forwarder::tests::forward_skips_when_fully_forwarded ... ok
  test worktree_init::worktree_init_tests::init_worktree_preserves_existing_destination_file ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_config_only ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_deduplicates ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_file_only ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_missing_include_file_ok ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_glob_expansion ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_ignores_directories ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_no_match_returns_empty ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_rejects_absolute_path_in_pattern ... ok
  test worktree_init::worktree_init_tests::run_setup_empty_multiple_commands_is_noop ... ok
  test worktree_init::worktree_init_tests::run_setup_empty_single_command_is_noop ... ok
  test worktree_init::worktree_init_tests::run_setup_first_command_fails_stops_sequence ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_rejects_path_traversal ... ok
  test worktree_init::worktree_init_tests::run_setup_multiple_commands_run_in_order ... ok
  test worktree_init::worktree_init_tests::run_setup_no_config_is_noop ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_union_of_config_and_file ... ok
  test worktree_init::worktree_init_tests::run_setup_single_command_invoked ... ok
  test ralph::validation::tests::run_extra_validation_success ... ok
  test event_forwarder::tests::forward_batches_correctly ... ok
  test event_forwarder::tests::forward_respects_checkpoint ... ok
  test viewer::tests::run_with_stdin_closes_pipe_after_write ... ok
  test event_forwarder::tests::forward_sends_all_new_events ... ok
  test event_forwarder::tests::forward_stops_on_permanent_failure ... ok
  test coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree ... ok
  test audit::stream::tests::read_initial_events_returns_last_n ... ok
  test audit::store::tests::legacy_worktree_log_is_removed_after_successful_migration ... ok
  test event_forwarder::tests::forward_reads_events_from_routed_local_store ... ok
  test coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination ... ok
  test audit::store::tests::read_all_merges_and_replays_fallback_events_when_branch_recovers ... ok
  test audit::stream::tests::poll_detects_new_events_from_routed_store ... ok

  test result: ok. 583 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.61s

       Running tests/archive.rs (target/llvm-cov-target/debug/deps/archive-b38c1f4deb635698)

  running 3 tests
  test generate_archive_name_prefixes_with_date ... ok
  test check_task_completion_handles_checkbox_and_enhanced_formats ... ok
  test discover_and_copy_specs_and_archive_change ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/audit_mirror.rs (target/llvm-cov-target/debug/deps/audit_mirror-a5e5640b80e1c911)

  running 6 tests
  test audit_mirror_default_local_store_falls_back_without_creating_worktree_log ... ok
  test audit_mirror_disabled_does_not_create_remote_branch ... ok
  test audit_mirror_failures_do_not_break_local_append ... ok
  test local_store_does_not_fall_back_when_internal_branch_exists_without_log_file ... ok
  test audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log ... ok
  test audit_mirror_enabled_pushes_to_configured_branch ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.78s

       Running tests/audit_storage.rs (target/llvm-cov-target/debug/deps/audit_storage-c6200a1026a5c687)

  running 3 tests
  test reads_events_from_injected_store_without_filesystem_path ... ok
  test memory_store_append_persists_events ... ok
  test filters_events_from_injected_store ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_archive.rs (target/llvm-cov-target/debug/deps/backend_archive-829515393398a15c)

  running 6 tests
  test backend_archive_fails_when_pull_unavailable ... ok
  test backend_archive_with_skip_specs_does_not_copy_specs ... ok
  test backend_archive_happy_path_produces_committable_state ... ok
  test backend_archive_fails_when_backend_unavailable_for_mark_archived ... ok
  test backend_archive_creates_backup_before_overwriting ... ok
  test backend_archive_does_not_mutate_local_module_markdown ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/backend_auth.rs (target/llvm-cov-target/debug/deps/backend_auth-561f90d65155d141)

  running 13 tests
  test resolve_token_seed_falls_back_to_config ... ok
  test resolve_admin_tokens_merges_all_sources ... ok
  test resolve_admin_tokens_deduplicates ... ok
  test resolve_token_seed_cli_takes_precedence ... ok
  test resolve_token_seed_returns_none_when_all_empty ... ok
  test resolve_admin_tokens_skips_empty_config_entries ... ok
  test write_auth_rejects_non_object_backend_server ... ok
  test init_skips_when_tokens_exist ... ok
  test write_auth_sets_restrictive_permissions ... ok
  test init_generates_tokens_when_none_exist ... ok
  test write_auth_creates_config_file ... ok
  test write_auth_rejects_non_object_root ... ok
  test write_auth_preserves_existing_config ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/backend_auth_service.rs (target/llvm-cov-target/debug/deps/backend_auth_service-c1efe4ec10dca47a)

  running 1 test
  test init_rejects_non_object_backend_server ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_client_mode.rs (target/llvm-cov-target/debug/deps/backend_client_mode-13d57a5ffa7d3b7e)

  running 15 tests
  test allocate_no_work_returns_none ... ok
  test allocate_returns_claimed_change ... ok
  test backend_unavailable_detection ... ok
  test config_disabled_returns_none ... ok
  test claim_success_returns_holder_info ... ok
  test config_enabled_with_token_resolves ... ok
  test config_enabled_missing_token_fails_with_clear_message ... ok
  test backend_task_repo_missing_returns_zero ... ok
  test retriable_status_codes ... ok
  test backend_change_repo_lists_and_filters ... ok
  test claim_conflict_returns_holder_error ... ok
  test backend_task_repo_parses_from_content ... ok
  test pull_writes_artifacts_and_revision ... ok
  test push_stale_revision_gives_actionable_error ... ok
  test push_success_updates_local_revision ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_module_repository.rs (target/llvm-cov-target/debug/deps/backend_module_repository-591d67d8e1f852ac)

  running 5 tests
  test backend_module_repository_list_sorts_by_id ... ok
  test backend_module_repository_list_sorts_deterministically ... ok
  test backend_module_repository_accepts_name_inputs ... ok
  test backend_module_repository_normalizes_full_name_inputs ... ok
  test read_module_markdown_falls_back_without_local_file ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_sub_module_support.rs (target/llvm-cov-target/debug/deps/backend_sub_module_support-78e96f73924fef26)

  running 9 tests
  test backend_module_repository_list_sub_modules_for_unknown_module_returns_error ... ok
  test backend_module_repository_list_includes_sub_module_summaries ... ok
  test backend_module_repository_get_sub_module_not_found_returns_error ... ok
  test backend_module_repository_list_sub_modules_returns_sorted_summaries ... ok
  test backend_module_repository_get_sub_module_by_composite_id ... ok
  test sqlite_store_legacy_change_has_no_sub_module_id ... ok
  test sqlite_store_list_changes_filters_by_sub_module_id ... ok
  test sqlite_store_persists_sub_module_id_on_change ... ok
  test sqlite_store_sub_module_change_roundtrips_through_artifact_bundle ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/change_repository_lifecycle.rs (target/llvm-cov-target/debug/deps/change_repository_lifecycle-a52d7554117a574f)

  running 2 tests
  test remote_runtime_ignores_local_change_dirs ... ok
  test filesystem_change_repository_filters_archived ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/change_repository_orchestrate_metadata.rs (target/llvm-cov-target/debug/deps/change_repository_orchestrate_metadata-af764814876d1691)

  running 1 test
  test change_repository_exposes_orchestrate_metadata_from_ito_yaml ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/change_repository_parity.rs (target/llvm-cov-target/debug/deps/change_repository_parity-f2291e8fc4b631d8)

  running 18 tests
  test backend_resolve_empty_input_returns_not_found ... ok
  test backend_list_by_module_normalizes_module_id ... ok
  test backend_resolve_lifecycle_filter_respected ... ok
  test backend_resolve_numeric_short_form_ambiguous ... ok
  test backend_resolve_numeric_short_form_matches_canonical_id ... ok
  test backend_resolve_module_scoped_slug_not_found ... ok
  test backend_resolve_module_scoped_slug_query ... ok
  test sqlite_resolve_prefix_match ... ok
  test sqlite_resolve_numeric_short_form_matches_canonical_id ... ok
  test sqlite_list_archived_filter_returns_empty ... ok
  test sqlite_resolve_empty_input_returns_not_found ... ok
  test sqlite_get_with_archived_filter_returns_not_found ... ok
  test sqlite_resolve_numeric_short_form_ambiguous ... ok
  test sqlite_resolve_archived_filter_returns_not_found ... ok
  test sqlite_get_with_all_filter_finds_change ... ok
  test sqlite_resolve_all_filter_finds_active_changes ... ok
  test sqlite_list_by_module_normalizes_module_id ... ok
  test sqlite_list_all_filter_returns_active_changes ... ok

  test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/change_target_resolution_parity.rs (target/llvm-cov-target/debug/deps/change_target_resolution_parity-d5a749c2bd9a3889)

  running 2 tests
  test sqlite_resolver_honors_archived_lifecycle_like_filesystem ... ok
  test change_target_resolution_matches_across_repository_modes ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/coordination_worktree.rs (target/llvm-cov-target/debug/deps/coordination_worktree-426c5f05aa2b8b0c)

  running 15 tests
  test symlink_tests::change_repo_list_through_symlink ... ok
  test symlink_tests::change_written_through_symlink_lands_in_worktree ... ok
  test symlink_tests::module_repo_get_through_symlink ... ok
  test symlink_tests::task_repo_load_tasks_through_symlink ... ok
  test symlink_tests::task_repo_has_tasks_through_symlink ... ok
  test symlink_tests::task_written_through_symlink_lands_in_worktree ... ok
  test symlink_tests::task_repo_missing_tasks_file_returns_zero_through_symlink ... ok
  test symlink_tests::module_repo_list_through_symlink ... ok
  test symlink_tests::change_repo_exists_through_symlink ... ok
  test symlink_tests::module_repo_exists_through_symlink ... ok
  test symlink_tests::change_repo_get_through_symlink ... ok
  test symlink_tests::module_repo_list_multiple_through_symlink ... ok
  test symlink_tests::all_repos_consistent_through_symlinks ... ok
  test symlink_tests::module_repo_change_counts_through_symlink ... ok
  test symlink_tests::change_repo_list_multiple_through_symlink ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/create.rs (target/llvm-cov-target/debug/deps/create-71f01a56aa0714f6)

  running 15 tests
  test create_change_rejects_uppercase_names ... ok
  test create_change_in_sub_module_rejects_missing_parent_module ... ok
  test create_change_in_sub_module_rejects_missing_sub_module_dir ... ok
  test create_module_creates_directory_and_module_md ... ok
  test create_module_writes_description_to_purpose_section ... ok
  test create_module_returns_existing_module_when_name_matches ... ok
  test create_change_creates_change_dir_and_updates_module_md ... ok
  test create_change_in_sub_module_checklist_is_sorted_ascending ... ok
  test create_change_in_sub_module_writes_checklist_to_sub_module_md ... ok
  test create_change_allocates_next_number_from_existing_change_dirs ... ok
  test create_change_rewrites_module_changes_in_ascending_change_id_order ... ok
  test create_change_in_sub_module_uses_composite_id_format ... ok
  test allocation_state_sub_module_keys_sort_after_parent ... ok
  test create_change_in_sub_module_allocates_independent_sequence ... ok
  test create_change_writes_allocation_modules_in_ascending_id_order ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/distribution.rs (target/llvm-cov-target/debug/deps/distribution-2af4bcde2e3526e9)

  running 11 tests
  test opencode_manifests_includes_plugin_and_skills ... ok
  test codex_manifests_includes_bootstrap_and_skills ... ok
  test github_manifests_includes_skills_and_commands ... ok
  test claude_manifests_includes_hooks_and_skills ... ok
  test install_manifests_writes_files_to_disk ... ok
  test install_manifests_make_tmux_skill_scripts_executable ... ok
  test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
  test install_manifests_renders_worktree_skill_enabled ... ok
  test install_manifests_renders_worktree_skill_with_context ... ok
  test install_manifests_creates_parent_directories ... ok
  test all_manifests_use_embedded_assets ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

       Running tests/event_forwarding.rs (target/llvm-cov-target/debug/deps/event_forwarding-1de14404dd665790)

  running 6 tests
  test forward_result_reports_diagnostics ... ok
  test permanent_failure_stops_forwarding ... ok
  test full_forwarding_workflow ... ok
  test batch_boundaries_preserved ... ok
  test transient_failure_retried_then_succeeds ... ok
  test incremental_forwarding ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/grep_scopes.rs (target/llvm-cov-target/debug/deps/grep_scopes-d2736a57ea797c72)

  running 4 tests
  test grep_scope_change_only_searches_one_change ... ok
  test grep_respects_limit_across_scopes ... ok
  test grep_scope_module_searches_all_changes_in_module ... ok
  test grep_scope_all_searches_all_changes ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/harness_context.rs (target/llvm-cov-target/debug/deps/harness_context-29252af123176354)

  running 6 tests
  test infer_context_from_cwd_infers_change_from_path ... ok
  test infer_context_from_cwd_infers_module_from_ito_modules_path ... ok
  test infer_context_from_cwd_returns_no_target_when_inconclusive ... ok
  test infer_context_from_cwd_prefers_path_over_git_branch ... ok
  test infer_context_from_cwd_infers_module_from_git_branch ... ok
  test infer_context_from_cwd_infers_change_from_git_branch ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

       Running tests/harness_opencode.rs (target/llvm-cov-target/debug/deps/harness_opencode-52d4db34fcbe5ef7)

  running 8 tests
  test claude_harness_errors_when_claude_missing ... ok
  test copilot_harness_errors_when_copilot_missing ... ok
  test codex_harness_errors_when_codex_missing ... ok
  test opencode_harness_errors_when_opencode_missing ... ok
  test codex_harness_passes_model_and_allow_all_flags ... ok
  test claude_harness_passes_model_and_allow_all_flags ... ok
  test opencode_harness_runs_opencode_binary_and_returns_outputs ... ok
  test github_copilot_harness_passes_model_and_allow_all_flags ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.03s

       Running tests/harness_streaming.rs (target/llvm-cov-target/debug/deps/harness_streaming-bd189cd1d4566c17)

  running 2 tests
  test no_timeout_when_process_exits_normally ... ok
  test inactivity_timeout_kills_stalled_process ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.24s

       Running tests/harness_stub.rs (target/llvm-cov-target/debug/deps/harness_stub-5d8c65058563465a)

  running 6 tests
  test stub_harness_default_returns_complete_promise ... ok
  test stub_harness_errors_on_empty_steps ... ok
  test stub_harness_from_env_prefers_env_over_default ... ok
  test stub_harness_errors_on_missing_and_invalid_json ... ok
  test stub_step_defaults_match_json_schema ... ok
  test stub_harness_from_json_path_runs_steps_and_repeats_last ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/import.rs (target/llvm-cov-target/debug/deps/import-656148bd59445b16)

  running 10 tests
  test active_local_change_fails_when_backend_only_has_archived_copy ... ok
  test dry_run_previews_without_importing ... ok
  test skips_already_imported_active_change_when_remote_bundle_matches ... ok
  test pushes_when_remote_active_bundle_differs ... ok
  test rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches ... ok
  test dry_run_uses_preview_logic_without_mutating_backend ... ok
  test archived_directory_with_empty_canonical_change_id_is_ignored ... ok
  test import_summary_records_failures_without_aborting_remaining_changes ... ok
  test ignores_unrecognized_archive_directories_during_discovery ... ok
  test imports_active_and_archived_changes_with_lifecycle_fidelity ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/io.rs (target/llvm-cov-target/debug/deps/io-e9a0f23bbf31aadb)

  running 3 tests
  test read_to_string_or_default_returns_empty_for_missing_file ... ok
  test read_to_string_optional_returns_none_for_missing_file ... ok
  test write_atomic_std_creates_parent_and_replaces_contents ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/orchestrate_run_state.rs (target/llvm-cov-target/debug/deps/orchestrate_run_state-984256ea4b1839e2)

  running 7 tests
  test orchestrate_max_parallel_aliases_resolve ... ok
  test orchestrate_resume_skips_terminal_gates ... ok
  test orchestrate_run_id_generation_matches_expected_format ... ok
  test orchestrate_dependency_cycle_is_rejected ... ok
  test orchestrate_change_state_is_written_and_readable ... ok
  test orchestrate_run_state_creates_expected_layout ... ok
  test orchestrate_event_log_appends_without_truncation ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/planning_init.rs (target/llvm-cov-target/debug/deps/planning_init-95e7fdcbd0b0c280)

  running 3 tests
  test read_planning_status_returns_error_for_missing_roadmap ... ok
  test read_planning_status_returns_contents_for_existing_roadmap ... ok
  test init_planning_structure_writes_files ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/ralph.rs (target/llvm-cov-target/debug/deps/ralph-c596d371a9759d11)

  running 30 tests
  test run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
  test run_ralph_errors_when_max_iterations_is_zero ... ok
  test run_ralph_gives_up_after_max_retriable_retries ... ok
  test run_ralph_continue_ready_errors_when_targeting_change_or_module ... ok
  test run_ralph_add_and_clear_context_paths ... ok
  test run_ralph_continues_after_harness_failure_by_default ... ok
  test run_ralph_opencode_counts_git_changes_when_in_repo ... ignored, Flaky in pre-commit: counts real uncommitted changes instead of test fixture
  test run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes ... ok
  test run_ralph_fails_after_error_threshold ... ok
  test run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight ... ok
  test run_ralph_module_resolves_single_change ... ok
  test run_ralph_retries_retriable_exit_code_with_exit_on_error ... ok
  test run_ralph_returns_error_on_harness_failure ... ok
  test run_ralph_prompt_includes_task_context_and_guidance ... ok
  test run_ralph_non_retriable_exit_still_counts_against_threshold ... ok
  test run_ralph_resets_retriable_counter_on_success ... ok
  test state_helpers_append_and_clear_context ... ok
  test run_ralph_status_path_works_with_no_state ... ok
  test run_ralph_retries_retriable_exit_code_without_counting_against_threshold ... ok
  test run_ralph_skip_validation_exits_immediately ... ok
  test run_ralph_module_multiple_changes_errors_when_non_interactive ... ok
  test run_ralph_continue_ready_reorients_when_repo_state_shifts ... ok
  test run_ralph_continue_ready_processes_all_eligible_changes_across_repo ... ok
  test run_ralph_continue_module_processes_all_ready_changes ... ok
  test run_ralph_continue_ready_accumulates_failures_after_processing_remaining_changes ... ok
  test run_ralph_loop_writes_state_and_honors_min_iterations ... ok
  test run_ralph_completion_promise_trims_whitespace ... ok
  test run_ralph_continues_when_completion_validation_fails ... ok
  test run_ralph_worktree_disabled_uses_fallback_cwd ... ok
  test run_ralph_worktree_enabled_state_written_to_effective_ito ... ok

  test result: ok. 29 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.16s

       Running tests/repo_index.rs (target/llvm-cov-target/debug/deps/repo_index-60d9a805f1ceeebc)

  running 1 test
  test repo_index_loads_and_excludes_archive_change_dir ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/repo_integrity.rs (target/llvm-cov-target/debug/deps/repo_integrity-77040c870b0aca94)

  running 3 tests
  test invalid_change_dir_names_are_reported ... ok
  test change_referring_to_missing_module_is_an_error ... ok
  test duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/repo_paths.rs (target/llvm-cov-target/debug/deps/repo_paths-023ec88aea2eaf45)

  running 11 tests
  test coordination_worktree_path_uses_explicit_worktree_path_when_set ... ok
  test coordination_worktree_path_correct_structure_with_home_fallback ... ok
  test coordination_worktree_path_correct_structure_with_xdg ... ok
  test coordination_worktree_path_falls_back_to_local_share_when_xdg_unset ... ok
  test coordination_worktree_path_ignores_xdg_when_explicit_path_set ... ok
  test coordination_worktree_path_last_resort_uses_ito_path ... ok
  test coordination_worktree_path_uses_xdg_data_home_when_set ... ok
  test resolve_worktree_paths_respects_bare_control_siblings_strategy ... ok
  Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpPjZAcY/
  test resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir ... ok
  test resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable ... ok
  Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmp8LUBzW/.git/
  test resolve_env_from_cwd_prefers_git_toplevel ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

       Running tests/repository_runtime.rs (target/llvm-cov-target/debug/deps/repository_runtime-e742bd02b25af530)

  running 6 tests
  test remote_runtime_uses_remote_factory ... ok
  test sqlite_mode_requires_db_path ... ok
  test filesystem_runtime_builds_repository_set ... ok
  test sqlite_runtime_builds_repository_set ... ok
  test repository_modes_return_consistent_change_names ... ok
  test resolve_target_parity_between_filesystem_and_sqlite ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/repository_runtime_config_validation.rs (target/llvm-cov-target/debug/deps/repository_runtime_config_validation-1e4aecf2a2f25935)

  running 1 test
  test invalid_repository_mode_fails_fast ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/show.rs (target/llvm-cov-target/debug/deps/show-eefa18a76b1b93cd)

  running 24 tests
  test parse_requirement_block_extracts_requirement_id ... ok
  test parse_contract_refs_splits_unknown_schemes_at_length_threshold ... ok
  test parse_contract_refs_preserves_lowercase_colon_text_inside_identifiers ... ok
  test parse_change_show_json_emits_deltas_with_operations ... ok
  test parse_contract_refs_preserves_commas_inside_identifiers ... ok
  test parse_delta_spec_requirement_id_is_extracted ... ok
  test parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions ... ok
  test parse_contract_refs_splits_lowercase_unknown_scheme_after_known_ref ... ok
  test parse_contract_refs_splits_unknown_scheme_after_known_ref ... ok
  test parse_contract_refs_accepts_comma_without_space_before_known_scheme ... ok
  test parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier ... ok
  test parse_requirement_block_multiple_requirements_with_ids ... ok
  test parse_requirement_block_requirement_id_absent_gives_none ... ok
  test parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets ... ok
  test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
  test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
  test read_module_markdown_returns_error_for_nonexistent_module ... ok
  test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
  test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
  test read_module_markdown_returns_empty_for_missing_module_md ... ok
  test read_module_markdown_returns_contents_for_existing_module ... ok
  test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
  test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
  test read_change_delta_spec_files_lists_specs_sorted ... ok

  test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/spec_repository_backends.rs (target/llvm-cov-target/debug/deps/spec_repository_backends-029e0961db94d6ac)

  running 2 tests
  test remote_runtime_exposes_spec_repository_without_local_specs ... ok
  test filesystem_runtime_exposes_promoted_specs ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/spec_show_repository.rs (target/llvm-cov-target/debug/deps/spec_show_repository-fa6ee4faf3a89cb9)

  running 3 tests
  test read_spec_markdown_from_repository_reads_remote_spec ... ok
  test bundle_specs_markdown_from_repository_adds_metadata_comments ... ok
  test bundle_specs_show_json_from_repository_sorts_ids ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/sqlite_archive_mirror.rs (target/llvm-cov-target/debug/deps/sqlite_archive_mirror-e5b15a45f892a8fd)

  running 1 test
  test sqlite_archive_promotes_specs_and_marks_change_archived ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/sqlite_task_mutations.rs (target/llvm-cov-target/debug/deps/sqlite_task_mutations-e7d35e5b0b789f0c)

  running 3 tests
  test sqlite_task_mutation_service_returns_not_found_for_missing_tasks ... ok
  test sqlite_task_mutation_service_updates_existing_markdown ... ok
  test sqlite_task_mutation_service_initializes_missing_tasks ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/stats.rs (target/llvm-cov-target/debug/deps/stats-1885822ec97b756b)

  running 2 tests
  test compute_command_stats_counts_command_end_events ... ok
  test collect_jsonl_files_finds_nested_jsonl_files ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/task_repository_summary.rs (target/llvm-cov-target/debug/deps/task_repository_summary-1ad0abd069c66f3d)

  running 1 test
  test repository_status_builds_summary_and_next_task ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks_api.rs (target/llvm-cov-target/debug/deps/tasks_api-3e8b9aa541ccc5f8)

  running 15 tests
  test list_ready_tasks_across_changes_handles_empty_repo ... ok
  test init_tasks_returns_true_when_file_already_exists ... ok
  test init_tasks_creates_file_when_missing ... ok
  test tasks_api_rejects_non_tasks_tracking_validator_for_schema_tracking ... ok
  test shelve_task_rejects_shelving_complete_task ... ok
  test complete_task_accepts_note_parameter ... ok
  test get_next_task_returns_none_when_all_tasks_complete ... ok
  test get_next_task_returns_first_ready_task_for_enhanced_format ... ok
  test start_task_rejects_starting_shelved_task_directly ... ok
  test add_task_appends_new_task_with_next_id ... ok
  test shelve_and_unshelve_task_round_trip_for_enhanced_format ... ok
  test shelve_task_accepts_reason_parameter ... ok
  test add_task_creates_wave_if_not_exists ... ok
  test tasks_api_operates_on_schema_apply_tracks_file ... ok
  test start_and_complete_task_enforced_by_dependencies_for_enhanced_format ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/tasks_checkbox_format.rs (target/llvm-cov-target/debug/deps/tasks_checkbox_format-309c86100c9d4bcc)

  running 3 tests
  test checkbox_tasks_do_not_support_shelving ... ok
  test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids ... ok
  test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks_orchestration.rs (target/llvm-cov-target/debug/deps/tasks_orchestration-749fac244fa887ac)

  running 26 tests
  test init_tasks_rejects_invalid_change_id ... ok
  test get_task_status_returns_error_when_file_missing ... ok
  test get_next_task_returns_current_in_progress_for_checkbox ... ok
  test init_tasks_creates_file_when_missing ... ok
  test add_task_rejects_checkbox_format ... ok
  test init_tasks_does_not_overwrite_existing_file ... ok
  test get_next_task_returns_none_when_all_complete ... ok
  test shelve_task_rejects_checkbox_format ... ok
  test complete_task_handles_checkbox_format ... ok
  test start_task_rejects_already_complete ... ok
  test complete_task_errors_with_parse_errors ... ok
  test shelve_task_errors_with_parse_errors ... ok
  test get_task_status_returns_diagnostics_for_malformed_file ... ok
  test add_task_assigns_next_id_in_wave ... ok
  test shelve_task_rejects_complete_task ... ok
  test add_task_errors_with_parse_errors ... ok
  test complete_task_handles_enhanced_format ... ok
  test get_next_task_returns_first_ready_for_enhanced ... ok
  test unshelve_task_rejects_not_shelved ... ok
  test add_task_defaults_to_wave_1 ... ok
  test add_task_creates_wave_when_missing ... ok
  test unshelve_task_transitions_to_pending ... ok
  test start_task_rejects_shelved_task ... ok
  test unshelve_task_errors_with_parse_errors ... ok
  test start_task_errors_with_parse_errors ... ok
  test start_task_validates_task_is_ready ... ok

  test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/templates_apply_instructions.rs (target/llvm-cov-target/debug/deps/templates_apply_instructions-3c66ac3bab0e8fd7)

  running 1 test
  test compute_apply_instructions_reports_blocked_states_and_progress ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/templates_change_status.rs (target/llvm-cov-target/debug/deps/templates_change_status-f9000e001917dd23)

  running 2 tests
  test compute_change_status_rejects_invalid_change_name ... ok
  test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/templates_review_context.rs (target/llvm-cov-target/debug/deps/templates_review_context-9438f07dcc26e15c)

  running 1 test
  test compute_review_context_collects_artifacts_validation_tasks_and_specs ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/templates_schema_resolution.rs (target/llvm-cov-target/debug/deps/templates_schema_resolution-f161856639322ac6)

  running 9 tests
  test resolve_schema_rejects_path_traversal_name ... ok
  test resolve_schema_rejects_absolute_and_backslash_names ... ok
  test resolve_schema_uses_embedded_when_no_overrides_exist ... ok
  test resolve_instructions_exposes_enhanced_spec_driven_templates ... ok
  test resolve_instructions_reads_embedded_templates ... ok
  test resolve_templates_rejects_traversal_template_path ... ok
  test resolve_instructions_rejects_traversal_template_path ... ok
  test resolve_schema_prefers_project_over_user_override ... ok
  test export_embedded_schemas_writes_then_skips_without_force ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/templates_schemas_listing.rs (target/llvm-cov-target/debug/deps/templates_schemas_listing-3363a73ea3341e78)

  running 9 tests
  test list_schemas_detail_spec_driven_has_expected_artifacts ... ok
  test list_schemas_detail_all_sources_are_embedded ... ok
  test list_schemas_detail_json_round_trips ... ok
  test built_in_minimalist_and_event_driven_spec_templates_use_delta_shape ... ok
  test list_schemas_detail_is_sorted ... ok
  test list_schemas_detail_returns_all_embedded_schemas ... ok
  test list_schemas_detail_entries_have_descriptions ... ok
  test list_schemas_detail_recommended_default_is_spec_driven ... ok
  test list_schemas_detail_entries_have_artifacts ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/templates_user_guidance.rs (target/llvm-cov-target/debug/deps/templates_user_guidance-ade63fd81f0ca713)

  running 7 tests
  test load_user_guidance_for_artifact_rejects_path_traversal_ids ... ok
  test load_user_guidance_strips_managed_header_block ... ok
  test load_user_guidance_for_artifact_reads_scoped_file ... ok
  test load_user_guidance_for_artifact_strips_managed_header_block ... ok
  test load_user_guidance_strips_ito_internal_comment_block ... ok
  test load_user_guidance_prefers_user_prompts_guidance_file ... ok
  test load_composed_user_guidance_combines_scoped_and_shared ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/traceability_e2e.rs (target/llvm-cov-target/debug/deps/traceability_e2e-146a83a15561be5d)

  running 15 tests
  test legacy_checkbox_change_validate_passes_without_traceability_checks ... ok
  test legacy_checkbox_change_trace_output_is_unavailable ... ok
  test duplicate_requirement_ids_trace_output_has_diagnostics ... ok
  test traced_change_uncovered_req_trace_output_shows_uncovered ... ok
  test shelved_task_leaves_requirement_uncovered ... ok
  test traced_change_all_covered_trace_output_is_ready ... ok
  test partial_ids_trace_output_is_invalid ... ok
  test traced_change_unresolved_ref_trace_output_shows_unresolved ... ok
  test traced_change_uncovered_req_is_warning_in_non_strict ... ok
  test partial_ids_validate_reports_error ... ok
  test traced_change_unresolved_ref_is_error_in_validate ... ok
  test traced_change_uncovered_req_is_error_in_strict ... ok
  test traced_change_all_covered_validate_passes ... ok
  test shelved_task_uncovered_req_is_warning_in_validate ... ok
  test duplicate_requirement_ids_produce_error_in_validate ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/validate.rs (target/llvm-cov-target/debug/deps/validate-89017ae9cd3e1cab)

  running 23 tests
  test validate_module_reports_missing_scope_and_short_purpose ... ok
  test validate_spec_markdown_reports_missing_purpose_and_requirements ... ok
  test validate_spec_markdown_strict_treats_warnings_as_invalid ... ok
  test validate_change_requires_at_least_one_delta ... ok
  test validate_module_errors_when_sub_module_has_invalid_naming ... ok
  test validate_change_skips_optional_validator_when_artifact_is_missing ... ok
  test validate_module_errors_when_sub_module_missing_module_md ... ok
  test validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas ... ok
  test validate_tasks_file_issues_cite_tasks_tracking_validator_id ... ok
  test validate_tasks_file_returns_error_for_missing_file ... ok
  test validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas ... ok
  test validate_module_warns_when_sub_module_purpose_too_short ... ok
  test validate_tasks_file_returns_diagnostics_for_malformed_content ... ok
  test validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas ... ok
  test validate_change_requires_shall_or_must_in_requirement_text ... ok
  test validate_module_passes_when_sub_modules_have_valid_module_md ... ok
  test validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking ... ok
  test validate_tasks_file_returns_empty_for_valid_tasks ... ok
  test validate_change_validates_apply_tracks_file_when_configured ... ok
  test validate_change_uses_validation_yaml_delta_specs_validator_when_configured ... ok
  test validate_change_uses_apply_tracks_for_legacy_delta_schemas ... ok
  test empty_tracking_file_is_warning_in_non_strict_and_error_in_strict ... ok
  test validate_tasks_file_uses_apply_tracks_when_set ... ok

  test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/validate_delta_rules.rs (target/llvm-cov-target/debug/deps/validate_delta_rules-9a550966f9fbaddf)

  running 18 tests
  test capabilities_consistency_rule_warns_on_invalid_change_shape_values ... ok
  test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
  test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
  test contract_refs_rule_rejects_lowercase_unknown_scheme_after_known_ref ... ok
  test contract_refs_rule_rejects_short_unknown_scheme_after_known_ref ... ok
  test scenario_grammar_rule_accepts_steps_without_bullets ... ok
  test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
  test scenario_grammar_rule_accepts_asterisk_bullets ... ok
  test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
  test contract_refs_rule_rejects_unknown_scheme_after_known_ref ... ok
  test scenario_grammar_rule_accepts_ordered_list_steps ... ok
  test scenario_grammar_rule_warns_on_excessive_step_count ... ok
  test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
  test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok
  test contract_refs_rule_rejects_unknown_schemes ... ok
  test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
  test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
  test ui_mechanics_rule_warns_only_for_ui_tags ... ok

  test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/validate_rules_extension.rs (target/llvm-cov-target/debug/deps/validate_rules_extension-0b6033f41b068055)

  running 4 tests
  test missing_tracking_file_uses_configured_missing_artifact_level ... ok
  test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
  test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok
  test validation_yaml_delta_rules_work_for_non_specs_artifact_ids ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/validate_tracking_rules.rs (target/llvm-cov-target/debug/deps/validate_tracking_rules-17c6dd6ce870b3f2)

  running 7 tests
  test task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable ... ok
  test task_quality_rule_errors_on_unknown_requirement_ids ... ok
  test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
  test task_quality_rule_respects_warning_floor_without_promoting_advisories ... ok
  test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
  test task_quality_rule_treats_gradle_files_as_implementation_work ... ok
  test task_quality_rule_errors_on_missing_status ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/worktree_ensure_e2e.rs (target/llvm-cov-target/debug/deps/worktree_ensure_e2e-d413fbd198cee883)

  running 3 tests
  test ensure_worktree_disabled_returns_cwd ... ok
  test ensure_worktree_creates_and_initializes_with_include_files ... ok
  test ensure_worktree_with_setup_script ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_domain-5a82275f9d0c1885)

  running 119 tests
  test audit::context::tests::resolve_harness_session_id_returns_none_without_env ... ok
  test audit::event::tests::actor_serializes_to_lowercase ... ok
  test audit::event::tests::builder_returns_none_without_required_fields ... ok
  test audit::event::tests::entity_type_display ... ok
  test audit::event::tests::builder_produces_valid_event ... ok
  test audit::event::tests::builder_with_meta ... ok
  test audit::event::tests::entity_type_as_str_matches_serde ... ok
  test audit::event::tests::entity_type_round_trip ... ok
  test audit::event::tests::audit_event_serializes_to_single_line ... ok
  test audit::event::tests::actor_round_trip ... ok
  test audit::event::tests::entity_type_serializes_to_lowercase ... ok
  test audit::event::tests::audit_event_round_trip_serialization ... ok
  test audit::event::tests::event_context_round_trip ... ok
  test audit::event::tests::optional_fields_omitted_when_none ... ok
  test audit::event::tests::schema_version_is_one ... ok
  test audit::materialize::tests::archive_event_without_to_uses_sentinel ... ok
  test audit::materialize::tests::empty_events_produce_empty_state ... ok
  test audit::materialize::tests::global_entities_have_no_scope ... ok
  test audit::materialize::tests::last_event_wins ... ok
  test audit::materialize::tests::multiple_entities_tracked_independently ... ok
  test audit::materialize::tests::reconciled_events_update_state ... ok
  test audit::materialize::tests::single_create_event ... ok
  test audit::materialize::tests::status_change_updates_state ... ok
  test audit::reconcile::tests::detect_diverged_status ... ok
  test audit::reconcile::tests::compensating_events_use_scope_from_drift_key ... ok
  test audit::reconcile::tests::detect_extra_in_log ... ok
  test audit::reconcile::tests::detect_missing_entity_in_log ... ok
  test audit::reconcile::tests::display_drift_items ... ok
  test audit::reconcile::tests::generate_compensating_events_for_diverged ... ok
  test audit::reconcile::tests::generate_compensating_events_for_extra ... ok
  test audit::reconcile::tests::generate_compensating_events_for_missing ... ok
  test audit::reconcile::tests::multiple_drift_types_detected ... ok
  test audit::reconcile::tests::no_drift_when_states_match ... ok
  test audit::writer::tests::noop_writer_is_object_safe ... ok
  test audit::writer::tests::noop_writer_is_send_sync ... ok
  test audit::writer::tests::noop_writer_returns_ok ... ok
  test audit::writer::tests::trait_is_object_safe_for_dyn_dispatch ... ok
  test backend::tests::archive_result_roundtrip ... ok
  test backend::tests::artifact_bundle_roundtrip ... ok
  test backend::tests::backend_error_display_lease_conflict ... ok
  test backend::tests::backend_error_display_not_found ... ok
  test backend::tests::backend_error_display_other ... ok
  test backend::tests::backend_error_display_revision_conflict ... ok
  test backend::tests::backend_error_display_unauthorized ... ok
  test backend::tests::backend_error_display_unavailable ... ok
  test backend::tests::event_batch_roundtrip ... ok
  test backend::tests::event_ingest_result_roundtrip ... ok
  test changes::tests::test_change_status_display ... ok
  test changes::tests::test_change_sub_module_id_field ... ok
  test changes::tests::test_change_summary_status ... ok
  test changes::tests::test_change_work_status ... ok
  test changes::tests::test_extract_module_id ... ok
  test changes::tests::test_extract_sub_module_id ... ok
  test changes::tests::test_normalize_id ... ok
  test changes::tests::test_parse_change_id ... ok
  test changes::tests::test_parse_change_id_sub_module_format ... ok
  test changes::tests::test_parse_module_id ... ok
  test errors::tests::io_constructor_preserves_context_and_source ... ok
  test errors::tests::ambiguous_target_joins_candidates_in_display_message ... ok
  test errors::tests::not_found_constructor_formats_display_message ... ok
  test modules::tests::test_module_creation ... ok
  test modules::tests::test_module_summary ... ok
  test modules::tests::test_module_summary_with_sub_modules ... ok
  test modules::tests::test_module_with_sub_modules ... ok
  test modules::tests::test_sub_module_creation ... ok
  test modules::tests::test_sub_module_summary_creation ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_accepts_valid_formats ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_handles_large_numbers ... ok
  test audit::context::tests::resolve_session_id_generates_uuid ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_rejects_invalid_formats ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_edge_case_single_digit_with_many_dots ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_extracts_id_and_rest ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_colon_suffix ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_dot_suffix ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_leading_whitespace ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_multiple_spaces ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_tab_separator ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_unicode_in_task_name ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_preserves_trailing_whitespace_in_rest ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_returns_none_for_invalid_inputs ... ok
  test tasks::compute::tests::checkbox_mode_returns_pending_sorted_and_no_blocked ... ok
  test tasks::compute::tests::enhanced_backcompat_blocks_later_waves_and_checkpoints_until_first_incomplete_wave_done ... ok
  test tasks::compute::tests::enhanced_ready_and_blocked_lists_are_sorted_by_task_id ... ok
  test tasks::compute::tests::enhanced_task_dependencies_produce_missing_crosswave_and_not_complete_blockers ... ok
  test tasks::compute::tests::enhanced_wave_dependency_blocks_by_wave_and_unblocks_when_complete ... ok
  test audit::context::tests::resolve_session_id_is_stable_across_calls ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_empty_graph ... ok
  test discovery::tests::list_changes_skips_archive_dir ... ok
  test discovery::tests::list_module_ids_extracts_numeric_prefixes ... ok
  test discovery::tests::list_modules_only_returns_directories ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_simple_two_node_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_cycle_in_complex_graph ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_diamond_pattern_without_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_special_characters_in_node_names ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_acyclic_graph ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_long_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_multiple_cycles_returns_one ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_with_numeric_node_names ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_self_loop ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_three_node_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_marks_errors_as_error_level ... ok
  test tasks::relational::relational_tests::validate_relational_detects_missing_task_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_accepts_valid_dependency_graph ... ok
  test tasks::relational::relational_tests::validate_relational_detects_duplicate_task_ids ... ok
  test tasks::relational::relational_tests::validate_relational_detects_dependency_on_shelved_task ... ok
  test tasks::relational::relational_tests::validate_relational_detects_self_referencing_task ... ok
  test tasks::relational::relational_tests::validate_relational_ignores_empty_and_checkpoint_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_handles_tasks_without_wave ... ok
  test tasks::relational::relational_tests::validate_relational_allows_shelved_task_depending_on_shelved_task ... ok
  test tasks::relational::relational_tests::validate_relational_detects_wave_dependency_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_detects_cross_wave_task_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_reports_line_numbers ... ok
  test tasks::relational::relational_tests::validate_relational_detects_task_dependency_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_multiple_errors_for_same_task ... ok
  test tasks::relational::relational_tests::validate_relational_detects_three_node_task_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_with_complex_valid_graph ... ok
  test audit::context::tests::resolve_user_identity_returns_at_prefixed_string ... ok
  test audit::context::tests::resolve_git_context_does_not_panic ... ok
  test audit::context::tests::resolve_context_populates_session_id ... ok

  test result: ok. 119 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

       Running tests/planning.rs (target/llvm-cov-target/debug/deps/planning-bb3b6fa480ddfc50)

  running 1 test
  test roadmap_parsing_extracts_current_progress_and_phases ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/schema_roundtrip.rs (target/llvm-cov-target/debug/deps/schema_roundtrip-f8731b98ba65acb9)

  running 3 tests
  test workflow_plan_json_roundtrip ... ok
  test workflow_execution_json_roundtrip ... ok
  test workflow_yaml_roundtrip ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/schema_validation.rs (target/llvm-cov-target/debug/deps/schema_validation-dc4bf608d378591f)

  running 12 tests
  test task_definition_validate_accepts_optional_fields ... ok
  test task_execution_validate_rejects_empty_optional_strings ... ok
  test workflow_definition_validate_rejects_duplicate_wave_ids ... ok
  test wave_definition_validate_rejects_invalid_shapes ... ok
  test workflow_definition_validate_rejects_requires_and_context_files_empty_entries ... ok
  test workflow_definition_validate_rejects_empty_fields ... ok
  test plan_validate_rejects_empty_prompt_content ... ok
  test execution_validate_rejects_out_of_bounds_wave_index ... ok
  test workflow_definition_validate_accepts_minimal_valid ... ok
  test task_definition_validate_rejects_invalid_fields ... ok
  test plan_validate_rejects_other_invalid_fields ... ok
  test execution_validate_rejects_invalid_fields_and_accepts_valid ... ok

  test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks.rs (target/llvm-cov-target/debug/deps/tasks-bcd4255e3efe98ba)

  running 2 tests
  test update_enhanced_task_status_inserts_or_replaces_status_line ... ok
  test enhanced_template_parses_and_has_checkpoint_warning ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks_parsing.rs (target/llvm-cov-target/debug/deps/tasks_parsing-549bc8dbcca8c096)

  running 32 tests
  test detect_tasks_format_enhanced_vs_checkbox ... ok
  test parse_checkbox_tasks_accepts_right_arrow_in_progress_marker ... ok
  test parse_checkbox_tasks_assigns_sequential_ids_when_not_explicit ... ok
  test parse_checkbox_tasks_supports_dash_and_star ... ok
  test parse_checkbox_tasks_uppercase_x_marks_complete ... ok
  test parse_checkbox_tasks_handles_empty_lines_and_non_checkbox_content ... ok
  test parse_checkbox_tasks_preserves_explicit_ids ... ok
  test parse_checkbox_tasks_handles_mixed_explicit_and_implicit_ids ... ok
  test parse_enhanced_tasks_parses_fields_and_action_block ... ok
  test parse_enhanced_tasks_requirements_single_entry ... ok
  test tasks_path_checked_rejects_traversal_like_change_ids ... ok
  test tasks_path_uses_safe_fallback_for_invalid_change_id ... ok
  test update_checkbox_task_status_by_explicit_id ... ok
  test update_checkbox_task_status_preserves_bullet_style ... ok
  test update_checkbox_task_status_sets_marker_and_preserves_text ... ok
  test parse_enhanced_tasks_handles_multiline_action ... ok
  test update_enhanced_task_status_inserts_missing_fields ... ok
  test parse_enhanced_tasks_requirements_not_carried_across_tasks ... ok
  test update_enhanced_task_status_preserves_existing_fields ... ok
  test update_enhanced_task_status_preserves_requirements_line ... ok
  test parse_enhanced_tasks_handles_wave_with_comma_in_title ... ok
  test parse_enhanced_tasks_handles_multiple_files ... ok
  test parse_enhanced_tasks_handles_empty_dependencies_field ... ok
  test parse_enhanced_tasks_extracts_requirements_field ... ok
  test parse_enhanced_tasks_handles_task_without_optional_prefix ... ok
  test enhanced_tasks_wave_gating_blocks_later_waves ... ok
  test parse_enhanced_tasks_accepts_all_prior_tasks_dependency_shorthand ... ok
  test enhanced_tasks_diagnostics_cover_common_errors ... ok
  test parse_enhanced_tasks_requirements_absent_gives_empty_vec ... ok
  test parse_enhanced_tasks_progress_counts_all_statuses ... ok
  test enhanced_tasks_cycles_and_shelved_deps_are_reported ... ok
  test parse_enhanced_tasks_accepts_wave_heading_titles ... ok

  test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/tasks_parsing_additional.rs (target/llvm-cov-target/debug/deps/tasks_parsing_additional-5ff20f81f200bd47)

  running 28 tests
  test checkbox_format_ignores_incomplete_checkbox_patterns ... ok
  test checkbox_format_handles_newlines_in_adjacent_lines ... ok
  test checkbox_format_handles_very_long_task_names ... ok
  test checkbox_format_handles_empty_task_text ... ok
  test checkbox_format_handles_special_characters_in_task_names ... ok
  test checkbox_format_progress_info_counts_correctly ... ok
  test parse_empty_file_returns_empty_result ... ok
  test parse_file_with_only_non_task_content ... ok
  test parse_file_with_only_whitespace ... ok
  test tasks_path_checked_accepts_valid_change_ids ... ok
  test tasks_path_checked_rejects_empty_change_id ... ok
  test tasks_path_checked_rejects_very_long_change_ids ... ok
  test enhanced_format_handles_very_long_file_paths ... ok
  test enhanced_format_handles_task_without_wave ... ok
  test enhanced_format_handles_very_large_wave_numbers ... ok
  test enhanced_format_handles_multiple_files_with_spaces ... ok
  test wave_dependencies_detect_forward_references ... ok
  test enhanced_format_handles_status_marker_mismatch ... ok
  test enhanced_format_handles_duplicate_wave_numbers ... ok
  test enhanced_format_handles_multiline_action_with_code ... ok
  test enhanced_format_handles_empty_action_block ... ok
  test enhanced_format_handles_checkpoints ... ok
  test enhanced_format_validates_missing_required_fields ... ok
  test enhanced_format_validates_date_format_strictly ... ok
  test enhanced_format_handles_uppercase_x_in_complete_marker ... ok
  test progress_info_calculates_remaining_correctly ... ok
  test wave_dependencies_handle_various_formats ... ok
  test enhanced_format_handles_complex_dependency_chains ... ok

  test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/tasks_quality_fields.rs (target/llvm-cov-target/debug/deps/tasks_quality_fields-0f260b00719f3387)

  running 2 tests
  test quality_fields_allow_missing_optional_metadata ... ok
  test quality_fields_round_trip_when_present ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks_update.rs (target/llvm-cov-target/debug/deps/tasks_update-6aae0d3892d83912)

  running 19 tests
  test update_checkbox_task_status_rejects_shelving ... ok
  test update_checkbox_task_status_handles_mixed_explicit_and_implicit_ids ... ok
  test update_checkbox_task_status_errors_for_invalid_or_missing_task_id ... ok
  test update_checkbox_task_status_handles_unicode_in_task_text ... ok
  test update_checkbox_task_status_matches_explicit_ids_over_index ... ok
  test update_checkbox_task_status_with_id_suffix_colon ... ok
  test update_checkbox_task_status_with_id_suffix_dot ... ok
  test update_checkbox_task_status_handles_various_markers ... ok
  test update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting ... ok
  test update_checkbox_task_status_preserves_bullet_style ... ok
  test update_enhanced_task_status_preserves_trailing_newline ... ok
  test update_enhanced_task_status_handles_task_prefix_optional ... ok
  test update_enhanced_task_status_handles_in_progress ... ok
  test update_enhanced_task_status_preserves_other_fields ... ok
  test update_enhanced_task_status_inserts_missing_fields ... ok
  test update_enhanced_task_status_only_updates_specified_task ... ok
  test update_enhanced_task_status_handles_complex_task_ids ... ok
  test update_enhanced_task_status_updates_status_and_date ... ok
  test update_enhanced_task_status_handles_shelved ... ok

  test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/traceability.rs (target/llvm-cov-target/debug/deps/traceability-1b5d7b2b41a1bcf1)

  running 13 tests
  test checkbox_format_gives_unavailable ... ok
  test uncovered_requirement_appears_in_uncovered_list ... ok
  test shelved_task_does_not_count_as_coverage ... ok
  test no_requirement_ids_gives_unavailable ... ok
  test duplicate_requirement_ids_flagged_in_diagnostics ... ok
  test declared_requirements_are_sorted_and_deduplicated ... ok
  test in_progress_task_counts_as_coverage ... ok
  test complete_task_counts_as_coverage ... ok
  test partial_ids_gives_invalid_with_missing_titles ... ok
  test all_requirements_covered_by_tasks ... ok
  test unresolved_task_reference_is_reported ... ok
  test empty_requirements_list_gives_unavailable ... ok
  test multiple_tasks_can_cover_same_requirement ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_logging-3d55c8a9a558f35f)

  running 2 tests
  test tests::unsafe_session_ids_are_rejected ... ok
  test tests::invalid_command_logger_writes_jsonl_entry ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_templates-b0356ae03387c79c)

  running 83 tests
  test agents::tests::render_template_replaces_variant ... ok
  test agents::tests::default_configs_has_all_combinations ... ok
  test agents::tests::render_template_replaces_model ... ok
  test agents::tests::render_template_removes_variant_line_if_not_set ... ok
  test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
  test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
  test instructions::tests::render_template_str_preserves_trailing_newline ... ok
  test instructions::tests::render_template_str_is_strict_on_undefined ... ok
  test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
  test instructions::tests::orchestrate_template_renders ... ok
  test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
  test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
  test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
  test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
  test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
  test instructions::tests::finish_template_prompts_for_archive ... ok
  test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
  test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
  test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
  test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
  test instructions::tests::apply_template_omits_capture_reminder_when_search_only_configured ... ok
  test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
  test instructions::tests::repo_sweep_template_renders ... ok
  test instructions::tests::apply_template_renders_capture_reminder_when_configured ... ok
  test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
  test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
  test project_templates::tests::default_context_is_disabled ... ok
  test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
  test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
  test instructions::tests::review_template_renders_conditional_sections ... ok
  test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
  test project_templates::tests::render_project_template_passes_plain_text_through ... ok
  test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
  test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
  test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
  test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
  test tests::default_home_files_returns_a_vec ... ok
  test project_templates::tests::render_project_template_renders_simple_variable ... ok
  test project_templates::tests::render_project_template_strict_on_undefined ... ok
  test project_templates::tests::render_project_template_renders_conditional ... ok
  test tests::default_project_files_contains_expected_files ... ok
  test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
  test tests::default_project_includes_orchestrate_user_prompt ... ok
  test tests::every_shipped_command_has_ito_prefix ... ok
  test tests::every_shipped_agent_has_ito_prefix ... ok
  test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
  test tests::extract_managed_block_rejects_inline_markers ... ok
  test tests::every_shipped_skill_has_ito_prefix ... ok
  test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
  test tests::extract_managed_block_returns_inner_content ... ok
  test tests::fix_and_feature_commands_are_embedded ... ok
  test tests::get_preset_file_returns_contents ... ok
  test tests::get_schema_file_returns_contents ... ok
  test tests::loop_command_template_uses_ito_loop_command_name ... ok
  test tests::loop_skill_template_includes_yaml_frontmatter ... ok
  test tests::memory_skill_is_embedded ... ok
  test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
  test tests::every_shipped_markdown_has_managed_markers ... ok
  test tests::normalize_ito_dir_prefixes_dot ... ok
  test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
  test tests::orchestrate_skills_and_command_are_embedded ... ok
  test tests::presets_files_contains_orchestrate_builtins ... ok
  test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
  test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
  test tests::proposal_intake_and_routing_skills_are_embedded ... ok
  test tests::render_bytes_preserves_non_utf8 ... ok
  test tests::render_bytes_returns_borrowed_when_no_rewrite_needed ... ok
  test tests::render_bytes_rewrites_dot_ito_paths ... ok
  test tests::render_rel_path_rewrites_ito_prefix ... ok
  test tests::schema_files_contains_builtins ... ok
  test tests::stamp_version_canonical_with_leading_whitespace_is_rewritten ... ok
  test tests::stamp_version_handles_crlf_line_endings ... ok
  test tests::stamp_version_handles_prerelease_semver ... ok
  test tests::stamp_version_idempotent_on_canonical_match ... ok
  test tests::stamp_version_idempotent_on_canonical_with_trailing_whitespace ... ok
  test tests::stamp_version_inserts_when_missing ... ok
  test tests::stamp_version_noop_without_marker ... ok
  test tests::stamp_version_preserves_frontmatter ... ok
  test tests::stamp_version_preserves_trailing_content ... ok
  test tests::stamp_version_rewrites_older_version ... ok
  test tests::stamp_version_rewrites_spaced_form_to_canonical ... ok
  test tests::stamp_version_round_trip_on_real_skill ... ok
  test tests::tmux_skill_and_scripts_are_embedded ... ok

  test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/managed_markers.rs (target/llvm-cov-target/debug/deps/managed_markers-95248097f86217a6)

  running 5 tests
  test commands_have_managed_markers ... ok
  test schema_files_have_managed_markers ... ok
  test agents_have_managed_markers ... ok
  test default_project_files_have_managed_markers ... ok
  test skills_have_managed_markers ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/prefix_rule.rs (target/llvm-cov-target/debug/deps/prefix_rule-5b98a2ae58171e0d)

  running 3 tests
  test commands_satisfy_ito_prefix_rule ... ok
  test agents_satisfy_ito_prefix_rule ... ok
  test skills_satisfy_ito_prefix_rule ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/stamp.rs (target/llvm-cov-target/debug/deps/stamp-dd9f57db337c70b3)

  running 8 tests
  test stamp_no_op_when_no_managed_block ... ok
  test stamp_inserts_when_no_existing_stamp ... ok
  test stamp_preserves_rest_of_file ... ok
  test stamp_idempotent_when_same_version ... ok
  test stamp_rewrites_older_version_stamp ... ok
  test stamp_works_with_frontmatter_before_marker ... ok
  test stamp_rewrites_spaced_stamp_to_canonical ... ok
  test stamp_round_trip_on_real_skill ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/template_markdown.rs (target/llvm-cov-target/debug/deps/template_markdown-b7c4f3a33f52ce1a)

  running 1 test
  test template_markdown_is_well_formed ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/user_guidance_template.rs (target/llvm-cov-target/debug/deps/user_guidance_template-d61017ba2fec19a9)

  running 2 tests
  test user_guidance_template_exists_and_has_markers ... ok
  test user_prompt_stub_templates_exist ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/worktree_template_rendering.rs (target/llvm-cov-target/debug/deps/worktree_template_rendering-0cb800878c96834f)

  running 8 tests
  test skill_disabled ... ok
  test agents_md_disabled ... ok
  test skill_bare_control_siblings ... ok
  test skill_checkout_siblings ... ok
  test agents_md_checkout_siblings ... ok
  test agents_md_checkout_subdir ... ok
  test skill_checkout_subdir ... ok
  test agents_md_bare_control_siblings ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_test_support-725fb2b42b988f90)

  running 4 tests
  test tests::normalize_replaces_home_path ... ok
  test tests::normalize_strips_ansi_and_crlf ... ok
  test tests::copy_dir_all_copies_nested_files ... ok
  test pty::tests::pty_can_echo_input_via_cat ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/mock_repos_smoke.rs (target/llvm-cov-target/debug/deps/mock_repos_smoke-5e1ad2a8a02de6c0)

  running 3 tests
  test mock_module_repo_resolves_by_id_or_name ... ok
  test mock_task_repo_returns_configured_tasks ... ok
  test mock_repos_basic_roundtrip ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
cargo test affected (ito-rs).............................................Failed
- hook id: cargo-test-affected
- files were modified by this hook

  bash ito-rs/tools/test-affected.sh
  Changed crates: ito-cli ito-core ito-domain
  Affected crates (with dependents): ito-cli ito-core ito-domain ito-web ito-test-support
  Running: cargo nextest run -p ito-cli -p ito-core -p ito-domain -p ito-web -p ito-test-support
      Finished `test` profile [optimized + debuginfo] target(s) in 0.66s
  ────────────
   Nextest run ID 9aebb252-5c91-41e8-b17d-82d904021e8d with nextest profile: default
      Starting 1609 tests across 122 binaries (4 tests skipped)
          PASS [   6.664s] (   1/1609) ito-cli::agent_instruction_bootstrap bootstrap_codex_success
          PASS [   6.666s] (   2/1609) ito-cli::agent_instruction_bootstrap bootstrap_output_is_short
          PASS [   6.679s] (   3/1609) ito-cli::agent_instruction_bootstrap bootstrap_github_copilot_success
          PASS [   6.679s] (   4/1609) ito-cli::agent_instruction_bootstrap bootstrap_json_output
          PASS [   6.679s] (   5/1609) ito-cli::agent_instruction_bootstrap bootstrap_rejects_invalid_tool
          PASS [   6.679s] (   6/1609) ito-cli::agent_instruction_bootstrap bootstrap_requires_tool_flag
          PASS [   6.679s] (   7/1609) ito-cli::agent_instruction_bootstrap bootstrap_opencode_success
          PASS [   6.679s] (   8/1609) ito-cli::agent_instruction_bootstrap bootstrap_claude_success
          PASS [   6.683s] (   9/1609) ito-cli::agent_instruction_context agent_instruction_context_prefers_path_inference_in_text_output
          PASS [   6.730s] (  10/1609) ito-cli::agent_instruction_context agent_instruction_context_supports_json_output
          LEAK [   6.868s] (  11/1609) ito-cli::agent_instruction_bootstrap bootstrap_contains_artifact_pointers
          PASS [   7.084s] (  12/1609) ito-cli::agent_instruction_memory agent_instruction_help_lists_memory_artifacts
          PASS [   7.185s] (  13/1609) ito-cli::agent_instruction_memory memory_capture_not_configured_branch_renders_setup_guidance
          PASS [   7.185s] (  14/1609) ito-cli::agent_instruction_memory memory_capture_skill_branch_emits_structured_inputs
          PASS [   7.185s] (  15/1609) ito-cli::agent_instruction_memory memory_capture_command_branch_renders_executable_command_line
          PASS [   7.186s] (  16/1609) ito-cli::agent_instruction_memory memory_capture_renders_skill_when_only_capture_configured
          PASS [   0.693s] (  17/1609) ito-cli::agent_instruction_memory memory_search_skill_branch_emits_structured_inputs
          PASS [   0.712s] (  18/1609) ito-cli::agent_instruction_memory memory_query_command_branch_substitutes_query
          PASS [   0.701s] (  19/1609) ito-cli::agent_instruction_memory memory_query_renders_not_configured_when_only_capture_set
          PASS [   0.702s] (  20/1609) ito-cli::agent_instruction_memory memory_query_skill_branch_emits_structured_inputs
          PASS [   0.702s] (  21/1609) ito-cli::agent_instruction_memory memory_search_not_configured_branch_renders_setup_guidance
          PASS [   0.703s] (  22/1609) ito-cli::agent_instruction_memory memory_search_command_branch_overrides_limit_when_supplied
          PASS [   0.702s] (  23/1609) ito-cli::agent_instruction_memory memory_search_requires_query_flag
          PASS [   0.703s] (  24/1609) ito-cli::agent_instruction_memory memory_search_command_branch_substitutes_query_and_default_limit
          PASS [   0.716s] (  25/1609) ito-cli::agent_instruction_memory memory_query_not_configured_branch_renders_setup_guidance
          PASS [   5.405s] (  26/1609) ito-cli::agent_instruction_orchestrate orchestrate_requires_orchestrate_md
          PASS [   5.543s] (  27/1609) ito-cli::agent_instruction_orchestrate orchestrate_json_output_has_correct_artifact_id
          PASS [   5.189s] (  28/1609) ito-cli::agent_instruction_orchestrate orchestrate_succeeds_when_orchestrate_md_exists
          PASS [   5.088s] (  29/1609) ito-cli::agent_instruction_orchestrate orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter
          PASS [   5.088s] (  30/1609) ito-cli::agent_instruction_orchestrate orchestrate_surfaces_recommended_skills_from_preset
          PASS [   6.627s] (  31/1609) ito-cli::agent_instruction_repo_sweep repo_sweep_succeeds_without_change_flag
          PASS [   6.818s] (  32/1609) ito-cli::agent_instruction_repo_sweep repo_sweep_json_output_has_correct_artifact_id
          PASS [   6.818s] (  33/1609) ito-cli::agent_instruction_repo_sweep repo_sweep_output_contains_key_phrases
          PASS [   7.895s] (  34/1609) ito-cli::agent_instruction_worktrees worktrees_instruction_does_not_require_change
          PASS [   7.894s] (  35/1609) ito-cli::agent_instruction_worktrees worktrees_instruction_json_output
          PASS [   8.417s] (  36/1609) ito-cli::aliases subcommand_aliases_work
          PASS [   8.423s] (  37/1609) ito-cli::aliases main_command_aliases_work
          PASS [   8.446s] (  38/1609) ito-cli::aliases main_command_aliases_execute
          PASS [   8.445s] (  39/1609) ito-cli::aliases short_flags_work
          PASS [   4.859s] (  40/1609) ito-cli::archive_completed archive_completed_conflict_with_positional
          PASS [   4.860s] (  41/1609) ito-cli::archive_completed archive_completed_no_completed_changes
          PASS [   4.906s] (  42/1609) ito-cli::archive_completed archive_completed_empty_confirmation_cancels
          PASS [   4.907s] (  43/1609) ito-cli::archive_completed archive_completed_decline_confirmation_cancels
          PASS [   5.436s] (  44/1609) ito-cli::archive_completed archive_completed_skip_specs
          PASS [  10.327s] (  45/1609) ito-cli::archive_completed archive_completed_accept_yes_confirmation_archives
          PASS [  10.328s] (  46/1609) ito-cli::archive_completed archive_completed_archives_all_completed
          PASS [   5.097s] (  47/1609) ito-cli::archive_smoke archive_with_specs_and_validation_smoke
          PASS [   4.647s] (  48/1609) ito-cli::audit_more audit_more_local_audit_writes_warn_and_fallback_without_worktree_log_when_branch_storage_is_unavailable
          PASS [   5.917s] (  49/1609) ito-cli::audit_more audit_log_stats_and_validate_json_outputs_are_well_formed
          PASS [   5.985s] (  50/1609) ito-cli::audit_more audit_more_local_audit_writes_use_internal_branch_without_worktree_log_churn
          PASS [   5.704s] (  51/1609) ito-cli::audit_more audit_subcommands_cover_text_output_limit_reconcile_and_stream
          PASS [   4.440s] (  52/1609) ito-cli::backend_import backend_import_rejects_local_mode
          PASS [   5.817s] (  53/1609) ito-cli::audit_more audit_stream_all_worktrees_dedupes_shared_routed_storage
          PASS [   7.671s] (  54/1609) ito-cli::audit_more audit_commands_migrate_legacy_worktree_log_into_routed_storage
          PASS [   2.572s] (  55/1609) ito-cli::backend_serve backend_serve_service_mode_reports_malformed_backend_config
          PASS [   4.662s] (  56/1609) ito-cli::backend_serve backend_serve_reports_unknown_fields_in_explicit_config_file
          PASS [   6.054s] (  57/1609) ito-cli::backend_serve backend_serve_init_prints_backend_command_guidance
          PASS [   2.505s] (  58/1609) ito-cli::backend_serve backend_serve_service_mode_reuses_existing_auth_without_printing_init_output
          PASS [   3.319s] (  59/1609) ito-cli::backend_serve backend_serve_service_mode_bootstraps_missing_auth_silently
          PASS [   3.945s] (  60/1609) ito-cli::backend_status_more backend_status_disabled_shows_informational_output
          PASS [   3.944s] (  61/1609) ito-cli::backend_status_more backend_status_incomplete_config_fails
          PASS [   3.891s] (  62/1609) ito-cli::backend_status_more backend_status_json_includes_config_details
          PASS [   4.034s] (  63/1609) ito-cli::backend_status_more backend_status_disabled_json_output
          PASS [  11.693s] (  64/1609) ito-cli::archive_remote_mode remote_archive_succeeds_without_local_active_change_markdown
          PASS [   0.294s] (  65/1609) ito-cli::backend_status_more generate_token_missing_repo_fails
          PASS [   0.295s] (  66/1609) ito-cli::backend_status_more generate_token_missing_org_fails
          PASS [   0.295s] (  67/1609) ito-cli::backend_status_more generate_token_flag_overrides_for_org_repo
          PASS [   0.296s] (  68/1609) ito-cli::backend_status_more generate_token_derives_deterministic_token
          PASS [   0.302s] (  69/1609) ito-cli::backend_status_more generate_token_no_seed_fails
          PASS [   0.550s] (  70/1609) ito-cli::backend_status_more silent_fallback_grep_warns_on_bad_config
          PASS [   0.554s] (  71/1609) ito-cli::backend_status_more generate_token_with_all_sources_prefers_env
          PASS [   0.555s] (  72/1609) ito-cli::backend_status_more generate_token_seed_from_env_takes_precedence
          PASS [   2.716s] (  73/1609) ito-cli::backend_status_more backend_status_with_valid_config_but_no_server
          PASS [   0.633s] (  74/1609) ito-cli::backend_status_more silent_fallback_event_forwarding_warns_on_bad_config
          PASS [   2.733s] (  75/1609) ito-cli::backend_status_more backend_status_token_security_warning
          PASS [   2.732s] (  76/1609) ito-cli::backend_status_more backend_status_with_env_token_no_warning
          PASS [   2.733s] (  77/1609) ito-cli::backend_status_more backend_status_unreachable_server_json_output
          PASS [   2.733s] (  78/1609) ito-cli::backend_status_more backend_status_unreachable_server_fails
          PASS [   0.509s] (  79/1609) ito-cli::backend_status_more silent_fallback_tasks_warns_on_bad_config
          PASS [   0.160s] (  80/1609) ito-cli::backend_status_more silent_fallback_with_valid_backend_no_warnings
          PASS [   3.603s] (  81/1609) ito-cli::cli_smoke cli_help_hides_top_level_serve_api_entrypoint
          PASS [   3.580s] (  82/1609) ito-cli::cli_smoke cli_top_level_serve_api_help_shows_backend_migration_guidance
          PASS [   3.568s] (  83/1609) ito-cli::cli_smoke cli_top_level_serve_api_shows_backend_migration_guidance
          PASS [   3.973s] (  84/1609) ito-cli::cli_smoke agent_instruction_status_archive_smoke
          PASS [   3.907s] (  85/1609) ito-cli::cli_smoke list_show_validate_smoke
          PASS [   4.326s] (  86/1609) ito-cli::cli_smoke create_workflow_plan_state_config_smoke
          PASS [   5.240s] (  87/1609) ito-cli::cli_snapshots snapshot_backend_help
          PASS [   5.177s] (  88/1609) ito-cli::cli_snapshots snapshot_backend_serve_help
          PASS [   5.251s] (  89/1609) ito-cli::cli_snapshots snapshot_agent_help
          PASS [   5.251s] (  90/1609) ito-cli::cli_snapshots snapshot_agent_instruction_help
          PASS [   0.044s] (  91/1609) ito-cli::cli_snapshots snapshot_version
          PASS [   0.044s] (  92/1609) ito-cli::cli_snapshots snapshot_ralph_help
          PASS [   0.044s] (  93/1609) ito-cli::cli_snapshots snapshot_tasks_help
          PASS [   0.044s] (  94/1609) ito-cli::cli_snapshots snapshot_validate_help
          PASS [  16.400s] (  95/1609) ito-cli::backend_import backend_import_dry_run_reports_scope_without_writing_backend
          PASS [  15.835s] (  96/1609) ito-cli::backend_import backend_import_writes_active_and_archived_changes_to_backend
          PASS [  17.853s] (  97/1609) ito-cli::audit_remote_mode audit_commands_in_backend_mode_use_server_only_storage
          PASS [  16.272s] (  98/1609) ito-cli::backend_qa_walkthrough backend_qa_script_verify_runs_end_to_end
          PASS [   3.596s] (  99/1609) ito-cli::cli_snapshots snapshot_init_help
          PASS [   3.177s] ( 100/1609) ito-cli::cli_snapshots snapshot_list_help
          PASS [   3.939s] ( 101/1609) ito-cli::cli_snapshots snapshot_help_all_global_flag
          PASS [   3.982s] ( 102/1609) ito-cli::cli_snapshots snapshot_create_help
          PASS [   3.939s] ( 103/1609) ito-cli::cli_snapshots snapshot_help
          PASS [   3.616s] ( 104/1609) ito-cli::cli_snapshots snapshot_help_all_subcommand
          PASS [  17.383s] ( 105/1609) ito-cli::audit_remote_mode validate_single_change_in_backend_mode_skips_local_audit_reconcile
          PASS [  17.371s] ( 106/1609) ito-cli::backend_import backend_import_is_idempotent_and_remote_reads_match_imported_changes
          PASS [   4.281s] ( 107/1609) ito-cli::config_more config_unknown_subcommand_errors
          PASS [   6.023s] ( 108/1609) ito-cli::config_more config_set_rejects_invalid_coordination_branch_name
          PASS [   6.023s] ( 109/1609) ito-cli::config_more config_set_rejects_invalid_audit_mirror_branch_name
          PASS [   6.559s] ( 110/1609) ito-cli::config_more config_help_path_list_unset_and_schema_smoke
          PASS [   6.679s] ( 111/1609) ito-cli::config_more config_set_get_supports_coordination_and_audit_mirror_keys
          PASS [   5.369s] ( 112/1609) ito-cli::coverage_smoke serve_errors_when_no_ito_dir_exists
          PASS [   5.749s] ( 113/1609) ito-cli::coverage_smoke completions_command_runs_for_all_shells
          PASS [   6.001s] ( 114/1609) ito-cli::coverage_smoke audit_validate_and_log_work_with_empty_event_log
          PASS [   6.317s] ( 115/1609) ito-cli::grep_more grep_change_scope_rejects_too_many_positional_args
          PASS [   6.828s] ( 116/1609) ito-cli::grep_more grep_change_scope_prints_matches_with_locations
          PASS [   6.281s] ( 117/1609) ito-cli::grep_more grep_limit_caps_output_and_prints_warning
          PASS [   3.017s] ( 118/1609) ito-cli::grep_more grep_module_scope_searches_all_changes_in_module
          PASS [   6.832s] ( 119/1609) ito-cli::grep_more grep_all_scope_searches_all_changes
          PASS [   8.386s] ( 120/1609) ito-cli::create_more create_change_sub_module_and_module_are_mutually_exclusive
          PASS [   8.414s] ( 121/1609) ito-cli::create_more create_change_sub_module_rejects_remote_persistence_mode
          PASS [   8.601s] ( 122/1609) ito-cli::create_more create_change_with_sub_module_flag_creates_composite_id_change
          PASS [   8.910s] ( 123/1609) ito-cli::create_more create_module_and_change_error_paths_and_outputs
          PASS [   4.165s] ( 124/1609) ito-cli::help help_shows_navigation_footer
          PASS [   4.281s] ( 125/1609) ito-cli::help help_prints_usage
          PASS [   5.898s] ( 126/1609) ito-cli::help agent_instruction_help_shows_instruction_details
          PASS [   5.367s] ( 127/1609) ito-cli::help help_all_global_flag_works
          PASS [   5.905s] ( 128/1609) ito-cli::help dash_h_help_matches_dash_dash_help
          PASS [   5.282s] ( 129/1609) ito-cli::help help_all_json_outputs_valid_json
          PASS [   4.404s] ( 130/1609) ito-cli::help help_all_shows_complete_reference
          PASS [   5.671s] ( 131/1609) ito-cli::init_coordination init_no_coordination_worktree_writes_embedded_storage
          PASS [   5.688s] ( 132/1609) ito-cli::init_coordination init_without_git_remote_falls_back_gracefully
          PASS [   5.700s] ( 133/1609) ito-cli::init_coordination init_upgrade_does_not_touch_coordination_storage
          PASS [   5.856s] ( 134/1609) ito-cli::init_coordination init_with_git_remote_creates_coordination_worktree
          PASS [   6.469s] ( 135/1609) ito-cli::init_gitignore_session_json init_writes_gitignore_session_json_and_is_idempotent
          PASS [   4.799s] ( 136/1609) ito-cli::init_more init_help_prints_usage
          PASS [   2.128s] ( 137/1609) ito-cli::init_more init_requires_tools_when_non_interactive
          PASS [   4.911s] ( 138/1609) ito-cli::init_more init_refuses_to_overwrite_existing_file_without_markers_when_not_forced
          PASS [   4.981s] ( 139/1609) ito-cli::init_more init_renders_agents_md_without_raw_jinja2_syntax
          PASS [   5.051s] ( 140/1609) ito-cli::init_more init_prints_project_setup_nudge_when_marker_incomplete
          PASS [   5.058s] ( 141/1609) ito-cli::init_more init_opencode_installs_audit_hook_plugin
          PASS [   5.075s] ( 142/1609) ito-cli::init_more init_github_copilot_installs_audit_preflight_assets
          PASS [   5.043s] ( 143/1609) ito-cli::init_more init_renders_skill_files_without_raw_jinja2_syntax
          PASS [   6.417s] ( 144/1609) ito-cli::init_more init_codex_installs_audit_instruction_assets
          PASS [   6.291s] ( 145/1609) ito-cli::init_more init_does_not_print_project_setup_nudge_when_marker_complete
          PASS [   5.991s] ( 146/1609) ito-cli::init_more init_force_overwrites_existing_user_prompt_stubs
          PASS [   6.486s] ( 147/1609) ito-cli::init_more init_does_not_print_project_setup_nudge_when_marker_absent
          PASS [   0.397s] ( 148/1609) ito-cli::init_more init_tools_csv_ignores_empty_segments
          PASS [   0.250s] ( 149/1609) ito-cli::init_more init_update_without_prior_init_creates_all_files
          PASS [   0.392s] ( 150/1609) ito-cli::init_more init_update_does_not_overwrite_existing_user_prompt_stubs
          PASS [   0.225s] ( 151/1609) ito-cli::init_more init_with_tools_none_installs_ito_skeleton
          PASS [   0.258s] ( 152/1609) ito-cli::init_more init_with_tools_csv_installs_selected_adapters
          PASS [   0.289s] ( 153/1609) ito-cli::init_more init_upgrade_refreshes_marker_managed_block_and_preserves_user_content
          PASS [   0.345s] ( 154/1609) ito-cli::init_more init_update_preserves_user_files_and_creates_missing
          PASS [   0.449s] ( 155/1609) ito-cli::init_more init_tools_parser_covers_all_and_invalid_id
          PASS [   0.343s] ( 156/1609) ito-cli::init_more init_update_renders_agents_md_without_raw_jinja2
          PASS [   0.216s] ( 157/1609) ito-cli::init_more init_writes_config_with_release_tag_schema_reference
          PASS [   0.240s] ( 158/1609) ito-cli::init_more init_with_tools_opencode_installs_orchestrator_agent_template
          PASS [   2.666s] ( 159/1609) ito-cli::init_more init_setup_coordination_branch_fails_without_origin_remote
          PASS [   2.856s] ( 160/1609) ito-cli::init_more init_setup_coordination_branch_creates_branch_on_origin
          PASS [   2.702s] ( 161/1609) ito-cli::init_more init_setup_coordination_branch_reports_ready_when_already_present
          PASS [   2.170s] ( 162/1609) ito-cli::init_more init_setup_coordination_branch_uses_configured_branch_name
          PASS [   5.643s] ( 163/1609) ito-cli::init_tmux init_writes_tmux_enabled_true_by_default
          PASS [   5.643s] ( 164/1609) ito-cli::init_tmux init_with_no_tmux_writes_tmux_enabled_false
          PASS [   5.760s] ( 165/1609) ito-cli::init_tmux init_uses_cascading_tmux_preference_from_global_config
          PASS [   5.767s] ( 166/1609) ito-cli::init_tmux init_update_preserves_existing_tmux_preference
          PASS [   6.773s] ( 167/1609) ito-cli::init_upgrade_more init_upgrade_flag_is_accepted
          PASS [   6.771s] ( 168/1609) ito-cli::init_upgrade_more init_upgrade_skips_and_warns_when_markers_missing
          PASS [   6.859s] ( 169/1609) ito-cli::init_upgrade_more init_update_does_not_error_on_existing_agents_md_without_markers
          PASS [   6.851s] ( 170/1609) ito-cli::init_upgrade_more init_upgrade_refreshes_marker_managed_block_and_preserves_user_content
          PASS [   6.893s] ( 171/1609) ito-cli::init_upgrade_more init_update_preserves_user_owned_files
          PASS [   8.551s] ( 172/1609) ito-cli::instructions_more agent_instruction_change_flag_reports_ambiguous_target
          PASS [   3.269s] ( 173/1609) ito-cli::instructions_more agent_instruction_proposal_honors_testing_policy_override
          PASS [   8.823s] ( 174/1609) ito-cli::instructions_more agent_instruction_apply_text_is_compact_and_has_trailing_newline
          PASS [   8.464s] ( 175/1609) ito-cli::instructions_more agent_instruction_change_flag_supports_slug_query
          PASS [   8.548s] ( 176/1609) ito-cli::instructions_more agent_instruction_change_flag_supports_shorthand
          PASS [   8.741s] ( 177/1609) ito-cli::instructions_more agent_instruction_archive_without_change_prints_generic_guidance
          PASS [   3.273s] ( 178/1609) ito-cli::instructions_more agent_instruction_proposal_without_change_prints_new_proposal_guide
          PASS [   8.808s] ( 179/1609) ito-cli::instructions_more agent_instruction_archive_with_invalid_change_fails
          PASS [   8.824s] ( 180/1609) ito-cli::instructions_more agent_instruction_archive_with_change_prints_targeted_instruction
          PASS [   3.273s] ( 181/1609) ito-cli::instructions_more agent_instruction_proposal_without_change_supports_json_output
          PASS [   3.275s] ( 182/1609) ito-cli::instructions_more agent_instruction_finish_with_change_prompts_for_archive
          PASS [   4.304s] ( 183/1609) ito-cli::instructions_more agent_instruction_review_requires_change_flag
          PASS [   4.290s] ( 184/1609) ito-cli::instructions_more agent_instruction_text_output_renders_artifact_envelope
          PASS [   4.333s] ( 185/1609) ito-cli::instructions_more agent_instruction_review_renders_review_template
          PASS [   4.956s] ( 186/1609) ito-cli::list_archive list_archive_reports_empty_archives
          PASS [   6.958s] ( 187/1609) ito-cli::list_archive list_archive_lists_archived_changes_only
          PASS [   6.985s] ( 188/1609) ito-cli::list_archive list_archive_json_lists_archived_changes_only
          PASS [   6.091s] ( 189/1609) ito-cli::misc_more list_errors_when_ito_changes_dir_missing
          PASS [   6.091s] ( 190/1609) ito-cli::misc_more plan_status_errors_when_roadmap_missing
          PASS [   6.092s] ( 191/1609) ito-cli::misc_more list_modules_empty_prints_hint
          PASS [   3.880s] ( 192/1609) ito-cli::misc_more status_change_flag_not_found_shows_suggestions
          PASS [   3.885s] ( 193/1609) ito-cli::misc_more show_unknown_item_offers_suggestions
          PASS [   6.142s] ( 194/1609) ito-cli::misc_more list_specs_empty_prints_sentence_even_for_json
          PASS [   6.151s] ( 195/1609) ito-cli::misc_more git_env_vars_do_not_override_runtime_root_detection
          PASS [   6.152s] ( 196/1609) ito-cli::misc_more commands_run_from_nested_dir_use_git_worktree_root
          PASS [   0.085s] ( 197/1609) ito-cli::misc_more status_schema_not_found_includes_available_schemas
          PASS [   0.088s] ( 198/1609) ito-cli::misc_more status_missing_change_flag_lists_available_changes
          PASS [   6.191s] ( 199/1609) ito-cli::misc_more show_module_errors_and_json_not_implemented
          PASS [   3.996s] ( 200/1609) ito-cli::misc_more show_spec_json_filters_and_requirement_index_errors
          PASS [   9.422s] ( 201/1609) ito-cli::list_regression list_default_text_and_json_shape_regression
          PASS [   9.422s] ( 202/1609) ito-cli::list_regression list_sort_regression
          PASS [   9.461s] ( 203/1609) ito-cli::list_regression list_filters_regression
          PASS [   5.835s] ( 204/1609) ito-cli::misc_more status_change_flag_reports_ambiguous_target
          PASS [   5.812s] ( 205/1609) ito-cli::misc_more status_change_flag_supports_module_scoped_slug_query
          PASS [   5.818s] ( 206/1609) ito-cli::misc_more status_change_flag_supports_shorthand_and_partial_match
          PASS [   6.174s] ( 207/1609) ito-cli::new_more new_change_covers_happy_and_error_paths
          PASS [   6.430s] ( 208/1609) ito-cli::parity_help_version version_prints_workspace_version
          PASS [   6.432s] ( 209/1609) ito-cli::parity_help_version help_prints_usage
          PASS [   7.680s] ( 210/1609) ito-cli::parity_tasks parity_tasks_init_writes_same_file
          PASS [   8.191s] ( 211/1609) ito-cli::path_more path_missing_subcommand_errors
          PASS [   8.567s] ( 212/1609) ito-cli::path_more path_errors_in_bare_repo
          PASS [   5.312s] ( 213/1609) ito-cli::path_more path_worktrees_root_requires_worktrees_enabled
          PASS [   8.623s] ( 214/1609) ito-cli::parity_tasks parity_tasks_status_next_start_complete_match_oracle
          PASS [   8.581s] ( 215/1609) ito-cli::path_more path_roots_json_includes_worktree_fields_when_enabled
          PASS [   5.356s] ( 216/1609) ito-cli::path_more path_worktree_requires_a_selector_flag
          PASS [   8.563s] ( 217/1609) ito-cli::path_more path_roots_text_renders_worktree_fields_when_available
          PASS [   5.779s] ( 218/1609) ito-cli::path_more path_worktrees_root_and_change_worktree_resolve_from_config
          PASS [   9.207s] ( 219/1609) ito-cli::path_more path_roots_are_absolute_in_initialized_repo
          PASS [   5.822s] ( 220/1609) ito-cli::plan_state_more plan_status_fails_without_roadmap
          PASS [   5.942s] ( 221/1609) ito-cli::plan_state_more plan_init_creates_structure
          PASS [   5.949s] ( 222/1609) ito-cli::plan_state_more plan_status_succeeds_after_init
          PASS [   5.286s] ( 223/1609) ito-cli::ralph_smoke ralph_change_flag_supports_shorthand_resolution
          PASS [   4.937s] ( 224/1609) ito-cli::ralph_smoke ralph_change_flag_supports_slug_query_resolution
          PASS [   4.886s] ( 225/1609) ito-cli::ralph_smoke ralph_file_flag_requires_readable_file
          PASS [   4.891s] ( 226/1609) ito-cli::ralph_smoke ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains
          PASS [   4.463s] ( 227/1609) ito-cli::ralph_smoke ralph_file_flag_runs_without_change_or_module
          PASS [   4.887s] ( 228/1609) ito-cli::ralph_smoke ralph_file_flag_allowed_without_change_or_module
          PASS [   4.891s] ( 229/1609) ito-cli::ralph_smoke ralph_continue_ready_exits_successfully_when_all_changes_complete
          PASS [   0.110s] ( 230/1609) ito-cli::ralph_smoke ralph_markdown_prd_source_marks_first_pending_task_complete
          PASS [   0.181s] ( 231/1609) ito-cli::ralph_smoke ralph_no_interactive_without_target_returns_clear_error
          PASS [   0.206s] ( 232/1609) ito-cli::ralph_smoke ralph_interactive_status_prompts_for_exactly_one_change
          PASS [   7.593s] ( 233/1609) ito-cli::ralph_smoke ralph_accepts_new_harness_names_for_status_flow
          PASS [   0.112s] ( 234/1609) ito-cli::ralph_smoke ralph_unknown_harness_returns_clear_error
          PASS [   0.168s] ( 235/1609) ito-cli::ralph_smoke ralph_yaml_source_marks_first_pending_task_complete
          PASS [   4.649s] ( 236/1609) ito-cli::ralph_smoke ralph_github_source_closes_issue_on_success
          PASS [   0.087s] ( 237/1609) ito-cli::show_specs_bundle show_specs_bundles_truth_specs_as_json_with_absolute_paths
          PASS [   0.389s] ( 238/1609) ito-cli::ralph_smoke ralph_stub_harness_writes_state_and_status_works
          PASS [   0.019s] ( 239/1609) ito-cli::source_file_size ito_cli_source_files_are_reasonably_sized
          PASS [   0.079s] ( 240/1609) ito-cli::show_specs_bundle show_specs_bundles_truth_specs_as_markdown_with_metadata
          PASS [   3.370s] ( 241/1609) ito-cli::ralph_smoke ralph_interactive_prompts_and_runs_selected_changes_sequentially
          PASS [   7.559s] ( 242/1609) ito-cli::ralph_smoke ralph_branch_per_task_requires_clean_worktree
          PASS [   0.077s] ( 243/1609) ito-cli::stats stats_counts_command_end_events
          PASS [   7.641s] ( 244/1609) ito-cli::ralph_smoke ralph_branch_per_task_creates_task_branch_for_prd_source
          PASS [   0.953s] ( 245/1609) ito-cli::ralph_smoke ralph_notify_emits_operator_notification_on_success
          PASS [   0.989s] ( 246/1609) ito-cli::ralph_smoke ralph_parallel_yaml_source_completes_grouped_tasks
          PASS [   6.853s] ( 247/1609) ito-cli::ralph_smoke ralph_browser_flag_injects_agent_browser_guidance_for_opencode
          PASS [   4.075s] ( 248/1609) ito-cli::ralph_smoke ralph_interactive_options_wizard_exit_on_error_stops_on_nonzero_harness_exit
          PASS [   1.088s] ( 249/1609) ito-cli::ralph_smoke ralph_sync_issue_updates_prd_back_to_github_issue
          PASS [   6.670s] ( 250/1609) ito-cli::ralph_smoke ralph_create_pr_uses_base_branch_and_fake_gh
          PASS [   5.399s] ( 251/1609) ito-cli::ralph_smoke ralph_interactive_options_wizard_prompts_for_missing_values_and_applies_them
          PASS [   3.495s] ( 252/1609) ito-cli::ralph_smoke ralph_parallel_preserves_worker_code_changes
          PASS [   5.871s] ( 253/1609) ito-cli::serve_more serve_errors_when_not_initialized
          PASS [   6.778s] ( 254/1609) ito-cli::tasks_more tasks_status_rejects_free_form_with_more_than_two_numbers
          PASS [   6.659s] ( 255/1609) ito-cli::tasks_more tasks_status_resolves_short_change_id
          PASS [   7.319s] ( 256/1609) ito-cli::tasks_more tasks_json_lists_are_sorted_by_task_id
          PASS [   6.857s] ( 257/1609) ito-cli::tasks_more tasks_status_resolves_free_form_two_numbers
          PASS [   7.420s] ( 258/1609) ito-cli::tasks_more tasks_commands_use_apply_tracks_filename_when_set
          PASS [   7.595s] ( 259/1609) ito-cli::tasks_more tasks_complete_supports_checkbox_compat_mode
          PASS [   7.664s] ( 260/1609) ito-cli::tasks_more tasks_error_paths_cover_more_branches
          PASS [   7.167s] ( 261/1609) ito-cli::tasks_more tasks_start_supports_checkbox_compat_mode_and_enforces_single_in_progress
          PASS [   7.453s] ( 262/1609) ito-cli::tasks_more tasks_next_supports_checkbox_compat_mode_and_shows_current_or_next
          PASS [   8.090s] ( 263/1609) ito-cli::tasks_more tasks_add_shelve_unshelve_show_cover_more_paths
          PASS [   8.308s] ( 264/1609) ito-cli::tasks_more tasks_commands_support_json_output
          PASS [  10.392s] ( 265/1609) ito-cli::show_specs_remote_mode show_specs_reads_backend_specs_without_local_markdown
          PASS [   9.042s] ( 266/1609) ito-cli::tasks_remote_mode remote_missing_tasks_commands_do_not_hard_fail
          PASS [   8.371s] ( 267/1609) ito-cli::tasks_remote_mode remote_task_start_updates_backend_without_local_tasks_file
          PASS [   9.186s] ( 268/1609) ito-cli::templates_schemas_export templates_help_includes_schemas_export
          PASS [   4.925s] ( 269/1609) ito-cli::templates_schemas_export templates_schemas_export_writes_embedded_files
          PASS [   6.705s] ( 270/1609) ito-cli::templates_schemas_export templates_schemas_export_skips_without_force_then_overwrites_with_force
          PASS [   6.193s] ( 271/1609) ito-cli::trace_more trace_partial_ids_json_shows_invalid_status
          PASS [   6.389s] ( 272/1609) ito-cli::trace_more trace_fully_covered_json_has_ready_status
          PASS [   6.371s] ( 273/1609) ito-cli::trace_more trace_missing_change_exits_nonzero
          PASS [   6.374s] ( 274/1609) ito-cli::trace_more trace_legacy_checkbox_change_shows_unavailable
          PASS [   6.396s] ( 275/1609) ito-cli::trace_more trace_fully_covered_exits_zero
          PASS [   5.842s] ( 276/1609) ito-cli::trace_more trace_unresolved_reference_shows_unresolved_in_output
          PASS [   6.114s] ( 277/1609) ito-cli::trace_more trace_uncovered_requirement_json_shows_uncovered_list
          PASS [   6.093s] ( 278/1609) ito-cli::trace_more trace_uncovered_requirement_shows_uncovered_in_output
          PASS [   6.240s] ( 279/1609) ito-cli::update_marker_scoped update_refuses_to_overwrite_partial_marker_pair
          PASS [   6.308s] ( 280/1609) ito-cli::update_marker_scoped update_still_refreshes_non_markdown_manifest_assets
          PASS [   6.379s] ( 281/1609) ito-cli::update_marker_scoped update_preserves_user_edits_after_end_marker_in_harness_skill
          PASS [   8.394s] ( 282/1609) ito-cli::update_marker_scoped update_preserves_user_edits_after_end_marker_in_harness_command
          PASS [   8.738s] ( 283/1609) ito-cli::update_marker_scoped second_update_is_a_noop_for_harness_skills
          PASS [   7.118s] ( 284/1609) ito-cli::update_smoke update_preserves_project_config_and_project_md
          PASS [   5.654s] ( 285/1609) ito-cli::update_smoke update_refreshes_opencode_plugin_and_preserves_user_config
          PASS [   5.659s] ( 286/1609) ito-cli::update_smoke update_refreshes_codex_audit_instruction_assets
          PASS [   5.663s] ( 287/1609) ito-cli::update_smoke update_preserves_user_guidance_and_user_prompt_files
          PASS [   5.665s] ( 288/1609) ito-cli::update_smoke update_refreshes_github_copilot_audit_assets
          PASS [   5.662s] ( 289/1609) ito-cli::update_smoke update_renders_agents_md_without_jinja2_syntax
          PASS [   7.229s] ( 290/1609) ito-cli::update_smoke update_installs_adapter_files_from_local_ito_skills
          PASS [   7.191s] ( 291/1609) ito-cli::update_smoke update_merges_claude_settings_without_clobbering_user_keys
          PASS [   6.440s] ( 292/1609) ito-cli::user_guidance_injection agent_instruction_includes_scoped_user_prompt_for_artifact
          PASS [   6.439s] ( 293/1609) ito-cli::user_guidance_injection agent_instruction_prefers_user_prompts_shared_guidance_file
          PASS [   6.439s] ( 294/1609) ito-cli::user_guidance_injection agent_instruction_includes_user_guidance_when_present
          PASS [   6.308s] ( 295/1609) ito-cli::validate_more validate_all_json_success_has_summary_and_by_type
          PASS [   3.484s] ( 296/1609) ito-cli::validate_more validate_type_module_special_cases_to_spec_by_id
          PASS [   6.278s] ( 297/1609) ito-cli::validate_more validate_ambiguous_item_is_an_error
          PASS [   3.517s] ( 298/1609) ito-cli::validate_more validate_unknown_spec_offers_suggestions
          PASS [   6.339s] ( 299/1609) ito-cli::validate_more validate_change_and_bulk_do_not_duplicate_schema_tracking_issues
          PASS [   3.593s] ( 300/1609) ito-cli::validate_more validate_change_runs_schema_rules_for_custom_schema
          PASS [   3.634s] ( 301/1609) ito-cli::validate_more validate_module_routes_and_error_paths
          PASS [   6.417s] ( 302/1609) ito-cli::validate_more validate_all_prints_failure_report_in_text_mode
          PASS [   6.629s] ( 303/1609) ito-cli::validate_more validate_change_reports_audit_drift_against_routed_storage
          PASS [   3.954s] ( 304/1609) ito-cli::validate_more validate_single_change_audit_flag_reports_only_audit_issues
          PASS [   6.061s] ( 305/1609) ito-cli::view_proposal view_proposal_help_shows_viewer_flag
          PASS [   6.064s] ( 306/1609) ito-cli::view_proposal view_proposal_html_viewer_errors_when_pandoc_missing
          PASS [   5.320s] ( 307/1609) ito-cli::view_proposal view_proposal_json_outputs_bundle
          PASS [   6.099s] ( 308/1609) ito-cli::view_proposal view_proposal_disabled_tmux_is_rejected
          PASS [   5.709s] ( 309/1609) ito-cli::view_proposal view_proposal_html_viewer_succeeds_with_stub_pandoc
          PASS [   7.536s] ( 310/1609) ito-cli::view_proposal view_proposal_html_viewer_is_recognized
          PASS [   4.859s] ( 311/1609) ito-cli::view_proposal view_proposal_unknown_viewer_is_rejected
          PASS [   4.876s] ( 312/1609) ito-cli::view_proposal view_proposal_unknown_change_fails
          PASS [   5.964s] ( 313/1609) ito-cli::bin/ito app::archive::tests::archive_follow_up_messages_cover_all_modes
          PASS [   5.534s] ( 314/1609) ito-cli::bin/ito app::instructions::tests::collect_tracking_diagnostic_counts_none_input
          PASS [   5.856s] ( 315/1609) ito-cli::bin/ito app::instructions::tests::collect_tracking_diagnostic_counts_empty_slice
          PASS [   5.964s] ( 316/1609) ito-cli::bin/ito app::archive::tests::only_filesystem_mode_requires_local_changes_dir
          PASS [   3.416s] ( 317/1609) ito-cli::bin/ito app::instructions::tests::json_get_returns_none_for_non_object_intermediate
          PASS [   3.419s] ( 318/1609) ito-cli::bin/ito app::instructions::tests::json_get_returns_none_for_missing_key
          PASS [   3.382s] ( 319/1609) ito-cli::bin/ito app::instructions::tests::json_get_traverses_nested_keys
          PASS [   5.534s] ( 320/1609) ito-cli::bin/ito app::instructions::tests::json_get_empty_keys_returns_root
          PASS [   5.835s] ( 321/1609) ito-cli::bin/ito app::instructions::tests::collect_tracking_diagnostic_counts_mixed_levels
          PASS [   2.994s] ( 322/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_checkout_siblings_sets_project_root
          PASS [   5.903s] ( 323/1609) ito-cli::bin/ito app::instructions::tests::collect_context_files_preserves_order
          PASS [   5.905s] ( 324/1609) ito-cli::bin/ito app::instructions::tests::backend_instruction_is_cli_first_for_remote_mode
          PASS [   0.015s] ( 325/1609) ito-cli::bin/ito app::list::tests::parse_sort_order_supports_separate_and_equals_forms
          PASS [   0.016s] ( 326/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_no_project_root_when_none_passed
          PASS [   0.016s] ( 327/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_parses_all_fields
          PASS [   0.015s] ( 328/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::load_worktree_result_from_config_returns_expected_defaults_and_values
          PASS [   0.016s] ( 329/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_parses_bare_control_siblings_strategy
          PASS [   0.017s] ( 330/1609) ito-cli::bin/ito app::list::tests::format_relative_time_covers_major_buckets
          PASS [   0.017s] ( 331/1609) ito-cli::bin/ito app::list::tests::format_task_status_handles_various_states
          PASS [   0.017s] ( 332/1609) ito-cli::bin/ito app::run::tests::removed_serve_api_replacement_preserves_flags_and_args
          PASS [   0.018s] ( 333/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::is_worktree_configured_detects_strategy_key
          PASS [   0.020s] ( 334/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_errors_when_enabled_missing_fields
          PASS [   0.020s] ( 335/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_disabled_and_preserves_other_keys
          PASS [   0.021s] ( 336/1609) ito-cli::bin/ito cli::ralph::ralph_tests::harness_arg_converts_to_core_harness_name
          PASS [   0.021s] ( 337/1609) ito-cli::bin/ito commands::backend::tests::resolve_project_root_returns_parent_directory
          PASS [   0.025s] ( 338/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_enabled_settings
          PASS [   0.023s] ( 339/1609) ito-cli::bin/ito commands::backend::tests::resolve_project_root_rejects_parentless_paths
          PASS [   0.026s] ( 340/1609) ito-cli::bin/ito app::worktree_wizard::worktree_wizard_tests::save_worktree_config_writes_config_and_runs_print_paths
          PASS [   0.042s] ( 341/1609) ito-cli::bin/ito app::list::tests::progress_filter_flags_are_mutually_exclusive
          PASS [   3.424s] ( 342/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_bare_control_siblings_calls_resolve
          PASS [   0.026s] ( 343/1609) ito-cli::bin/ito commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default
          PASS [   0.029s] ( 344/1609) ito-cli::bin/ito cli::cli_tests::parses_top_level_sync_force_flag
          PASS [   0.029s] ( 345/1609) ito-cli::bin/ito cli::cli_tests::parses_top_level_sync_command
          PASS [   0.027s] ( 346/1609) ito-cli::bin/ito commands::config::config_tests::config_schema_includes_coordination_sync_interval_default
          PASS [   0.026s] ( 347/1609) ito-cli::bin/ito commands::config::config_tests::handle_config_schema_writes_file_when_output_is_set
          PASS [   0.026s] ( 348/1609) ito-cli::bin/ito commands::config::config_tests::json_render_value_renders_common_json_types
          PASS [   0.012s] ( 349/1609) ito-cli::bin/ito commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_empty_ip
          PASS [   0.010s] ( 350/1609) ito-cli::bin/ito commands::serve::serve_tests::detect_tailscale_ip_with_cmd_success
          PASS [   0.011s] ( 351/1609) ito-cli::bin/ito commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_when_command_missing
          PASS [   0.014s] ( 352/1609) ito-cli::bin/ito commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_non_zero_exit
          PASS [   0.011s] ( 353/1609) ito-cli::bin/ito commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_missing
          PASS [   0.011s] ( 354/1609) ito-cli::bin/ito commands::serve::serve_tests::ensure_ito_dir_exists_ok_when_present
          PASS [   0.010s] ( 355/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::builds_allowlist_from_allow_org_args
          PASS [   0.011s] ( 356/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::builds_config_with_defaults
          PASS [   0.013s] ( 357/1609) ito-cli::bin/ito commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_path_is_file
          PASS [   0.011s] ( 358/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::load_backend_server_config_file_accepts_full_ito_json_config
          PASS [   0.012s] ( 359/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::load_backend_server_config_file_reads_toml
          PASS [   0.011s] ( 360/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_trailing_json_content
          PASS [   0.010s] ( 361/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::merge_allow_orgs_preserves_existing_repo_rules
          PASS [   0.011s] ( 362/1609) ito-cli::bin/ito commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_unknown_json_fields
          PASS [   0.010s] ( 363/1609) ito-cli::bin/ito diagnostics::tests::blocking_task_error_message_includes_rendered_errors
          PASS [   0.010s] ( 364/1609) ito-cli::bin/ito diagnostics::tests::format_path_line_includes_optional_line_number
          PASS [   0.010s] ( 365/1609) ito-cli::bin/ito diagnostics::tests::blocking_task_error_message_returns_none_when_no_errors
          PASS [   0.010s] ( 366/1609) ito-cli::bin/ito diagnostics::tests::render_task_diagnostics_filters_by_level_and_renders_task_id_when_present
          PASS [   0.010s] ( 367/1609) ito-cli::bin/ito diagnostics::tests::render_validation_issues_renders_rule_id_when_present
          PASS [   0.010s] ( 368/1609) ito-cli::bin/ito diagnostics::tests::render_validation_issues_renders_level_path_and_message
          PASS [   0.010s] ( 369/1609) ito-cli::bin/ito util::tests::command_id_maps_x_templates_to_templates
          PASS [   0.010s] ( 370/1609) ito-cli::bin/ito util::tests::command_id_maps_gr_to_grep
          PASS [   0.009s] ( 371/1609) ito-cli::bin/ito util::tests::command_id_uses_positional_args_and_normalizes_hyphens
          PASS [   0.010s] ( 372/1609) ito-cli::bin/ito util::tests::sanitize_args_redacts_equals_form
          PASS [   0.010s] ( 373/1609) ito-cli::bin/ito util::tests::sanitize_args_redacts_sensitive_flags
          PASS [   0.009s] ( 374/1609) ito-cli::bin/ito util::tests::split_csv_trims_parts
          PASS [   0.010s] ( 375/1609) ito-cli::bin/ito util::tests::sanitize_args_replaces_paths
          PASS [   5.302s] ( 376/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_checkout_subdir_sets_project_root
          PASS [   5.276s] ( 377/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_ignores_empty_strings
          PASS [   5.277s] ( 378/1609) ito-cli::bin/ito app::instructions::tests::worktree_config_defaults_when_no_worktrees_key
          PASS [   5.410s] ( 379/1609) ito-core audit::reader::reader_tests::reads_events_from_injected_store
          PASS [   5.416s] ( 380/1609) ito-core audit::mirror::tests::merge_jsonl_dedupes_and_appends_local_lines
          PASS [   5.415s] ( 381/1609) ito-core audit::mirror::tests::merge_jsonl_ignores_blank_lines
          PASS [   5.418s] ( 382/1609) ito-core audit::reconcile::tests::build_file_state_from_default_tasks_md
          PASS [   5.418s] ( 383/1609) ito-core audit::reconcile::tests::build_file_state_uses_apply_tracks_when_set
          PASS [   0.073s] ( 384/1609) ito-core audit::store::tests::internal_branch_location_keys_include_branch_identity
          PASS [   0.075s] ( 385/1609) ito-core audit::stream::tests::default_config_has_sensible_values
          PASS [   5.561s] ( 386/1609) ito-core audit::reader::reader_tests::read_from_missing_file_returns_empty
          PASS [   5.638s] ( 387/1609) ito-core audit::reader::reader_tests::skips_malformed_lines
          PASS [   5.642s] ( 388/1609) ito-core audit::reader::reader_tests::filter_by_scope
          PASS [   5.645s] ( 389/1609) ito-core audit::reader::reader_tests::combined_filters
          PASS [   5.646s] ( 390/1609) ito-core audit::reader::reader_tests::filter_by_entity_type
          PASS [   5.642s] ( 391/1609) ito-core audit::reader::reader_tests::skips_empty_lines
          PASS [   5.644s] ( 392/1609) ito-core audit::reader::reader_tests::read_parses_valid_events
          PASS [   5.646s] ( 393/1609) ito-core audit::reader::reader_tests::filter_by_operation
          PASS [   0.040s] ( 394/1609) ito-core audit::validate::tests::empty_events_no_issues
          PASS [   0.107s] ( 395/1609) ito-core audit::validate::tests::detect_status_transition_mismatch
          PASS [   0.109s] ( 396/1609) ito-core audit::validate::tests::detect_duplicate_create
          PASS [   0.107s] ( 397/1609) ito-core audit::validate::tests::different_scopes_are_independent
          PASS [   0.108s] ( 398/1609) ito-core audit::validate::tests::detect_timestamp_ordering_violation
          PASS [   0.346s] ( 399/1609) ito-core audit::reconcile::tests::reconcile_missing_tasks_file
          PASS [   0.073s] ( 400/1609) ito-core audit::validate::tests::no_issues_for_valid_sequence
          PASS [   0.346s] ( 401/1609) ito-core audit::reconcile::tests::reconcile_no_drift
          PASS [   0.042s] ( 402/1609) ito-core audit::worktree::tests::find_worktree_matching_branch
          PASS [   0.043s] ( 403/1609) ito-core audit::worktree::tests::find_worktree_bare_excluded
          PASS [   0.044s] ( 404/1609) ito-core audit::worktree::tests::aggregate_empty_worktrees
          PASS [   0.044s] ( 405/1609) ito-core audit::worktree::tests::find_worktree_multiple_returns_first_match
          PASS [   0.043s] ( 406/1609) ito-core audit::worktree::tests::parse_bare_worktree_excluded
          PASS [   0.043s] ( 407/1609) ito-core audit::worktree::tests::find_worktree_no_match
          PASS [   0.050s] ( 408/1609) ito-core audit::worktree::tests::parse_single_worktree
          PASS [   0.050s] ( 409/1609) ito-core audit::worktree::tests::parse_detached_head
          PASS [   0.050s] ( 410/1609) ito-core audit::worktree::tests::parse_multiple_worktrees
          PASS [   0.049s] ( 411/1609) ito-core audit::writer::tests::appends_events_to_existing_file
          PASS [   0.079s] ( 412/1609) ito-core audit::writer::tests::audit_log_path_resolves_correctly
          PASS [   0.079s] ( 413/1609) ito-core audit::worktree::tests::worktree_audit_log_path_resolves
          PASS [   0.042s] ( 414/1609) ito-core audit::writer::tests::best_effort_returns_ok_even_on_failure
          PASS [   0.049s] ( 415/1609) ito-core audit::writer::tests::creates_directory_and_file_on_first_write
          PASS [   0.068s] ( 416/1609) ito-core audit::writer::tests::each_line_is_valid_json
          PASS [   0.068s] ( 417/1609) ito-core audit::writer::tests::events_deserialize_back_correctly
          PASS [   0.051s] ( 418/1609) ito-core backend_change_repository::tests::get_delegates_to_reader
          PASS [   0.052s] ( 419/1609) ito-core audit::writer::tests::preserves_existing_content
          PASS [   0.292s] ( 420/1609) ito-core audit::stream::tests::poll_returns_empty_when_no_new_events
          PASS [   0.182s] ( 421/1609) ito-core audit::worktree::tests::aggregate_worktree_with_events
          PASS [   0.018s] ( 422/1609) ito-core backend_change_repository::tests::resolve_target_ambiguous
          PASS [   0.056s] ( 423/1609) ito-core backend_change_repository::tests::list_complete_filters_correctly
          PASS [   0.032s] ( 424/1609) ito-core backend_change_repository::tests::list_returns_all_changes
          PASS [   0.067s] ( 425/1609) ito-core backend_change_repository::tests::list_incomplete_filters_correctly
          PASS [   0.032s] ( 426/1609) ito-core backend_change_repository::tests::resolve_target_exact_match
          PASS [   0.402s] ( 427/1609) ito-core audit::stream::tests::poll_detects_new_events
          PASS [   0.033s] ( 428/1609) ito-core backend_change_repository::tests::resolve_target_prefix_match
          PASS [   0.033s] ( 429/1609) ito-core backend_client::tests::custom_backup_dir_is_used
          PASS [   0.035s] ( 430/1609) ito-core backend_change_repository::tests::resolve_target_not_found
          PASS [   0.041s] ( 431/1609) ito-core backend_client::tests::disabled_backend_returns_none
          PASS [   0.046s] ( 432/1609) ito-core backend_client::tests::enabled_backend_empty_token_fails
          PASS [   0.031s] ( 433/1609) ito-core backend_client::tests::enabled_backend_missing_token_fails
          PASS [   0.057s] ( 434/1609) ito-core backend_client::tests::default_backup_dir_uses_home
          PASS [   0.066s] ( 435/1609) ito-core backend_client::tests::enabled_backend_with_env_var_token_resolves
          PASS [   0.065s] ( 436/1609) ito-core backend_client::tests::enabled_backend_with_explicit_token_resolves
          PASS [   0.062s] ( 437/1609) ito-core backend_client::tests::is_retriable_status_checks
          PASS [   0.046s] ( 438/1609) ito-core backend_client::tests::project_api_prefix_formats_correctly
          PASS [   0.064s] ( 439/1609) ito-core backend_client::tests::idempotency_key_includes_operation
          PASS [   0.064s] ( 440/1609) ito-core backend_client::tests::env_var_token_takes_precedence_over_config_token
          PASS [   0.073s] ( 441/1609) ito-core backend_client::tests::project_namespace_empty_string_falls_through_to_env
          PASS [   0.072s] ( 442/1609) ito-core backend_client::tests::project_namespace_env_takes_precedence_over_config
          PASS [   0.082s] ( 443/1609) ito-core backend_client::tests::project_namespace_from_config
          PASS [   0.047s] ( 444/1609) ito-core backend_coordination::tests::allocate_no_work
          PASS [   0.047s] ( 445/1609) ito-core backend_client::tests::project_namespace_missing_repo_fails
          PASS [   0.048s] ( 446/1609) ito-core backend_client::tests::project_namespace_from_env_vars
          PASS [   0.049s] ( 447/1609) ito-core backend_client::tests::project_namespace_missing_org_fails
          PASS [   0.051s] ( 448/1609) ito-core backend_coordination::tests::allocate_with_work
          PASS [   2.011s] ( 449/1609) ito-core audit::reconcile::tests::reconcile_empty_log
          PASS [   0.056s] ( 450/1609) ito-core backend_coordination::tests::archive_with_backend_backend_unavailable
          PASS [   0.024s] ( 451/1609) ito-core backend_coordination::tests::archive_with_backend_skip_specs
          PASS [   0.024s] ( 452/1609) ito-core backend_coordination::tests::archive_with_backend_happy_path
          PASS [   0.045s] ( 453/1609) ito-core backend_coordination::tests::claim_success
          PASS [   0.043s] ( 454/1609) ito-core backend_health::tests::backend_health_status_default_is_all_false
          PASS [   0.044s] ( 455/1609) ito-core backend_coordination::tests::release_success
          PASS [   0.046s] ( 456/1609) ito-core backend_coordination::tests::claim_conflict
          PASS [   0.045s] ( 457/1609) ito-core backend_coordination::tests::is_backend_unavailable_detects_process_error
          PASS [   0.043s] ( 458/1609) ito-core backend_health::tests::backend_health_status_serializes_error_state
          PASS [   0.043s] ( 459/1609) ito-core backend_health::tests::backend_health_status_serializes_to_json
          PASS [   0.040s] ( 460/1609) ito-core backend_http::backend_http_tests::get_requests_are_retried_by_default
          PASS [   0.053s] ( 461/1609) ito-core backend_http::backend_http_tests::audit_ingest_posts_can_opt_into_retries
          PASS [   0.057s] ( 462/1609) ito-core backend_http::backend_http_tests::archived_task_fallback_only_treats_not_found_as_missing
          PASS [   0.039s] ( 463/1609) ito-core backend_http::backend_http_tests::post_requests_are_not_retried_by_default
          PASS [   0.039s] ( 464/1609) ito-core backend_sync::tests::backend_error_mapping_produces_correct_error_types
          PASS [   2.091s] ( 465/1609) ito-core audit::reconcile::tests::reconcile_detects_drift
          PASS [   0.041s] ( 466/1609) ito-core backend_http::backend_http_tests::optional_task_text_body_uses_empty_object_when_absent
          PASS [   0.041s] ( 467/1609) ito-core backend_http::backend_http_tests::parse_timestamp_returns_error_for_invalid_rfc3339
          PASS [   0.038s] ( 468/1609) ito-core backend_sync::tests::path_traversal_in_change_id_rejected
          PASS [   0.043s] ( 469/1609) ito-core backend_http::backend_http_tests::optional_task_text_body_serializes_payload_when_present
          PASS [   0.020s] ( 470/1609) ito-core backend_sync::tests::push_conflict_returns_actionable_error
          PASS [   0.040s] ( 471/1609) ito-core backend_sync::tests::path_traversal_in_capability_rejected
          PASS [   0.025s] ( 472/1609) ito-core backend_sync::tests::pull_writes_artifacts_locally
          PASS [   0.038s] ( 473/1609) ito-core backend_sync::tests::pull_creates_backup
          PASS [   0.041s] ( 474/1609) ito-core backend_sync::tests::push_missing_change_dir_fails
          PASS [   0.041s] ( 475/1609) ito-core backend_task_repository::tests::has_tasks_detects_content
          PASS [   0.041s] ( 476/1609) ito-core backend_task_repository::tests::checkbox_tasks_parsed_correctly
          PASS [   0.039s] ( 477/1609) ito-core backend_task_repository::tests::missing_tasks_returns_empty
          PASS [   0.041s] ( 478/1609) ito-core backend_task_repository::tests::get_task_counts_from_backend
          PASS [   0.040s] ( 479/1609) ito-core backend_task_repository::tests::has_tasks_empty_content
          PASS [   0.042s] ( 480/1609) ito-core backend_sync::tests::read_local_bundle_sorts_specs
          PASS [   0.039s] ( 481/1609) ito-core change_repository::tests::resolve_target_includes_archive_when_requested
          PASS [   0.045s] ( 482/1609) ito-core backend_sync::tests::push_sends_local_bundle
          PASS [   0.042s] ( 483/1609) ito-core change_repository::tests::exists_and_get_work
          PASS [   0.042s] ( 484/1609) ito-core change_repository::tests::list_skips_archive_dir
          PASS [   0.032s] ( 485/1609) ito-core config::tests::validate_config_value_accepts_positive_sync_interval
          PASS [   0.032s] ( 486/1609) ito-core config::tests::validate_config_value_accepts_archive_main_integration_mode
          PASS [   0.034s] ( 487/1609) ito-core config::tests::resolve_worktree_template_defaults_uses_defaults_when_missing
          PASS [   0.037s] ( 488/1609) ito-core config::tests::is_valid_worktree_strategy_checks_correctly
          PASS [   0.037s] ( 489/1609) ito-core config::tests::is_valid_repository_mode_checks_correctly
          PASS [   0.035s] ( 490/1609) ito-core config::tests::skill_id_resolves_returns_false_when_no_paths_exist
          PASS [   0.038s] ( 491/1609) ito-core config::tests::is_valid_integration_mode_checks_correctly
          PASS [   0.039s] ( 492/1609) ito-core config::tests::resolve_worktree_template_defaults_reads_overrides
          PASS [   0.043s] ( 493/1609) ito-core change_repository::tests::resolve_target_reports_ambiguity
          PASS [   0.046s] ( 494/1609) ito-core change_repository::tests::resolve_target_module_scoped_query
          PASS [   0.045s] ( 495/1609) ito-core change_repository::tests::suggest_targets_prioritizes_slug_matches
          PASS [   0.023s] ( 496/1609) ito-core config::tests::validate_config_value_accepts_unknown_keys
          PASS [   0.023s] ( 497/1609) ito-core config::tests::validate_config_value_accepts_valid_coordination_branch_name
          PASS [   0.023s] ( 498/1609) ito-core config::tests::validate_config_value_accepts_valid_audit_mirror_branch_name
          PASS [   0.022s] ( 499/1609) ito-core config::tests::validate_config_value_accepts_valid_memory_kind
          PASS [   0.023s] ( 500/1609) ito-core config::tests::validate_config_value_accepts_valid_integration_mode
          PASS [   0.026s] ( 501/1609) ito-core config::tests::validate_config_value_rejects_empty_memory_command_template
          PASS [   0.023s] ( 502/1609) ito-core config::tests::validate_config_value_rejects_empty_memory_skill_id
          PASS [   0.029s] ( 503/1609) ito-core config::tests::validate_config_value_accepts_valid_repository_mode
          PASS [   0.022s] ( 504/1609) ito-core config::tests::validate_config_value_rejects_invalid_archive_main_integration_mode
          PASS [   0.028s] ( 505/1609) ito-core config::tests::validate_config_value_accepts_valid_strategy
          PASS [   0.022s] ( 506/1609) ito-core config::tests::validate_config_value_rejects_invalid_audit_mirror_branch_name
          PASS [   0.040s] ( 507/1609) ito-core config::tests::validate_config_value_rejects_invalid_strategy
          PASS [   0.040s] ( 508/1609) ito-core config::tests::validate_config_value_rejects_lock_suffix_in_path_segment
          PASS [   0.040s] ( 509/1609) ito-core config::tests::validate_config_value_rejects_invalid_coordination_branch_name
          PASS [   0.041s] ( 510/1609) ito-core config::tests::validate_config_value_rejects_invalid_integration_mode
          PASS [   0.041s] ( 511/1609) ito-core config::tests::validate_config_value_rejects_invalid_repository_mode
          PASS [   0.038s] ( 512/1609) ito-core config::tests::validate_config_value_rejects_memory_op_unknown_kind
          PASS [   0.037s] ( 513/1609) ito-core config::tests::validate_config_value_rejects_unknown_memory_kind
          PASS [   0.037s] ( 514/1609) ito-core config::tests::validate_config_value_rejects_zero_sync_interval
          PASS [   0.038s] ( 515/1609) ito-core config::tests::validate_config_value_rejects_unknown_memory_op_key
          PASS [   0.070s] ( 516/1609) ito-core config::tests::validate_config_value_rejects_memory_op_missing_required_field
          PASS [   0.036s] ( 517/1609) ito-core config::tests::validate_memory_config_passes_when_no_skill_provider
          PASS [   0.070s] ( 518/1609) ito-core config::tests::validate_config_value_rejects_non_string_strategy
          PASS [   0.037s] ( 519/1609) ito-core config::tests::validate_memory_config_passes_when_skill_resolves_in_flat_layout
          PASS [   0.040s] ( 520/1609) ito-core config::tests::validate_memory_config_rejects_missing_skill
          PASS [   0.040s] ( 521/1609) ito-core coordination::tests::create_dir_link_creates_symlink
          PASS [   0.041s] ( 522/1609) ito-core config::tests::validate_memory_config_passes_when_skill_resolves_in_grouped_layout
          PASS [   0.039s] ( 523/1609) ito-core coordination::tests::format_message_embedded_is_none
          PASS [   0.038s] ( 524/1609) ito-core coordination::tests::format_message_healthy_is_none
          PASS [   0.040s] ( 525/1609) ito-core coordination::tests::create_dir_link_fails_when_dst_exists
          PASS [   0.040s] ( 526/1609) ito-core coordination::tests::format_message_broken_symlinks_contains_paths_and_hint
          PASS [   0.008s] ( 527/1609) ito-core coordination::tests::format_message_worktree_missing_contains_path_and_hint
          PASS [   0.008s] ( 528/1609) ito-core coordination::tests::format_message_wrong_target_contains_paths_and_hint
          PASS [   0.009s] ( 529/1609) ito-core coordination::tests::format_message_not_wired_contains_dir_and_hint
          PASS [   0.029s] ( 530/1609) ito-core coordination::tests::gitignore_no_duplicates_on_second_call
          PASS [   0.030s] ( 531/1609) ito-core coordination::tests::gitignore_entries_added_when_missing
          PASS [   0.029s] ( 532/1609) ito-core coordination::tests::gitignore_preserves_existing_content
          PASS [   0.034s] ( 533/1609) ito-core coordination::tests::gitignore_created_when_absent
          PASS [   0.029s] ( 534/1609) ito-core coordination::tests::gitignore_skips_already_present_entries
          PASS [   0.030s] ( 535/1609) ito-core coordination::tests::health_broken_symlinks_when_target_missing
          PASS [   0.030s] ( 536/1609) ito-core coordination::tests::health_worktree_missing_when_dir_absent
          PASS [   0.036s] ( 537/1609) ito-core coordination::tests::health_healthy_when_all_symlinks_correct
          PASS [   0.066s] ( 538/1609) ito-core coordination::tests::health_not_wired_when_real_dirs_present
          PASS [   0.066s] ( 539/1609) ito-core coordination::tests::health_missing_link_is_not_wired
          PASS [   0.068s] ( 540/1609) ito-core coordination::tests::health_embedded_returns_embedded
          PASS [   0.042s] ( 541/1609) ito-core coordination::tests::health_wrong_target_when_symlink_points_elsewhere
          PASS [   0.045s] ( 542/1609) ito-core coordination::tests::remove_is_noop_when_dirs_absent
          PASS [   0.048s] ( 543/1609) ito-core coordination::tests::remove_is_noop_for_real_dirs
          PASS [   0.049s] ( 544/1609) ito-core coordination::tests::wire_creates_symlinks_for_all_dirs
          PASS [   0.048s] ( 545/1609) ito-core coordination::tests::wire_handles_empty_real_dir
          PASS [   0.041s] ( 546/1609) ito-core coordination::tests::wire_migrates_real_dir_content
          PASS [   0.048s] ( 547/1609) ito-core coordination::tests::wire_is_idempotent
          PASS [   0.051s] ( 548/1609) ito-core coordination::tests::remove_restores_real_dirs_with_content
          PASS [   0.038s] ( 549/1609) ito-core coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails
          PASS [   0.039s] ( 550/1609) ito-core coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged
          PASS [   0.037s] ( 551/1609) ito-core coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist
          PASS [   0.040s] ( 552/1609) ito-core coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails
          PASS [   0.032s] ( 553/1609) ito-core coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails
          PASS [   0.037s] ( 554/1609) ito-core coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback
          PASS [   0.034s] ( 555/1609) ito-core coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote
          PASS [   0.035s] ( 556/1609) ito-core coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo
          PASS [   0.041s] ( 557/1609) ito-core coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local
          PASS [   0.046s] ( 558/1609) ito-core coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly
          PASS [   0.047s] ( 559/1609) ito-core coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured
          PASS [   0.039s] ( 560/1609) ito-core coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails
          PASS [   0.039s] ( 561/1609) ito-core coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch
          PASS [   2.423s] ( 562/1609) ito-core audit::reconcile::tests::reconcile_fix_writes_compensating_events
          PASS [   0.063s] ( 563/1609) ito-core coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails
          PASS [   0.074s] ( 564/1609) ito-core coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails
          PASS [   0.076s] ( 565/1609) ito-core coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails
          PASS [   0.077s] ( 566/1609) ito-core coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded
          PASS [   0.064s] ( 567/1609) ito-core coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune
          PASS [   0.048s] ( 568/1609) ito-core coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy
          PASS [   0.048s] ( 569/1609) ito-core coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded
          PASS [   0.087s] ( 570/1609) ito-core coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit
          PASS [   0.135s] ( 571/1609) ito-core coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist
          PASS [   0.068s] ( 572/1609) ito-core coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target
          PASS [   0.062s] ( 573/1609) ito-core create::create_sub_module_tests::create_sub_module_errors_on_unknown_parent_module
          PASS [   0.069s] ( 574/1609) ito-core create::create_sub_module_tests::create_sub_module_accepts_full_module_folder_name
          PASS [   0.069s] ( 575/1609) ito-core create::create_sub_module_tests::create_sub_module_creates_directory_and_module_md
          PASS [   0.025s] ( 576/1609) ito-core create::create_sub_module_tests::create_sub_module_rejects_invalid_name
          PASS [   0.071s] ( 577/1609) ito-core coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean
          PASS [   0.071s] ( 578/1609) ito-core create::create_sub_module_tests::create_sub_module_allocates_sequential_numbers
          PASS [   0.066s] ( 579/1609) ito-core create::create_sub_module_tests::create_sub_module_errors_on_duplicate_name
          PASS [   0.996s] ( 580/1609) ito-core audit::stream::tests::read_initial_events_returns_last_n
          PASS [   0.046s] ( 581/1609) ito-core create::create_sub_module_tests::create_sub_module_with_description_writes_purpose
          PASS [   0.035s] ( 582/1609) ito-core event_forwarder::tests::checkpoint_missing_returns_zero
          PASS [   0.039s] ( 583/1609) ito-core distribution::tests::pi_adapter_asset_exists_in_embedded_templates
          PASS [   0.037s] ( 584/1609) ito-core distribution::tests::pi_manifests_commands_match_opencode_commands
          PASS [   0.040s] ( 585/1609) ito-core distribution::tests::ensure_manifest_script_is_executable_only_adds_execute_bits
          PASS [   0.037s] ( 586/1609) ito-core errors::tests::core_error_helpers_construct_expected_variants
          PASS [   0.038s] ( 587/1609) ito-core distribution::tests::pi_manifests_includes_adapter_skills_and_commands
          PASS [   0.038s] ( 588/1609) ito-core distribution::tests::pi_manifests_skills_match_opencode_skills
          PASS [   0.041s] ( 589/1609) ito-core distribution::tests::pi_agent_templates_discoverable
          PASS [   1.223s] ( 590/1609) ito-core audit::store::tests::legacy_worktree_log_is_removed_after_successful_migration
          PASS [   0.040s] ( 591/1609) ito-core event_forwarder::tests::checkpoint_roundtrip
          PASS [   0.039s] ( 592/1609) ito-core event_forwarder::tests::forward_result_equality
          PASS [   0.161s] ( 593/1609) ito-core event_forwarder::tests::is_retriable_backend_error_checks
          PASS [   0.202s] ( 594/1609) ito-core event_forwarder::tests::forward_no_events_returns_zero
          PASS [   0.087s] ( 595/1609) ito-core front_matter::tests::created_at_dt_returns_none_for_invalid_timestamp
          PASS [   0.088s] ( 596/1609) ito-core front_matter::tests::body_sha256_is_deterministic
          PASS [   0.072s] ( 597/1609) ito-core front_matter::tests::format_timestamp_produces_rfc3339
          PASS [   0.073s] ( 598/1609) ito-core front_matter::tests::created_at_dt_returns_none_when_absent
          PASS [   0.553s] ( 599/1609) ito-core coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists
          PASS [   0.097s] ( 600/1609) ito-core front_matter::tests::parse_empty_front_matter
          PASS [   0.460s] ( 601/1609) ito-core event_forwarder::tests::forward_retries_transient_failure
          PASS [   0.122s] ( 602/1609) ito-core front_matter::tests::parse_delimiter_with_extra_text_on_first_line
          PASS [   0.172s] ( 603/1609) ito-core front_matter::tests::parse_invalid_yaml
          PASS [   0.544s] ( 604/1609) ito-core event_forwarder::tests::forward_skips_when_fully_forwarded
          PASS [   0.547s] ( 605/1609) ito-core event_forwarder::tests::forward_reports_duplicates
          PASS [   0.089s] ( 606/1609) ito-core front_matter::tests::parse_no_front_matter
          PASS [   0.092s] ( 607/1609) ito-core front_matter::tests::parse_no_closing_delimiter
          PASS [   0.080s] ( 608/1609) ito-core front_matter::tests::parse_preserves_extra_fields
          PASS [   0.072s] ( 609/1609) ito-core front_matter::tests::parse_valid_front_matter
          PASS [   0.617s] ( 610/1609) ito-core event_forwarder::tests::forward_persists_checkpoint_per_batch
          PASS [   0.085s] ( 611/1609) ito-core front_matter::tests::parse_with_integrity
          PASS [   0.103s] ( 612/1609) ito-core front_matter::tests::roundtrip_write_parse
          PASS [   0.105s] ( 613/1609) ito-core front_matter::tests::update_integrity_sets_checksum
          PASS [   0.125s] ( 614/1609) ito-core front_matter::tests::touch_creates_new_front_matter
          PASS [   0.124s] ( 615/1609) ito-core front_matter::tests::touch_updates_existing
          PASS [   0.065s] ( 616/1609) ito-core front_matter::tests::validate_id_fails_on_mismatch
          PASS [   0.677s] ( 617/1609) ito-core event_forwarder::tests::forward_sends_all_new_events
          PASS [   0.680s] ( 618/1609) ito-core event_forwarder::tests::forward_respects_checkpoint
          PASS [   0.077s] ( 619/1609) ito-core front_matter::tests::validate_id_passes_when_absent
          PASS [   0.056s] ( 620/1609) ito-core front_matter::tests::validate_integrity_passes_when_matching
          PASS [   0.074s] ( 621/1609) ito-core front_matter::tests::validate_integrity_fails_on_mismatch
          PASS [   0.092s] ( 622/1609) ito-core front_matter::tests::validate_id_passes_when_matching
          PASS [   0.049s] ( 623/1609) ito-core front_matter::tests::write_no_front_matter_returns_body
          PASS [   0.052s] ( 624/1609) ito-core front_matter::tests::validate_integrity_passes_when_no_checksum
          PASS [   0.052s] ( 625/1609) ito-core fs_project_store::tests::change_repository_returns_box_trait
          PASS [   0.057s] ( 626/1609) ito-core fs_project_store::tests::ito_path_rejects_path_traversal
          PASS [   0.071s] ( 627/1609) ito-core fs_project_store::tests::ensure_project_creates_directory
          PASS [   0.069s] ( 628/1609) ito-core fs_project_store::tests::ito_path_resolves_correctly
          PASS [   0.041s] ( 629/1609) ito-core fs_project_store::tests::project_exists_returns_false_for_missing
          PASS [   0.041s] ( 630/1609) ito-core fs_project_store::tests::store_is_send_sync
          PASS [   0.041s] ( 631/1609) ito-core fs_project_store::tests::task_repository_returns_box_trait
          PASS [   0.044s] ( 632/1609) ito-core fs_project_store::tests::module_repository_returns_box_trait
          PASS [   0.774s] ( 633/1609) ito-core event_forwarder::tests::forward_batches_correctly
          PASS [   0.034s] ( 634/1609) ito-core git::tests::fetch_coordination_branch_succeeds_on_clean_fetch
          PASS [   0.045s] ( 635/1609) ito-core git::tests::fetch_coordination_branch_classifies_missing_remote_branch
          PASS [   0.738s] ( 636/1609) ito-core event_forwarder::tests::forward_stops_on_permanent_failure
          PASS [   0.022s] ( 637/1609) ito-core git::tests::push_coordination_branch_classifies_missing_remote_configuration
          PASS [   0.044s] ( 638/1609) ito-core git::tests::fetch_coordination_branch_classifies_missing_remote_configuration
          PASS [   1.998s] ( 639/1609) ito-core audit::store::tests::read_all_merges_and_replays_fallback_events_when_branch_recovers
          PASS [   0.030s] ( 640/1609) ito-core git::tests::push_coordination_branch_classifies_non_fast_forward_rejection
          PASS [   0.037s] ( 641/1609) ito-core git::tests::push_coordination_branch_classifies_protection_rejection
          PASS [   0.031s] ( 642/1609) ito-core git::tests::setup_coordination_branch_reports_missing_origin_when_create_push_fails
          PASS [   0.036s] ( 643/1609) ito-core git::tests::setup_coordination_branch_creates_branch_when_remote_missing
          PASS [   0.036s] ( 644/1609) ito-core git::tests::setup_coordination_branch_fails_when_not_git_worktree
          PASS [   0.035s] ( 645/1609) ito-core git_remote::tests::ignores_empty_config_strings_and_falls_back_to_remote
          PASS [   0.037s] ( 646/1609) ito-core git::tests::setup_coordination_branch_returns_ready_when_remote_branch_exists
          PASS [   0.036s] ( 647/1609) ito-core git_remote::tests::falls_back_to_remote_when_config_repo_missing
          PASS [   0.037s] ( 648/1609) ito-core git_remote::tests::falls_back_to_remote_when_config_empty
          PASS [   0.016s] ( 649/1609) ito-core git_remote::tests::returns_config_values_when_both_set
          PASS [   0.031s] ( 650/1609) ito-core git_remote::tests::reexport_delegates_to_common_parser
          PASS [   0.037s] ( 651/1609) ito-core git_remote::tests::falls_back_to_remote_when_config_org_missing
          PASS [   0.033s] ( 652/1609) ito-core git_remote::tests::returns_none_when_remote_command_fails
          PASS [   0.033s] ( 653/1609) ito-core git_remote::tests::returns_none_when_remote_url_unrecognised
          PASS [   0.034s] ( 654/1609) ito-core git_remote::tests::returns_none_when_remote_output_is_empty
          PASS [   0.033s] ( 655/1609) ito-core grep::tests::collect_change_artifact_files_finds_all_md_files
          PASS [   0.035s] ( 656/1609) ito-core harness::claude_code::tests::build_args_with_allow_all
          PASS [   0.036s] ( 657/1609) ito-core harness::claude_code::tests::binary_is_claude
          PASS [   0.036s] ( 658/1609) ito-core grep::tests::search_files_rejects_invalid_regex
          PASS [   0.079s] ( 659/1609) ito-core git::tests::setup_coordination_branch_core_wraps_process_error
          PASS [   0.016s] ( 660/1609) ito-core harness::claude_code::tests::build_args_without_model
          PASS [   0.043s] ( 661/1609) ito-core grep::tests::search_files_finds_matching_lines
          PASS [   0.043s] ( 662/1609) ito-core grep::tests::search_files_returns_empty_for_no_matches
          PASS [   0.015s] ( 663/1609) ito-core harness::claude_code::tests::harness_name_is_claude
          PASS [   0.016s] ( 664/1609) ito-core harness::claude_code::tests::build_args_without_allow_all
          PASS [   0.015s] ( 665/1609) ito-core harness::codex::tests::binary_is_codex
          PASS [   0.044s] ( 666/1609) ito-core grep::tests::search_files_respects_limit
          PASS [   0.044s] ( 667/1609) ito-core grep::tests::search_files_includes_correct_line_numbers
          PASS [   0.012s] ( 668/1609) ito-core harness::codex::tests::build_args_with_allow_all
          PASS [   0.038s] ( 669/1609) ito-core harness::codex::tests::harness_name_is_codex
          PASS [   0.038s] ( 670/1609) ito-core harness::codex::tests::build_args_without_allow_all
          PASS [   0.038s] ( 671/1609) ito-core harness::github_copilot::tests::binary_is_copilot
          PASS [   0.031s] ( 672/1609) ito-core harness::stub::tests::name_returns_stub
          PASS [   0.037s] ( 673/1609) ito-core harness::github_copilot::tests::build_args_with_allow_all
          PASS [   0.037s] ( 674/1609) ito-core harness::opencode::tests::binary_is_opencode
          PASS [   0.038s] ( 675/1609) ito-core harness::github_copilot::tests::build_args_without_allow_all
          PASS [   1.078s] ( 676/1609) ito-core coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree
          PASS [   0.038s] ( 677/1609) ito-core harness::stub::tests::from_env_or_default_with_explicit_path
          PASS [   0.038s] ( 678/1609) ito-core harness::opencode::tests::build_args_without_model
          PASS [   0.038s] ( 679/1609) ito-core harness::opencode::tests::harness_name_is_opencode
          PASS [   0.039s] ( 680/1609) ito-core harness::github_copilot::tests::harness_name_is_github_copilot
          PASS [   0.039s] ( 681/1609) ito-core harness::opencode::tests::build_args_with_model
          PASS [   0.010s] ( 682/1609) ito-core harness::stub::tests::run_sets_nonzero_duration
          PASS [   0.028s] ( 683/1609) ito-core harness::stub::tests::run_sets_timed_out_false
          PASS [   0.028s] ( 684/1609) ito-core harness::stub::tests::streams_output_returns_false
          PASS [   0.023s] ( 685/1609) ito-core harness::types::tests::display_matches_as_str
          PASS [   0.024s] ( 686/1609) ito-core harness::types::tests::from_str_invalid_returns_error
          PASS [   0.026s] ( 687/1609) ito-core harness::types::tests::from_str_valid_variants
          PASS [   0.026s] ( 688/1609) ito-core installers::json_tests::classify_project_file_ownership_handles_user_owned_paths
          PASS [   0.029s] ( 689/1609) ito-core harness::types::tests::is_not_retriable_for_normal_codes
          PASS [   0.032s] ( 690/1609) ito-core harness::types::tests::is_retriable_for_all_retriable_codes
          PASS [   0.040s] ( 691/1609) ito-core harness::types::tests::harness_help_matches_user_facing
          PASS [   0.042s] ( 692/1609) ito-core harness::types::tests::as_str_all_variants
          PASS [   0.054s] ( 693/1609) ito-core harness::types::tests::parse_error_display
          PASS [   0.033s] ( 694/1609) ito-core installers::markers::tests::errors_when_only_one_marker_found
          PASS [   0.054s] ( 695/1609) ito-core installers::json_tests::merge_json_objects_appends_and_deduplicates_array_entries
          PASS [   0.052s] ( 696/1609) ito-core installers::json_tests::merge_json_objects_keeps_existing_and_adds_template_keys
          PASS [   0.035s] ( 697/1609) ito-core installers::json_tests::write_claude_settings_preserves_invalid_json_on_update
          PASS [   0.037s] ( 698/1609) ito-core installers::json_tests::write_claude_settings_merges_existing_file_on_update
          PASS [   0.035s] ( 699/1609) ito-core installers::markers::tests::idempotent_when_applying_same_content_twice
          PASS [   0.030s] ( 700/1609) ito-core installers::markers::tests::replaces_existing_block_preserving_unmanaged_content
          PASS [   0.027s] ( 701/1609) ito-core installers::markers::tests::updates_file_on_disk
          PASS [   0.033s] ( 702/1609) ito-core installers::markers::tests::marker_must_be_on_own_line
          PASS [   0.019s] ( 703/1609) ito-core installers::tests::gitignore_both_session_entries
          PASS [   0.021s] ( 704/1609) ito-core installers::tests::gitignore_audit_session_added
          PASS [   0.027s] ( 705/1609) ito-core installers::tests::gitignore_exact_line_matching_trims_whitespace
          PASS [   0.057s] ( 706/1609) ito-core installers::markers::tests::inserts_block_when_missing
          PASS [   0.028s] ( 707/1609) ito-core installers::tests::gitignore_does_not_duplicate_on_repeated_calls
          PASS [   0.028s] ( 708/1609) ito-core installers::tests::gitignore_full_audit_setup
          PASS [   0.030s] ( 709/1609) ito-core installers::tests::gitignore_created_when_missing
          PASS [   0.027s] ( 710/1609) ito-core installers::tests::release_tag_is_prefixed_with_v
          PASS [   0.028s] ( 711/1609) ito-core installers::tests::should_install_project_rel_filters_pi
          PASS [   0.032s] ( 712/1609) ito-core installers::tests::gitignore_ignores_local_configs
          PASS [   0.030s] ( 713/1609) ito-core installers::tests::gitignore_noop_when_already_present
          PASS [   0.031s] ( 714/1609) ito-core installers::tests::gitignore_legacy_audit_events_unignore_removed
          PASS [   0.031s] ( 715/1609) ito-core installers::tests::gitignore_legacy_audit_events_unignore_noop_when_absent
          PASS [   0.030s] ( 716/1609) ito-core installers::tests::gitignore_preserves_existing_content_and_adds_newline_if_missing
          PASS [   0.030s] ( 717/1609) ito-core installers::tests::should_install_project_rel_filters_by_tool_id
          PASS [   0.010s] ( 718/1609) ito-core installers::tests::update_agent_model_field_updates_frontmatter_when_present
          PASS [   0.022s] ( 719/1609) ito-core installers::tests::update_model_in_yaml_replaces_or_inserts
          PASS [   0.018s] ( 720/1609) ito-core installers::tests::write_one_non_marker_files_skip_on_init_update_mode
          PASS [   0.023s] ( 721/1609) ito-core installers::tests::write_one_marker_managed_files_error_when_markers_missing_in_update_mode
          PASS [   0.018s] ( 722/1609) ito-core installers::tests::write_one_non_marker_user_owned_files_preserve_on_update_mode
          PASS [   0.023s] ( 723/1609) ito-core installers::tests::write_one_marker_managed_files_refuse_overwrite_without_markers
          PASS [   0.023s] ( 724/1609) ito-core installers::tests::write_one_marker_managed_files_update_existing_markers
          PASS [   0.019s] ( 725/1609) ito-core installers::tests::write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode
          PASS [   0.018s] ( 726/1609) ito-core list::tests::counts_requirements_from_headings
          PASS [   0.016s] ( 727/1609) ito-core memory::rendering_tests::capture_command_empty_lists_render_as_empty_strings
          PASS [   0.019s] ( 728/1609) ito-core list::tests::iso_millis_matches_expected_shape
          PASS [   0.021s] ( 729/1609) ito-core list::tests::parse_modular_change_module_id_allows_overflow_change_numbers
          PASS [   0.025s] ( 730/1609) ito-core list::tests::list_changes_sorts_by_name_and_recent
          PASS [   0.029s] ( 731/1609) ito-core list::tests::list_changes_filters_by_progress_status
          PASS [   0.026s] ( 732/1609) ito-core memory::rendering_tests::capture_command_preserves_unknown_placeholders_literally
          PASS [   0.027s] ( 733/1609) ito-core memory::rendering_tests::capture_command_expands_files_as_repeated_flags
          PASS [   0.025s] ( 734/1609) ito-core memory::rendering_tests::capture_command_substitutes_missing_context_with_empty_quoted_string
          PASS [   0.027s] ( 735/1609) ito-core memory::rendering_tests::capture_command_expands_folders_with_explicit_flag_name
          PASS [   0.026s] ( 736/1609) ito-core memory::rendering_tests::capture_command_substitutes_context_with_quoting
          PASS [   0.025s] ( 737/1609) ito-core memory::rendering_tests::capture_not_configured_when_only_search_is_set
          PASS [   0.025s] ( 738/1609) ito-core memory::rendering_tests::capture_skill_emits_structured_inputs_and_options
          PASS [   0.026s] ( 739/1609) ito-core memory::rendering_tests::capture_not_configured_when_memory_section_absent
          PASS [   0.027s] ( 740/1609) ito-core memory::rendering_tests::mixed_shapes_render_independently
          PASS [   0.030s] ( 741/1609) ito-core memory::rendering_tests::capture_command_quotes_shell_metacharacters
          PASS [   0.019s] ( 742/1609) ito-core memory::rendering_tests::search_command_renders_scope_as_quoted_value
          PASS [   0.024s] ( 743/1609) ito-core memory::rendering_tests::search_command_renders_scope_as_empty_quoted_token_when_absent
          PASS [   0.028s] ( 744/1609) ito-core memory::rendering_tests::query_command_substitutes_query
          PASS [   0.008s] ( 745/1609) ito-core memory::rendering_tests::search_skill_includes_default_limit_in_structured_inputs
          PASS [   0.008s] ( 746/1609) ito-core memory::rendering_tests::search_not_configured_when_only_capture_is_set
          PASS [   0.008s] ( 747/1609) ito-core memory::rendering_tests::search_command_uses_supplied_limit_when_present
          PASS [   0.008s] ( 748/1609) ito-core memory::rendering_tests::shell_quote_handles_empty_string
          PASS [   0.012s] ( 749/1609) ito-core memory::rendering_tests::shell_quote_preserves_unicode_bytes
          PASS [   0.017s] ( 750/1609) ito-core memory::rendering_tests::search_command_substitutes_query_and_default_limit
          PASS [   0.027s] ( 751/1609) ito-core memory::rendering_tests::shell_quote_wraps_simple_strings_in_single_quotes
          PASS [   0.028s] ( 752/1609) ito-core memory::rendering_tests::shell_quote_escapes_embedded_single_quotes
          PASS [   0.025s] ( 753/1609) ito-core module_repository::tests::test_exists
          PASS [   0.028s] ( 754/1609) ito-core module_repository::tests::regression_change_repository_populates_sub_module_id
          PASS [   0.029s] ( 755/1609) ito-core module_repository::tests::regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes
          PASS [   0.025s] ( 756/1609) ito-core orchestrate::gates::tests::remediation_includes_failed_gate_and_downstream_run_gates
          PASS [   0.018s] ( 757/1609) ito-core orchestrate::gates::tests::remediation_returns_empty_when_failed_gate_not_found
          PASS [   0.029s] ( 758/1609) ito-core module_repository::tests::test_get
          PASS [   0.023s] ( 759/1609) ito-core orchestrate::gates::tests::remediation_includes_failed_gate_even_when_policy_is_skip
          PASS [   0.028s] ( 760/1609) ito-core module_repository::tests::test_get_not_found
          PASS [   0.028s] ( 761/1609) ito-core module_repository::tests::test_get_uses_full_name_input
          PASS [   0.028s] ( 762/1609) ito-core module_repository::tests::test_list
          PASS [   0.029s] ( 763/1609) ito-core module_repository::tests::test_list_with_change_counts
          PASS [   0.010s] ( 764/1609) ito-core orchestrate::gates::tests::remediation_skips_downstream_skip_gates
          PASS [   0.014s] ( 765/1609) ito-core process::tests::missing_executable_is_spawn_failure
          PASS [   0.052s] ( 766/1609) ito-core process::tests::captures_non_zero_exit
          PASS [   0.045s] ( 767/1609) ito-core process::tests::rejects_nul_in_program
          PASS [   0.046s] ( 768/1609) ito-core process::tests::rejects_excessive_argument_bytes
          PASS [   0.045s] ( 769/1609) ito-core ralph::duration::tests::test_format_duration
          PASS [   0.045s] ( 770/1609) ito-core process::tests::rejects_relative_program_with_components
          PASS [   0.048s] ( 771/1609) ito-core process::tests::rejects_empty_program
          PASS [   0.051s] ( 772/1609) ito-core process::tests::captures_stdout_and_stderr
          PASS [   0.049s] ( 773/1609) ito-core process::tests::rejects_current_dir_with_parent_component
          PASS [   0.045s] ( 774/1609) ito-core ralph::duration::tests::test_parse_bare_number
          PASS [   0.045s] ( 775/1609) ito-core process::tests::run_returns_invalid_request_before_spawn
          PASS [   0.047s] ( 776/1609) ito-core process::tests::rejects_nul_in_argument
          PASS [   0.041s] ( 777/1609) ito-core ralph::duration::tests::test_parse_combined
          PASS [   0.049s] ( 778/1609) ito-core ralph::duration::tests::test_parse_case_insensitive
          PASS [   0.009s] ( 779/1609) ito-core ralph::duration::tests::test_parse_hours
          PASS [   0.009s] ( 780/1609) ito-core ralph::duration::tests::test_parse_errors
          PASS [   0.009s] ( 781/1609) ito-core ralph::duration::tests::test_parse_minutes
          PASS [   0.008s] ( 782/1609) ito-core ralph::prompt::tests::build_prompt_preamble_omits_context_when_none
          PASS [   0.009s] ( 783/1609) ito-core ralph::duration::tests::test_parse_with_whitespace
          PASS [   0.009s] ( 784/1609) ito-core ralph::prompt::tests::build_prompt_preamble_includes_iteration
          PASS [   0.010s] ( 785/1609) ito-core ralph::duration::tests::test_parse_seconds
          PASS [   0.010s] ( 786/1609) ito-core ralph::prompt::tests::build_prompt_preamble_includes_context
          PASS [   0.010s] ( 787/1609) ito-core ralph::prompt::tests::build_prompt_preamble_omits_validation_when_none
          PASS [   0.010s] ( 788/1609) ito-core ralph::prompt::tests::build_prompt_preamble_includes_completion_promise
          PASS [   0.011s] ( 789/1609) ito-core ralph::prompt::tests::build_prompt_preamble_includes_validation_failure
          PASS [   0.015s] ( 790/1609) ito-core ralph::runner::runner_tests::commit_iteration_errors_on_git_add_failure
          PASS [   0.015s] ( 791/1609) ito-core ralph::runner::runner_tests::commit_iteration_errors_when_failed_commit_still_has_staged_changes
          PASS [   0.023s] ( 792/1609) ito-core ralph::runner::runner_tests::filter_eligible
          PASS [   0.025s] ( 793/1609) ito-core ralph::runner::runner_tests::commit_iteration_noops_when_no_changes
          PASS [   0.025s] ( 794/1609) ito-core ralph::runner::runner_tests::commit_iteration_treats_no_staged_changes_after_failed_commit_as_success
          PASS [   0.024s] ( 795/1609) ito-core ralph::runner::runner_tests::filter_incomplete
          PASS [   0.023s] ( 796/1609) ito-core ralph::runner::runner_tests::filter_unprocessed_changes
          PASS [   0.024s] ( 797/1609) ito-core ralph::runner::runner_tests::count_git_changes_returns_zero_on_git_failure
          PASS [   0.024s] ( 798/1609) ito-core ralph::runner::runner_tests::filter_module_incomplete
          PASS [   0.026s] ( 799/1609) ito-core ralph::runner::runner_tests::count_git_changes_counts_non_empty_lines
          PASS [   0.024s] ( 800/1609) ito-core ralph::runner::runner_tests::filter_ready
          PASS [   0.026s] ( 801/1609) ito-core ralph::runner::runner_tests::commit_iteration_succeeds_when_git_add_and_commit_succeed
          PASS [   0.026s] ( 802/1609) ito-core ralph::runner::runner_tests::finalize_queue_results_errors_with_failed_change_ids
          PASS [   1.330s] ( 803/1609) ito-core coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination
          PASS [   0.017s] ( 804/1609) ito-core ralph::runner::runner_tests::infer_module_no_hyphen
          PASS [   0.018s] ( 805/1609) ito-core ralph::runner::runner_tests::infer_module_ok
          PASS [   0.017s] ( 806/1609) ito-core ralph::runner::runner_tests::promise_whitespace_trimmed
          PASS [   0.019s] ( 807/1609) ito-core ralph::runner::runner_tests::promise_nested
          PASS [   0.018s] ( 808/1609) ito-core ralph::runner::runner_tests::promise_second_match
          PASS [   0.019s] ( 809/1609) ito-core ralph::runner::runner_tests::promise_no_tags
          PASS [   0.020s] ( 810/1609) ito-core ralph::runner::runner_tests::promise_incomplete
          PASS [   0.019s] ( 811/1609) ito-core ralph::runner::runner_tests::promise_single_match
          PASS [   0.020s] ( 812/1609) ito-core ralph::runner::runner_tests::promise_empty_token
          PASS [   0.021s] ( 813/1609) ito-core ralph::runner::runner_tests::print_helpers
          PASS [   0.021s] ( 814/1609) ito-core ralph::runner::runner_tests::now_ms_returns_positive_value
          PASS [   0.021s] ( 815/1609) ito-core ralph::runner::runner_tests::promise_empty_stdout
          PASS [   0.019s] ( 816/1609) ito-core ralph::runner::runner_tests::render_validation_pass
          PASS [   0.019s] ( 817/1609) ito-core ralph::runner::runner_tests::render_validation_fail_with_output
          PASS [   0.019s] ( 818/1609) ito-core ralph::runner::runner_tests::render_failure_empty
          PASS [   0.020s] ( 819/1609) ito-core ralph::runner::runner_tests::render_failure_both
          PASS [   0.008s] ( 820/1609) ito-core ralph::runner::runner_tests::render_validation_whitespace_output
          PASS [   0.010s] ( 821/1609) ito-core ralph::runner::runner_tests::resolve_cwd_no_worktree_found_fallback
          PASS [   0.009s] ( 822/1609) ito-core ralph::state::tests::append_context_no_op_on_whitespace
          PASS [   0.014s] ( 823/1609) ito-core ralph::state::tests::is_safe_change_id_segment_accepts_valid
          PASS [   0.019s] ( 824/1609) ito-core ralph::state::tests::is_safe_change_id_segment_rejects_backslash
          PASS [   0.019s] ( 825/1609) ito-core ralph::state::tests::is_safe_change_id_segment_rejects_empty
          PASS [   0.020s] ( 826/1609) ito-core ralph::runner::runner_tests::worktree_task_validation_repo_selection
          PASS [   0.021s] ( 827/1609) ito-core ralph::runner::runner_tests::resolve_cwd_worktree_found
          PASS [   0.017s] ( 828/1609) ito-core ralph::state::tests::load_state_backfills_missing_new_fields
          PASS [   0.018s] ( 829/1609) ito-core ralph::state::tests::is_safe_change_id_segment_rejects_too_long
          PASS [   0.022s] ( 830/1609) ito-core ralph::runner::runner_tests::resolve_cwd_no_change_targeted_fallback
          PASS [   0.018s] ( 831/1609) ito-core ralph::state::tests::load_state_returns_none_when_missing
          PASS [   0.018s] ( 832/1609) ito-core ralph::state::tests::load_context_returns_empty_when_missing
          PASS [   0.022s] ( 833/1609) ito-core ralph::runner::runner_tests::resolve_cwd_worktrees_not_enabled_fallback
          PASS [   0.016s] ( 834/1609) ito-core ralph::state::tests::ralph_state_json_path_correct
          PASS [   0.011s] ( 835/1609) ito-core ralph::state::tests::save_and_load_state_round_trip
          PASS [   0.019s] ( 836/1609) ito-core ralph::state::tests::ralph_context_path_correct
          PASS [   0.017s] ( 837/1609) ito-core ralph::state::tests::ralph_state_dir_uses_safe_fallback_for_invalid_change_ids
          PASS [   0.010s] ( 838/1609) ito-core ralph::validation::tests::discover_commands_falls_back_to_claude_md
          PASS [   0.021s] ( 839/1609) ito-core ralph::validation::tests::extract_commands_from_markdown_finds_make_check
          PASS [   0.021s] ( 840/1609) ito-core ralph::validation::tests::extract_commands_from_markdown_finds_make_test
          PASS [   0.021s] ( 841/1609) ito-core ralph::validation::tests::extract_commands_from_json_multiple_paths
          PASS [   0.021s] ( 842/1609) ito-core ralph::validation::tests::normalize_commands_value_array
          PASS [   0.022s] ( 843/1609) ito-core ralph::validation::tests::extract_commands_from_markdown_ignores_other_lines
          PASS [   0.023s] ( 844/1609) ito-core ralph::validation::tests::discover_commands_priority_ito_json_first
          PASS [   0.024s] ( 845/1609) ito-core ralph::validation::tests::discover_commands_falls_back_to_agents_md
          PASS [   0.023s] ( 846/1609) ito-core ralph::validation::tests::discover_commands_returns_empty_when_nothing_configured
          PASS [   0.023s] ( 847/1609) ito-core ralph::validation::tests::discover_commands_ito_config_json
          PASS [   0.021s] ( 848/1609) ito-core ralph::validation::tests::normalize_commands_value_non_string
          PASS [   0.021s] ( 849/1609) ito-core ralph::validation::tests::project_validation_discovers_commands_from_repo_json
          PASS [   1.206s] ( 850/1609) ito-core event_forwarder::tests::forward_reads_events_from_routed_local_store
          PASS [   0.023s] ( 851/1609) ito-core ralph::validation::tests::normalize_commands_value_string
          PASS [   0.023s] ( 852/1609) ito-core ralph::validation::tests::normalize_commands_value_null
          PASS [   0.020s] ( 853/1609) ito-core ralph::validation::tests::truncate_for_context_short_unchanged
          PASS [   0.021s] ( 854/1609) ito-core ralph::validation::tests::truncate_for_context_long_truncated
          PASS [   0.021s] ( 855/1609) ito-core ralph::validation::tests::truncate_for_context_multibyte_utf8
          PASS [   0.021s] ( 856/1609) ito-core sqlite_project_store::repositories::tests::ensure_project_creates_row
          PASS [   0.021s] ( 857/1609) ito-core sqlite_project_store::repositories::tests::archive_change_rolls_back_when_spec_promotion_fails
          PASS [   0.023s] ( 858/1609) ito-core ralph::validation::tests::task_completion_passes_when_no_tasks
          PASS [   0.024s] ( 859/1609) ito-core ralph::validation::tests::task_completion_fails_when_remaining
          PASS [   0.022s] ( 860/1609) ito-core sqlite_project_store::repositories::tests::get_change_returns_full_data
          PASS [   0.023s] ( 861/1609) ito-core sqlite_project_store::repositories::tests::ensure_project_is_idempotent
          PASS [   0.023s] ( 862/1609) ito-core sqlite_project_store::repositories::tests::on_disk_database_persists
          PASS [   0.044s] ( 863/1609) ito-core ralph::validation::tests::run_extra_validation_failure
          PASS [   0.027s] ( 864/1609) ito-core sqlite_project_store::repositories::tests::get_module_by_id
          PASS [   0.028s] ( 865/1609) ito-core sqlite_project_store::repositories::tests::get_missing_change_returns_not_found
          PASS [   0.026s] ( 866/1609) ito-core sqlite_project_store::repositories::tests::task_repository_missing_change_returns_empty
          PASS [   0.028s] ( 867/1609) ito-core sqlite_project_store::repositories::tests::open_in_memory_creates_schema
          PASS [   0.028s] ( 868/1609) ito-core sqlite_project_store::repositories::tests::store_is_send_sync
          PASS [   0.027s] ( 869/1609) ito-core sqlite_project_store::repositories::tests::task_repository_loads_tasks
          PASS [   0.029s] ( 870/1609) ito-core sqlite_project_store::repositories::tests::push_artifact_bundle_rolls_back_partial_writes_on_failure
          PASS [   0.028s] ( 871/1609) ito-core sqlite_project_store::repositories::tests::task_mutation_service_reports_poisoned_connection_without_panicking
          PASS [   0.054s] ( 872/1609) ito-core ralph::validation::tests::run_extra_validation_success
          PASS [   0.027s] ( 873/1609) ito-core sqlite_project_store::repositories::tests::upsert_and_list_modules
          PASS [   0.030s] ( 874/1609) ito-core sqlite_project_store::repositories::tests::two_projects_are_isolated
          PASS [   0.030s] ( 875/1609) ito-core sqlite_project_store::repositories::tests::upsert_and_list_changes
          PASS [   0.027s] ( 876/1609) ito-core task_repository::tests::load_tasks_uses_schema_apply_tracks_when_set
          PASS [   0.024s] ( 877/1609) ito-core task_repository::tests::test_has_tasks
          PASS [   0.027s] ( 878/1609) ito-core task_repository::tests::test_get_task_counts_checkbox_format
          PASS [   2.340s] ( 879/1609) ito-core audit::stream::tests::poll_detects_new_events_from_routed_store
          PASS [   0.027s] ( 880/1609) ito-core task_repository::tests::test_get_task_counts_enhanced_format
          PASS [   0.011s] ( 881/1609) ito-core tasks::tests::read_tasks_markdown_returns_error_for_missing_file
          PASS [   0.012s] ( 882/1609) ito-core tasks::tests::read_tasks_markdown_rejects_traversal_like_change_id
          PASS [   0.013s] ( 883/1609) ito-core task_repository::tests::test_missing_tasks_file_returns_zero
          PASS [   0.013s] ( 884/1609) ito-core tasks::tests::read_tasks_markdown_returns_contents_for_existing_file
          PASS [   0.013s] ( 885/1609) ito-core tasks::tests::returns_empty_when_no_ready_tasks_exist
          PASS [   0.010s] ( 886/1609) ito-core templates::schema_assets::tests::safe_relative_path_validation_blocks_traversal_and_absolute_paths
          PASS [   0.012s] ( 887/1609) ito-core templates::guidance::tests::strip_ito_internal_comment_blocks_removes_internal_template_guidance
          PASS [   0.011s] ( 888/1609) ito-core templates::schema_assets::tests::safe_schema_name_rejects_dot_segments_and_periods
          PASS [   0.010s] ( 889/1609) ito-core templates::task_parsing::tests::parse_enhanced_tasks_extracts_ids_status_and_done
          PASS [   0.017s] ( 890/1609) ito-core tasks::tests::returns_ready_tasks_for_ready_changes
          PASS [   0.009s] ( 891/1609) ito-core templates::types::tests::validation_yaml_parses_proposal_entry_with_rules
          PASS [   0.011s] ( 892/1609) ito-core templates::types::tests::schema_source_as_str_returns_expected_labels
          PASS [   0.010s] ( 893/1609) ito-core templates::types::tests::validation_yaml_parses_minimal_config
          PASS [   0.011s] ( 894/1609) ito-core templates::types::tests::validation_yaml_parses_rules_extension_without_breaking_existing_shape
          PASS [   0.010s] ( 895/1609) ito-core token::tests::generated_token_is_url_safe
          PASS [   0.010s] ( 896/1609) ito-core token::tests::two_tokens_are_distinct
          PASS [   0.009s] ( 897/1609) ito-core token::tests::url_safe_base64_roundtrip_known_value
          PASS [   0.011s] ( 898/1609) ito-core token::tests::url_safe_base64_encode_known_vector
          PASS [   0.009s] ( 899/1609) ito-core validate::issue::tests::format_spec_is_idempotent_for_message_suffix
          PASS [   0.012s] ( 900/1609) ito-core token::tests::generated_token_has_expected_length
          PASS [   0.010s] ( 901/1609) ito-core validate::issue::tests::constructors_set_expected_fields
          PASS [   0.008s] ( 902/1609) ito-core validate::issue::tests::metadata_helper_attaches_json_context
          PASS [   0.009s] ( 903/1609) ito-core validate::issue::tests::location_helpers_set_line_and_column
          PASS [   0.011s] ( 904/1609) ito-core validate::issue::tests::format_spec_preserves_non_object_metadata
          PASS [   0.008s] ( 905/1609) ito-core validate::issue::tests::rule_id_helper_marks_issue_and_is_reflected_in_metadata
          PASS [   0.010s] ( 906/1609) ito-core validate::report::tests::extend_collects_multiple_issues
          PASS [   0.009s] ( 907/1609) ito-core viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change
          PASS [   0.012s] ( 908/1609) ito-core validate::report::tests::finish_strict_fails_on_warnings
          PASS [   0.012s] ( 909/1609) ito-core validate::report::tests::finish_non_strict_only_fails_on_errors
          PASS [   0.010s] ( 910/1609) ito-core viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files
          PASS [   0.009s] ( 911/1609) ito-core viewer::html::tests::html_viewer_reports_expected_name
          PASS [   0.012s] ( 912/1609) ito-core viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content
          PASS [   0.010s] ( 913/1609) ito-core viewer::html::tests::html_viewer_open_errors_when_pandoc_missing
          PASS [   0.011s] ( 914/1609) ito-core viewer::html::tests::html_viewer_reports_expected_description
          PASS [   0.012s] ( 915/1609) ito-core viewer::html::tests::html_viewer_availability_depends_on_pandoc
          PASS [   0.010s] ( 916/1609) ito-core viewer::tests::concrete_viewers_report_expected_names
          PASS [   0.011s] ( 917/1609) ito-core viewer::tests::default_registry_includes_html_viewer
          PASS [   0.009s] ( 918/1609) ito-core viewer::tests::viewer_backend_trait_exposes_required_methods
          PASS [   0.010s] ( 919/1609) ito-core viewer::tests::viewer_registry_filters_and_finds_available_viewers
          PASS [   0.089s] ( 920/1609) ito-core ralph::validation::tests::shell_timeout_is_failure
          PASS [   0.013s] ( 921/1609) ito-core viewer::tests::viewer_registry_hides_tmux_when_disabled
          PASS [   0.012s] ( 922/1609) ito-core worktree_ensure::worktree_ensure_tests::ensure_creates_worktree_when_absent
          PASS [   0.016s] ( 923/1609) ito-core worktree_ensure::worktree_ensure_tests::ensure_git_failure_returns_error
          PASS [   0.012s] ( 924/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_separators
          PASS [   0.013s] ( 925/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_nul
          PASS [   0.013s] ( 926/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_leading_dash
          PASS [   0.014s] ( 927/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_accepts_normal_ids
          PASS [   0.015s] ( 928/1609) ito-core worktree_ensure::worktree_ensure_tests::ensure_worktrees_disabled_returns_cwd
          PASS [   0.018s] ( 929/1609) ito-core worktree_ensure::worktree_ensure_tests::ensure_existing_worktree_returns_path_without_creation
          PASS [   0.015s] ( 930/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_empty
          PASS [   0.018s] ( 931/1609) ito-core worktree_ensure::worktree_ensure_tests::ensure_with_include_files_copies_them
          PASS [   0.016s] ( 932/1609) ito-core worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_traversal
          PASS [   0.017s] ( 933/1609) ito-core worktree_init::worktree_init_tests::copy_include_files_copies_to_dest
          PASS [   0.015s] ( 934/1609) ito-core worktree_init::worktree_init_tests::copy_include_files_empty_config_and_no_file
          PASS [   0.012s] ( 935/1609) ito-core worktree_init::worktree_init_tests::copy_include_files_skips_missing_source
          PASS [   0.015s] ( 936/1609) ito-core worktree_init::worktree_init_tests::copy_include_files_skips_existing_destination
          PASS [   0.012s] ( 937/1609) ito-core worktree_init::worktree_init_tests::init_worktree_copies_files_and_runs_setup
          PASS [   0.029s] ( 938/1609) ito-core viewer::tests::run_with_stdin_closes_pipe_after_write
          PASS [   0.009s] ( 939/1609) ito-core worktree_init::worktree_init_tests::parse_worktree_include_file_comments_only
          PASS [   0.010s] ( 940/1609) ito-core worktree_init::worktree_init_tests::init_worktree_no_setup_copies_files_only
          PASS [   0.010s] ( 941/1609) ito-core worktree_init::worktree_init_tests::parse_worktree_include_file_empty_content
          PASS [   0.011s] ( 942/1609) ito-core worktree_init::worktree_init_tests::init_worktree_setup_failure_returns_error
          PASS [   0.011s] ( 943/1609) ito-core worktree_init::worktree_init_tests::init_worktree_preserves_existing_destination_file
          PASS [   0.011s] ( 944/1609) ito-core worktree_init::worktree_init_tests::parse_worktree_include_file_strips_comments_and_blanks
          PASS [   0.011s] ( 945/1609) ito-core worktree_init::worktree_init_tests::parse_worktree_include_file_trims_whitespace
          PASS [   0.013s] ( 946/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_deduplicates
          PASS [   0.010s] ( 947/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_missing_include_file_ok
          PASS [   0.015s] ( 948/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_config_only
          PASS [   0.012s] ( 949/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_ignores_directories
          PASS [   0.011s] ( 950/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_rejects_absolute_path_in_pattern
          PASS [   0.012s] ( 951/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_no_match_returns_empty
          PASS [   0.013s] ( 952/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_glob_expansion
          PASS [   0.014s] ( 953/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_file_only
          PASS [   0.009s] ( 954/1609) ito-core worktree_init::worktree_init_tests::run_setup_empty_multiple_commands_is_noop
          PASS [   0.009s] ( 955/1609) ito-core worktree_init::worktree_init_tests::run_setup_first_command_fails_stops_sequence
          PASS [   0.013s] ( 956/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_rejects_path_traversal
          PASS [   0.009s] ( 957/1609) ito-core worktree_init::worktree_init_tests::run_setup_multiple_commands_run_in_order
          PASS [   0.011s] ( 958/1609) ito-core worktree_init::worktree_init_tests::run_setup_empty_single_command_is_noop
          PASS [   0.010s] ( 959/1609) ito-core worktree_init::worktree_init_tests::run_setup_no_config_is_noop
          PASS [   0.013s] ( 960/1609) ito-core worktree_init::worktree_init_tests::resolve_include_files_union_of_config_and_file
          PASS [   0.010s] ( 961/1609) ito-core worktree_init::worktree_init_tests::run_setup_single_command_invoked
          PASS [   5.249s] ( 962/1609) ito-core::archive check_task_completion_handles_checkbox_and_enhanced_formats
          PASS [   5.247s] ( 963/1609) ito-core::archive generate_archive_name_prefixes_with_date
          PASS [   5.250s] ( 964/1609) ito-core::archive discover_and_copy_specs_and_archive_change
          PASS [   6.599s] ( 965/1609) ito-core::audit_mirror audit_mirror_default_local_store_falls_back_without_creating_worktree_log
          PASS [   6.862s] ( 966/1609) ito-core::audit_mirror audit_mirror_disabled_does_not_create_remote_branch
          PASS [   6.946s] ( 967/1609) ito-core::audit_mirror audit_mirror_failures_do_not_break_local_append
          PASS [   7.040s] ( 968/1609) ito-core::audit_mirror local_store_does_not_fall_back_when_internal_branch_exists_without_log_file
          PASS [   7.125s] ( 969/1609) ito-core::audit_mirror audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log
          PASS [   7.192s] ( 970/1609) ito-core::audit_mirror audit_mirror_enabled_pushes_to_configured_branch
          PASS [   7.594s] ( 971/1609) ito-core::audit_storage memory_store_append_persists_events
          PASS [   7.595s] ( 972/1609) ito-core::audit_storage filters_events_from_injected_store
          PASS [   7.593s] ( 973/1609) ito-core::audit_storage reads_events_from_injected_store_without_filesystem_path
          PASS [   9.635s] ( 974/1609) ito-core::backend_archive backend_archive_fails_when_pull_unavailable
          PASS [   4.404s] ( 975/1609) ito-core::backend_archive backend_archive_with_skip_specs_does_not_copy_specs
          PASS [   9.648s] ( 976/1609) ito-core::backend_archive backend_archive_fails_when_backend_unavailable_for_mark_archived
          PASS [   9.649s] ( 977/1609) ito-core::backend_archive backend_archive_creates_backup_before_overwriting
          PASS [   4.407s] ( 978/1609) ito-core::backend_archive backend_archive_happy_path_produces_committable_state
          PASS [   9.649s] ( 979/1609) ito-core::backend_archive backend_archive_does_not_mutate_local_module_markdown
          PASS [   4.455s] ( 980/1609) ito-core::backend_auth resolve_admin_tokens_merges_all_sources
          PASS [   4.277s] ( 981/1609) ito-core::backend_auth resolve_token_seed_cli_takes_precedence
          PASS [   3.806s] ( 982/1609) ito-core::backend_auth resolve_token_seed_returns_none_when_all_empty
          PASS [   4.540s] ( 983/1609) ito-core::backend_auth resolve_admin_tokens_deduplicates
          PASS [   4.209s] ( 984/1609) ito-core::backend_auth resolve_token_seed_falls_back_to_config
          PASS [   4.362s] ( 985/1609) ito-core::backend_auth resolve_admin_tokens_skips_empty_config_entries
          PASS [   3.807s] ( 986/1609) ito-core::backend_auth write_auth_preserves_existing_config
          PASS [   3.808s] ( 987/1609) ito-core::backend_auth write_auth_creates_config_file
          PASS [   4.806s] ( 988/1609) ito-core::backend_auth init_skips_when_tokens_exist
          PASS [   6.156s] ( 989/1609) ito-core::backend_auth init_generates_tokens_when_none_exist
          PASS [   5.085s] ( 990/1609) ito-core::backend_auth write_auth_sets_restrictive_permissions
          PASS [   5.088s] ( 991/1609) ito-core::backend_auth write_auth_rejects_non_object_root
          PASS [   5.099s] ( 992/1609) ito-core::backend_auth write_auth_rejects_non_object_backend_server
          PASS [   6.341s] ( 993/1609) ito-core::backend_client_mode allocate_returns_claimed_change
          PASS [   4.591s] ( 994/1609) ito-core::backend_client_mode config_enabled_with_token_resolves
          PASS [   4.591s] ( 995/1609) ito-core::backend_client_mode config_enabled_missing_token_fails_with_clear_message
          PASS [   4.592s] ( 996/1609) ito-core::backend_client_mode config_disabled_returns_none
          PASS [   4.592s] ( 997/1609) ito-core::backend_client_mode claim_success_returns_holder_info
          PASS [   4.593s] ( 998/1609) ito-core::backend_client_mode backend_change_repo_lists_and_filters
          PASS [   4.594s] ( 999/1609) ito-core::backend_client_mode backend_task_repo_missing_returns_zero
          PASS [   4.593s] (1000/1609) ito-core::backend_client_mode claim_conflict_returns_holder_error
          PASS [   4.593s] (1001/1609) ito-core::backend_client_mode backend_unavailable_detection
          PASS [   6.344s] (1002/1609) ito-core::backend_client_mode allocate_no_work_returns_none
          PASS [   4.593s] (1003/1609) ito-core::backend_client_mode pull_writes_artifacts_and_revision
          PASS [   4.596s] (1004/1609) ito-core::backend_client_mode backend_task_repo_parses_from_content
          PASS [   9.751s] (1005/1609) ito-core::backend_auth_service init_rejects_non_object_backend_server
          PASS [   5.649s] (1006/1609) ito-core::backend_client_mode retriable_status_codes
          PASS [   5.651s] (1007/1609) ito-core::backend_client_mode push_stale_revision_gives_actionable_error
          PASS [   5.651s] (1008/1609) ito-core::backend_client_mode push_success_updates_local_revision
          PASS [   5.687s] (1009/1609) ito-core::backend_sub_module_support backend_module_repository_list_includes_sub_module_summaries
          PASS [   5.687s] (1010/1609) ito-core::backend_sub_module_support backend_module_repository_list_sub_modules_for_unknown_module_returns_error
          PASS [   5.688s] (1011/1609) ito-core::backend_sub_module_support backend_module_repository_get_sub_module_by_composite_id
          PASS [   5.688s] (1012/1609) ito-core::backend_sub_module_support backend_module_repository_get_sub_module_not_found_returns_error
          PASS [   5.687s] (1013/1609) ito-core::backend_sub_module_support backend_module_repository_list_sub_modules_returns_sorted_summaries
          PASS [   2.282s] (1014/1609) ito-core::backend_sub_module_support sqlite_store_persists_sub_module_id_on_change
          PASS [   5.687s] (1015/1609) ito-core::backend_sub_module_support sqlite_store_list_changes_filters_by_sub_module_id
          PASS [   5.689s] (1016/1609) ito-core::backend_sub_module_support sqlite_store_legacy_change_has_no_sub_module_id
          PASS [   7.963s] (1017/1609) ito-core::backend_module_repository backend_module_repository_list_sorts_by_id
          PASS [   7.963s] (1018/1609) ito-core::backend_module_repository backend_module_repository_accepts_name_inputs
          PASS [   7.962s] (1019/1609) ito-core::backend_module_repository backend_module_repository_normalizes_full_name_inputs
          PASS [   7.963s] (1020/1609) ito-core::backend_module_repository read_module_markdown_falls_back_without_local_file
          PASS [   7.963s] (1021/1609) ito-core::backend_module_repository backend_module_repository_list_sorts_deterministically
          PASS [   5.329s] (1022/1609) ito-core::change_repository_lifecycle remote_runtime_ignores_local_change_dirs
          PASS [   5.337s] (1023/1609) ito-core::change_repository_lifecycle filesystem_change_repository_filters_archived
          PASS [   6.249s] (1024/1609) ito-core::backend_sub_module_support sqlite_store_sub_module_change_roundtrips_through_artifact_bundle
          PASS [   5.690s] (1025/1609) ito-core::change_repository_parity backend_resolve_empty_input_returns_not_found
          PASS [   5.690s] (1026/1609) ito-core::change_repository_parity backend_resolve_lifecycle_filter_respected
          PASS [   5.687s] (1027/1609) ito-core::change_repository_parity backend_resolve_numeric_short_form_matches_canonical_id
          PASS [   5.688s] (1028/1609) ito-core::change_repository_parity backend_resolve_numeric_short_form_ambiguous
          PASS [   5.690s] (1029/1609) ito-core::change_repository_parity backend_list_by_module_normalizes_module_id
          PASS [   5.692s] (1030/1609) ito-core::change_repository_parity backend_resolve_module_scoped_slug_not_found
          PASS [   5.690s] (1031/1609) ito-core::change_repository_parity backend_resolve_module_scoped_slug_query
          PASS [   3.428s] (1032/1609) ito-core::change_repository_parity sqlite_get_with_archived_filter_returns_not_found
          PASS [   3.427s] (1033/1609) ito-core::change_repository_parity sqlite_list_archived_filter_returns_empty
          PASS [   1.659s] (1034/1609) ito-core::change_repository_parity sqlite_resolve_archived_filter_returns_not_found
          PASS [   1.668s] (1035/1609) ito-core::change_repository_parity sqlite_resolve_all_filter_finds_active_changes
          PASS [   3.430s] (1036/1609) ito-core::change_repository_parity sqlite_list_all_filter_returns_active_changes
          PASS [   3.431s] (1037/1609) ito-core::change_repository_parity sqlite_get_with_all_filter_finds_change
          PASS [   3.429s] (1038/1609) ito-core::change_repository_parity sqlite_list_by_module_normalizes_module_id
          PASS [   0.017s] (1039/1609) ito-core::change_repository_parity sqlite_resolve_numeric_short_form_matches_canonical_id
          PASS [   0.017s] (1040/1609) ito-core::change_repository_parity sqlite_resolve_prefix_match
          PASS [   0.017s] (1041/1609) ito-core::change_repository_parity sqlite_resolve_numeric_short_form_ambiguous
          PASS [   9.925s] (1042/1609) ito-core::change_repository_orchestrate_metadata change_repository_exposes_orchestrate_metadata_from_ito_yaml
          PASS [   5.956s] (1043/1609) ito-core::change_repository_parity sqlite_resolve_empty_input_returns_not_found
          PASS [   5.948s] (1044/1609) ito-core::coordination_worktree symlink_tests::change_written_through_symlink_lands_in_worktree
          PASS [   5.949s] (1045/1609) ito-core::coordination_worktree symlink_tests::module_repo_list_through_symlink
          PASS [   5.963s] (1046/1609) ito-core::coordination_worktree symlink_tests::change_repo_exists_through_symlink
          PASS [   5.952s] (1047/1609) ito-core::coordination_worktree symlink_tests::module_repo_get_through_symlink
          PASS [   5.954s] (1048/1609) ito-core::coordination_worktree symlink_tests::module_repo_exists_through_symlink
          PASS [   5.957s] (1049/1609) ito-core::coordination_worktree symlink_tests::change_repo_list_through_symlink
          PASS [   5.954s] (1050/1609) ito-core::coordination_worktree symlink_tests::module_repo_list_multiple_through_symlink
          PASS [   1.735s] (1051/1609) ito-core::coordination_worktree symlink_tests::task_repo_load_tasks_through_symlink
          PASS [   5.956s] (1052/1609) ito-core::coordination_worktree symlink_tests::module_repo_change_counts_through_symlink
          PASS [   5.959s] (1053/1609) ito-core::coordination_worktree symlink_tests::change_repo_list_multiple_through_symlink
          PASS [   5.954s] (1054/1609) ito-core::coordination_worktree symlink_tests::task_repo_has_tasks_through_symlink
          PASS [   5.960s] (1055/1609) ito-core::coordination_worktree symlink_tests::change_repo_get_through_symlink
          PASS [   5.970s] (1056/1609) ito-core::coordination_worktree symlink_tests::all_repos_consistent_through_symlinks
          PASS [   0.019s] (1057/1609) ito-core::coordination_worktree symlink_tests::task_written_through_symlink_lands_in_worktree
          PASS [   9.724s] (1058/1609) ito-core::change_target_resolution_parity sqlite_resolver_honors_archived_lifecycle_like_filesystem
          PASS [   9.728s] (1059/1609) ito-core::change_target_resolution_parity change_target_resolution_matches_across_repository_modes
          PASS [   5.766s] (1060/1609) ito-core::coordination_worktree symlink_tests::task_repo_missing_tasks_file_returns_zero_through_symlink
          PASS [   5.758s] (1061/1609) ito-core::create create_change_rejects_uppercase_names
          PASS [   5.755s] (1062/1609) ito-core::create create_module_creates_directory_and_module_md
          PASS [   5.769s] (1063/1609) ito-core::create create_change_in_sub_module_rejects_missing_sub_module_dir
          PASS [   5.771s] (1064/1609) ito-core::create create_change_in_sub_module_rejects_missing_parent_module
          PASS [   2.016s] (1065/1609) ito-core::create create_module_writes_description_to_purpose_section
          PASS [   2.023s] (1066/1609) ito-core::create create_module_returns_existing_module_when_name_matches
          PASS [   5.785s] (1067/1609) ito-core::create create_change_allocates_next_number_from_existing_change_dirs
          PASS [   5.780s] (1068/1609) ito-core::create create_change_rewrites_module_changes_in_ascending_change_id_order
          PASS [   5.781s] (1069/1609) ito-core::create create_change_in_sub_module_writes_checklist_to_sub_module_md
          PASS [   5.787s] (1070/1609) ito-core::create create_change_creates_change_dir_and_updates_module_md
          PASS [   5.786s] (1071/1609) ito-core::create create_change_in_sub_module_uses_composite_id_format
          PASS [   5.788s] (1072/1609) ito-core::create create_change_in_sub_module_checklist_is_sorted_ascending
          PASS [   5.792s] (1073/1609) ito-core::create allocation_state_sub_module_keys_sort_after_parent
          PASS [   5.790s] (1074/1609) ito-core::create create_change_in_sub_module_allocates_independent_sequence
          PASS [   5.793s] (1075/1609) ito-core::create create_change_writes_allocation_modules_in_ascending_id_order
          PASS [   4.726s] (1076/1609) ito-core::distribution opencode_manifests_includes_plugin_and_skills
          PASS [   4.746s] (1077/1609) ito-core::distribution codex_manifests_includes_bootstrap_and_skills
          PASS [   4.750s] (1078/1609) ito-core::distribution claude_manifests_includes_hooks_and_skills
          PASS [   4.769s] (1079/1609) ito-core::distribution install_manifests_make_tmux_skill_scripts_executable
          PASS [   4.775s] (1080/1609) ito-core::distribution install_manifests_keeps_non_worktree_placeholders_verbatim
          PASS [   4.767s] (1081/1609) ito-core::distribution install_manifests_renders_worktree_skill_with_context
          PASS [   4.778s] (1082/1609) ito-core::distribution install_manifests_creates_parent_directories
          PASS [   5.579s] (1083/1609) ito-core::distribution all_manifests_use_embedded_assets
          PASS [   7.246s] (1084/1609) ito-core::event_forwarding forward_result_reports_diagnostics
          PASS [   7.357s] (1085/1609) ito-core::event_forwarding permanent_failure_stops_forwarding
          PASS [   7.364s] (1086/1609) ito-core::event_forwarding full_forwarding_workflow
          PASS [   7.366s] (1087/1609) ito-core::event_forwarding batch_boundaries_preserved
          PASS [   7.430s] (1088/1609) ito-core::event_forwarding incremental_forwarding
          PASS [   8.682s] (1089/1609) ito-core::distribution github_manifests_includes_skills_and_commands
          PASS [   8.696s] (1090/1609) ito-core::distribution install_manifests_writes_files_to_disk
          PASS [   8.697s] (1091/1609) ito-core::distribution install_manifests_renders_worktree_skill_enabled
          PASS [   5.446s] (1092/1609) ito-core::harness_context infer_context_from_cwd_infers_change_from_path
          PASS [   2.965s] (1093/1609) ito-core::harness_context infer_context_from_cwd_infers_module_from_ito_modules_path
          PASS [   2.881s] (1094/1609) ito-core::harness_context infer_context_from_cwd_returns_no_target_when_inconclusive
          PASS [   3.070s] (1095/1609) ito-core::harness_context infer_context_from_cwd_prefers_path_over_git_branch
          PASS [   5.692s] (1096/1609) ito-core::harness_context infer_context_from_cwd_infers_change_from_git_branch
          PASS [   5.648s] (1097/1609) ito-core::harness_context infer_context_from_cwd_infers_module_from_git_branch
          PASS [   7.059s] (1098/1609) ito-core::event_forwarding transient_failure_retried_then_succeeds
          PASS [   8.025s] (1099/1609) ito-core::grep_scopes grep_scope_change_only_searches_one_change
          PASS [   8.025s] (1100/1609) ito-core::grep_scopes grep_scope_module_searches_all_changes_in_module
          PASS [   8.065s] (1101/1609) ito-core::grep_scopes grep_scope_all_searches_all_changes
          PASS [   8.065s] (1102/1609) ito-core::grep_scopes grep_respects_limit_across_scopes
          PASS [   5.495s] (1103/1609) ito-core::harness_opencode copilot_harness_errors_when_copilot_missing
          PASS [   6.821s] (1104/1609) ito-core::harness_opencode claude_harness_errors_when_claude_missing
          PASS [   3.976s] (1105/1609) ito-core::harness_opencode opencode_harness_errors_when_opencode_missing
          PASS [   5.523s] (1106/1609) ito-core::harness_opencode codex_harness_errors_when_codex_missing
          PASS [   3.871s] (1107/1609) ito-core::harness_stub stub_harness_errors_on_empty_steps
          PASS [   5.199s] (1108/1609) ito-core::harness_stub stub_harness_default_returns_complete_promise
          PASS [   2.866s] (1109/1609) ito-core::harness_stub stub_harness_from_env_prefers_env_over_default
          PASS [   2.866s] (1110/1609) ito-core::harness_stub stub_step_defaults_match_json_schema
          PASS [   2.869s] (1111/1609) ito-core::harness_stub stub_harness_errors_on_missing_and_invalid_json
          PASS [   2.867s] (1112/1609) ito-core::harness_stub stub_harness_from_json_path_runs_steps_and_repeats_last
          PASS [   2.375s] (1113/1609) ito-core::import pushes_when_remote_active_bundle_differs
          PASS [   2.376s] (1114/1609) ito-core::import skips_already_imported_active_change_when_remote_bundle_matches
          PASS [   3.843s] (1115/1609) ito-core::import active_local_change_fails_when_backend_only_has_archived_copy
          PASS [   3.843s] (1116/1609) ito-core::import dry_run_uses_preview_logic_without_mutating_backend
          PASS [   3.844s] (1117/1609) ito-core::import dry_run_previews_without_importing
          PASS [   2.379s] (1118/1609) ito-core::import rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches
          PASS [   2.382s] (1119/1609) ito-core::import ignores_unrecognized_archive_directories_during_discovery
          PASS [   2.381s] (1120/1609) ito-core::import imports_active_and_archived_changes_with_lifecycle_fidelity
          PASS [   3.848s] (1121/1609) ito-core::import archived_directory_with_empty_canonical_change_id_is_ignored
          PASS [   2.382s] (1122/1609) ito-core::import import_summary_records_failures_without_aborting_remaining_changes
          PASS [   7.984s] (1123/1609) ito-core::harness_opencode github_copilot_harness_passes_model_and_allow_all_flags
          PASS [   9.506s] (1124/1609) ito-core::harness_opencode codex_harness_passes_model_and_allow_all_flags
          PASS [  10.766s] (1125/1609) ito-core::harness_opencode claude_harness_passes_model_and_allow_all_flags
          PASS [   7.952s] (1126/1609) ito-core::harness_opencode opencode_harness_runs_opencode_binary_and_returns_outputs
          PASS [   7.883s] (1127/1609) ito-core::harness_streaming no_timeout_when_process_exits_normally
          PASS [  10.616s] (1128/1609) ito-core::harness_streaming inactivity_timeout_kills_stalled_process
          PASS [   5.313s] (1129/1609) ito-core::orchestrate_run_state orchestrate_resume_skips_terminal_gates
          PASS [   5.313s] (1130/1609) ito-core::orchestrate_run_state orchestrate_max_parallel_aliases_resolve
          PASS [   5.317s] (1131/1609) ito-core::orchestrate_run_state orchestrate_dependency_cycle_is_rejected
          PASS [   5.314s] (1132/1609) ito-core::orchestrate_run_state orchestrate_run_id_generation_matches_expected_format
          PASS [   5.316s] (1133/1609) ito-core::orchestrate_run_state orchestrate_run_state_creates_expected_layout
          PASS [   5.319s] (1134/1609) ito-core::orchestrate_run_state orchestrate_event_log_appends_without_truncation
          PASS [   5.329s] (1135/1609) ito-core::orchestrate_run_state orchestrate_change_state_is_written_and_readable
          PASS [   7.255s] (1136/1609) ito-core::io read_to_string_or_default_returns_empty_for_missing_file
          PASS [   7.256s] (1137/1609) ito-core::io read_to_string_optional_returns_none_for_missing_file
          PASS [   7.257s] (1138/1609) ito-core::io write_atomic_std_creates_parent_and_replaces_contents
          PASS [   1.408s] (1139/1609) ito-core::ralph run_ralph_errors_when_max_iterations_is_zero
          PASS [   3.353s] (1140/1609) ito-core::ralph run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains
          PASS [   8.504s] (1141/1609) ito-core::ralph run_ralph_add_and_clear_context_paths
          PASS [   3.354s] (1142/1609) ito-core::ralph run_ralph_continue_ready_errors_when_targeting_change_or_module
          PASS [   3.355s] (1143/1609) ito-core::ralph run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes
          PASS [   1.420s] (1144/1609) ito-core::ralph run_ralph_continues_after_harness_failure_by_default
          PASS [   3.355s] (1145/1609) ito-core::ralph run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight
          PASS [   3.350s] (1146/1609) ito-core::ralph run_ralph_continue_ready_reorients_when_repo_state_shifts
          PASS [   3.356s] (1147/1609) ito-core::ralph run_ralph_continue_ready_processes_all_eligible_changes_across_repo
          PASS [   5.662s] (1148/1609) ito-core::ralph run_ralph_continue_module_processes_all_ready_changes
          PASS [   3.368s] (1149/1609) ito-core::ralph run_ralph_continue_ready_accumulates_failures_after_processing_remaining_changes
          PASS [   0.022s] (1150/1609) ito-core::ralph run_ralph_fails_after_error_threshold
          PASS [   0.022s] (1151/1609) ito-core::ralph run_ralph_gives_up_after_max_retriable_retries
          PASS [   0.021s] (1152/1609) ito-core::ralph run_ralph_module_resolves_single_change
          PASS [   0.025s] (1153/1609) ito-core::ralph run_ralph_module_multiple_changes_errors_when_non_interactive
          PASS [   0.038s] (1154/1609) ito-core::ralph run_ralph_non_retriable_exit_still_counts_against_threshold
          PASS [   0.028s] (1155/1609) ito-core::ralph run_ralph_returns_error_on_harness_failure
          PASS [   0.030s] (1156/1609) ito-core::ralph run_ralph_retries_retriable_exit_code_without_counting_against_threshold
          PASS [   0.038s] (1157/1609) ito-core::ralph run_ralph_prompt_includes_task_context_and_guidance
          PASS [   0.036s] (1158/1609) ito-core::ralph run_ralph_resets_retriable_counter_on_success
          PASS [   8.415s] (1159/1609) ito-core::ralph run_ralph_completion_promise_trims_whitespace
          PASS [   0.035s] (1160/1609) ito-core::ralph run_ralph_skip_validation_exits_immediately
          PASS [   0.040s] (1161/1609) ito-core::ralph run_ralph_retries_retriable_exit_code_with_exit_on_error
          PASS [   1.467s] (1162/1609) ito-core::ralph run_ralph_continues_when_completion_validation_fails
          PASS [   0.011s] (1163/1609) ito-core::ralph state_helpers_append_and_clear_context
          PASS [   0.031s] (1164/1609) ito-core::ralph run_ralph_status_path_works_with_no_state
          PASS [   0.075s] (1165/1609) ito-core::ralph run_ralph_loop_writes_state_and_honors_min_iterations
          PASS [   0.062s] (1166/1609) ito-core::ralph run_ralph_worktree_disabled_uses_fallback_cwd
          PASS [   0.099s] (1167/1609) ito-core::ralph run_ralph_worktree_enabled_state_written_to_effective_ito
          PASS [  11.616s] (1168/1609) ito-core::planning_init read_planning_status_returns_error_for_missing_roadmap
          PASS [  11.618s] (1169/1609) ito-core::planning_init read_planning_status_returns_contents_for_existing_roadmap
          PASS [  11.623s] (1170/1609) ito-core::planning_init init_planning_structure_writes_files
          PASS [   5.217s] (1171/1609) ito-core::repo_index repo_index_loads_and_excludes_archive_change_dir
          PASS [   5.901s] (1172/1609) ito-core::repo_integrity invalid_change_dir_names_are_reported
          PASS [   5.903s] (1173/1609) ito-core::repo_integrity change_referring_to_missing_module_is_an_error
          PASS [   5.903s] (1174/1609) ito-core::repo_integrity duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs
          PASS [   7.137s] (1175/1609) ito-core::repo_paths coordination_worktree_path_ignores_xdg_when_explicit_path_set
          PASS [   7.137s] (1176/1609) ito-core::repo_paths coordination_worktree_path_correct_structure_with_home_fallback
          PASS [   7.135s] (1177/1609) ito-core::repo_paths coordination_worktree_path_last_resort_uses_ito_path
          PASS [   7.111s] (1178/1609) ito-core::repo_paths coordination_worktree_path_uses_xdg_data_home_when_set
          PASS [   7.137s] (1179/1609) ito-core::repo_paths coordination_worktree_path_falls_back_to_local_share_when_xdg_unset
          PASS [   7.135s] (1180/1609) ito-core::repo_paths coordination_worktree_path_uses_explicit_worktree_path_when_set
          PASS [   7.137s] (1181/1609) ito-core::repo_paths coordination_worktree_path_correct_structure_with_xdg
          PASS [   4.075s] (1182/1609) ito-core::repo_paths resolve_worktree_paths_respects_bare_control_siblings_strategy
          PASS [   7.219s] (1183/1609) ito-core::repo_paths resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir
          PASS [   7.180s] (1184/1609) ito-core::repo_paths resolve_env_from_cwd_prefers_git_toplevel
          PASS [   4.200s] (1185/1609) ito-core::repo_paths resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable
          PASS [   4.948s] (1186/1609) ito-core::repository_runtime remote_runtime_uses_remote_factory
          PASS [   4.263s] (1187/1609) ito-core::repository_runtime sqlite_mode_requires_db_path
          PASS [   7.100s] (1188/1609) ito-core::repository_runtime filesystem_runtime_builds_repository_set
          PASS [   4.275s] (1189/1609) ito-core::repository_runtime repository_modes_return_consistent_change_names
          PASS [   4.275s] (1190/1609) ito-core::repository_runtime resolve_target_parity_between_filesystem_and_sqlite
          PASS [   2.124s] (1191/1609) ito-core::show parse_contract_refs_splits_unknown_scheme_after_known_ref
          PASS [   2.129s] (1192/1609) ito-core::show parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier
          PASS [   2.137s] (1193/1609) ito-core::show parse_contract_refs_preserves_lowercase_colon_text_inside_identifiers
          PASS [   5.155s] (1194/1609) ito-core::show parse_change_show_json_emits_deltas_with_operations
          PASS [   5.038s] (1195/1609) ito-core::show parse_contract_refs_accepts_comma_without_space_before_known_scheme
          PASS [   2.135s] (1196/1609) ito-core::show parse_contract_refs_splits_lowercase_unknown_scheme_after_known_ref
          PASS [   5.030s] (1197/1609) ito-core::show parse_contract_refs_preserves_commas_inside_identifiers
          PASS [   5.038s] (1198/1609) ito-core::show parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions
          PASS [   2.123s] (1199/1609) ito-core::show parse_contract_refs_splits_unknown_schemes_at_length_threshold
          PASS [   5.156s] (1200/1609) ito-core::show bundle_main_specs_show_json_returns_not_found_when_no_specs_exist
          PASS [   5.158s] (1201/1609) ito-core::show load_delta_spec_file_uses_parent_dir_name_as_spec
          PASS [   5.158s] (1202/1609) ito-core::show bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing
          PASS [   5.161s] (1203/1609) ito-core::show bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas
          PASS [   5.162s] (1204/1609) ito-core::show bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths
          PASS [   0.012s] (1205/1609) ito-core::show parse_delta_spec_requirement_id_is_extracted
          PASS [   0.012s] (1206/1609) ito-core::show parse_requirement_block_requirement_id_absent_gives_none
          PASS [   0.012s] (1207/1609) ito-core::show parse_requirement_block_multiple_requirements_with_ids
          PASS [   0.012s] (1208/1609) ito-core::show parse_requirement_block_extracts_requirement_id
          PASS [   0.012s] (1209/1609) ito-core::show read_module_markdown_returns_error_for_nonexistent_module
          PASS [   0.013s] (1210/1609) ito-core::show parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets
          PASS [   0.013s] (1211/1609) ito-core::show parse_spec_show_json_extracts_overview_requirements_and_scenarios
          PASS [   0.013s] (1212/1609) ito-core::show read_module_markdown_returns_empty_for_missing_module_md
          PASS [   0.014s] (1213/1609) ito-core::show read_module_markdown_returns_contents_for_existing_module
          PASS [   0.014s] (1214/1609) ito-core::show read_change_delta_spec_files_lists_specs_sorted
          PASS [   9.121s] (1215/1609) ito-core::repository_runtime sqlite_runtime_builds_repository_set
          PASS [  10.244s] (1216/1609) ito-core::repository_runtime_config_validation invalid_repository_mode_fails_fast
          PASS [   5.871s] (1217/1609) ito-core::spec_repository_backends remote_runtime_exposes_spec_repository_without_local_specs
          PASS [   5.872s] (1218/1609) ito-core::spec_repository_backends filesystem_runtime_exposes_promoted_specs
          PASS [   6.847s] (1219/1609) ito-core::spec_show_repository bundle_specs_show_json_from_repository_sorts_ids
          PASS [   6.848s] (1220/1609) ito-core::spec_show_repository bundle_specs_markdown_from_repository_adds_metadata_comments
          PASS [   6.841s] (1221/1609) ito-core::spec_show_repository read_spec_markdown_from_repository_reads_remote_spec
          PASS [   8.116s] (1222/1609) ito-core::sqlite_task_mutations sqlite_task_mutation_service_returns_not_found_for_missing_tasks
          PASS [   8.116s] (1223/1609) ito-core::sqlite_task_mutations sqlite_task_mutation_service_initializes_missing_tasks
          PASS [   8.116s] (1224/1609) ito-core::sqlite_task_mutations sqlite_task_mutation_service_updates_existing_markdown
          PASS [   9.251s] (1225/1609) ito-core::sqlite_archive_mirror sqlite_archive_promotes_specs_and_marks_change_archived
          PASS [   3.084s] (1226/1609) ito-core::tasks_api list_ready_tasks_across_changes_handles_empty_repo
          PASS [   4.065s] (1227/1609) ito-core::tasks_api init_tasks_creates_file_when_missing
          PASS [   3.087s] (1228/1609) ito-core::tasks_api init_tasks_returns_true_when_file_already_exists
          PASS [   1.820s] (1229/1609) ito-core::tasks_api shelve_task_rejects_shelving_complete_task
          PASS [   4.075s] (1230/1609) ito-core::tasks_api get_next_task_returns_none_when_all_tasks_complete
          PASS [   5.983s] (1231/1609) ito-core::tasks_api complete_task_accepts_note_parameter
          PASS [   1.820s] (1232/1609) ito-core::tasks_api shelve_task_accepts_reason_parameter
          PASS [   4.860s] (1233/1609) ito-core::tasks_api get_next_task_returns_first_ready_task_for_enhanced_format
          PASS [   9.935s] (1234/1609) ito-core::tasks_api add_task_appends_new_task_with_next_id
          PASS [   3.095s] (1235/1609) ito-core::tasks_api shelve_and_unshelve_task_round_trip_for_enhanced_format
          PASS [   9.934s] (1236/1609) ito-core::tasks_api add_task_creates_wave_if_not_exists
          PASS [   1.821s] (1237/1609) ito-core::tasks_api start_and_complete_task_enforced_by_dependencies_for_enhanced_format
          PASS [   0.014s] (1238/1609) ito-core::tasks_api tasks_api_rejects_non_tasks_tracking_validator_for_schema_tracking
          PASS [   0.019s] (1239/1609) ito-core::tasks_api tasks_api_operates_on_schema_apply_tracks_file
          PASS [  13.249s] (1240/1609) ito-core::stats compute_command_stats_counts_command_end_events
          PASS [  13.251s] (1241/1609) ito-core::stats collect_jsonl_files_finds_nested_jsonl_files
          PASS [  14.315s] (1242/1609) ito-core::task_repository_summary repository_status_builds_summary_and_next_task
          PASS [   5.747s] (1243/1609) ito-core::tasks_api start_task_rejects_starting_shelved_task_directly
          PASS [   2.281s] (1244/1609) ito-core::tasks_orchestration get_next_task_returns_none_when_all_complete
          PASS [   5.596s] (1245/1609) ito-core::tasks_orchestration add_task_rejects_checkbox_format
          PASS [   5.596s] (1246/1609) ito-core::tasks_orchestration complete_task_handles_checkbox_format
          PASS [   5.588s] (1247/1609) ito-core::tasks_orchestration get_next_task_returns_current_in_progress_for_checkbox
          PASS [   1.220s] (1248/1609) ito-core::tasks_orchestration get_task_status_returns_diagnostics_for_malformed_file
          PASS [   5.601s] (1249/1609) ito-core::tasks_orchestration add_task_assigns_next_id_in_wave
          PASS [   5.601s] (1250/1609) ito-core::tasks_orchestration add_task_errors_with_parse_errors
          PASS [   5.601s] (1251/1609) ito-core::tasks_orchestration complete_task_errors_with_parse_errors
          PASS [   2.287s] (1252/1609) ito-core::tasks_orchestration get_next_task_returns_first_ready_for_enhanced
          PASS [   5.601s] (1253/1609) ito-core::tasks_orchestration add_task_defaults_to_wave_1
          PASS [   5.596s] (1254/1609) ito-core::tasks_orchestration complete_task_handles_enhanced_format
          PASS [   5.601s] (1255/1609) ito-core::tasks_orchestration add_task_creates_wave_when_missing
          PASS [   0.025s] (1256/1609) ito-core::tasks_orchestration init_tasks_rejects_invalid_change_id
          PASS [   0.025s] (1257/1609) ito-core::tasks_orchestration init_tasks_creates_file_when_missing
          PASS [   0.022s] (1258/1609) ito-core::tasks_orchestration shelve_task_rejects_checkbox_format
          PASS [   0.023s] (1259/1609) ito-core::tasks_orchestration shelve_task_rejects_complete_task
          PASS [   0.022s] (1260/1609) ito-core::tasks_orchestration unshelve_task_rejects_not_shelved
          PASS [   0.023s] (1261/1609) ito-core::tasks_orchestration start_task_rejects_already_complete
          PASS [   0.027s] (1262/1609) ito-core::tasks_orchestration shelve_task_errors_with_parse_errors
          PASS [   0.027s] (1263/1609) ito-core::tasks_orchestration init_tasks_does_not_overwrite_existing_file
          PASS [   0.024s] (1264/1609) ito-core::tasks_orchestration start_task_errors_with_parse_errors
          PASS [   0.024s] (1265/1609) ito-core::tasks_orchestration start_task_validates_task_is_ready
          PASS [   0.024s] (1266/1609) ito-core::tasks_orchestration unshelve_task_errors_with_parse_errors
          PASS [   0.043s] (1267/1609) ito-core::tasks_orchestration start_task_rejects_shelved_task
          PASS [   0.046s] (1268/1609) ito-core::tasks_orchestration unshelve_task_transitions_to_pending
          PASS [   8.911s] (1269/1609) ito-core::tasks_checkbox_format checkbox_tasks_do_not_support_shelving
          PASS [   8.904s] (1270/1609) ito-core::tasks_checkbox_format checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback
          PASS [   8.905s] (1271/1609) ito-core::tasks_checkbox_format checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids
          PASS [   5.496s] (1272/1609) ito-core::tasks_orchestration get_task_status_returns_error_when_file_missing
          PASS [   5.678s] (1273/1609) ito-core::templates_schema_resolution resolve_schema_rejects_absolute_and_backslash_names
          PASS [   5.660s] (1274/1609) ito-core::templates_schema_resolution resolve_schema_rejects_path_traversal_name
          PASS [   5.637s] (1275/1609) ito-core::templates_schema_resolution resolve_schema_uses_embedded_when_no_overrides_exist
          PASS [   5.682s] (1276/1609) ito-core::templates_schema_resolution resolve_instructions_reads_embedded_templates
          PASS [   5.682s] (1277/1609) ito-core::templates_schema_resolution resolve_instructions_exposes_enhanced_spec_driven_templates
          PASS [   2.405s] (1278/1609) ito-core::templates_schema_resolution resolve_templates_rejects_traversal_template_path
          PASS [   5.685s] (1279/1609) ito-core::templates_schema_resolution resolve_instructions_rejects_traversal_template_path
          PASS [   5.685s] (1280/1609) ito-core::templates_schema_resolution resolve_schema_prefers_project_over_user_override
          PASS [   5.692s] (1281/1609) ito-core::templates_schema_resolution export_embedded_schemas_writes_then_skips_without_force
          PASS [   8.421s] (1282/1609) ito-core::templates_change_status compute_change_status_rejects_invalid_change_name
          PASS [   8.425s] (1283/1609) ito-core::templates_change_status compute_change_status_marks_ready_and_blocked_based_on_generated_files
          PASS [   9.433s] (1284/1609) ito-core::templates_review_context compute_review_context_collects_artifacts_validation_tasks_and_specs
          PASS [  10.318s] (1285/1609) ito-core::templates_apply_instructions compute_apply_instructions_reports_blocked_states_and_progress
          PASS [   6.674s] (1286/1609) ito-core::templates_schemas_listing list_schemas_detail_entries_have_artifacts
          PASS [   5.928s] (1287/1609) ito-core::templates_schemas_listing list_schemas_detail_entries_have_descriptions
          PASS [   5.926s] (1288/1609) ito-core::templates_schemas_listing list_schemas_detail_returns_all_embedded_schemas
          PASS [   8.328s] (1289/1609) ito-core::templates_schemas_listing list_schemas_detail_all_sources_are_embedded
          PASS [   5.928s] (1290/1609) ito-core::templates_schemas_listing list_schemas_detail_json_round_trips
          PASS [   5.929s] (1291/1609) ito-core::templates_schemas_listing list_schemas_detail_is_sorted
          PASS [   5.925s] (1292/1609) ito-core::templates_schemas_listing list_schemas_detail_spec_driven_has_expected_artifacts
          PASS [   5.927s] (1293/1609) ito-core::templates_schemas_listing list_schemas_detail_recommended_default_is_spec_driven
          PASS [   8.329s] (1294/1609) ito-core::templates_schemas_listing built_in_minimalist_and_event_driven_spec_templates_use_delta_shape
          PASS [   8.697s] (1295/1609) ito-core::templates_user_guidance load_user_guidance_for_artifact_reads_scoped_file
          PASS [   4.067s] (1296/1609) ito-core::templates_user_guidance load_user_guidance_strips_managed_header_block
          PASS [   8.691s] (1297/1609) ito-core::templates_user_guidance load_user_guidance_for_artifact_rejects_path_traversal_ids
          PASS [   4.950s] (1298/1609) ito-core::templates_user_guidance load_user_guidance_strips_ito_internal_comment_block
          PASS [   5.963s] (1299/1609) ito-core::templates_user_guidance load_user_guidance_for_artifact_strips_managed_header_block
          PASS [   8.698s] (1300/1609) ito-core::templates_user_guidance load_composed_user_guidance_combines_scoped_and_shared
          PASS [   5.961s] (1301/1609) ito-core::templates_user_guidance load_user_guidance_prefers_user_prompts_guidance_file
          PASS [   5.046s] (1302/1609) ito-core::traceability_e2e legacy_checkbox_change_validate_passes_without_traceability_checks
          PASS [   5.047s] (1303/1609) ito-core::traceability_e2e legacy_checkbox_change_trace_output_is_unavailable
          PASS [   5.061s] (1304/1609) ito-core::traceability_e2e duplicate_requirement_ids_trace_output_has_diagnostics
          PASS [   5.061s] (1305/1609) ito-core::traceability_e2e shelved_task_leaves_requirement_uncovered
          PASS [   2.287s] (1306/1609) ito-core::traceability_e2e traced_change_uncovered_req_trace_output_shows_uncovered
          PASS [   2.289s] (1307/1609) ito-core::traceability_e2e traced_change_uncovered_req_is_error_in_strict
          PASS [   5.064s] (1308/1609) ito-core::traceability_e2e partial_ids_trace_output_is_invalid
          PASS [   5.064s] (1309/1609) ito-core::traceability_e2e partial_ids_validate_reports_error
          PASS [   2.289s] (1310/1609) ito-core::traceability_e2e traced_change_uncovered_req_is_warning_in_non_strict
          PASS [   5.064s] (1311/1609) ito-core::traceability_e2e traced_change_all_covered_trace_output_is_ready
          PASS [   2.290s] (1312/1609) ito-core::traceability_e2e traced_change_all_covered_validate_passes
          PASS [   2.289s] (1313/1609) ito-core::traceability_e2e traced_change_unresolved_ref_is_error_in_validate
          PASS [   5.064s] (1314/1609) ito-core::traceability_e2e shelved_task_uncovered_req_is_warning_in_validate
          PASS [   5.065s] (1315/1609) ito-core::traceability_e2e duplicate_requirement_ids_produce_error_in_validate
          PASS [   2.290s] (1316/1609) ito-core::traceability_e2e traced_change_unresolved_ref_trace_output_shows_unresolved
          PASS [   4.682s] (1317/1609) ito-core::validate validate_module_errors_when_sub_module_has_invalid_naming
          PASS [   4.685s] (1318/1609) ito-core::validate validate_change_skips_optional_validator_when_artifact_is_missing
          PASS [   4.702s] (1319/1609) ito-core::validate validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas
          PASS [   6.974s] (1320/1609) ito-core::validate empty_tracking_file_is_warning_in_non_strict_and_error_in_strict
          PASS [   0.020s] (1321/1609) ito-core::validate validate_spec_markdown_strict_treats_warnings_as_invalid
          PASS [   0.021s] (1322/1609) ito-core::validate validate_spec_markdown_reports_missing_purpose_and_requirements
          PASS [   0.022s] (1323/1609) ito-core::validate validate_tasks_file_returns_diagnostics_for_malformed_content
          PASS [   0.024s] (1324/1609) ito-core::validate validate_tasks_file_issues_cite_tasks_tracking_validator_id
          PASS [   0.015s] (1325/1609) ito-core::validate validate_tasks_file_returns_error_for_missing_file
          PASS [   0.016s] (1326/1609) ito-core::validate validate_tasks_file_returns_empty_for_valid_tasks
          PASS [   0.012s] (1327/1609) ito-core::validate validate_tasks_file_uses_apply_tracks_when_set
          PASS [   6.098s] (1328/1609) ito-core::validate validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas
          PASS [   6.113s] (1329/1609) ito-core::validate validate_change_requires_at_least_one_delta
          PASS [   6.111s] (1330/1609) ito-core::validate validate_change_validates_apply_tracks_file_when_configured
          PASS [   6.111s] (1331/1609) ito-core::validate validate_module_reports_missing_scope_and_short_purpose
          PASS [   6.113s] (1332/1609) ito-core::validate validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas
          PASS [   6.115s] (1333/1609) ito-core::validate validate_module_errors_when_sub_module_missing_module_md
          PASS [   6.115s] (1334/1609) ito-core::validate validate_module_warns_when_sub_module_purpose_too_short
          PASS [   6.134s] (1335/1609) ito-core::validate validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking
          PASS [   6.117s] (1336/1609) ito-core::validate validate_change_uses_apply_tracks_for_legacy_delta_schemas
          PASS [   6.117s] (1337/1609) ito-core::validate validate_module_passes_when_sub_modules_have_valid_module_md
          PASS [   6.120s] (1338/1609) ito-core::validate validate_change_requires_shall_or_must_in_requirement_text
          PASS [   6.118s] (1339/1609) ito-core::validate validate_change_uses_validation_yaml_delta_specs_validator_when_configured
          PASS [   3.798s] (1340/1609) ito-core::validate_delta_rules scenario_grammar_rule_accepts_ordered_list_steps
          PASS [   3.801s] (1341/1609) ito-core::validate_delta_rules scenario_grammar_rule_reports_missing_when_then_and_given
          PASS [   3.804s] (1342/1609) ito-core::validate_delta_rules contract_refs_rule_rejects_unknown_schemes
          PASS [   5.214s] (1343/1609) ito-core::validate_delta_rules capabilities_consistency_rule_errors_for_listed_capability_without_delta
          PASS [   5.214s] (1344/1609) ito-core::validate_delta_rules capabilities_consistency_rule_errors_for_unlisted_delta_capability
          PASS [   5.212s] (1345/1609) ito-core::validate_delta_rules capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets
          PASS [   3.823s] (1346/1609) ito-core::validate_delta_rules contract_refs_rule_accepts_known_schemes_and_emits_single_advisory
          PASS [   3.823s] (1347/1609) ito-core::validate_delta_rules contract_refs_rule_rejects_short_unknown_scheme_after_known_ref
          PASS [   3.836s] (1348/1609) ito-core::validate_delta_rules capabilities_consistency_rule_warns_on_invalid_change_shape_values
          PASS [   3.826s] (1349/1609) ito-core::validate_delta_rules scenario_grammar_rule_accepts_steps_without_bullets
          PASS [   3.832s] (1350/1609) ito-core::validate_delta_rules contract_refs_rule_rejects_lowercase_unknown_scheme_after_known_ref
          PASS [   3.827s] (1351/1609) ito-core::validate_delta_rules scenario_grammar_rule_accepts_asterisk_bullets
          PASS [   3.825s] (1352/1609) ito-core::validate_delta_rules scenario_grammar_rule_warns_on_excessive_step_count
          PASS [   3.831s] (1353/1609) ito-core::validate_delta_rules contract_refs_rule_rejects_unknown_scheme_after_known_ref
          PASS [   3.829s] (1354/1609) ito-core::validate_delta_rules contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor
          PASS [   5.237s] (1355/1609) ito-core::validate_delta_rules capabilities_consistency_rule_checks_new_vs_modified_against_baseline
          PASS [   0.032s] (1356/1609) ito-core::validate_delta_rules ui_mechanics_rule_warns_only_for_ui_tags
          PASS [   0.035s] (1357/1609) ito-core::validate_delta_rules ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error
          PASS [   0.022s] (1358/1609) ito-core::validate_rules_extension validation_yaml_proposal_entry_dispatches_rule_configuration
          PASS [   0.035s] (1359/1609) ito-core::validate_rules_extension missing_tracking_file_uses_configured_missing_artifact_level
          PASS [   0.026s] (1360/1609) ito-core::validate_rules_extension validation_yaml_rules_extension_warns_for_unknown_rule_names
          PASS [   0.028s] (1361/1609) ito-core::validate_rules_extension validation_yaml_delta_rules_work_for_non_specs_artifact_ids
          PASS [   0.029s] (1362/1609) ito-core::validate_tracking_rules task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable
          PASS [   0.036s] (1363/1609) ito-core::validate_tracking_rules task_quality_rule_enforces_done_when_and_verify_for_impl_tasks
          PASS [   0.027s] (1364/1609) ito-core::validate_tracking_rules task_quality_rule_errors_on_unknown_requirement_ids
          PASS [   0.026s] (1365/1609) ito-core::validate_tracking_rules task_quality_rule_treats_gradle_files_as_implementation_work
          PASS [   0.036s] (1366/1609) ito-core::validate_tracking_rules task_quality_rule_errors_on_missing_status
          PASS [   0.031s] (1367/1609) ito-core::validate_tracking_rules task_quality_rule_respects_warning_floor_without_promoting_advisories
          PASS [   0.042s] (1368/1609) ito-core::worktree_ensure_e2e ensure_worktree_disabled_returns_cwd
          PASS [   0.050s] (1369/1609) ito-core::validate_tracking_rules task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify
          PASS [   0.022s] (1370/1609) ito-domain audit::event::tests::builder_returns_none_without_required_fields
          PASS [   0.029s] (1371/1609) ito-domain audit::event::tests::audit_event_serializes_to_single_line
          PASS [   0.036s] (1372/1609) ito-domain audit::event::tests::actor_serializes_to_lowercase
          PASS [   0.052s] (1373/1609) ito-domain audit::context::tests::resolve_harness_session_id_returns_none_without_env
          PASS [   0.036s] (1374/1609) ito-domain audit::event::tests::audit_event_round_trip_serialization
          PASS [   0.043s] (1375/1609) ito-domain audit::event::tests::actor_round_trip
          PASS [   0.047s] (1376/1609) ito-domain audit::context::tests::resolve_session_id_is_stable_across_calls
          PASS [   0.042s] (1377/1609) ito-domain audit::event::tests::builder_produces_valid_event
          PASS [   0.058s] (1378/1609) ito-domain audit::context::tests::resolve_session_id_generates_uuid
          PASS [   0.039s] (1379/1609) ito-domain audit::event::tests::builder_with_meta
          PASS [   0.045s] (1380/1609) ito-domain audit::event::tests::entity_type_as_str_matches_serde
          PASS [   0.043s] (1381/1609) ito-domain audit::event::tests::entity_type_display
          PASS [   0.080s] (1382/1609) ito-domain audit::context::tests::resolve_user_identity_returns_at_prefixed_string
          PASS [   0.039s] (1383/1609) ito-domain audit::event::tests::entity_type_serializes_to_lowercase
          PASS [   0.039s] (1384/1609) ito-domain audit::event::tests::event_context_round_trip
          PASS [   0.046s] (1385/1609) ito-domain audit::event::tests::entity_type_round_trip
          PASS [   0.040s] (1386/1609) ito-domain audit::materialize::tests::empty_events_produce_empty_state
          PASS [   0.046s] (1387/1609) ito-domain audit::event::tests::optional_fields_omitted_when_none
          PASS [   0.046s] (1388/1609) ito-domain audit::event::tests::schema_version_is_one
          PASS [   0.039s] (1389/1609) ito-domain audit::materialize::tests::global_entities_have_no_scope
          PASS [   0.045s] (1390/1609) ito-domain audit::materialize::tests::archive_event_without_to_uses_sentinel
          PASS [   0.039s] (1391/1609) ito-domain audit::materialize::tests::last_event_wins
          PASS [   0.032s] (1392/1609) ito-domain audit::materialize::tests::single_create_event
          PASS [   0.033s] (1393/1609) ito-domain audit::materialize::tests::reconciled_events_update_state
          PASS [   0.033s] (1394/1609) ito-domain audit::materialize::tests::multiple_entities_tracked_independently
          PASS [   0.032s] (1395/1609) ito-domain audit::reconcile::tests::detect_diverged_status
          PASS [   0.032s] (1396/1609) ito-domain audit::materialize::tests::status_change_updates_state
          PASS [   0.032s] (1397/1609) ito-domain audit::reconcile::tests::compensating_events_use_scope_from_drift_key
          PASS [   0.038s] (1398/1609) ito-domain audit::reconcile::tests::display_drift_items
          PASS [   0.038s] (1399/1609) ito-domain audit::reconcile::tests::detect_missing_entity_in_log
          PASS [   0.038s] (1400/1609) ito-domain audit::reconcile::tests::generate_compensating_events_for_diverged
          PASS [   0.038s] (1401/1609) ito-domain audit::reconcile::tests::generate_compensating_events_for_extra
          PASS [   0.038s] (1402/1609) ito-domain audit::reconcile::tests::detect_extra_in_log
          PASS [   0.032s] (1403/1609) ito-domain audit::reconcile::tests::generate_compensating_events_for_missing
          PASS [   0.039s] (1404/1609) ito-domain audit::writer::tests::noop_writer_returns_ok
          PASS [   0.045s] (1405/1609) ito-domain audit::writer::tests::noop_writer_is_object_safe
          PASS [   0.039s] (1406/1609) ito-domain audit::writer::tests::trait_is_object_safe_for_dyn_dispatch
          PASS [   0.039s] (1407/1609) ito-domain audit::writer::tests::noop_writer_is_send_sync
          PASS [   0.045s] (1408/1609) ito-domain audit::reconcile::tests::no_drift_when_states_match
          PASS [   0.045s] (1409/1609) ito-domain audit::reconcile::tests::multiple_drift_types_detected
          PASS [   0.033s] (1410/1609) ito-domain backend::tests::backend_error_display_revision_conflict
          PASS [   0.040s] (1411/1609) ito-domain backend::tests::artifact_bundle_roundtrip
          PASS [   0.041s] (1412/1609) ito-domain backend::tests::backend_error_display_lease_conflict
          PASS [   0.041s] (1413/1609) ito-domain backend::tests::backend_error_display_other
          PASS [   0.041s] (1414/1609) ito-domain backend::tests::archive_result_roundtrip
          PASS [   0.041s] (1415/1609) ito-domain backend::tests::backend_error_display_not_found
          PASS [   0.183s] (1416/1609) ito-domain audit::context::tests::resolve_context_populates_session_id
          PASS [   0.184s] (1417/1609) ito-domain audit::context::tests::resolve_git_context_does_not_panic
          PASS [   0.018s] (1418/1609) ito-domain changes::tests::test_change_sub_module_id_field
          PASS [   0.019s] (1419/1609) ito-domain backend::tests::backend_error_display_unauthorized
          PASS [   0.019s] (1420/1609) ito-domain backend::tests::event_ingest_result_roundtrip
          PASS [   0.019s] (1421/1609) ito-domain backend::tests::backend_error_display_unavailable
          PASS [   0.021s] (1422/1609) ito-domain changes::tests::test_change_status_display
          PASS [   0.021s] (1423/1609) ito-domain backend::tests::event_batch_roundtrip
          PASS [   0.008s] (1424/1609) ito-domain changes::tests::test_normalize_id
          PASS [   0.008s] (1425/1609) ito-domain changes::tests::test_parse_change_id
          PASS [   0.009s] (1426/1609) ito-domain changes::tests::test_change_summary_status
          PASS [   0.009s] (1427/1609) ito-domain changes::tests::test_extract_module_id
          PASS [   0.010s] (1428/1609) ito-domain changes::tests::test_change_work_status
          PASS [   0.009s] (1429/1609) ito-domain changes::tests::test_parse_change_id_sub_module_format
          PASS [   0.011s] (1430/1609) ito-domain changes::tests::test_extract_sub_module_id
          PASS [   0.008s] (1431/1609) ito-domain changes::tests::test_parse_module_id
          PASS [   0.010s] (1432/1609) ito-domain discovery::tests::list_changes_skips_archive_dir
          PASS [   0.009s] (1433/1609) ito-domain errors::tests::ambiguous_target_joins_candidates_in_display_message
          PASS [   0.010s] (1434/1609) ito-domain discovery::tests::list_module_ids_extracts_numeric_prefixes
          PASS [   0.008s] (1435/1609) ito-domain errors::tests::io_constructor_preserves_context_and_source
          PASS [   0.011s] (1436/1609) ito-domain discovery::tests::list_modules_only_returns_directories
          PASS [   0.009s] (1437/1609) ito-domain modules::tests::test_module_with_sub_modules
          PASS [   0.010s] (1438/1609) ito-domain errors::tests::not_found_constructor_formats_display_message
          PASS [   0.009s] (1439/1609) ito-domain modules::tests::test_module_creation
          PASS [   0.019s] (1440/1609) ito-domain modules::tests::test_module_summary
          PASS [   0.018s] (1441/1609) ito-domain modules::tests::test_sub_module_creation
          PASS [   0.018s] (1442/1609) ito-domain tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_accepts_valid_formats
          PASS [   0.018s] (1443/1609) ito-domain tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_handles_large_numbers
          PASS [   0.020s] (1444/1609) ito-domain modules::tests::test_module_summary_with_sub_modules
          PASS [   0.018s] (1445/1609) ito-domain modules::tests::test_sub_module_summary_creation
          PASS [   0.016s] (1446/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_colon_suffix
          PASS [   0.018s] (1447/1609) ito-domain tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_rejects_invalid_formats
          PASS [   0.016s] (1448/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_dot_suffix
          PASS [   0.020s] (1449/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_edge_case_single_digit_with_many_dots
          PASS [   0.021s] (1450/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_extracts_id_and_rest
          PASS [   0.027s] (1451/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_multiple_spaces
          PASS [   0.027s] (1452/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_tab_separator
          PASS [   0.017s] (1453/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_returns_none_for_invalid_inputs
          PASS [   0.017s] (1454/1609) ito-domain tasks::compute::tests::enhanced_ready_and_blocked_lists_are_sorted_by_task_id
          PASS [   0.018s] (1455/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_unicode_in_task_name
          PASS [   0.028s] (1456/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_leading_whitespace
          PASS [   0.017s] (1457/1609) ito-domain tasks::compute::tests::checkbox_mode_returns_pending_sorted_and_no_blocked
          PASS [   0.018s] (1458/1609) ito-domain tasks::checkbox::checkbox_tests::split_checkbox_task_label_preserves_trailing_whitespace_in_rest
          PASS [   0.020s] (1459/1609) ito-domain tasks::compute::tests::enhanced_backcompat_blocks_later_waves_and_checkpoints_until_first_incomplete_wave_done
          PASS [   0.020s] (1460/1609) ito-domain tasks::compute::tests::enhanced_task_dependencies_produce_missing_crosswave_and_not_complete_blockers
          PASS [   0.021s] (1461/1609) ito-domain tasks::compute::tests::enhanced_wave_dependency_blocks_by_wave_and_unblocks_when_complete
          PASS [   0.017s] (1462/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_detects_simple_two_node_cycle
          PASS [   0.021s] (1463/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_detects_cycle_in_complex_graph
          PASS [   0.019s] (1464/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_detects_self_loop
          PASS [   0.010s] (1465/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_handles_diamond_pattern_without_cycle
          PASS [   0.012s] (1466/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_acyclic_graph
          PASS [   0.012s] (1467/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_handles_long_cycle
          PASS [   0.012s] (1468/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_handles_multiple_cycles_returns_one
          PASS [   0.013s] (1469/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_empty_graph
          PASS [   0.014s] (1470/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_detects_three_node_cycle
          PASS [   0.014s] (1471/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_handles_special_characters_in_node_names
          PASS [   0.011s] (1472/1609) ito-domain tasks::relational::relational_tests::validate_relational_accepts_valid_dependency_graph
          PASS [   0.014s] (1473/1609) ito-domain tasks::cycle::cycle_tests::find_cycle_path_with_numeric_node_names
          PASS [   0.011s] (1474/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_duplicate_task_ids
          PASS [   0.011s] (1475/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_cross_wave_task_dependencies
          PASS [   0.011s] (1476/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_dependency_on_shelved_task
          PASS [   0.014s] (1477/1609) ito-domain tasks::relational::relational_tests::validate_relational_allows_shelved_task_depending_on_shelved_task
          PASS [   0.013s] (1478/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_missing_task_dependencies
          PASS [   0.012s] (1479/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_self_referencing_task
          PASS [   0.010s] (1480/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_three_node_task_cycle
          PASS [   0.010s] (1481/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_task_dependency_cycle
          PASS [   0.010s] (1482/1609) ito-domain tasks::relational::relational_tests::validate_relational_marks_errors_as_error_level
          PASS [   0.011s] (1483/1609) ito-domain tasks::relational::relational_tests::validate_relational_detects_wave_dependency_cycle
          PASS [   0.013s] (1484/1609) ito-domain tasks::relational::relational_tests::validate_relational_handles_tasks_without_wave
          PASS [   0.260s] (1485/1609) ito-core::worktree_ensure_e2e ensure_worktree_creates_and_initializes_with_include_files
          PASS [   0.017s] (1486/1609) ito-domain tasks::relational::relational_tests::validate_relational_multiple_errors_for_same_task
          PASS [   0.025s] (1487/1609) ito-domain tasks::relational::relational_tests::validate_relational_ignores_empty_and_checkpoint_dependencies
          PASS [   0.021s] (1488/1609) ito-domain tasks::relational::relational_tests::validate_relational_with_complex_valid_graph
          PASS [   0.025s] (1489/1609) ito-domain tasks::relational::relational_tests::validate_relational_reports_line_numbers
          PASS [   0.022s] (1490/1609) ito-domain::schema_roundtrip workflow_execution_json_roundtrip
          PASS [   0.024s] (1491/1609) ito-domain::planning roadmap_parsing_extracts_current_progress_and_phases
          PASS [   0.012s] (1492/1609) ito-domain::schema_validation wave_definition_validate_rejects_invalid_shapes
          PASS [   0.022s] (1493/1609) ito-domain::schema_validation execution_validate_rejects_invalid_fields_and_accepts_valid
          PASS [   0.021s] (1494/1609) ito-domain::schema_validation execution_validate_rejects_out_of_bounds_wave_index
          PASS [   0.020s] (1495/1609) ito-domain::schema_validation task_definition_validate_accepts_optional_fields
          PASS [   0.025s] (1496/1609) ito-domain::schema_roundtrip workflow_plan_json_roundtrip
          PASS [   0.021s] (1497/1609) ito-domain::schema_validation plan_validate_rejects_other_invalid_fields
          PASS [   0.018s] (1498/1609) ito-domain::schema_validation task_definition_validate_rejects_invalid_fields
          PASS [   0.278s] (1499/1609) ito-core::worktree_ensure_e2e ensure_worktree_with_setup_script
          PASS [   0.030s] (1500/1609) ito-domain::schema_roundtrip workflow_yaml_roundtrip
          PASS [   0.023s] (1501/1609) ito-domain::schema_validation task_execution_validate_rejects_empty_optional_strings
          PASS [   0.012s] (1502/1609) ito-domain::schema_validation workflow_definition_validate_accepts_minimal_valid
          PASS [   0.028s] (1503/1609) ito-domain::schema_validation plan_validate_rejects_empty_prompt_content
          PASS [   0.015s] (1504/1609) ito-domain::schema_validation workflow_definition_validate_rejects_duplicate_wave_ids
          PASS [   0.016s] (1505/1609) ito-domain::schema_validation workflow_definition_validate_rejects_empty_fields
          PASS [   0.015s] (1506/1609) ito-domain::schema_validation workflow_definition_validate_rejects_requires_and_context_files_empty_entries
          PASS [   0.016s] (1507/1609) ito-domain::tasks update_enhanced_task_status_inserts_or_replaces_status_line
          PASS [   0.018s] (1508/1609) ito-domain::tasks_parsing detect_tasks_format_enhanced_vs_checkbox
          PASS [   0.017s] (1509/1609) ito-domain::tasks_parsing parse_checkbox_tasks_accepts_right_arrow_in_progress_marker
          PASS [   0.014s] (1510/1609) ito-domain::tasks_parsing parse_checkbox_tasks_handles_empty_lines_and_non_checkbox_content
          PASS [   0.011s] (1511/1609) ito-domain::tasks_parsing parse_checkbox_tasks_uppercase_x_marks_complete
          PASS [   0.011s] (1512/1609) ito-domain::tasks_parsing parse_checkbox_tasks_handles_mixed_explicit_and_implicit_ids
          PASS [   0.011s] (1513/1609) ito-domain::tasks_parsing parse_checkbox_tasks_supports_dash_and_star
          PASS [   0.017s] (1514/1609) ito-domain::tasks_parsing parse_checkbox_tasks_assigns_sequential_ids_when_not_explicit
          PASS [   0.022s] (1515/1609) ito-domain::tasks_parsing enhanced_tasks_wave_gating_blocks_later_waves
          PASS [   0.024s] (1516/1609) ito-domain::tasks enhanced_template_parses_and_has_checkpoint_warning
          PASS [   0.023s] (1517/1609) ito-domain::tasks_parsing enhanced_tasks_diagnostics_cover_common_errors
          PASS [   0.023s] (1518/1609) ito-domain::tasks_parsing enhanced_tasks_cycles_and_shelved_deps_are_reported
          PASS [   0.018s] (1519/1609) ito-domain::tasks_parsing parse_checkbox_tasks_preserves_explicit_ids
          PASS [   0.014s] (1520/1609) ito-domain::tasks_parsing parse_enhanced_tasks_accepts_wave_heading_titles
          PASS [   0.016s] (1521/1609) ito-domain::tasks_parsing parse_enhanced_tasks_accepts_all_prior_tasks_dependency_shorthand
          PASS [   0.014s] (1522/1609) ito-domain::tasks_parsing parse_enhanced_tasks_extracts_requirements_field
          PASS [   0.011s] (1523/1609) ito-domain::tasks_parsing parse_enhanced_tasks_progress_counts_all_statuses
          PASS [   0.013s] (1524/1609) ito-domain::tasks_parsing parse_enhanced_tasks_handles_empty_dependencies_field
          PASS [   0.012s] (1525/1609) ito-domain::tasks_parsing parse_enhanced_tasks_handles_wave_with_comma_in_title
          PASS [   0.012s] (1526/1609) ito-domain::tasks_parsing parse_enhanced_tasks_requirements_absent_gives_empty_vec
          PASS [   0.012s] (1527/1609) ito-domain::tasks_parsing parse_enhanced_tasks_handles_multiline_action
          PASS [   0.012s] (1528/1609) ito-domain::tasks_parsing parse_enhanced_tasks_parses_fields_and_action_block
          PASS [   0.013s] (1529/1609) ito-domain::tasks_parsing parse_enhanced_tasks_handles_multiple_files
          PASS [   0.013s] (1530/1609) ito-domain::tasks_parsing parse_enhanced_tasks_handles_task_without_optional_prefix
          PASS [   0.011s] (1531/1609) ito-domain::tasks_parsing parse_enhanced_tasks_requirements_not_carried_across_tasks
          PASS [   0.016s] (1532/1609) ito-domain::tasks_parsing tasks_path_checked_rejects_traversal_like_change_ids
          PASS [   0.016s] (1533/1609) ito-domain::tasks_parsing tasks_path_uses_safe_fallback_for_invalid_change_id
          PASS [   0.013s] (1534/1609) ito-domain::tasks_parsing update_checkbox_task_status_sets_marker_and_preserves_text
          PASS [   0.014s] (1535/1609) ito-domain::tasks_parsing update_checkbox_task_status_preserves_bullet_style
          PASS [   0.015s] (1536/1609) ito-domain::tasks_parsing update_checkbox_task_status_by_explicit_id
          PASS [   0.014s] (1537/1609) ito-domain::tasks_parsing update_enhanced_task_status_inserts_missing_fields
          PASS [   0.020s] (1538/1609) ito-domain::tasks_parsing parse_enhanced_tasks_requirements_single_entry
          PASS [   0.015s] (1539/1609) ito-domain::tasks_parsing update_enhanced_task_status_preserves_existing_fields
          PASS [   0.015s] (1540/1609) ito-domain::tasks_parsing update_enhanced_task_status_preserves_requirements_line
          PASS [   0.018s] (1541/1609) ito-domain::tasks_parsing_additional checkbox_format_handles_empty_task_text
          PASS [   0.018s] (1542/1609) ito-domain::tasks_parsing_additional checkbox_format_handles_special_characters_in_task_names
          PASS [   0.016s] (1543/1609) ito-domain::tasks_parsing_additional checkbox_format_progress_info_counts_correctly
          PASS [   0.018s] (1544/1609) ito-domain::tasks_parsing_additional checkbox_format_handles_newlines_in_adjacent_lines
          PASS [   0.018s] (1545/1609) ito-domain::tasks_parsing_additional checkbox_format_ignores_incomplete_checkbox_patterns
          PASS [   0.018s] (1546/1609) ito-domain::tasks_parsing_additional checkbox_format_handles_very_long_task_names
          PASS [   0.018s] (1547/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_checkpoints
          PASS [   0.012s] (1548/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_multiple_files_with_spaces
          PASS [   0.013s] (1549/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_duplicate_wave_numbers
          PASS [   0.012s] (1550/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_status_marker_mismatch
          PASS [   0.014s] (1551/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_multiline_action_with_code
          PASS [   0.017s] (1552/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_complex_dependency_chains
          PASS [   0.018s] (1553/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_empty_action_block
          PASS [   0.014s] (1554/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_very_long_file_paths
          PASS [   0.017s] (1555/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_very_large_wave_numbers
          PASS [   0.019s] (1556/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_task_without_wave
          PASS [   0.010s] (1557/1609) ito-domain::tasks_parsing_additional tasks_path_checked_rejects_very_long_change_ids
          PASS [   0.019s] (1558/1609) ito-domain::tasks_parsing_additional enhanced_format_handles_uppercase_x_in_complete_marker
          PASS [   0.016s] (1559/1609) ito-domain::tasks_parsing_additional parse_empty_file_returns_empty_result
          PASS [   0.013s] (1560/1609) ito-domain::tasks_parsing_additional progress_info_calculates_remaining_correctly
          PASS [   0.017s] (1561/1609) ito-domain::tasks_parsing_additional enhanced_format_validates_date_format_strictly
          PASS [   0.018s] (1562/1609) ito-domain::tasks_parsing_additional enhanced_format_validates_missing_required_fields
          PASS [   0.016s] (1563/1609) ito-domain::tasks_parsing_additional tasks_path_checked_rejects_empty_change_id
          PASS [   0.016s] (1564/1609) ito-domain::tasks_parsing_additional tasks_path_checked_accepts_valid_change_ids
          PASS [   0.021s] (1565/1609) ito-domain::tasks_parsing_additional parse_file_with_only_whitespace
          PASS [   0.021s] (1566/1609) ito-domain::tasks_parsing_additional parse_file_with_only_non_task_content
          PASS [   0.020s] (1567/1609) ito-domain::tasks_parsing_additional wave_dependencies_detect_forward_references
          PASS [   0.019s] (1568/1609) ito-domain::tasks_parsing_additional wave_dependencies_handle_various_formats
          PASS [   0.016s] (1569/1609) ito-domain::tasks_update update_checkbox_task_status_errors_for_invalid_or_missing_task_id
          PASS [   0.016s] (1570/1609) ito-domain::tasks_update update_checkbox_task_status_handles_mixed_explicit_and_implicit_ids
          PASS [   0.015s] (1571/1609) ito-domain::tasks_update update_checkbox_task_status_handles_various_markers
          PASS [   0.010s] (1572/1609) ito-domain::tasks_update update_checkbox_task_status_with_id_suffix_dot
          PASS [   0.013s] (1573/1609) ito-domain::tasks_update update_checkbox_task_status_rejects_shelving
          PASS [   0.014s] (1574/1609) ito-domain::tasks_update update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting
          PASS [   0.011s] (1575/1609) ito-domain::tasks_update update_enhanced_task_status_handles_in_progress
          PASS [   0.014s] (1576/1609) ito-domain::tasks_update update_checkbox_task_status_preserves_bullet_style
          PASS [   0.012s] (1577/1609) ito-domain::tasks_update update_checkbox_task_status_with_id_suffix_colon
          PASS [   0.017s] (1578/1609) ito-domain::tasks_update update_checkbox_task_status_handles_unicode_in_task_text
          PASS [   0.012s] (1579/1609) ito-domain::tasks_update update_enhanced_task_status_handles_complex_task_ids
          PASS [   0.017s] (1580/1609) ito-domain::tasks_update update_checkbox_task_status_matches_explicit_ids_over_index
          PASS [   0.024s] (1581/1609) ito-domain::tasks_quality_fields quality_fields_allow_missing_optional_metadata
          PASS [   0.020s] (1582/1609) ito-domain::tasks_quality_fields quality_fields_round_trip_when_present
          PASS [   0.014s] (1583/1609) ito-domain::tasks_update update_enhanced_task_status_handles_task_prefix_optional
          PASS [   0.016s] (1584/1609) ito-domain::tasks_update update_enhanced_task_status_handles_shelved
          PASS [   0.015s] (1585/1609) ito-domain::tasks_update update_enhanced_task_status_preserves_trailing_newline
          PASS [   0.015s] (1586/1609) ito-domain::tasks_update update_enhanced_task_status_updates_status_and_date
          PASS [   0.016s] (1587/1609) ito-domain::tasks_update update_enhanced_task_status_preserves_other_fields
          PASS [   0.016s] (1588/1609) ito-domain::tasks_update update_enhanced_task_status_inserts_missing_fields
          PASS [   0.016s] (1589/1609) ito-domain::tasks_update update_enhanced_task_status_only_updates_specified_task
          PASS [   0.016s] (1590/1609) ito-domain::traceability checkbox_format_gives_unavailable
          PASS [   0.019s] (1591/1609) ito-domain::traceability duplicate_requirement_ids_flagged_in_diagnostics
          PASS [   0.018s] (1592/1609) ito-domain::traceability empty_requirements_list_gives_unavailable
          PASS [   0.018s] (1593/1609) ito-domain::traceability in_progress_task_counts_as_coverage
          PASS [   0.018s] (1594/1609) ito-domain::traceability multiple_tasks_can_cover_same_requirement
          PASS [   0.020s] (1595/1609) ito-domain::traceability all_requirements_covered_by_tasks
          PASS [   0.019s] (1596/1609) ito-domain::traceability declared_requirements_are_sorted_and_deduplicated
          PASS [   0.019s] (1597/1609) ito-domain::traceability complete_task_counts_as_coverage
          PASS [   0.018s] (1598/1609) ito-domain::traceability no_requirement_ids_gives_unavailable
          PASS [   0.016s] (1599/1609) ito-domain::traceability partial_ids_gives_invalid_with_missing_titles
          PASS [   0.016s] (1600/1609) ito-domain::traceability shelved_task_does_not_count_as_coverage
          PASS [   0.012s] (1601/1609) ito-test-support tests::normalize_strips_ansi_and_crlf
          PASS [   0.013s] (1602/1609) ito-test-support tests::normalize_replaces_home_path
          PASS [   0.009s] (1603/1609) ito-test-support::mock_repos_smoke mock_task_repo_returns_configured_tasks
          PASS [   0.009s] (1604/1609) ito-test-support::mock_repos_smoke mock_module_repo_resolves_by_id_or_name
          PASS [   0.009s] (1605/1609) ito-test-support::mock_repos_smoke mock_repos_basic_roundtrip
          PASS [   0.014s] (1606/1609) ito-test-support tests::copy_dir_all_copies_nested_files
          PASS [   0.015s] (1607/1609) ito-domain::traceability uncovered_requirement_appears_in_uncovered_list
          PASS [   0.015s] (1608/1609) ito-domain::traceability unresolved_task_reference_is_reported
          PASS [   0.019s] (1609/1609) ito-test-support pty::tests::pty_can_echo_input_via_cat
  ────────────
       Summary [ 235.768s] 1609 tests run: 1609 passed (1 leaky), 4 skipped
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
make: *** [check-prek] Error 1
```
