# Iteration 4: DDD Discovery Workflow Verification

*2026-05-11T20:31:17Z by Showboat 0.6.1*
<!-- showboat-id: 73c99c35-036f-4592-80fa-9366ebbb7e90 -->

Verified the DDD discovery workflow after repairing optional artifact apply gating, translation-boundary handoff wording, and the max-lines test split.

```bash
cargo test -p ito-core --test templates_apply_instructions --test templates_change_status --test validate_domain_discovery_rules
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.21s
     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-62f16cea2c1fd28e)

running 2 tests
test compute_apply_instructions_ignores_optional_artifacts_when_apply_requires_is_omitted ... ok
test compute_apply_instructions_reports_blocked_states_and_progress ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-124bdabc9a3b5538)

running 3 tests
test compute_change_status_rejects_invalid_change_name ... ok
test compute_change_status_treats_missing_optional_artifacts_as_non_blocking ... ok
test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/validate_domain_discovery_rules.rs (target/debug/deps/validate_domain_discovery_rules-adec62e6c5766afb)

running 5 tests
test context_boundary_consistency_rule_is_silent_for_single_context_discovery ... ok
test context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing ... ok
test context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation ... ok
test domain_rules_are_silent_without_domain_discovery_handoff ... ok
test domain_rules_can_run_from_artifact_rules_for_event_driven_schemas ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

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
  ito-backend/src/api.rs                                    729               165    77.37%          66                15    77.27%         536               106    80.22%           0                 0         -
  ito-backend/src/auth.rs                                   333                 7    97.90%          24                 0   100.00%         172                 3    98.26%           0                 0         -
  ito-backend/src/error.rs                                  165                34    79.39%          20                 2    90.00%         152                21    86.18%           0                 0         -
  ito-backend/src/server.rs                                 143                55    61.54%          10                 5    50.00%          92                39    57.61%           0                 0         -
  ito-backend/src/state.rs                                  132                14    89.39%           8                 1    87.50%          81                 8    90.12%           0                 0         -
  ito-cli/src/app/archive.rs                                661               146    77.91%          23                 4    82.61%         429                84    80.42%           0                 0         -
  ito-cli/src/app/common.rs                                 205                16    92.20%          11                 0   100.00%         131                 8    93.89%           0                 0         -
  ito-cli/src/app/entrypoint.rs                              23                 0   100.00%           1                 0   100.00%          16                 0   100.00%           0                 0         -
  ito-cli/src/app/grep.rs                                   210                95    54.76%           4                 0   100.00%         116                48    58.62%           0                 0         -
  ito-cli/src/app/init.rs                                   739               194    73.75%          43                14    67.44%         475               143    69.89%           0                 0         -
  ito-cli/src/app/instructions.rs                          1381               361    73.86%          77                30    61.04%         813               174    78.60%           0                 0         -
  ito-cli/src/app/list.rs                                   569                88    84.53%          29                 7    75.86%         350                62    82.29%           0                 0         -
  ito-cli/src/app/manifesto_instructions.rs                 991               191    80.73%          42                10    76.19%         627                98    84.37%           0                 0         -
  ito-cli/src/app/memory_instructions.rs                    308                74    75.97%          18                 6    66.67%         220                58    73.64%           0                 0         -
  ito-cli/src/app/run.rs                                    402                39    90.30%          35                 3    91.43%         283                31    89.05%           0                 0         -
  ito-cli/src/app/show.rs                                   541               158    70.79%          27                 8    70.37%         304                92    69.74%           0                 0         -
  ito-cli/src/app/status.rs                                 171                45    73.68%           5                 1    80.00%          92                39    57.61%           0                 0         -
  ito-cli/src/app/trace.rs                                  109                22    79.82%           1                 0   100.00%          80                17    78.75%           0                 0         -
  ito-cli/src/app/update.rs                                 180                51    71.67%          10                 3    70.00%         117                32    72.65%           0                 0         -
  ito-cli/src/app/validate.rs                               728               150    79.40%          23                 7    69.57%         469               108    76.97%           0                 0         -
  ito-cli/src/app/validate_repo.rs                          236                86    63.56%          19                11    42.11%         147                49    66.67%           0                 0         -
  ito-cli/src/app/worktree_wizard.rs                        203                73    64.04%          15                 5    66.67%         179                79    55.87%           0                 0         -
  ito-cli/src/cli.rs                                         17                 0   100.00%           1                 0   100.00%           7                 0   100.00%           0                 0         -
  ito-cli/src/cli/agent.rs                                  145                16    88.97%           1                 0   100.00%          70                 4    94.29%           0                 0         -
  ito-cli/src/cli/ralph.rs                                    8                 0   100.00%           1                 0   100.00%           8                 0   100.00%           0                 0         -
  ito-cli/src/cli_error.rs                                   35                 0   100.00%          10                 0   100.00%          46                 0   100.00%           0                 0         -
  ito-cli/src/commands/artifacts.rs                         119                20    83.19%           8                 1    87.50%          89                 7    92.13%           0                 0         -
  ito-cli/src/commands/audit.rs                             376                70    81.38%           9                 5    44.44%         223                47    78.92%           0                 0         -
  ito-cli/src/commands/backend.rs                           376                64    82.98%          26                 5    80.77%         286                46    83.92%           0                 0         -
  ito-cli/src/commands/completions.rs                        15                 0   100.00%           1                 0   100.00%          10                 0   100.00%           0                 0         -
  ito-cli/src/commands/config.rs                            407                55    86.49%          16                 4    75.00%         168                22    86.90%           0                 0         -
  ito-cli/src/commands/create.rs                            777               155    80.05%          25                 3    88.00%         434                75    82.72%           0                 0         -
  ito-cli/src/commands/help.rs                              181                29    83.98%          11                 1    90.91%         119                14    88.24%           0                 0         -
  ito-cli/src/commands/path.rs                              194                16    91.75%          10                 0   100.00%          95                 6    93.68%           0                 0         -
  ito-cli/src/commands/plan.rs                               89                 8    91.01%           4                 1    75.00%          59                 7    88.14%           0                 0         -
  ito-cli/src/commands/ralph.rs                             744               122    83.60%          21                 7    66.67%         463                63    86.39%           0                 0         -
  ito-cli/src/commands/ralph/support.rs                    1015               303    70.15%          63                26    58.73%         710               171    75.92%           0                 0         -
  ito-cli/src/commands/serve.rs                              93                60    35.48%          13                 9    30.77%          65                41    36.92%           0                 0         -
  ito-cli/src/commands/serve_api.rs                         282                67    76.24%          22                 7    68.18%         176                33    81.25%           0                 0         -
  ito-cli/src/commands/stats.rs                              22                 3    86.36%           1                 0   100.00%          18                 2    88.89%           0                 0         -
  ito-cli/src/commands/sync.rs                               87                48    44.83%           4                 1    75.00%          62                37    40.32%           0                 0         -
  ito-cli/src/commands/tasks.rs                            1418               392    72.36%          38                 9    76.32%         882               265    69.95%           0                 0         -
  ito-cli/src/commands/tasks/backend.rs                     286               254    11.19%          12                10    16.67%         192               175     8.85%           0                 0         -
  ito-cli/src/commands/tasks/support.rs                     122                37    69.67%          10                 1    90.00%          87                22    74.71%           0                 0         -
  ito-cli/src/commands/templates.rs                          34                 4    88.24%           1                 0   100.00%          22                 2    90.91%           0                 0         -
  ito-cli/src/commands/util.rs                               79                79     0.00%           3                 3     0.00%          40                40     0.00%           0                 0         -
  ito-cli/src/commands/view.rs                              140                47    66.43%           7                 3    57.14%          75                27    64.00%           0                 0         -
  ito-cli/src/commands/worktree.rs                          226               137    39.38%          15                10    33.33%         117                68    41.88%           0                 0         -
  ito-cli/src/diagnostics.rs                                171                 2    98.83%          11                 0   100.00%         121                 2    98.35%           0                 0         -
  ito-cli/src/main.rs                                         3                 0   100.00%           1                 0   100.00%           3                 0   100.00%           0                 0         -
  ito-cli/src/runtime.rs                                    128                 6    95.31%          20                 2    90.00%          91                 3    96.70%           0                 0         -
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
  ito-config/src/config/mod.rs                             1036                62    94.02%          65                 7    89.23%         542                42    92.25%           0                 0         -
  ito-config/src/config/schema.rs                            62                12    80.65%           8                 4    50.00%          30                 8    73.33%           0                 0         -
  ito-config/src/config/types.rs                            271                54    80.07%          56                 8    85.71%         299                41    86.29%           0                 0         -
  ito-config/src/config/worktree_init_types.rs               23                 0   100.00%           3                 0   100.00%          12                 0   100.00%           0                 0         -
  ito-config/src/context.rs                                  94                 8    91.49%           5                 1    80.00%          48                 4    91.67%           0                 0         -
  ito-config/src/ito_dir/mod.rs                             246                20    91.87%          15                 1    93.33%         142                13    90.85%           0                 0         -
  ito-config/src/output/mod.rs                               88                 1    98.86%           8                 0   100.00%          71                 1    98.59%           0                 0         -
  ito-core/src/archive.rs                                   318                60    81.13%          20                 9    55.00%         184                28    84.78%           0                 0         -
  ito-core/src/artifact_mutations.rs                        558               257    53.94%          42                23    45.24%         411               177    56.93%           0                 0         -
  ito-core/src/audit/mirror.rs                             1147               251    78.12%          69                19    72.46%         800               188    76.50%           0                 0         -
  ito-core/src/audit/reader.rs                               57                 0   100.00%           5                 0   100.00%          39                 0   100.00%           0                 0         -
  ito-core/src/audit/reconcile.rs                           499                20    95.99%          19                 0   100.00%         274                17    93.80%           0                 0         -
  ito-core/src/audit/store.rs                               596                68    88.59%          39                 4    89.74%         352                41    88.35%           0                 0         -
  ito-core/src/audit/stream.rs                              347                35    89.91%          12                 1    91.67%         194                23    88.14%           0                 0         -
  ito-core/src/audit/validate.rs                            325                 4    98.77%          14                 1    92.86%         259                 1    99.61%           0                 0         -
  ito-core/src/audit/worktree.rs                            333                27    91.89%          18                 0   100.00%         259                22    91.51%           0                 0         -
  ito-core/src/audit/writer.rs                              334                18    94.61%          20                 2    90.00%         172                10    94.19%           0                 0         -
  ito-core/src/backend_auth.rs                              217                60    72.35%          23                 9    60.87%         132                25    81.06%           0                 0         -
  ito-core/src/backend_change_repository.rs                 626                63    89.94%          37                 3    91.89%         355                45    87.32%           0                 0         -
  ito-core/src/backend_client.rs                            432                 5    98.84%          36                 2    94.44%         334                 3    99.10%           0                 0         -
  ito-core/src/backend_coordination.rs                      383                28    92.69%          33                 5    84.85%         286                28    90.21%           0                 0         -
  ito-core/src/backend_health.rs                            183                93    49.18%           7                 3    57.14%         169                93    44.97%           0                 0         -
  ito-core/src/backend_http.rs                              891               337    62.18%          55                21    61.82%         613               221    63.95%           0                 0         -
  ito-core/src/backend_import.rs                            274                28    89.78%          18                 3    83.33%         230                12    94.78%           0                 0         -
  ito-core/src/backend_module_repository.rs                 105                13    87.62%           9                 0   100.00%          62                 9    85.48%           0                 0         -
  ito-core/src/backend_spec_repository.rs                    12                 0   100.00%           3                 0   100.00%           9                 0   100.00%           0                 0         -
  ito-core/src/backend_sync.rs                              848               107    87.38%          54                22    59.26%         421                32    92.40%           0                 0         -
  ito-core/src/backend_task_repository.rs                    91                 1    98.90%          10                 0   100.00%          51                 0   100.00%           0                 0         -
  ito-core/src/change_meta.rs                                45                 4    91.11%           5                 1    80.00%          37                 3    91.89%           0                 0         -
  ito-core/src/change_repository.rs                        1278               132    89.67%          84                12    85.71%         722                78    89.20%           0                 0         -
  ito-core/src/config.rs                                   1183                96    91.89%          64                 5    92.19%         748                94    87.43%           0                 0         -
  ito-core/src/coordination.rs                              739               435    41.14%          53                37    30.19%         504               321    36.31%           0                 0         -
  ito-core/src/coordination_worktree.rs                    1072               273    74.53%          69                28    59.42%         791               207    73.83%           0                 0         -
  ito-core/src/create/mod.rs                               1310               198    84.89%          78                15    80.77%         806               142    82.38%           0                 0         -
  ito-core/src/distribution.rs                              566                55    90.28%          39                 8    79.49%         366                55    84.97%           0                 0         -
  ito-core/src/error_bridge.rs                                4                 0   100.00%           1                 0   100.00%           3                 0   100.00%           0                 0         -
  ito-core/src/errors.rs                                     75                 7    90.67%           8                 0   100.00%          59                 7    88.14%           0                 0         -
  ito-core/src/event_forwarder.rs                           650                48    92.62%          42                 3    92.86%         400                28    93.00%           0                 0         -
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
  ito-core/src/installers/agents_cleanup.rs                 157                30    80.89%           9                 4    55.56%          83                14    83.13%           0                 0         -
  ito-core/src/installers/markers.rs                        217                11    94.93%          11                 0   100.00%         155                 9    94.19%           0                 0         -
  ito-core/src/installers/mod.rs                           1609               234    85.46%          80                24    70.00%         902               109    87.92%           0                 0         -
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
  ito-core/src/repo_paths.rs                                360                70    80.56%          33                10    69.70%         215                36    83.26%           0                 0         -
  ito-core/src/repository_runtime.rs                        442                61    86.20%          56                10    82.14%         338                51    84.91%           0                 0         -
  ito-core/src/show/mod.rs                                  687                53    92.29%          36                 4    88.89%         429                33    92.31%           0                 0         -
  ito-core/src/spec_repository.rs                            75                 5    93.33%           6                 0   100.00%          46                 3    93.48%           0                 0         -
  ito-core/src/sqlite_project_store.rs                      431               125    71.00%          47                29    38.30%         318                37    88.36%           0                 0         -
  ito-core/src/sqlite_project_store_backend.rs              254                61    75.98%          29                10    65.52%         221                45    79.64%           0                 0         -
  ito-core/src/sqlite_project_store_mutations.rs            225                85    62.22%          27                12    55.56%         180                76    57.78%           0                 0         -
  ito-core/src/sqlite_project_store_repositories.rs        1238               212    82.88%          67                19    71.64%         738               126    82.93%           0                 0         -
  ito-core/src/stats.rs                                      93                 8    91.40%           5                 0   100.00%          97                 7    92.78%           0                 0         -
  ito-core/src/task_mutations.rs                            181                52    71.27%          13                 1    92.31%         134                19    85.82%           0                 0         -
  ito-core/src/task_repository.rs                           281                 9    96.80%          16                 2    87.50%         165                 4    97.58%           0                 0         -
  ito-core/src/tasks.rs                                    1077               187    82.64%          81                25    69.14%         734               122    83.38%           0                 0         -
  ito-core/src/templates/guidance.rs                        134                 4    97.01%           7                 0   100.00%          94                 1    98.94%           0                 0         -
  ito-core/src/templates/mod.rs                             982               147    85.03%          64                10    84.38%         642                99    84.58%           0                 0         -
  ito-core/src/templates/review.rs                          279                75    73.12%          11                 1    90.91%         195                56    71.28%           0                 0         -
  ito-core/src/templates/schema_assets.rs                   244                47    80.74%          19                 5    73.68%         164                38    76.83%           0                 0         -
  ito-core/src/templates/task_parsing.rs                    218                25    88.53%           9                 2    77.78%         144                18    87.50%           0                 0         -
  ito-core/src/templates/types.rs                           127                 1    99.21%           8                 0   100.00%          93                 1    98.92%           0                 0         -
  ito-core/src/time.rs                                       10                 5    50.00%           2                 1    50.00%           6                 3    50.00%           0                 0         -
  ito-core/src/token.rs                                      56                 0   100.00%           7                 0   100.00%          31                 0   100.00%           0                 0         -
  ito-core/src/trace.rs                                      88                 5    94.32%           1                 0   100.00%          55                 4    92.73%           0                 0         -
  ito-core/src/validate/delta_rules.rs                     1263               137    89.15%          61                 1    98.36%         867                92    89.39%           0                 0         -
  ito-core/src/validate/issue.rs                            204                 2    99.02%          20                 0   100.00%         132                 2    98.48%           0                 0         -
  ito-core/src/validate/mod.rs                             1029               231    77.55%          31                 1    96.77%         763               180    76.41%           0                 0         -
  ito-core/src/validate/repo_integrity.rs                   190                25    86.84%          10                 3    70.00%          95                10    89.47%           0                 0         -
  ito-core/src/validate/report.rs                            97                 0   100.00%           9                 0   100.00%          60                 0   100.00%           0                 0         -
  ito-core/src/validate/rules_engine.rs                     138                37    73.19%          10                 0   100.00%         134                20    85.07%           0                 0         -
  ito-core/src/validate/tracking_rules.rs                   205                 7    96.59%           9                 0   100.00%         171                 6    96.49%           0                 0         -
  ito-core/src/validate_repo/audit_rules.rs                 366                 9    97.54%          28                 2    92.86%         241                17    92.95%           0                 0         -
  ito-core/src/validate_repo/backend_rules.rs               778                17    97.81%          52                 2    96.15%         434                16    96.31%           0                 0         -
  ito-core/src/validate_repo/coordination_rules.rs          737                21    97.15%          49                 3    93.88%         411                27    93.43%           0                 0         -
  ito-core/src/validate_repo/mod.rs                          99                16    83.84%           4                 0   100.00%          51                 5    90.20%           0                 0         -
  ito-core/src/validate_repo/pre_commit_detect.rs           364                12    96.70%          33                 1    96.97%         230                 6    97.39%           0                 0         -
  ito-core/src/validate_repo/registry.rs                    372                16    95.70%          40                 3    92.50%         310                15    95.16%           0                 0         -
  ito-core/src/validate_repo/repository_rules.rs            650                33    94.92%          39                 3    92.31%         359                26    92.76%           0                 0         -
  ito-core/src/validate_repo/rule.rs                         73                 8    89.04%          12                 2    83.33%          62                 6    90.32%           0                 0         -
  ito-core/src/validate_repo/staged.rs                      279                 5    98.21%          26                 1    96.15%         182                 7    96.15%           0                 0         -
  ito-core/src/validate_repo/worktrees_rules.rs             520                23    95.58%          36                 3    91.67%         293                21    92.83%           0                 0         -
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
  ito-core/src/worktree_validate.rs                         140                 4    97.14%          14                 0   100.00%         128                 3    97.66%           0                 0         -
  ito-domain/src/audit/context.rs                           207                13    93.72%          15                 1    93.33%         113                 4    96.46%           0                 0         -
  ito-domain/src/audit/event.rs                             475                12    97.47%          36                 2    94.44%         349                 6    98.28%           0                 0         -
  ito-domain/src/audit/materialize.rs                       324                 1    99.69%          11                 0   100.00%         233                 0   100.00%           0                 0         -
  ito-domain/src/audit/reconcile.rs                         492                35    92.89%          17                 0   100.00%         249                17    93.17%           0                 0         -
  ito-domain/src/audit/writer.rs                             60                 0   100.00%           8                 0   100.00%          45                 0   100.00%           0                 0         -
  ito-domain/src/backend.rs                                 178                 0   100.00%          11                 0   100.00%         117                 0   100.00%           0                 0         -
  ito-domain/src/changes/mod.rs                             381                21    94.49%          26                 1    96.15%         264                15    94.32%           0                 0         -
  ito-domain/src/changes/mutations.rs                        29                17    41.38%           6                 4    33.33%          25                17    32.00%           0                 0         -
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
  ito-domain/src/tasks/compute.rs                           519                18    96.53%          33                 0   100.00%         328                12    96.34%           0                 0         -
  ito-domain/src/tasks/cycle.rs                              61                 8    86.89%           3                 0   100.00%          42                 0   100.00%           0                 0         -
  ito-domain/src/tasks/mutations.rs                          13                 4    69.23%           4                 1    75.00%          15                 6    60.00%           0                 0         -
  ito-domain/src/tasks/parse.rs                             931                36    96.13%          50                 2    96.00%         653                36    94.49%           0                 0         -
  ito-domain/src/tasks/relational.rs                        282                70    75.18%           2                 0   100.00%         258                73    71.71%           0                 0         -
  ito-domain/src/tasks/repository.rs                         32                11    65.62%           4                 1    75.00%          16                 4    75.00%           0                 0         -
  ito-domain/src/tasks/update.rs                            238                 6    97.48%           3                 0   100.00%         140                 6    95.71%           0                 0         -
  ito-domain/src/traceability.rs                            136                 0   100.00%           1                 0   100.00%         107                 0   100.00%           0                 0         -
  ito-logging/src/lib.rs                                    431                43    90.02%          22                 2    90.91%         293                30    89.76%           0                 0         -
  ito-templates/src/agents.rs                               251                 9    96.41%          17                 1    94.12%         224                 7    96.88%           0                 0         -
  ito-templates/src/instructions.rs                         103                 6    94.17%          10                 0   100.00%          59                 1    98.31%           0                 0         -
  ito-templates/src/lib.rs                                 1307                54    95.87%          98                 2    97.96%         680                36    94.71%           0                 0         -
  ito-templates/src/project_templates.rs                    309                 0   100.00%          19                 0   100.00%         195                 0   100.00%           0                 0         -
  ito-test-support/src/lib.rs                               327                86    73.70%          19                 3    84.21%         182                53    70.88%           0                 0         -
  ito-test-support/src/mock_repos.rs                        304                59    80.59%          35                 6    82.86%         288                57    80.21%           0                 0         -
  ito-test-support/src/pty/mod.rs                           137                 1    99.27%           5                 0   100.00%          81                 1    98.77%           0                 0         -
  -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  TOTAL                                                   72792             11816    83.77%        4552               910    80.01%       47051              7929    83.15%           0                 0         -
  info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
     Compiling ito-common v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-common)
     Compiling ito-templates v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-templates)
     Compiling ito-cli v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-cli)
     Compiling ito-logging v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-logging)
     Compiling ito-domain v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-domain)
     Compiling ito-config v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-config)
     Compiling ito-test-support v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-test-support)
     Compiling ito-core v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-core)
     Compiling ito-backend v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-backend)
     Compiling ito-web v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/ito-rs/crates/ito-web)
      Finished `test` profile [optimized + debuginfo] target(s) in 34.84s
       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_backend-4bc40e9b8855df78)

  running 27 tests
  test auth::tests::extract_org_repo_valid_path ... ok
  test auth::tests::extract_org_repo_non_project_path ... ok
  test auth::tests::exempt_paths_are_health_and_ready ... ok
  test auth::tests::token_scope_serializes_admin ... ok
  test auth::tests::extract_org_repo_no_trailing ... ok
  test auth::tests::token_scope_serializes_project ... ok
  test auth::tests::validate_token_admin_matches ... ok
  test auth::tests::derive_project_token_is_deterministic ... ok
  test auth::tests::derive_project_token_differs_by_project ... ok
  test auth::tests::validate_token_invalid_fails ... ok
  test auth::tests::derive_project_token_is_64_hex_chars ... ok
  test auth::tests::validate_token_project_matches ... ok
  test auth::tests::validate_token_wrong_project_fails ... ok
  test error::tests::api_error_serializes_to_json_with_error_and_code ... ok
  test error::tests::bad_request_response_has_400_status ... ok
  test error::tests::core_not_found_maps_to_404 ... ok
  test error::tests::forbidden_response_has_403_status ... ok
  test error::tests::core_validation_maps_to_400 ... ok
  test error::tests::internal_response_has_500_status ... ok
  test auth::tests::derive_project_token_differs_by_seed ... ok
  test error::tests::not_found_response_has_404_status ... ok
  test error::tests::service_unavailable_response_has_503_status ... ok
  test error::tests::unauthorized_response_has_401_status ... ok
  test error::tests::into_response_produces_json_content_type ... ok
  test state::tests::ito_path_for_rejects_path_traversal ... ok
  test state::tests::ito_path_for_resolves_to_expected_path ... ok
  test state::tests::ensure_project_dir_creates_directories ... ok

  test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/archive_sync.rs (target/llvm-cov-target/debug/deps/archive_sync-20f9b13f9c0bc874)

  running 3 tests
  test sync_pull_returns_artifact_bundle ... ok
  test sync_push_updates_backend_artifacts ... ok
  test archive_endpoint_promotes_specs_and_moves_change ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/bootstrap_endpoints.rs (target/llvm-cov-target/debug/deps/bootstrap_endpoints-9554aaae7d198a06)

  running 9 tests
  test ready_endpoint_returns_ready_when_data_dir_exists ... ok
  test project_route_rejects_non_allowlisted_org ... ok
  test health_endpoint_returns_status_and_version ... ok
  test project_route_accepts_derived_project_token ... ok
  test project_route_rejects_invalid_token ... ok
  test project_route_accepts_admin_token ... ok
  test project_route_rejects_missing_token ... ok
  test ready_endpoint_does_not_require_auth ... ok
  test health_endpoint_does_not_require_auth ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

       Running tests/event_ingest.rs (target/llvm-cov-target/debug/deps/event_ingest-308bffa9f8306832)

  running 6 tests
  test ingest_requires_authentication ... ok
  test ingest_missing_idempotency_key_rejected ... ok
  test ingest_empty_batch_accepted ... ok
  test ingest_accepts_event_batch ... ok
  test list_events_returns_backend_managed_audit_log ... ok
  test ingest_idempotent_retry_returns_duplicates ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

       Running tests/multi_tenant.rs (target/llvm-cov-target/debug/deps/multi_tenant-d93e1fbd5c7170e2)

  running 13 tests
  test non_allowlisted_repo_in_allowed_org_is_rejected ... ok
  test get_single_module_returns_detail ... ok
  test derived_token_for_project_a_cannot_access_project_b ... ok
  test get_nonexistent_change_returns_404 ... ok
  test get_nonexistent_module_returns_404 ... ok
  test derived_token_for_project_b_cannot_access_project_a ... ok
  test modules_are_isolated_between_projects ... ok
  test admin_token_lists_changes_for_project_a ... ok
  test derived_token_for_project_a_accesses_project_a ... ok
  test get_change_tasks_returns_task_list ... ok
  test admin_token_lists_changes_for_project_b ... ok
  test get_single_change_returns_detail ... ok
  test events_are_isolated_between_projects ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running tests/specs.rs (target/llvm-cov-target/debug/deps/specs-d5d0fadf2e2b9495)

  running 2 tests
  test list_specs_returns_promoted_specs ... ok
  test get_spec_returns_markdown ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

       Running tests/task_mutations.rs (target/llvm-cov-target/debug/deps/task_mutations-1098f3633c98f879)

  running 5 tests
  test start_task_endpoint_reports_missing_tasks_as_not_found ... ok
  test tasks_markdown_endpoint_returns_none_for_missing_artifact ... ok
  test start_task_endpoint_updates_remote_tasks ... ok
  test shelve_task_endpoint_accepts_reason_payload ... ok
  test complete_task_endpoint_accepts_note_payload ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

       Running unittests src/main.rs (target/llvm-cov-target/debug/deps/ito-07e10bd2c950a00f)

  running 73 tests
  test app::archive::tests::only_filesystem_mode_requires_local_changes_dir ... ok
  test app::archive::tests::archive_follow_up_messages_cover_all_modes ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_none_input ... ok
  test app::instructions::tests::json_get_traverses_nested_keys ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_mixed_levels ... ok
  test app::instructions::tests::json_get_returns_none_for_non_object_intermediate ... ok
  test app::instructions::tests::collect_tracking_diagnostic_counts_empty_slice ... ok
  test app::init::repo_validation_advisory_tests::returns_false_for_empty_project ... ok
  test app::instructions::tests::json_get_returns_none_for_missing_key ... ok
  test app::instructions::tests::worktree_config_checkout_siblings_sets_project_root ... ok
  test app::instructions::tests::worktree_config_checkout_subdir_sets_project_root ... ok
  test app::instructions::tests::json_get_empty_keys_returns_root ... ok
  test app::instructions::tests::worktree_config_no_project_root_when_none_passed ... ok
  test app::instructions::tests::worktree_config_ignores_empty_strings ... ok
  test app::instructions::tests::worktree_config_defaults_when_no_worktrees_key ... ok
  test app::instructions::tests::collect_context_files_preserves_order ... ok
  test app::instructions::tests::backend_instruction_is_cli_first_for_remote_mode ... ok
  test app::init::repo_validation_advisory_tests::detects_lefthook_config ... ok
  test app::init::repo_validation_advisory_tests::ignores_unrelated_pre_commit_yaml ... ok
  test app::instructions::tests::worktree_config_parses_all_fields ... ok
  test app::init::repo_validation_advisory_tests::detects_pre_commit_yaml_with_hook_id ... ok
  test app::instructions::tests::worktree_config_parses_bare_control_siblings_strategy ... ok
  test app::list::tests::parse_sort_order_supports_separate_and_equals_forms ... ok
  test app::run::tests::removed_serve_api_replacement_preserves_flags_and_args ... ok
  test app::list::tests::format_task_status_handles_various_states ... ok
  test app::list::tests::format_relative_time_covers_major_buckets ... ok
  test app::worktree_wizard::worktree_wizard_tests::load_worktree_result_from_config_returns_expected_defaults_and_values ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_errors_when_enabled_missing_fields ... ok
  test app::worktree_wizard::worktree_wizard_tests::is_worktree_configured_detects_strategy_key ... ok
  test commands::backend::tests::resolve_project_root_rejects_parentless_paths ... ok
  test commands::artifacts::tests::artifact_kind_from_selector_maps_expected_variants ... ok
  test app::init::repo_validation_advisory_tests::detects_husky_pre_commit_script_with_command_line ... ok
  test cli::ralph::ralph_tests::harness_arg_converts_to_core_harness_name ... ok
  test commands::backend::tests::resolve_project_root_returns_parent_directory ... ok
  test commands::config::config_tests::json_render_value_renders_common_json_types ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_empty_ip ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_non_zero_exit ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_when_command_missing ... ok
  test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_success ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_missing ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_path_is_file ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_disabled_and_preserves_other_keys ... ok
  test cli::cli_tests::parses_worktree_validate_with_json_flag ... ok
  test cli::cli_tests::parses_top_level_sync_command ... ok
  test cli::cli_tests::parses_top_level_sync_force_flag ... ok
  test commands::serve_api::serve_api_tests::builds_allowlist_from_allow_org_args ... ok
  test commands::serve_api::serve_api_tests::builds_config_with_defaults ... ok
  test commands::serve::serve_tests::ensure_ito_dir_exists_ok_when_present ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_trailing_json_content ... ok
  test commands::config::config_tests::handle_config_schema_writes_file_when_output_is_set ... ok
  test commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default ... ok
  test commands::config::config_tests::config_schema_includes_coordination_sync_interval_default ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_accepts_full_ito_json_config ... ok
  test commands::serve_api::serve_api_tests::merge_allow_orgs_preserves_existing_repo_rules ... ok
  test diagnostics::tests::blocking_task_error_message_includes_rendered_errors ... ok
  test diagnostics::tests::blocking_task_error_message_returns_none_when_no_errors ... ok
  test diagnostics::tests::format_path_line_includes_optional_line_number ... ok
  test diagnostics::tests::render_task_diagnostics_filters_by_level_and_renders_task_id_when_present ... ok
  test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_enabled_settings ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_reads_toml ... ok
  test diagnostics::tests::render_validation_issues_renders_level_path_and_message ... ok
  test diagnostics::tests::render_validation_issues_renders_rule_id_when_present ... ok
  test util::tests::command_id_maps_gr_to_grep ... ok
  test util::tests::command_id_maps_x_templates_to_templates ... ok
  test util::tests::command_id_uses_positional_args_and_normalizes_hyphens ... ok
  test util::tests::sanitize_args_redacts_equals_form ... ok
  test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_unknown_json_fields ... ok
  test util::tests::sanitize_args_replaces_paths ... ok
  test util::tests::sanitize_args_redacts_sensitive_flags ... ok
  test util::tests::split_csv_trims_parts ... ok
  test app::worktree_wizard::worktree_wizard_tests::save_worktree_config_writes_config_and_runs_print_paths ... ok
  test app::list::tests::progress_filter_flags_are_mutually_exclusive ... ok
  test app::instructions::tests::worktree_config_bare_control_siblings_calls_resolve ... ok

  test result: ok. 73 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/agent_instruction_bootstrap.rs (target/llvm-cov-target/debug/deps/agent_instruction_bootstrap-a342076b6a28ee91)

  running 9 tests
  test bootstrap_opencode_success ... ok
  test bootstrap_claude_success ... ok
  test bootstrap_json_output ... ok
  test bootstrap_github_copilot_success ... ok
  test bootstrap_requires_tool_flag ... ok
  test bootstrap_output_is_short ... ok
  test bootstrap_rejects_invalid_tool ... ok
  test bootstrap_codex_success ... ok
  test bootstrap_contains_artifact_pointers ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s

       Running tests/agent_instruction_context.rs (target/llvm-cov-target/debug/deps/agent_instruction_context-8e35937e69c351e8)

  running 2 tests
  Switched to a new branch '023-07_harness-context-inference'
  test agent_instruction_context_prefers_path_inference_in_text_output ... ok
  test agent_instruction_context_supports_json_output ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running tests/agent_instruction_memory.rs (target/llvm-cov-target/debug/deps/agent_instruction_memory-4b6a2916f86d60b7)

  running 14 tests
  test memory_query_skill_branch_emits_structured_inputs ... ok
  test memory_search_not_configured_branch_renders_setup_guidance ... ok
  test agent_instruction_help_lists_memory_artifacts ... ok
  test memory_capture_skill_branch_emits_structured_inputs ... ok
  test memory_query_not_configured_branch_renders_setup_guidance ... ok
  test memory_capture_renders_skill_when_only_capture_configured ... ok
  test memory_query_command_branch_substitutes_query ... ok
  test memory_capture_not_configured_branch_renders_setup_guidance ... ok
  test memory_search_skill_branch_emits_structured_inputs ... ok
  test memory_capture_command_branch_renders_executable_command_line ... ok
  test memory_query_renders_not_configured_when_only_capture_set ... ok
  test memory_search_command_branch_substitutes_query_and_default_limit ... ok
  test memory_search_command_branch_overrides_limit_when_supplied ... ok
  test memory_search_requires_query_flag ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.84s

       Running tests/agent_instruction_orchestrate.rs (target/llvm-cov-target/debug/deps/agent_instruction_orchestrate-3b510431e779095b)

  running 5 tests
  test orchestrate_json_output_has_correct_artifact_id ... ok
  test orchestrate_requires_orchestrate_md ... ok
  test orchestrate_succeeds_when_orchestrate_md_exists ... ok
  test orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter ... ok
  test orchestrate_surfaces_recommended_skills_from_preset ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/agent_instruction_repo_sweep.rs (target/llvm-cov-target/debug/deps/agent_instruction_repo_sweep-22b49a4bc7af4e95)

  running 3 tests
  test repo_sweep_json_output_has_correct_artifact_id ... ok
  test repo_sweep_output_contains_key_phrases ... ok
  test repo_sweep_succeeds_without_change_flag ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/agent_instruction_worktrees.rs (target/llvm-cov-target/debug/deps/agent_instruction_worktrees-b48c58f55b0fbb9e)

  running 2 tests
  test worktrees_instruction_json_output ... ok
  test worktrees_instruction_does_not_require_change ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/aliases.rs (target/llvm-cov-target/debug/deps/aliases-4ec903075d2119aa)

  running 4 tests
  test subcommand_aliases_work ... ok
  test main_command_aliases_work ... ok
  test main_command_aliases_execute ... ok
  test short_flags_work ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/archive_completed.rs (target/llvm-cov-target/debug/deps/archive_completed-5ae78ceeca73fd3a)

  running 7 tests
  test archive_completed_conflict_with_positional ... ok
  test archive_completed_no_completed_changes ... ok
  test archive_completed_decline_confirmation_cancels ... ok
  test archive_completed_empty_confirmation_cancels ... ok
  test archive_completed_accept_yes_confirmation_archives ... ok
  test archive_completed_archives_all_completed ... ok
  test archive_completed_skip_specs ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s

       Running tests/archive_remote_mode.rs (target/llvm-cov-target/debug/deps/archive_remote_mode-f3d21c6bd01bd03f)

  running 1 test
  test remote_archive_succeeds_without_local_active_change_markdown ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s

       Running tests/archive_smoke.rs (target/llvm-cov-target/debug/deps/archive_smoke-2f27a2594891f8ed)

  running 1 test
  test archive_with_specs_and_validation_smoke ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/artifact_mutations.rs (target/llvm-cov-target/debug/deps/artifact_mutations-1ed3d06b4107626b)

  running 3 tests
  test write_change_proposal_replaces_contents ... ok
  test patch_change_proposal_applies_unified_diff ... ok
  test write_change_spec_delta_creates_missing_capability_file ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/audit_more.rs (target/llvm-cov-target/debug/deps/audit_more-34d3ebf94d638d15)

  running 6 tests
  test audit_more_local_audit_writes_warn_and_fallback_without_worktree_log_when_branch_storage_is_unavailable ... ok
  test audit_log_stats_and_validate_json_outputs_are_well_formed ... ok
  test audit_more_local_audit_writes_use_internal_branch_without_worktree_log_churn ... ok
  test audit_subcommands_cover_text_output_limit_reconcile_and_stream ... ok
  test audit_stream_all_worktrees_dedupes_shared_routed_storage ... ok
  test audit_commands_migrate_legacy_worktree_log_into_routed_storage ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.58s

       Running tests/audit_remote_mode.rs (target/llvm-cov-target/debug/deps/audit_remote_mode-b268702bf9831693)

  running 1 test
  test audit_commands_in_backend_mode_use_server_only_storage ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s

       Running tests/backend_import.rs (target/llvm-cov-target/debug/deps/backend_import-d6d93f13f375d09d)

  running 4 tests
  test backend_import_rejects_local_mode ... ok
  test backend_import_dry_run_reports_scope_without_writing_backend ... ok
  test backend_import_writes_active_and_archived_changes_to_backend ... ok
  test backend_import_is_idempotent_and_remote_reads_match_imported_changes ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.41s

       Running tests/backend_qa_walkthrough.rs (target/llvm-cov-target/debug/deps/backend_qa_walkthrough-3de9a4415b0e074f)

  running 1 test
  test backend_qa_script_verify_runs_end_to_end ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.48s

       Running tests/backend_serve.rs (target/llvm-cov-target/debug/deps/backend_serve-91aa8231b4e46cdc)

  running 5 tests
  test backend_serve_init_prints_backend_command_guidance ... ok
  test backend_serve_reports_unknown_fields_in_explicit_config_file ... ok
  test backend_serve_service_mode_reports_malformed_backend_config ... ok
  test backend_serve_service_mode_bootstraps_missing_auth_silently ... ok
  test backend_serve_service_mode_reuses_existing_auth_without_printing_init_output ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

       Running tests/backend_status_more.rs (target/llvm-cov-target/debug/deps/backend_status_more-285514561483a919)

  running 20 tests
  test backend_status_unreachable_server_fails ... ok
  test generate_token_missing_repo_fails ... ok
  test generate_token_seed_from_env_takes_precedence ... ok
  test generate_token_flag_overrides_for_org_repo ... ok
  test backend_status_token_security_warning ... ok
  test generate_token_with_all_sources_prefers_env ... ok
  test generate_token_missing_org_fails ... ok
  test generate_token_derives_deterministic_token ... ok
  test backend_status_unreachable_server_json_output ... ok
  test backend_status_incomplete_config_fails ... ok
  test backend_status_with_env_token_no_warning ... ok
  test generate_token_no_seed_fails ... ok
  test backend_status_with_valid_config_but_no_server ... ok
  test backend_status_json_includes_config_details ... ok
  test backend_status_disabled_json_output ... ok
  test backend_status_disabled_shows_informational_output ... ok
  test silent_fallback_grep_warns_on_bad_config ... ok
  test silent_fallback_with_valid_backend_no_warnings ... ok
  test silent_fallback_tasks_warns_on_bad_config ... ok
  test silent_fallback_event_forwarding_warns_on_bad_config ... ok

  test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.25s

       Running tests/cli_smoke.rs (target/llvm-cov-target/debug/deps/cli_smoke-613d5d8b47289f8b)

  running 6 tests
  test cli_help_hides_top_level_serve_api_entrypoint ... ok
  test cli_top_level_serve_api_help_shows_backend_migration_guidance ... ok
  test cli_top_level_serve_api_shows_backend_migration_guidance ... ok
  test agent_instruction_status_archive_smoke ... ok
  test list_show_validate_smoke ... ok
  test create_workflow_plan_state_config_smoke ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s

       Running tests/cli_snapshots.rs (target/llvm-cov-target/debug/deps/cli_snapshots-6bbcb2e50ba46499)

  running 14 tests
  test snapshot_agent_help ... ok
  test snapshot_version ... ok
  test snapshot_backend_help ... ok
  test snapshot_ralph_help ... ok
  test snapshot_agent_instruction_help ... ok
  test snapshot_tasks_help ... ok
  test snapshot_create_help ... ok
  test snapshot_help ... ok
  test snapshot_backend_serve_help ... ok
  test snapshot_init_help ... ok
  test snapshot_validate_help ... ok
  test snapshot_list_help ... ok
  test snapshot_help_all_subcommand ... ok
  test snapshot_help_all_global_flag ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/config_more.rs (target/llvm-cov-target/debug/deps/config_more-da12828e0fa0a429)

  running 5 tests
  test config_set_rejects_invalid_audit_mirror_branch_name ... ok
  test config_set_rejects_invalid_coordination_branch_name ... ok
  test config_unknown_subcommand_errors ... ok
  test config_help_path_list_unset_and_schema_smoke ... ok
  test config_set_get_supports_coordination_and_audit_mirror_keys ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.64s

       Running tests/coverage_smoke.rs (target/llvm-cov-target/debug/deps/coverage_smoke-fc42662bd6144115)

  running 3 tests
  test serve_errors_when_no_ito_dir_exists ... ok
  test completions_command_runs_for_all_shells ... ok
  test audit_validate_and_log_work_with_empty_event_log ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s

       Running tests/create_more.rs (target/llvm-cov-target/debug/deps/create_more-fd182cc804dee111)

  running 4 tests
  test create_change_sub_module_and_module_are_mutually_exclusive ... ok
  test create_change_sub_module_rejects_remote_persistence_mode ... ok
  test create_change_with_sub_module_flag_creates_composite_id_change ... ok
  test create_module_and_change_error_paths_and_outputs ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.60s

       Running tests/grep_more.rs (target/llvm-cov-target/debug/deps/grep_more-b77e74aed643e3bb)

  running 5 tests
  test grep_module_scope_searches_all_changes_in_module ... ok
  test grep_change_scope_rejects_too_many_positional_args ... ok
  test grep_change_scope_prints_matches_with_locations ... ok
  test grep_limit_caps_output_and_prints_warning ... ok
  test grep_all_scope_searches_all_changes ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

       Running tests/help.rs (target/llvm-cov-target/debug/deps/help-4ba76f593f1f21ec)

  running 7 tests
  test agent_instruction_help_shows_instruction_details ... ok
  test help_shows_navigation_footer ... ok
  test help_prints_usage ... ok
  test help_all_global_flag_works ... ok
  test dash_h_help_matches_dash_dash_help ... ok
  test help_all_shows_complete_reference ... ok
  test help_all_json_outputs_valid_json ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/init_coordination.rs (target/llvm-cov-target/debug/deps/init_coordination-596ee5831276052c)

  running 4 tests
  test init_no_coordination_worktree_writes_embedded_storage ... ok
  test init_without_git_remote_falls_back_gracefully ... ok
  test init_upgrade_does_not_touch_coordination_storage ... ok
  test init_with_git_remote_creates_coordination_worktree ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s

       Running tests/init_gitignore_session_json.rs (target/llvm-cov-target/debug/deps/init_gitignore_session_json-ed3217d94ed9de73)

  running 1 test
  test init_writes_gitignore_session_json_and_is_idempotent ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/init_more.rs (target/llvm-cov-target/debug/deps/init_more-bfd20a38e62c6e5f)

  running 32 tests
  test init_interactive_detects_tools_and_installs_adapter_files ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
  test init_help_prints_usage ... ok
  test init_requires_tools_when_non_interactive ... ok
  test init_refuses_to_overwrite_existing_file_without_markers_when_not_forced ... ok
  test init_prints_project_setup_nudge_when_marker_incomplete ... ok
  test init_opencode_installs_audit_hook_plugin ... ok
  test init_github_copilot_installs_audit_preflight_assets ... ok
  test init_renders_agents_md_without_raw_jinja2_syntax ... ok
  test init_renders_skill_files_without_raw_jinja2_syntax ... ok
  test init_codex_installs_audit_instruction_assets ... ok
  test init_tools_csv_ignores_empty_segments ... ok
  test init_update_preserves_existing_markerless_opencode_agent_template_body ... ok
  test init_does_not_print_project_setup_nudge_when_marker_absent ... ok
  test init_force_overwrites_existing_user_prompt_stubs ... ok
  test init_does_not_print_project_setup_nudge_when_marker_complete ... ok
  test init_update_preserves_existing_partial_marker_opencode_agent_template_body ... ok
  test init_update_refreshes_existing_opencode_orchestrator_agent_template ... ok
  test init_update_without_prior_init_creates_all_files ... ok
  test init_update_with_tools_all_installs_all_orchestrator_agent_templates ... ok
  test init_tools_parser_covers_all_and_invalid_id ... ok
  test init_update_does_not_overwrite_existing_user_prompt_stubs ... ok
  test init_update_preserves_user_files_and_creates_missing ... ok
  test init_writes_config_with_release_tag_schema_reference ... ok
  test init_update_renders_agents_md_without_raw_jinja2 ... ok
  test init_with_tools_none_installs_ito_skeleton ... ok
  test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
  test init_with_tools_opencode_installs_orchestrator_agent_template ... ok
  test init_with_tools_csv_installs_selected_adapters ... ok
  test init_setup_coordination_branch_fails_without_origin_remote ... ok
  test init_setup_coordination_branch_reports_ready_when_already_present ... ok
  test init_setup_coordination_branch_creates_branch_on_origin ... ok
  test init_setup_coordination_branch_uses_configured_branch_name ... ok

  test result: ok. 31 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.92s

       Running tests/init_obsolete_cleanup.rs (target/llvm-cov-target/debug/deps/init_obsolete_cleanup-bb360471500cedd5)

  running 2 tests
  test init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok
  test init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/init_tmux.rs (target/llvm-cov-target/debug/deps/init_tmux-f4490dca8294cd74)

  running 5 tests
  test init_interactive_can_disable_tmux_preference ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
  test init_writes_tmux_enabled_true_by_default ... ok
  test init_update_preserves_existing_tmux_preference ... ok
  test init_uses_cascading_tmux_preference_from_global_config ... ok
  test init_with_no_tmux_writes_tmux_enabled_false ... ok

  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.11s

       Running tests/init_upgrade_more.rs (target/llvm-cov-target/debug/deps/init_upgrade_more-0a0099e66629ac7b)

  running 5 tests
  test init_upgrade_skips_and_warns_when_markers_missing ... ok
  test init_upgrade_flag_is_accepted ... ok
  test init_update_does_not_error_on_existing_agents_md_without_markers ... ok
  test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
  test init_update_preserves_user_owned_files ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

       Running tests/instructions_more.rs (target/llvm-cov-target/debug/deps/instructions_more-63ae78be8f3eb2a2)

  running 31 tests
  test agent_instruction_archive_without_change_prints_generic_guidance ... ok
  test agent_instruction_finish_with_change_prompts_for_archive ... ok
  test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
  test agent_instruction_change_flag_supports_shorthand ... ok
  test agent_instruction_change_flag_reports_ambiguous_target ... ok
  test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
  test agent_instruction_change_flag_supports_slug_query ... ok
  test agent_instruction_archive_with_invalid_change_fails ... ok
  test agent_instruction_manifesto_full_variant_rejects_incompatible_operation ... ok
  test agent_instruction_manifesto_full_variant_embeds_requested_proposal_instruction ... ok
  test agent_instruction_manifesto_change_scope_includes_change_state ... ok
  test agent_instruction_manifesto_full_variant_embeds_allowed_default_set ... ok
  test agent_instruction_manifesto_change_scope_json_reports_state ... ok
  test agent_instruction_manifesto_change_scope_reports_apply_ready_state ... ok
  test agent_instruction_manifesto_change_scope_reports_applying_state ... ok
  test agent_instruction_manifesto_full_variant_renders_full_section ... ok
  test agent_instruction_manifesto_rejects_operation_for_light_variant ... ok
  test agent_instruction_review_requires_change_flag ... ok
  test agent_instruction_manifesto_json_includes_resolved_defaults ... ok
  test agent_instruction_manifesto_full_variant_supports_finish_for_archived_change ... ok
  test agent_instruction_manifesto_memory_config_embeds_operation_instructions ... ok
  test agent_instruction_manifesto_redacts_explicit_coordination_path ... ok
  test agent_instruction_text_output_renders_artifact_envelope ... ok
  test agent_instruction_manifesto_rejects_operation_without_change ... ok
  test agent_instruction_proposal_without_change_supports_json_output ... ok
  test agent_instruction_manifesto_planning_profile_is_advisory ... ok
  test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
  test agent_instruction_manifesto_planning_profile_embeds_no_mutating_artifacts ... ok
  test agent_instruction_proposal_honors_testing_policy_override ... ok
  test agent_instruction_manifesto_uses_default_variant_and_profile ... ok
  test agent_instruction_review_renders_review_template ... ok

  test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.69s

       Running tests/list_archive.rs (target/llvm-cov-target/debug/deps/list_archive-65f048a59af9cbdf)

  running 3 tests
  test list_archive_reports_empty_archives ... ok
  test list_archive_json_lists_archived_changes_only ... ok
  test list_archive_lists_archived_changes_only ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/list_regression.rs (target/llvm-cov-target/debug/deps/list_regression-120398dee7f4e26f)

  running 3 tests
  test list_default_text_and_json_shape_regression ... ok
  test list_sort_regression ... ok
  test list_filters_regression ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

       Running tests/misc_more.rs (target/llvm-cov-target/debug/deps/misc_more-646a291a093c7b5c)

  running 16 tests
  test archive_prompts_on_incomplete_tasks_and_proceeds_when_confirmed ... ignored, PTY interactive test — can hang in CI; run with --ignored locally
  test list_errors_when_ito_changes_dir_missing ... ok
  test plan_status_errors_when_roadmap_missing ... ok
  test list_modules_empty_prints_hint ... ok
  test status_change_flag_not_found_shows_suggestions ... ok
  test status_change_flag_reports_ambiguous_target ... ok
  test status_schema_not_found_includes_available_schemas ... ok
  test status_change_flag_supports_module_scoped_slug_query ... ok
  test show_unknown_item_offers_suggestions ... ok
  test status_missing_change_flag_lists_available_changes ... ok
  test list_specs_empty_prints_sentence_even_for_json ... ok
  test git_env_vars_do_not_override_runtime_root_detection ... ok
  test commands_run_from_nested_dir_use_git_worktree_root ... ok
  test status_change_flag_supports_shorthand_and_partial_match ... ok
  test show_module_errors_and_json_not_implemented ... ok
  test show_spec_json_filters_and_requirement_index_errors ... ok

  test result: ok. 15 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.25s

       Running tests/new_more.rs (target/llvm-cov-target/debug/deps/new_more-13046582400a6c3b)

  running 1 test
  test new_change_covers_happy_and_error_paths ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

       Running tests/parity_help_version.rs (target/llvm-cov-target/debug/deps/parity_help_version-aafe7e907d58a362)

  running 2 tests
  test version_prints_workspace_version ... ok
  test help_prints_usage ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/parity_tasks.rs (target/llvm-cov-target/debug/deps/parity_tasks-910490a3fd090304)

  running 2 tests
  test parity_tasks_init_writes_same_file ... ok
  test parity_tasks_status_next_start_complete_match_oracle ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

       Running tests/path_more.rs (target/llvm-cov-target/debug/deps/path_more-cf4bb2c1a491b5ce)

  running 8 tests
  test path_missing_subcommand_errors ... ok
  test path_roots_text_renders_worktree_fields_when_available ... ok
  test path_worktree_requires_a_selector_flag ... ok
  test path_errors_in_bare_repo ... ok
  test path_roots_json_includes_worktree_fields_when_enabled ... ok
  test path_worktrees_root_requires_worktrees_enabled ... ok
  test path_worktrees_root_and_change_worktree_resolve_from_config ... ok
  test path_roots_are_absolute_in_initialized_repo ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.70s

       Running tests/plan_state_more.rs (target/llvm-cov-target/debug/deps/plan_state_more-2fde96e65761825e)

  running 3 tests
  test plan_status_fails_without_roadmap ... ok
  test plan_init_creates_structure ... ok
  test plan_status_succeeds_after_init ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

       Running tests/ralph_smoke.rs (target/llvm-cov-target/debug/deps/ralph_smoke-e5c74726fd5d0969)

  running 26 tests
  test ralph_change_flag_supports_shorthand_resolution ... ok
  test ralph_file_flag_allowed_without_change_or_module ... ok
  test ralph_change_flag_supports_slug_query_resolution ... ok
  test ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
  test ralph_file_flag_requires_readable_file ... ok
  test ralph_continue_ready_exits_successfully_when_all_changes_complete ... ok
  test ralph_file_flag_runs_without_change_or_module ... ok
  test ralph_interactive_status_prompts_for_exactly_one_change ... ok
  test ralph_interactive_prompts_and_runs_selected_changes_sequentially ... ok
  test ralph_markdown_prd_source_marks_first_pending_task_complete ... ok
  test ralph_unknown_harness_returns_clear_error ... ok
  test ralph_no_interactive_without_target_returns_clear_error ... ok
  test ralph_yaml_source_marks_first_pending_task_complete ... ok
  [main (root-commit) 5672c32] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  [main (root-commit) 5672c32] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  test ralph_accepts_new_harness_names_for_status_flow ... ok
  test ralph_github_source_closes_issue_on_success ... ok
  [main (root-commit) 8d04ef2] init
   6 files changed, 44 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 README.md
   create mode 100644 tasks.yaml
  [main (root-commit) 733398a] init
   6 files changed, 38 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 README.md
   create mode 100644 tasks.yaml
  [main (root-commit) 5672c32] init
   6 files changed, 35 insertions(+)
   create mode 100644 .ito/changes/000-01_test-change/proposal.md
   create mode 100644 .ito/changes/000-01_test-change/tasks.md
   create mode 100644 .ito/modules/000_ungrouped/module.md
   create mode 100644 .ito/specs/alpha/spec.md
   create mode 100644 PRD.md
   create mode 100644 README.md
  test ralph_branch_per_task_requires_clean_worktree ... ok
  test ralph_stub_harness_writes_state_and_status_works ... ok
  test ralph_branch_per_task_creates_task_branch_for_prd_source ... ok
  To /var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpfVend8
   * [new branch]      main -> main
  branch 'main' set up to track 'origin/main'.
  test ralph_sync_issue_updates_prd_back_to_github_issue ... ok
  test ralph_parallel_yaml_source_completes_grouped_tasks ... ok
  test ralph_notify_emits_operator_notification_on_success ... ok
  test ralph_browser_flag_injects_agent_browser_guidance_for_opencode ... ok
  test ralph_interactive_options_wizard_exit_on_error_stops_on_nonzero_harness_exit ... ok
  test ralph_create_pr_uses_base_branch_and_fake_gh ... ok
  test ralph_interactive_options_wizard_prompts_for_missing_values_and_applies_them ... ok
  test ralph_parallel_preserves_worker_code_changes ... ok

  test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.53s

       Running tests/serve_more.rs (target/llvm-cov-target/debug/deps/serve_more-738d7616a3bc3a75)

  running 1 test
  test serve_errors_when_not_initialized ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/show_specs_bundle.rs (target/llvm-cov-target/debug/deps/show_specs_bundle-c53bc53c484b6c83)

  running 2 tests
  test show_specs_bundles_truth_specs_as_markdown_with_metadata ... ok
  test show_specs_bundles_truth_specs_as_json_with_absolute_paths ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

       Running tests/show_specs_remote_mode.rs (target/llvm-cov-target/debug/deps/show_specs_remote_mode-16591eb989a07f12)

  running 1 test
  test show_specs_reads_backend_specs_without_local_markdown ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running tests/source_file_size.rs (target/llvm-cov-target/debug/deps/source_file_size-2f357392a3e7d6c7)

  running 1 test
  test ito_cli_source_files_are_reasonably_sized ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/stats.rs (target/llvm-cov-target/debug/deps/stats-2ca7c4f18248df04)

  running 1 test
  test stats_counts_command_end_events ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/tasks_more.rs (target/llvm-cov-target/debug/deps/tasks_more-ee62f38e6c4f9d0a)

  running 11 tests
  test tasks_status_rejects_free_form_with_more_than_two_numbers ... ok
  test tasks_commands_use_apply_tracks_filename_when_set ... ok
  test tasks_status_resolves_short_change_id ... ok
  test tasks_json_lists_are_sorted_by_task_id ... ok
  test tasks_status_resolves_free_form_two_numbers ... ok
  test tasks_complete_supports_checkbox_compat_mode ... ok
  test tasks_error_paths_cover_more_branches ... ok
  test tasks_start_supports_checkbox_compat_mode_and_enforces_single_in_progress ... ok
  test tasks_next_supports_checkbox_compat_mode_and_shows_current_or_next ... ok
  test tasks_add_shelve_unshelve_show_cover_more_paths ... ok
  test tasks_commands_support_json_output ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.29s

       Running tests/tasks_remote_mode.rs (target/llvm-cov-target/debug/deps/tasks_remote_mode-91a6c3342ec5c833)

  running 2 tests
  test remote_missing_tasks_commands_do_not_hard_fail ... ok
  test remote_task_start_updates_backend_without_local_tasks_file ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s

       Running tests/templates_schemas_export.rs (target/llvm-cov-target/debug/deps/templates_schemas_export-82239e22a0a3edf3)

  running 3 tests
  test templates_help_includes_schemas_export ... ok
  test templates_schemas_export_writes_embedded_files ... ok
  test templates_schemas_export_skips_without_force_then_overwrites_with_force ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

       Running tests/trace_more.rs (target/llvm-cov-target/debug/deps/trace_more-5e602cc37668fe47)

  running 8 tests
  test trace_missing_change_exits_nonzero ... ok
  test trace_partial_ids_json_shows_invalid_status ... ok
  test trace_legacy_checkbox_change_shows_unavailable ... ok
  test trace_uncovered_requirement_shows_uncovered_in_output ... ok
  test trace_fully_covered_json_has_ready_status ... ok
  test trace_unresolved_reference_shows_unresolved_in_output ... ok
  test trace_fully_covered_exits_zero ... ok
  test trace_uncovered_requirement_json_shows_uncovered_list ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

       Running tests/update_marker_scoped.rs (target/llvm-cov-target/debug/deps/update_marker_scoped-f73d1765785357de)

  running 5 tests
  test update_refuses_to_overwrite_partial_marker_pair ... ok
  test update_preserves_user_edits_after_end_marker_in_harness_command ... ok
  test update_still_refreshes_non_markdown_manifest_assets ... ok
  test update_preserves_user_edits_after_end_marker_in_harness_skill ... ok
  test second_update_is_a_noop_for_harness_skills ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

       Running tests/update_smoke.rs (target/llvm-cov-target/debug/deps/update_smoke-3830b4b85f455571)

  running 8 tests
  test update_preserves_project_config_and_project_md ... ok
  test update_installs_adapter_files_from_local_ito_skills ... ok
  test update_refreshes_opencode_plugin_and_preserves_user_config ... ok
  test update_preserves_user_guidance_and_user_prompt_files ... ok
  test update_merges_claude_settings_without_clobbering_user_keys ... ok
  test update_refreshes_codex_audit_instruction_assets ... ok
  test update_renders_agents_md_without_jinja2_syntax ... ok
  test update_refreshes_github_copilot_audit_assets ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.57s

       Running tests/user_guidance_injection.rs (target/llvm-cov-target/debug/deps/user_guidance_injection-e5bb71308ebf89e5)

  running 3 tests
  test agent_instruction_includes_user_guidance_when_present ... ok
  test agent_instruction_includes_scoped_user_prompt_for_artifact ... ok
  test agent_instruction_prefers_user_prompts_shared_guidance_file ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

       Running tests/validate_more.rs (target/llvm-cov-target/debug/deps/validate_more-db5722f306b82366)

  running 7 tests
  test validate_unknown_spec_offers_suggestions ... ok
  test validate_ambiguous_item_is_an_error ... ok
  test validate_type_module_special_cases_to_spec_by_id ... ok
  test validate_all_json_success_has_summary_and_by_type ... ok
  test validate_module_routes_and_error_paths ... ok
  test validate_all_prints_failure_report_in_text_mode ... ok
  test validate_change_reports_audit_drift_against_routed_storage ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

       Running tests/validate_repo_cli.rs (target/llvm-cov-target/debug/deps/validate_repo_cli-ba93ead556c88a80)

  running 12 tests
  test validate_help_lists_repo_subcommand ... ok
  test validate_repo_help_lists_documented_flags ... ok
  test validate_repo_json_emits_validation_report_envelope ... ok
  test validate_repo_list_rules_json_returns_array ... ok
  test validate_repo_list_rules_enumerates_built_in_rules ... ok
  test validate_repo_explain_audit_mirror_distinct_rule ... ok
  test validate_repo_rule_and_no_rule_mutually_exclusive_exit_2 ... ok
  test validate_repo_explain_unknown_rule_exits_2 ... ok
  test validate_repo_url_scheme_valid_fails_for_non_http_scheme ... ok
  test validate_repo_explain_prints_metadata ... ok
  test validate_repo_strict_promotes_branch_name_warning_to_failure ... ok
  test validate_repo_backend_token_not_committed_fails_when_token_in_config ... ok

  test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

       Running tests/view_proposal.rs (target/llvm-cov-target/debug/deps/view_proposal-e2f8e39fb567bf97)

  running 8 tests
  test view_proposal_help_shows_viewer_flag ... ok
  test view_proposal_html_viewer_errors_when_pandoc_missing ... ok
  test view_proposal_unknown_change_fails ... ok
  test view_proposal_disabled_tmux_is_rejected ... ok
  test view_proposal_unknown_viewer_is_rejected ... ok
  test view_proposal_json_outputs_bundle ... ok
  test view_proposal_html_viewer_succeeds_with_stub_pandoc ... ok
  test view_proposal_html_viewer_is_recognized ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.47s

       Running tests/worktree_validate.rs (target/llvm-cov-target/debug/deps/worktree_validate-2c38025dbf10306e)

  running 4 tests
  test worktree_validate_fails_on_main_checkout_and_emits_json ... ok
  test worktree_validate_succeeds_when_disabled ... ok
  test worktree_validate_accepts_same_change_suffix_worktree ... ok
  test worktree_validate_reports_mismatch_without_failing ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_common-cbd0930c872e85bb)

  running 59 tests
  test git_url::tests::handles_ssh_url_without_user ... ok
  test git_url::tests::handles_trailing_slash_in_https_url ... ok
  test git_url::tests::parses_http_scheme ... ok
  test git_url::tests::parses_https_url_with_git_suffix ... ok
  test git_url::tests::parses_git_protocol_url ... ok
  test git_url::tests::parses_https_url_without_git_suffix ... ok
  test git_url::tests::parses_gitlab_style_subgroup_takes_last_two_segments ... ok
  test git_url::tests::parses_scp_ssh_url ... ok
  test git_url::tests::parses_ssh_with_explicit_port ... ok
  test git_url::tests::returns_none_for_bare_string_without_separator ... ok
  test git_url::tests::returns_none_for_empty_string ... ok
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
  test id::change_id::tests::parse_change_id_sub_module_format_canonical ... ok
  test id::change_id::tests::parse_change_id_sub_module_format_lowercases_name ... ok
  test id::change_id::tests::parse_change_id_sub_module_format_pads_all_parts ... ok
  test id::change_id::tests::parse_change_id_sub_module_missing_name_is_error ... ok
  test id::change_id::tests::parse_change_id_sub_module_rejects_module_overflow ... ok
  test id::change_id::tests::parse_change_id_sub_module_rejects_sub_overflow ... ok
  test id::change_id::tests::parse_change_id_supports_extra_leading_zeros_for_change_num ... ok
  test id::module_id::tests::parse_module_id_pads_and_lowercases_name ... ok
  test id::change_id::tests::parse_change_id_uses_specific_hint_for_wrong_separator ... ok
  test id::module_id::tests::parse_module_id_rejects_overflow ... ok
  test id::module_id::tests::parse_module_id_rejects_overlong_input ... ok
  test id::change_id::tests::parse_change_id_missing_name_has_specific_error ... ok
  test id::spec_id::tests::parse_spec_id_preserves_value ... ok
  test id::sub_module_id::tests::parse_sub_module_id_canonical_form ... ok
  test id::sub_module_id::tests::parse_sub_module_id_lowercases_name ... ok
  test id::sub_module_id::tests::parse_sub_module_id_pads_both_parts ... ok
  test id::spec_id::tests::parse_spec_id_rejects_path_traversal_sequences ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_empty ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_missing_dot ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_module_overflow ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_non_digit_module ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_sub_overflow ... ok
  test id::sub_module_id::tests::parse_sub_module_id_with_name_suffix ... ok
  test id::sub_module_id::tests::parse_sub_module_id_strips_extra_leading_zeros ... ok
  test id::sub_module_id::tests::parse_sub_module_id_rejects_overlong_input ... ok
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

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_config-f6db7f9e730260d6)

  running 68 tests
  test config::tests::global_config_path_prefers_xdg ... ok
  test config::tests::ito_config_dir_prefers_xdg ... ok
  test config::tests::load_global_ito_config_returns_defaults_when_no_file ... ok
  test config::tests::logging_invalid_commands_defaults_exist_in_cascading_config ... ok
  test config::tests::audit_mirror_defaults_exist_in_cascading_config ... ok
  test config::tests::audit_mirror_defaults_can_be_overridden ... ok
  test config::tests::cascading_project_config_ignores_invalid_json_sources ... ok
  test config::tests::coordination_branch_defaults_exist_in_cascading_config ... ok
  test config::tests::cascading_project_config_ignores_schema_ref_key ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_missing_storage_defaults_to_worktree ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_missing_worktree_path_is_none ... ok
  test config::schema::tests::schema_contains_expected_sections ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_absent_not_serialized ... ok
  test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_round_trips ... ok
  test config::types::coordination_storage_tests::coordination_storage_default_is_worktree ... ok
  test config::types::coordination_storage_tests::coordination_storage_round_trips_embedded ... ok
  test config::types::coordination_storage_tests::coordination_storage_round_trips_worktree ... ok
  test config::types::coordination_storage_tests::coordination_storage_serializes_embedded_as_lowercase ... ok
  test config::tests::logging_invalid_commands_can_be_enabled ... ok
  test config::tests::coordination_branch_defaults_can_be_overridden ... ok
  test config::tests::worktrees_config_has_defaults_in_cascading_config ... ok
  test config::types::coordination_storage_tests::coordination_storage_serializes_worktree_as_lowercase ... ok
  test config::types::memory_tests::memory_default_is_absent_on_ito_config ... ok
  test config::types::memory_tests::memory_op_config_command_variant_requires_command_field ... ok
  test config::types::memory_tests::memory_op_config_skill_variant_requires_skill_field ... ok
  test config::types::memory_tests::memory_op_config_unknown_kind_is_rejected ... ok
  test config::types::memory_tests::memory_section_accepts_capture_only ... ok
  test config::types::memory_tests::memory_section_accepts_skill_with_options ... ok
  test config::tests::tools_tmux_enabled_defaults_to_true_in_cascading_config ... ok
  test config::types::memory_tests::memory_section_omits_absent_ops_when_serialized ... ok
  test config::tests::new_worktree_keys_take_precedence_over_legacy ... ok
  test config::tests::legacy_worktree_local_files_key_migrates ... ok
  test config::types::memory_tests::memory_section_round_trips_full_config ... ok
  test config::types::memory_tests::memory_section_round_trips_when_absent ... ok
  test config::types::memory_tests::memory_section_skill_options_are_optional ... ok
  test config::types::memory_tests::memory_section_supports_mixed_per_op_shapes ... ok
  test config::types::memory_tests::memory_section_unknown_op_key_is_rejected ... ok
  test config::tests::cascading_project_config_merges_sources_in_order_with_scalar_override ... ok
  test config::types::worktree_init_tests::worktree_init_config_absent_deserializes_to_default ... ok
  test config::types::worktree_init_tests::worktree_init_config_default_has_empty_include_and_no_setup ... ok
  test config::types::worktree_init_tests::full_ito_config_with_worktree_init_round_trips ... ok
  test config::types::worktree_init_tests::worktree_init_config_with_multiple_setup_deserializes ... ok
  test config::types::worktree_init_tests::worktree_init_config_deserializes_with_include_only ... ok
  test config::types::worktree_init_tests::worktree_init_config_with_single_setup_deserializes ... ok
  test config::types::worktree_init_tests::worktree_setup_config_array_deserializes ... ok
  test config::tests::load_global_ito_config_reads_backend_server_auth ... ok
  test config::tests::legacy_worktree_default_branch_key_migrates ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_empty_multiple_empty_vec ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_empty_single_empty_string ... ok
  test config::types::worktree_init_tests::worktree_setup_config_is_not_empty_with_command ... ok
  test config::types::worktree_init_tests::worktree_setup_config_multiple_round_trips ... ok
  test config::types::worktree_init_tests::worktree_setup_config_single_round_trips ... ok
  test config::types::worktree_init_tests::worktree_setup_config_single_string_deserializes ... ok
  test config::types::worktree_init_tests::worktrees_config_init_does_not_break_existing_fields ... ok
  test config::types::worktree_init_tests::worktrees_config_with_init_section_deserializes ... ok
  test config::types::worktree_init_tests::worktrees_config_without_init_section_uses_defaults ... ok
  test context::tests::resolve_with_ctx_sets_none_when_ito_dir_is_missing ... ok
  test context::tests::resolve_with_ctx_uses_explicit_config_context_paths ... ok
  test context::tests::resolve_with_ctx_sets_ito_path_when_directory_exists ... ok
  test ito_dir::tests::get_ito_dir_name_defaults_to_dot_ito ... ok
  test ito_dir::tests::sanitize_rejects_path_separators_and_overlong_values ... ok
  test output::tests::no_color_env_set_matches_ts_values ... ok
  test output::tests::resolve_interactive_respects_cli_and_env ... ok
  test output::tests::resolve_ui_options_combines_sources ... ok
  test ito_dir::tests::get_ito_path_normalizes_dotdot_segments ... ok
  test ito_dir::tests::invalid_repo_project_path_falls_back_to_default ... ok
  test ito_dir::tests::dot_repo_config_overrides_repo_config ... ok
  test ito_dir::tests::repo_config_overrides_global_config ... ok

  test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_core-9ecf9da96fa2a548)

  running 707 tests
  test audit::mirror::tests::merge_jsonl_dedupes_and_appends_local_lines ... ok
  test audit::mirror::tests::merge_jsonl_drops_events_older_than_one_month_from_newest_event ... ok
  test artifact_mutations::tests::bundle_service_patches_design_and_returns_revision ... ok
  test audit::mirror::tests::merge_jsonl_ignores_blank_lines ... ok
  test audit::mirror::tests::merge_jsonl_keeps_reconciled_events_after_different_event ... ok
  test audit::mirror::tests::merge_jsonl_aggregates_adjacent_equivalent_reconciled_events ... ok
  test artifact_mutations::tests::fs_service_writes_and_patches_proposal ... ok
  test artifact_mutations::tests::fs_service_creates_spec_delta_directory_on_write ... ok
  test audit::reader::reader_tests::reads_events_from_injected_store ... ok
  test audit::mirror::tests::merge_jsonl_caps_git_log_to_newest_1000_events ... ok
  test audit::mirror::tests::merge_jsonl_count_cap_uses_timestamp_not_input_position ... ok
  test audit::reconcile::tests::build_file_state_from_default_tasks_md ... ok
  test audit::reconcile::tests::build_file_state_uses_apply_tracks_when_set ... ok
  test audit::store::tests::internal_branch_location_keys_include_branch_identity ... ok
  test audit::reader::reader_tests::read_from_missing_file_returns_empty ... ok
  test audit::stream::tests::default_config_has_sensible_values ... ok
  test audit::reconcile::tests::reconcile_empty_log ... ok
  test audit::reader::reader_tests::read_parses_valid_events ... ok
  test audit::reader::reader_tests::filter_by_operation ... ok
  test audit::reader::reader_tests::combined_filters ... ok
  test audit::validate::tests::detect_duplicate_create ... ok
  test audit::validate::tests::detect_status_transition_mismatch ... ok
  test audit::validate::tests::detect_timestamp_ordering_violation ... ok
  test audit::validate::tests::different_scopes_are_independent ... ok
  test audit::validate::tests::empty_events_no_issues ... ok
  test audit::validate::tests::no_issues_for_valid_sequence ... ok
  test audit::worktree::tests::aggregate_empty_worktrees ... ok
  test audit::reader::reader_tests::skips_empty_lines ... ok
  test audit::worktree::tests::find_worktree_bare_excluded ... ok
  test audit::worktree::tests::find_worktree_matching_branch ... ok
  test audit::worktree::tests::find_worktree_multiple_returns_first_match ... ok
  test audit::worktree::tests::find_worktree_no_match ... ok
  test audit::worktree::tests::parse_bare_worktree_excluded ... ok
  test audit::reader::reader_tests::filter_by_scope ... ok
  test audit::worktree::tests::parse_multiple_worktrees ... ok
  test audit::worktree::tests::parse_detached_head ... ok
  test audit::worktree::tests::parse_single_worktree ... ok
  test audit::worktree::tests::worktree_audit_log_path_resolves ... ok
  test audit::writer::tests::audit_log_path_resolves_correctly ... ok
  test audit::reader::reader_tests::filter_by_entity_type ... ok
  test audit::writer::tests::best_effort_returns_ok_even_on_failure ... ok
  test audit::reader::reader_tests::skips_malformed_lines ... ok
  test audit::writer::tests::appends_events_to_existing_file ... ok
  test audit::writer::tests::creates_directory_and_file_on_first_write ... ok
  test backend_change_repository::tests::get_delegates_to_reader ... ok
  test backend_change_repository::tests::list_complete_filters_correctly ... ok
  test backend_change_repository::tests::list_incomplete_filters_correctly ... ok
  test backend_change_repository::tests::list_returns_all_changes ... ok
  test backend_change_repository::tests::resolve_target_ambiguous ... ok
  test backend_change_repository::tests::resolve_target_exact_match ... ok
  test backend_change_repository::tests::resolve_target_not_found ... ok
  test backend_change_repository::tests::resolve_target_prefix_match ... ok
  test backend_client::tests::custom_backup_dir_is_used ... ok
  test backend_client::tests::default_backup_dir_uses_home ... ok
  test backend_client::tests::disabled_backend_returns_none ... ok
  test audit::writer::tests::events_deserialize_back_correctly ... ok
  test backend_client::tests::enabled_backend_empty_token_fails ... ok
  test backend_client::tests::enabled_backend_missing_token_fails ... ok
  test backend_client::tests::enabled_backend_with_env_var_token_resolves ... ok
  test backend_client::tests::enabled_backend_with_explicit_token_resolves ... ok
  test backend_client::tests::env_var_token_takes_precedence_over_config_token ... ok
  test backend_client::tests::idempotency_key_includes_operation ... ok
  test backend_client::tests::is_retriable_status_checks ... ok
  test audit::writer::tests::each_line_is_valid_json ... ok
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
  test backend_coordination::tests::claim_conflict ... ok
  test backend_coordination::tests::claim_success ... ok
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
  test backend_http::backend_http_tests::parse_timestamp_returns_error_for_invalid_rfc3339 ... ok
  test backend_http::backend_http_tests::post_requests_are_not_retried_by_default ... ok
  test backend_sync::tests::backend_error_mapping_produces_correct_error_types ... ok
  test backend_sync::tests::path_traversal_in_capability_rejected ... ok
  test backend_sync::tests::path_traversal_in_change_id_rejected ... ok
  test backend_coordination::tests::archive_with_backend_skip_specs ... ok
  test backend_coordination::tests::archive_with_backend_backend_unavailable ... ok
  test backend_coordination::tests::archive_with_backend_happy_path ... ok
  test backend_sync::tests::pull_creates_backup ... ok
  test backend_sync::tests::push_missing_change_dir_fails ... ok
  test backend_sync::tests::push_conflict_returns_actionable_error ... ok
  test backend_sync::tests::pull_writes_artifacts_locally ... ok
  test backend_task_repository::tests::checkbox_tasks_parsed_correctly ... ok
  test backend_task_repository::tests::get_task_counts_from_backend ... ok
  test backend_task_repository::tests::has_tasks_empty_content ... ok
  test backend_task_repository::tests::has_tasks_detects_content ... ok
  test backend_task_repository::tests::missing_tasks_returns_empty ... ok
  test backend_sync::tests::read_local_bundle_sorts_specs ... ok
  test change_repository::tests::resolve_target_includes_archive_when_requested ... ok
  test backend_sync::tests::push_sends_local_bundle ... ok
  test change_repository::tests::exists_and_get_work ... ok
  test change_repository::tests::list_skips_archive_dir ... ok
  test config::tests::is_valid_integration_mode_checks_correctly ... ok
  test config::tests::is_valid_repository_mode_checks_correctly ... ok
  test config::tests::is_valid_worktree_strategy_checks_correctly ... ok
  test config::tests::resolve_worktree_template_defaults_reads_overrides ... ok
  test config::tests::resolve_worktree_template_defaults_uses_defaults_when_missing ... ok
  test config::tests::skill_id_resolves_returns_false_when_no_paths_exist ... ok
  test config::tests::validate_config_value_accepts_archive_main_integration_mode ... ok
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
  test config::tests::validate_config_value_rejects_memory_op_missing_required_field ... ok
  test config::tests::validate_config_value_rejects_memory_op_unknown_kind ... ok
  test config::tests::validate_config_value_rejects_non_string_strategy ... ok
  test config::tests::validate_config_value_rejects_unknown_memory_kind ... ok
  test config::tests::validate_config_value_rejects_unknown_memory_op_key ... ok
  test config::tests::validate_config_value_rejects_zero_sync_interval ... ok
  test config::tests::validate_memory_config_passes_when_no_skill_provider ... ok
  test change_repository::tests::resolve_target_module_scoped_query ... ok
  test change_repository::tests::resolve_target_reports_ambiguity ... ok
  test config::tests::validate_memory_config_rejects_missing_skill ... ok
  test config::tests::validate_memory_config_passes_when_skill_resolves_in_flat_layout ... ok
  test coordination::tests::create_dir_link_creates_symlink ... ok
  test coordination::tests::format_message_broken_symlinks_contains_paths_and_hint ... ok
  test coordination::tests::format_message_embedded_is_none ... ok
  test coordination::tests::format_message_healthy_is_none ... ok
  test coordination::tests::format_message_not_wired_contains_dir_and_hint ... ok
  test coordination::tests::format_message_worktree_missing_contains_path_and_hint ... ok
  test coordination::tests::format_message_wrong_target_contains_paths_and_hint ... ok
  test config::tests::validate_memory_config_passes_when_skill_resolves_in_grouped_layout ... ok
  test coordination::tests::create_dir_link_fails_when_dst_exists ... ok
  test coordination::tests::gitignore_entries_match_coordination_dirs ... ok
  test coordination::tests::gitignore_entries_returns_static_slice ... ok
  test change_repository::tests::suggest_targets_prioritizes_slug_matches ... ok
  test coordination::tests::gitignore_created_when_absent ... ok
  test coordination::tests::gitignore_entries_added_when_missing ... ok
  test coordination::tests::gitignore_no_duplicates_on_second_call ... ok
  test coordination::tests::gitignore_preserves_existing_content ... ok
  test coordination::tests::gitignore_skips_already_present_entries ... ok
  test coordination::tests::health_embedded_returns_embedded ... ok
  test coordination::tests::health_missing_link_is_not_wired ... ok
  test coordination::tests::health_worktree_missing_when_dir_absent ... ok
  test coordination::tests::health_not_wired_when_real_dirs_present ... ok
  test coordination::tests::health_broken_symlinks_when_target_missing ... ok
  test coordination::tests::health_healthy_when_all_symlinks_correct ... ok
  test coordination::tests::remove_is_noop_when_dirs_absent ... ok
  test coordination::tests::remove_is_noop_for_real_dirs ... ok
  test coordination::tests::health_wrong_target_when_symlink_points_elsewhere ... ok
  test coordination::tests::wire_creates_symlinks_for_all_dirs ... ok
  test coordination::tests::wire_handles_empty_real_dir ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails ... ok
  test coordination::tests::wire_is_idempotent ... ok
  test coordination::tests::remove_restores_real_dirs_with_content ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails ... ok
  test coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist ... ok
  test coordination::tests::wire_migrates_real_dir_content ... ok
  test coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails ... ok
  test coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote ... ok
  test coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured ... ok
  test coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded ... ok
  test audit::reconcile::tests::reconcile_detects_drift ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist ... ok
  test coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails ... ok
  test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails ... ok
  test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails ... ok
  test audit::reconcile::tests::reconcile_missing_tasks_file ... ok
  test audit::reconcile::tests::reconcile_no_drift ... ok
  test coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit ... ok
  test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target ... ok
  test create::create_sub_module_tests::create_sub_module_accepts_full_module_folder_name ... ok
  test create::create_sub_module_tests::create_sub_module_creates_directory_and_module_md ... ok
  test create::create_sub_module_tests::create_sub_module_errors_on_duplicate_name ... ok
  test create::create_sub_module_tests::create_sub_module_errors_on_unknown_parent_module ... ok
  test create::create_sub_module_tests::create_sub_module_allocates_sequential_numbers ... ok
  test distribution::tests::pi_adapter_asset_exists_in_embedded_templates ... ok
  test distribution::tests::pi_agent_templates_discoverable ... ok
  test distribution::tests::pi_manifests_commands_match_opencode_commands ... ok
  test distribution::tests::pi_manifests_includes_adapter_skills_and_commands ... ok
  test distribution::tests::pi_manifests_skills_match_opencode_skills ... ok
  test errors::tests::core_error_helpers_construct_expected_variants ... ok
  test event_forwarder::tests::checkpoint_missing_returns_zero ... ok
  test create::create_sub_module_tests::create_sub_module_rejects_invalid_name ... ok
  test distribution::tests::ensure_manifest_script_is_executable_only_adds_execute_bits ... ok
  test event_forwarder::tests::checkpoint_roundtrip ... ok
  test create::create_sub_module_tests::create_sub_module_with_description_writes_purpose ... ok
  test audit::worktree::tests::aggregate_worktree_with_events ... ok
  test event_forwarder::tests::forward_no_events_returns_zero ... ok
  test audit::stream::tests::poll_returns_empty_when_no_new_events ... ok
  test event_forwarder::tests::forward_result_equality ... ok
  test audit::stream::tests::poll_detects_new_events ... ok
  test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists ... ok
  test event_forwarder::tests::forward_persists_checkpoint_per_batch ... ok
  test event_forwarder::tests::forward_reports_duplicates ... ok
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
  test event_forwarder::tests::forward_retries_transient_failure ... ok
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
  test installers::agents_cleanup::tests::removes_broken_specialist_symlinks_and_prunes_empty_dirs ... ok
  test installers::agents_cleanup::tests::removes_regular_specialist_files_and_prunes_empty_dirs ... ok
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
  test installers::tests::gitignore_does_not_duplicate_on_repeated_calls ... ok
  test installers::tests::gitignore_exact_line_matching_trims_whitespace ... ok
  test installers::tests::gitignore_full_audit_setup ... ok
  test installers::tests::gitignore_ignores_local_configs ... ok
  test installers::tests::gitignore_legacy_audit_events_unignore_noop_when_absent ... ok
  test installers::tests::gitignore_legacy_audit_events_unignore_removed ... ok
  test installers::tests::gitignore_noop_when_already_present ... ok
  test installers::tests::gitignore_preserves_existing_content_and_adds_newline_if_missing ... ok
  test installers::tests::release_tag_is_prefixed_with_v ... ok
  test installers::tests::should_install_project_rel_filters_by_tool_id ... ok
  test installers::tests::should_install_project_rel_filters_pi ... ok
  test installers::tests::update_agent_model_field_updates_frontmatter_when_present ... ok
  test installers::tests::update_model_in_yaml_replaces_or_inserts ... ok
  test installers::tests::write_one_marker_managed_files_error_when_markers_missing_in_update_mode ... ok
  test installers::tests::write_one_marker_managed_files_refuse_overwrite_without_markers ... ok
  test installers::tests::write_one_marker_managed_files_update_existing_markers ... ok
  test installers::tests::write_one_non_marker_files_skip_on_init_update_mode ... ok
  test installers::tests::write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode ... ok
  test installers::tests::write_one_non_marker_user_owned_files_preserve_on_update_mode ... ok
  test list::tests::counts_requirements_from_headings ... ok
  test list::tests::iso_millis_matches_expected_shape ... ok
  test list::tests::list_changes_filters_by_progress_status ... ok
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
  test module_repository::tests::regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes ... ok
  test module_repository::tests::test_exists ... ok
  test module_repository::tests::test_get ... ok
  test module_repository::tests::test_get_not_found ... ok
  test module_repository::tests::test_get_uses_full_name_input ... ok
  test module_repository::tests::test_list ... ok
  test module_repository::tests::test_list_with_change_counts ... ok
  test orchestrate::gates::tests::remediation_includes_failed_gate_and_downstream_run_gates ... ok
  test orchestrate::gates::tests::remediation_includes_failed_gate_even_when_policy_is_skip ... ok
  test orchestrate::gates::tests::remediation_returns_empty_when_failed_gate_not_found ... ok
  test orchestrate::gates::tests::remediation_skips_downstream_skip_gates ... ok
  test git::tests::setup_coordination_branch_core_wraps_process_error ... ok
  test process::tests::captures_non_zero_exit ... ok
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
  test event_forwarder::tests::forward_batches_correctly ... ok
  test process::tests::captures_stdout_and_stderr ... ok
  test event_forwarder::tests::forward_respects_checkpoint ... ok
  test ralph::validation::tests::task_completion_fails_when_remaining ... ok
  test ralph::validation::tests::task_completion_passes_when_no_tasks ... ok
  test ralph::validation::tests::truncate_for_context_long_truncated ... ok
  test ralph::validation::tests::truncate_for_context_multibyte_utf8 ... ok
  test ralph::validation::tests::truncate_for_context_short_unchanged ... ok
  test sqlite_project_store::repositories::tests::archive_change_rolls_back_when_spec_promotion_fails ... ok
  test sqlite_project_store::repositories::tests::ensure_project_creates_row ... ok
  test sqlite_project_store::repositories::tests::ensure_project_is_idempotent ... ok
  test sqlite_project_store::repositories::tests::get_change_returns_full_data ... ok
  test sqlite_project_store::repositories::tests::get_missing_change_returns_not_found ... ok
  test sqlite_project_store::repositories::tests::get_module_by_id ... ok
  test sqlite_project_store::repositories::tests::on_disk_database_persists ... ok
  test sqlite_project_store::repositories::tests::open_in_memory_creates_schema ... ok
  test sqlite_project_store::repositories::tests::push_artifact_bundle_rolls_back_partial_writes_on_failure ... ok
  test sqlite_project_store::repositories::tests::store_is_send_sync ... ok
  test sqlite_project_store::repositories::tests::task_mutation_service_reports_poisoned_connection_without_panicking ... ok
  test sqlite_project_store::repositories::tests::task_repository_loads_tasks ... ok
  test sqlite_project_store::repositories::tests::task_repository_missing_change_returns_empty ... ok
  test sqlite_project_store::repositories::tests::two_projects_are_isolated ... ok
  test sqlite_project_store::repositories::tests::upsert_and_list_changes ... ok
  test sqlite_project_store::repositories::tests::upsert_and_list_modules ... ok
  test task_repository::tests::load_tasks_uses_schema_apply_tracks_when_set ... ok
  test task_repository::tests::test_get_task_counts_checkbox_format ... ok
  test task_repository::tests::test_get_task_counts_enhanced_format ... ok
  test task_repository::tests::test_has_tasks ... ok
  test task_repository::tests::test_missing_tasks_file_returns_zero ... ok
  test tasks::tests::read_tasks_markdown_rejects_traversal_like_change_id ... ok
  test tasks::tests::read_tasks_markdown_returns_contents_for_existing_file ... ok
  test tasks::tests::read_tasks_markdown_returns_error_for_missing_file ... ok
  test tasks::tests::returns_empty_when_no_ready_tasks_exist ... ok
  test tasks::tests::returns_ready_tasks_for_ready_changes ... ok
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
  test validate_repo::audit_rules::tests::mirror_branch_distinct_active_when_both_enabled ... ok
  test validate_repo::audit_rules::tests::mirror_branch_distinct_fails_when_branches_match ... ok
  test validate_repo::audit_rules::tests::mirror_branch_distinct_inactive_when_mirror_disabled ... ok
  test validate_repo::audit_rules::tests::mirror_branch_distinct_inactive_when_storage_embedded ... ok
  test validate_repo::audit_rules::tests::mirror_branch_distinct_passes_when_branches_differ ... ok
  test validate_repo::audit_rules::tests::mirror_branch_distinct_passes_when_either_branch_empty ... ok
  test validate_repo::audit_rules::tests::mirror_branch_set_active_when_mirror_enabled ... ok
  test validate_repo::audit_rules::tests::mirror_branch_set_inactive_when_mirror_disabled ... ok
  test validate_repo::audit_rules::tests::mirror_branch_set_passes_for_canonical_name ... ok
  test validate_repo::audit_rules::tests::mirror_branch_set_warns_on_empty_branch ... ok
  test validate_repo::audit_rules::tests::mirror_branch_set_warns_on_non_conventional_name ... ok
  test validate_repo::backend_rules::tests::project_org_repo_fails_when_org_missing ... ok
  test validate_repo::backend_rules::tests::project_org_repo_fails_when_repo_missing ... ok
  test validate_repo::backend_rules::tests::project_org_repo_passes_when_both_set ... ok
  test validate_repo::backend_rules::tests::project_org_repo_reports_both_when_both_missing ... ok
  test validate_repo::backend_rules::tests::rules_active_when_backend_enabled ... ok
  test validate_repo::backend_rules::tests::rules_inactive_when_backend_disabled ... ok
  test validate_repo::backend_rules::tests::token_not_committed_emits_error_severity_directly ... ok
  test validate_repo::backend_rules::tests::token_not_committed_fails_when_token_in_tracked_config ... ok
  test validate_repo::backend_rules::tests::token_not_committed_passes_when_no_layer_sets_token ... ok
  test ralph::validation::tests::shell_timeout_is_failure ... ok
  test validate_repo::backend_rules::tests::token_not_committed_passes_when_token_in_local_config ... ok
  test validate_repo::backend_rules::tests::url_scheme_fails_for_empty ... ok
  test validate_repo::backend_rules::tests::url_scheme_fails_for_ftp ... ok
  test validate_repo::backend_rules::tests::token_not_committed_strict_flag_does_not_weaken_severity ... ok
  test validate_repo::backend_rules::tests::url_scheme_fails_for_scheme_only_no_host ... ok
  test validate_repo::backend_rules::tests::url_scheme_fails_for_unparseable ... ok
  test validate_repo::backend_rules::tests::url_scheme_passes_for_http ... ok
  test validate_repo::backend_rules::tests::url_scheme_passes_for_https ... ok
  test validate_repo::coordination_rules::tests::branch_name_set_passes_for_canonical_name ... ok
  test validate_repo::coordination_rules::tests::branch_name_set_warns_on_empty_name ... ok
  test validate_repo::coordination_rules::tests::branch_name_set_warns_on_non_conventional_name ... ok
  test validate_repo::coordination_rules::tests::gitignore_entries_passes_when_all_canonical_lines_present ... ok
  test validate_repo::coordination_rules::tests::gitignore_entries_warns_on_each_missing_canonical_line ... ok
  test validate_repo::coordination_rules::tests::gitignore_entries_warns_when_gitignore_missing_entirely ... ok
  test validate_repo::coordination_rules::tests::rules_active_when_storage_is_worktree ... ok
  test validate_repo::coordination_rules::tests::rules_inactive_when_storage_is_embedded ... ok
  test validate_repo::coordination_rules::tests::staged_symlinked_paths_fails_for_each_path_under_coordination_dir ... ok
  test validate_repo::coordination_rules::tests::staged_symlinked_paths_passes_when_no_staged_files ... ok
  test validate_repo::coordination_rules::tests::staged_symlinked_paths_passes_when_staged_paths_outside_coordination_dirs ... ok
  test validate_repo::coordination_rules::tests::staged_symlinked_paths_skips_dot_ito_itself ... ok
  test validate_repo::coordination_rules::tests::symlinks_wired_message_includes_why_clause_when_health_check_fails ... ok
  test validate_repo::pre_commit_detect::tests::detection_is_read_only_for_every_variant ... ok
  test validate_repo::pre_commit_detect::tests::dot_lefthook_yaml_returns_lefthook ... ok
  test validate_repo::pre_commit_detect::tests::dot_mise_mentioning_prek_returns_prek ... ok
  test validate_repo::pre_commit_detect::tests::empty_repo_returns_none ... ok
  test validate_repo::pre_commit_detect::tests::husky_directory_returns_husky ... ok
  test validate_repo::pre_commit_detect::tests::lefthook_yml_returns_lefthook ... ok
  test validate_repo::pre_commit_detect::tests::mise_mentioning_prek_returns_prek ... ok
  test validate_repo::pre_commit_detect::tests::package_json_with_husky_key_returns_husky ... ok
  test validate_repo::pre_commit_detect::tests::pre_commit_classification_overrides_husky ... ok
  test validate_repo::pre_commit_detect::tests::pre_commit_config_alone_returns_pre_commit ... ok
  test validate_repo::pre_commit_detect::tests::pre_commit_config_with_prek_in_yaml_returns_prek ... ok
  test validate_repo::pre_commit_detect::tests::pre_commit_system_as_str_round_trips ... ok
  test validate_repo::pre_commit_detect::tests::prek_on_path_promotes_pre_commit_to_prek ... ok
  test validate_repo::registry::tests::built_in_registry_contains_every_built_in_rule ... ok
  test validate_repo::registry::tests::empty_registry_has_no_rules ... ok
  test validate_repo::registry::tests::list_active_rules_for_empty_registry_returns_empty ... ok
  test validate_repo::registry::tests::list_active_rules_for_inactive_rule_reports_active_false ... ok
  test validate_repo::registry::tests::list_active_rules_for_returns_rules_sorted_by_id ... ok
  test validate_repo::registry::tests::list_active_rules_for_single_active_rule_reports_active_true ... ok
  test validate_repo::registry::tests::list_active_rules_for_surfaces_gate_metadata ... ok
  test validate_repo::registry::tests::list_active_rules_matrix_matches_specification ... ok
  test validate_repo::registry::tests::public_list_active_rules_delegates_to_built_in_registry ... ok
  test validate_repo::repository_rules::tests::rules_active_in_sqlite_mode ... ok
  test validate_repo::repository_rules::tests::rules_inactive_in_filesystem_mode ... ok
  test validate_repo::repository_rules::tests::sqlite_db_not_committed_errors_when_file_is_tracked ... ok
  test validate_repo::repository_rules::tests::sqlite_db_not_committed_passes_when_untracked_and_ignored ... ok
  test validate_repo::repository_rules::tests::sqlite_db_not_committed_silent_when_path_unset ... ok
  test validate_repo::repository_rules::tests::sqlite_db_not_committed_warns_when_untracked_and_not_ignored ... ok
  test validate_repo::repository_rules::tests::sqlite_db_path_set_errors_when_path_escapes_root_via_dotdot ... ok
  test validate_repo::repository_rules::tests::sqlite_db_path_set_errors_when_path_outside_project_root ... ok
  test validate_repo::repository_rules::tests::sqlite_db_path_set_errors_when_path_unset ... ok
  test validate_repo::repository_rules::tests::sqlite_db_path_set_passes_when_parent_exists ... ok
  test validate_repo::repository_rules::tests::sqlite_db_path_set_warns_when_parent_directory_missing ... ok
  test validate_repo::rule::tests::rule_id_is_orderable_for_deterministic_output ... ok
  test validate_repo::rule::tests::rule_id_round_trips_through_as_str ... ok
  test validate_repo::rule::tests::rule_severity_string_matches_validation_levels ... ok
  test validate_repo::staged::tests::contains_matches_a_staged_path ... ok
  test validate_repo::staged::tests::empty_snapshot_reports_zero_length ... ok
  test validate_repo::staged::tests::from_git_parses_z_delimited_paths_and_handles_newlines_in_filenames ... ok
  test validate_repo::staged::tests::from_git_propagates_spawn_error_with_what_why_fix_message ... ok
  test validate_repo::staged::tests::from_git_returns_empty_snapshot_for_empty_index ... ok
  test validate_repo::staged::tests::from_git_returns_error_when_git_exits_non_zero ... ok
  test validate_repo::staged::tests::from_paths_deduplicates_and_orders_lexicographically ... ok
  test validate_repo::staged::tests::from_z_separated_handles_consecutive_nuls ... ok
  test validate_repo::staged::tests::from_z_separated_handles_empty_input ... ok
  test validate_repo::staged::tests::from_z_separated_handles_only_nuls ... ok
  test validate_repo::staged::tests::from_z_separated_handles_trailing_nul ... ok
  test validate_repo::tests::run_repo_validation_skips_inactive_rules ... ok
  test validate_repo::tests::run_repo_validation_strict_promotes_warnings_to_errors ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_inactive_when_worktrees_disabled ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_quiet_for_bare_control_siblings_strategy ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_quiet_for_checkout_siblings_strategy ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_quiet_when_gitignore_has_entry ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_warns_on_empty_dir_name ... ok
  test validate_repo::worktrees_rules::tests::layout_consistent_warns_when_checkout_subdir_missing_gitignore_entry ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_active_when_worktrees_enabled ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_fails_when_on_default_branch_with_staged_files ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_inactive_when_worktrees_disabled ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_passes_in_change_branch ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_passes_on_detached_head ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_passes_when_git_command_fails ... ok
  test validate_repo::worktrees_rules::tests::no_write_on_control_passes_when_no_staged_files ... ok
  test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
  test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok
  test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
  test viewer::html::tests::html_viewer_availability_depends_on_pandoc ... ok
  test viewer::html::tests::html_viewer_open_errors_when_pandoc_missing ... ok
  test viewer::html::tests::html_viewer_reports_expected_description ... ok
  test viewer::html::tests::html_viewer_reports_expected_name ... ok
  test viewer::tests::concrete_viewers_report_expected_names ... ok
  test viewer::tests::default_registry_includes_html_viewer ... ok
  test ralph::validation::tests::run_extra_validation_failure ... ok
  test viewer::tests::viewer_backend_trait_exposes_required_methods ... ok
  test viewer::tests::viewer_registry_filters_and_finds_available_viewers ... ok
  test viewer::tests::viewer_registry_hides_tmux_when_disabled ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_creates_worktree_when_absent ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_existing_worktree_returns_path_without_creation ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_git_failure_returns_error ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_with_include_files_copies_them ... ok
  test worktree_ensure::worktree_ensure_tests::ensure_worktrees_disabled_returns_cwd ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_accepts_normal_ids ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_empty ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_leading_dash ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_nul ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_separators ... ok
  test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_traversal ... ok
  test worktree_init::worktree_init_tests::copy_include_files_copies_to_dest ... ok
  test worktree_init::worktree_init_tests::copy_include_files_empty_config_and_no_file ... ok
  test worktree_init::worktree_init_tests::copy_include_files_skips_existing_destination ... ok
  test worktree_init::worktree_init_tests::copy_include_files_skips_missing_source ... ok
  test worktree_init::worktree_init_tests::init_worktree_copies_files_and_runs_setup ... ok
  test worktree_init::worktree_init_tests::init_worktree_no_setup_copies_files_only ... ok
  test worktree_init::worktree_init_tests::init_worktree_preserves_existing_destination_file ... ok
  test worktree_init::worktree_init_tests::init_worktree_setup_failure_returns_error ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_comments_only ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_empty_content ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_strips_comments_and_blanks ... ok
  test worktree_init::worktree_init_tests::parse_worktree_include_file_trims_whitespace ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_config_only ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_deduplicates ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_file_only ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_glob_expansion ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_ignores_directories ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_missing_include_file_ok ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_no_match_returns_empty ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_rejects_absolute_path_in_pattern ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_rejects_path_traversal ... ok
  test worktree_init::worktree_init_tests::resolve_include_files_union_of_config_and_file ... ok
  test worktree_init::worktree_init_tests::run_setup_empty_multiple_commands_is_noop ... ok
  test worktree_init::worktree_init_tests::run_setup_empty_single_command_is_noop ... ok
  test worktree_init::worktree_init_tests::run_setup_first_command_fails_stops_sequence ... ok
  test worktree_init::worktree_init_tests::run_setup_multiple_commands_run_in_order ... ok
  test worktree_init::worktree_init_tests::run_setup_no_config_is_noop ... ok
  test worktree_init::worktree_init_tests::run_setup_single_command_invoked ... ok
  test worktree_validate::tests::worktree_validate_accepts_branch_match_when_path_differs ... ok
  test worktree_validate::tests::worktree_validate_accepts_same_change_suffix_path ... ok
  test worktree_validate::tests::worktree_validate_disabled_reports_disabled_status ... ok
  test worktree_validate::tests::worktree_validate_does_not_treat_checkout_subdir_worktree_as_main ... ok
  test worktree_validate::tests::worktree_validate_rejects_main_checkout ... ok
  test worktree_validate::tests::worktree_validate_rejects_superstring_false_positive ... ok
  test worktree_validate::tests::worktree_validate_reports_mismatch_outside_main_checkout ... ok
  test audit::reconcile::tests::reconcile_fix_clears_extra_task_drift ... ok
  test audit::reconcile::tests::reconcile_fix_writes_compensating_events ... ok
  test ralph::validation::tests::run_extra_validation_success ... ok
  test validate_repo::coordination_rules::tests::symlinks_wired_emits_resolution_error_when_no_remote_or_backend_project ... ok
  test viewer::tests::run_with_stdin_closes_pipe_after_write ... ok
  test event_forwarder::tests::forward_skips_when_fully_forwarded ... ok
  test event_forwarder::tests::forward_sends_all_new_events ... ok
  test event_forwarder::tests::forward_stops_on_permanent_failure ... ok
  test coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree ... ok
  test audit::stream::tests::read_initial_events_returns_last_n ... ok
  test audit::store::tests::legacy_worktree_log_is_removed_after_successful_migration ... ok
  test event_forwarder::tests::forward_reads_events_from_routed_local_store ... ok
  test coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination ... ok
  test audit::store::tests::read_all_merges_and_replays_fallback_events_when_branch_recovers ... ok
  test audit::stream::tests::poll_detects_new_events_from_routed_store ... ok

  test result: ok. 707 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.48s

       Running tests/archive.rs (target/llvm-cov-target/debug/deps/archive-d85ab2e97e5a220b)

  running 3 tests
  test check_task_completion_handles_checkbox_and_enhanced_formats ... ok
  test generate_archive_name_prefixes_with_date ... ok
  test discover_and_copy_specs_and_archive_change ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/audit_mirror.rs (target/llvm-cov-target/debug/deps/audit_mirror-2da1a7817f1b6526)

  running 6 tests
  test audit_mirror_default_local_store_falls_back_without_creating_worktree_log ... ok
  test audit_mirror_disabled_does_not_create_remote_branch ... ok
  test audit_mirror_failures_do_not_break_local_append ... ok
  test local_store_does_not_fall_back_when_internal_branch_exists_without_log_file ... ok
  test audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log ... ok
  test audit_mirror_enabled_pushes_to_configured_branch ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.78s

       Running tests/audit_storage.rs (target/llvm-cov-target/debug/deps/audit_storage-c939f4e7a62b683e)

  running 3 tests
  test filters_events_from_injected_store ... ok
  test reads_events_from_injected_store_without_filesystem_path ... ok
  test memory_store_append_persists_events ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_archive.rs (target/llvm-cov-target/debug/deps/backend_archive-e8f93f1aa014a9fc)

  running 6 tests
  test backend_archive_fails_when_pull_unavailable ... ok
  test backend_archive_with_skip_specs_does_not_copy_specs ... ok
  test backend_archive_fails_when_backend_unavailable_for_mark_archived ... ok
  test backend_archive_creates_backup_before_overwriting ... ok
  test backend_archive_happy_path_produces_committable_state ... ok
  test backend_archive_does_not_mutate_local_module_markdown ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/backend_auth.rs (target/llvm-cov-target/debug/deps/backend_auth-73aa843c771d507a)

  running 13 tests
  test resolve_admin_tokens_deduplicates ... ok
  test resolve_admin_tokens_merges_all_sources ... ok
  test resolve_admin_tokens_skips_empty_config_entries ... ok
  test resolve_token_seed_cli_takes_precedence ... ok
  test resolve_token_seed_falls_back_to_config ... ok
  test resolve_token_seed_returns_none_when_all_empty ... ok
  test write_auth_creates_config_file ... ok
  test write_auth_sets_restrictive_permissions ... ok
  test write_auth_rejects_non_object_backend_server ... ok
  test init_generates_tokens_when_none_exist ... ok
  test init_skips_when_tokens_exist ... ok
  test write_auth_rejects_non_object_root ... ok
  test write_auth_preserves_existing_config ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/backend_auth_service.rs (target/llvm-cov-target/debug/deps/backend_auth_service-75f5b08e8526de06)

  running 1 test
  test init_rejects_non_object_backend_server ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_client_mode.rs (target/llvm-cov-target/debug/deps/backend_client_mode-3a33b78fcc0a64fd)

  running 15 tests
  test allocate_returns_claimed_change ... ok
  test backend_unavailable_detection ... ok
  test backend_change_repo_lists_and_filters ... ok
  test claim_success_returns_holder_info ... ok
  test config_enabled_missing_token_fails_with_clear_message ... ok
  test config_enabled_with_token_resolves ... ok
  test backend_task_repo_missing_returns_zero ... ok
  test allocate_no_work_returns_none ... ok
  test retriable_status_codes ... ok
  test config_disabled_returns_none ... ok
  test claim_conflict_returns_holder_error ... ok
  test backend_task_repo_parses_from_content ... ok
  test pull_writes_artifacts_and_revision ... ok
  test push_success_updates_local_revision ... ok
  test push_stale_revision_gives_actionable_error ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/backend_module_repository.rs (target/llvm-cov-target/debug/deps/backend_module_repository-00199f761d08a3dd)

  running 5 tests
  test backend_module_repository_normalizes_full_name_inputs ... ok
  test backend_module_repository_list_sorts_deterministically ... ok
  test read_module_markdown_falls_back_without_local_file ... ok
  test backend_module_repository_list_sorts_by_id ... ok
  test backend_module_repository_accepts_name_inputs ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/backend_sub_module_support.rs (target/llvm-cov-target/debug/deps/backend_sub_module_support-c560207b3d1d39e6)

  running 9 tests
  test backend_module_repository_list_includes_sub_module_summaries ... ok
  test backend_module_repository_list_sub_modules_for_unknown_module_returns_error ... ok
  test backend_module_repository_get_sub_module_not_found_returns_error ... ok
  test backend_module_repository_get_sub_module_by_composite_id ... ok
  test backend_module_repository_list_sub_modules_returns_sorted_summaries ... ok
  test sqlite_store_legacy_change_has_no_sub_module_id ... ok
  test sqlite_store_list_changes_filters_by_sub_module_id ... ok
  test sqlite_store_sub_module_change_roundtrips_through_artifact_bundle ... ok
  test sqlite_store_persists_sub_module_id_on_change ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/change_repository_lifecycle.rs (target/llvm-cov-target/debug/deps/change_repository_lifecycle-5bb27e86f2cb48b8)

  running 2 tests
  test remote_runtime_ignores_local_change_dirs ... ok
  test filesystem_change_repository_filters_archived ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/change_repository_orchestrate_metadata.rs (target/llvm-cov-target/debug/deps/change_repository_orchestrate_metadata-4871146e8d386ca2)

  running 1 test
  test change_repository_exposes_orchestrate_metadata_from_ito_yaml ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/change_repository_parity.rs (target/llvm-cov-target/debug/deps/change_repository_parity-265bd1abfac130a4)

  running 18 tests
  test backend_list_by_module_normalizes_module_id ... ok
  test backend_resolve_empty_input_returns_not_found ... ok
  test backend_resolve_lifecycle_filter_respected ... ok
  test backend_resolve_numeric_short_form_ambiguous ... ok
  test backend_resolve_numeric_short_form_matches_canonical_id ... ok
  test backend_resolve_module_scoped_slug_not_found ... ok
  test backend_resolve_module_scoped_slug_query ... ok
  test sqlite_get_with_archived_filter_returns_not_found ... ok
  test sqlite_resolve_numeric_short_form_matches_canonical_id ... ok
  test sqlite_resolve_archived_filter_returns_not_found ... ok
  test sqlite_resolve_prefix_match ... ok
  test sqlite_resolve_numeric_short_form_ambiguous ... ok
  test sqlite_resolve_all_filter_finds_active_changes ... ok
  test sqlite_resolve_empty_input_returns_not_found ... ok
  test sqlite_list_archived_filter_returns_empty ... ok
  test sqlite_get_with_all_filter_finds_change ... ok
  test sqlite_list_all_filter_returns_active_changes ... ok
  test sqlite_list_by_module_normalizes_module_id ... ok

  test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/change_target_resolution_parity.rs (target/llvm-cov-target/debug/deps/change_target_resolution_parity-c45debb3af7b60f4)

  running 2 tests
  test sqlite_resolver_honors_archived_lifecycle_like_filesystem ... ok
  test change_target_resolution_matches_across_repository_modes ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/coordination_worktree.rs (target/llvm-cov-target/debug/deps/coordination_worktree-2d9d44961cb599dc)

  running 15 tests
  test symlink_tests::task_repo_missing_tasks_file_returns_zero_through_symlink ... ok
  test symlink_tests::change_repo_list_through_symlink ... ok
  test symlink_tests::module_repo_change_counts_through_symlink ... ok
  test symlink_tests::change_repo_exists_through_symlink ... ok
  test symlink_tests::task_written_through_symlink_lands_in_worktree ... ok
  test symlink_tests::task_repo_has_tasks_through_symlink ... ok
  test symlink_tests::change_written_through_symlink_lands_in_worktree ... ok
  test symlink_tests::module_repo_exists_through_symlink ... ok
  test symlink_tests::module_repo_list_through_symlink ... ok
  test symlink_tests::task_repo_load_tasks_through_symlink ... ok
  test symlink_tests::change_repo_get_through_symlink ... ok
  test symlink_tests::module_repo_list_multiple_through_symlink ... ok
  test symlink_tests::change_repo_list_multiple_through_symlink ... ok
  test symlink_tests::all_repos_consistent_through_symlinks ... ok
  test symlink_tests::module_repo_get_through_symlink ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/create.rs (target/llvm-cov-target/debug/deps/create-bab35743ec362b88)

  running 15 tests
  test create_change_rejects_uppercase_names ... ok
  test create_change_in_sub_module_rejects_missing_parent_module ... ok
  test create_change_in_sub_module_rejects_missing_sub_module_dir ... ok
  test create_module_creates_directory_and_module_md ... ok
  test create_module_writes_description_to_purpose_section ... ok
  test create_change_creates_change_dir_and_updates_module_md ... ok
  test create_module_returns_existing_module_when_name_matches ... ok
  test create_change_allocates_next_number_from_existing_change_dirs ... ok
  test create_change_rewrites_module_changes_in_ascending_change_id_order ... ok
  test create_change_in_sub_module_writes_checklist_to_sub_module_md ... ok
  test create_change_in_sub_module_uses_composite_id_format ... ok
  test create_change_in_sub_module_checklist_is_sorted_ascending ... ok
  test allocation_state_sub_module_keys_sort_after_parent ... ok
  test create_change_in_sub_module_allocates_independent_sequence ... ok
  test create_change_writes_allocation_modules_in_ascending_id_order ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/distribution.rs (target/llvm-cov-target/debug/deps/distribution-f76c17371c46588e)

  running 11 tests
  test github_manifests_includes_skills_and_commands ... ok
  test opencode_manifests_includes_plugin_and_skills ... ok
  test claude_manifests_includes_hooks_and_skills ... ok
  test codex_manifests_includes_bootstrap_and_skills ... ok
  test install_manifests_make_tmux_skill_scripts_executable ... ok
  test install_manifests_renders_worktree_skill_with_context ... ok
  test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
  test install_manifests_writes_files_to_disk ... ok
  test install_manifests_renders_worktree_skill_enabled ... ok
  test install_manifests_creates_parent_directories ... ok
  test all_manifests_use_embedded_assets ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

       Running tests/event_forwarding.rs (target/llvm-cov-target/debug/deps/event_forwarding-29c5bbb63c00f4df)

  running 6 tests
  test forward_result_reports_diagnostics ... ok
  test full_forwarding_workflow ... ok
  test batch_boundaries_preserved ... ok
  test permanent_failure_stops_forwarding ... ok
  test transient_failure_retried_then_succeeds ... ok
  test incremental_forwarding ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

       Running tests/grep_scopes.rs (target/llvm-cov-target/debug/deps/grep_scopes-4c9860b357972adf)

  running 4 tests
  test grep_scope_change_only_searches_one_change ... ok
  test grep_scope_module_searches_all_changes_in_module ... ok
  test grep_respects_limit_across_scopes ... ok
  test grep_scope_all_searches_all_changes ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/harness_context.rs (target/llvm-cov-target/debug/deps/harness_context-10e2fc1b6bfeebc0)

  running 6 tests
  test infer_context_from_cwd_infers_change_from_path ... ok
  test infer_context_from_cwd_infers_module_from_ito_modules_path ... ok
  test infer_context_from_cwd_returns_no_target_when_inconclusive ... ok
  test infer_context_from_cwd_prefers_path_over_git_branch ... ok
  test infer_context_from_cwd_infers_change_from_git_branch ... ok
  test infer_context_from_cwd_infers_module_from_git_branch ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

       Running tests/harness_opencode.rs (target/llvm-cov-target/debug/deps/harness_opencode-ef05cc8eade42d49)

  running 8 tests
  test codex_harness_errors_when_codex_missing ... ok
  test claude_harness_errors_when_claude_missing ... ok
  test copilot_harness_errors_when_copilot_missing ... ok
  test opencode_harness_errors_when_opencode_missing ... ok
  test opencode_harness_runs_opencode_binary_and_returns_outputs ... ok
  test claude_harness_passes_model_and_allow_all_flags ... ok
  test codex_harness_passes_model_and_allow_all_flags ... ok
  test github_copilot_harness_passes_model_and_allow_all_flags ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.03s

       Running tests/harness_streaming.rs (target/llvm-cov-target/debug/deps/harness_streaming-614848e32647e1a7)

  running 2 tests
  test no_timeout_when_process_exits_normally ... ok
  test inactivity_timeout_kills_stalled_process ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.25s

       Running tests/harness_stub.rs (target/llvm-cov-target/debug/deps/harness_stub-eec1b254e10234dd)

  running 6 tests
  test stub_harness_default_returns_complete_promise ... ok
  test stub_harness_errors_on_empty_steps ... ok
  test stub_harness_from_json_path_runs_steps_and_repeats_last ... ok
  test stub_step_defaults_match_json_schema ... ok
  test stub_harness_errors_on_missing_and_invalid_json ... ok
  test stub_harness_from_env_prefers_env_over_default ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/import.rs (target/llvm-cov-target/debug/deps/import-a16ff58824fb0e02)

  running 10 tests
  test pushes_when_remote_active_bundle_differs ... ok
  test rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches ... ok
  test dry_run_previews_without_importing ... ok
  test active_local_change_fails_when_backend_only_has_archived_copy ... ok
  test imports_active_and_archived_changes_with_lifecycle_fidelity ... ok
  test skips_already_imported_active_change_when_remote_bundle_matches ... ok
  test archived_directory_with_empty_canonical_change_id_is_ignored ... ok
  test dry_run_uses_preview_logic_without_mutating_backend ... ok
  test ignores_unrecognized_archive_directories_during_discovery ... ok
  test import_summary_records_failures_without_aborting_remaining_changes ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/io.rs (target/llvm-cov-target/debug/deps/io-ba2899db3d9b4aa1)

  running 3 tests
  test read_to_string_or_default_returns_empty_for_missing_file ... ok
  test read_to_string_optional_returns_none_for_missing_file ... ok
  test write_atomic_std_creates_parent_and_replaces_contents ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/orchestrate_run_state.rs (target/llvm-cov-target/debug/deps/orchestrate_run_state-e7cb4c104dfa42b0)

  running 7 tests
  test orchestrate_max_parallel_aliases_resolve ... ok
  test orchestrate_dependency_cycle_is_rejected ... ok
  test orchestrate_resume_skips_terminal_gates ... ok
  test orchestrate_run_id_generation_matches_expected_format ... ok
  test orchestrate_run_state_creates_expected_layout ... ok
  test orchestrate_change_state_is_written_and_readable ... ok
  test orchestrate_event_log_appends_without_truncation ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/planning_init.rs (target/llvm-cov-target/debug/deps/planning_init-9e1b0bc6cb92e511)

  running 3 tests
  test read_planning_status_returns_error_for_missing_roadmap ... ok
  test read_planning_status_returns_contents_for_existing_roadmap ... ok
  test init_planning_structure_writes_files ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/ralph.rs (target/llvm-cov-target/debug/deps/ralph-ddb0540487cc94d0)

  running 30 tests
  test run_ralph_add_and_clear_context_paths ... ok
  test run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
  test run_ralph_errors_when_max_iterations_is_zero ... ok
  test run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes ... ok
  test run_ralph_opencode_counts_git_changes_when_in_repo ... ignored, Flaky in pre-commit: counts real uncommitted changes instead of test fixture
  test run_ralph_continue_ready_errors_when_targeting_change_or_module ... ok
  test run_ralph_gives_up_after_max_retriable_retries ... ok
  test run_ralph_fails_after_error_threshold ... ok
  test run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight ... ok
  test run_ralph_continues_after_harness_failure_by_default ... ok
  test run_ralph_non_retriable_exit_still_counts_against_threshold ... ok
  test run_ralph_module_resolves_single_change ... ok
  test run_ralph_retries_retriable_exit_code_with_exit_on_error ... ok
  test run_ralph_prompt_includes_task_context_and_guidance ... ok
  test run_ralph_returns_error_on_harness_failure ... ok
  test run_ralph_resets_retriable_counter_on_success ... ok
  test run_ralph_status_path_works_with_no_state ... ok
  test run_ralph_continue_ready_reorients_when_repo_state_shifts ... ok
  test run_ralph_retries_retriable_exit_code_without_counting_against_threshold ... ok
  test state_helpers_append_and_clear_context ... ok
  test run_ralph_skip_validation_exits_immediately ... ok
  test run_ralph_module_multiple_changes_errors_when_non_interactive ... ok
  test run_ralph_continue_ready_processes_all_eligible_changes_across_repo ... ok
  test run_ralph_continue_module_processes_all_ready_changes ... ok
  test run_ralph_continue_ready_accumulates_failures_after_processing_remaining_changes ... ok
  test run_ralph_completion_promise_trims_whitespace ... ok
  test run_ralph_loop_writes_state_and_honors_min_iterations ... ok
  test run_ralph_continues_when_completion_validation_fails ... ok
  test run_ralph_worktree_disabled_uses_fallback_cwd ... ok
  test run_ralph_worktree_enabled_state_written_to_effective_ito ... ok

  test result: ok. 29 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.16s

       Running tests/repo_index.rs (target/llvm-cov-target/debug/deps/repo_index-7d14f33d5ad32843)

  running 1 test
  test repo_index_loads_and_excludes_archive_change_dir ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/repo_integrity.rs (target/llvm-cov-target/debug/deps/repo_integrity-68a582adea4f2bc0)

  running 3 tests
  test invalid_change_dir_names_are_reported ... ok
  test change_referring_to_missing_module_is_an_error ... ok
  test duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/repo_paths.rs (target/llvm-cov-target/debug/deps/repo_paths-adcf287b8661b0f5)

  running 11 tests
  test coordination_worktree_path_uses_explicit_worktree_path_when_set ... ok
  test coordination_worktree_path_correct_structure_with_home_fallback ... ok
  test coordination_worktree_path_falls_back_to_local_share_when_xdg_unset ... ok
  test coordination_worktree_path_correct_structure_with_xdg ... ok
  test coordination_worktree_path_ignores_xdg_when_explicit_path_set ... ok
  test coordination_worktree_path_last_resort_uses_ito_path ... ok
  test coordination_worktree_path_uses_xdg_data_home_when_set ... ok
  test resolve_worktree_paths_respects_bare_control_siblings_strategy ... ok
  Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpvTs2kG/
  test resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir ... ok
  test resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable ... ok
  Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpZ8F4IB/.git/
  test resolve_env_from_cwd_prefers_git_toplevel ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

       Running tests/repository_runtime.rs (target/llvm-cov-target/debug/deps/repository_runtime-b669f0418c003560)

  running 6 tests
  test remote_runtime_uses_remote_factory ... ok
  test sqlite_mode_requires_db_path ... ok
  test filesystem_runtime_builds_repository_set ... ok
  test sqlite_runtime_builds_repository_set ... ok
  test repository_modes_return_consistent_change_names ... ok
  test resolve_target_parity_between_filesystem_and_sqlite ... ok

  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/repository_runtime_config_validation.rs (target/llvm-cov-target/debug/deps/repository_runtime_config_validation-5b7d209da2247079)

  running 1 test
  test invalid_repository_mode_fails_fast ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/show.rs (target/llvm-cov-target/debug/deps/show-1434f93abb11fee3)

  running 17 tests
  test parse_requirement_block_extracts_requirement_id ... ok
  test parse_change_show_json_emits_deltas_with_operations ... ok
  test parse_contract_refs_preserves_commas_inside_identifiers ... ok
  test parse_delta_spec_requirement_id_is_extracted ... ok
  test parse_requirement_block_multiple_requirements_with_ids ... ok
  test parse_requirement_block_requirement_id_absent_gives_none ... ok
  test parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets ... ok
  test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
  test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
  test read_module_markdown_returns_error_for_nonexistent_module ... ok
  test read_module_markdown_returns_empty_for_missing_module_md ... ok
  test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
  test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
  test read_module_markdown_returns_contents_for_existing_module ... ok
  test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
  test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
  test read_change_delta_spec_files_lists_specs_sorted ... ok

  test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/spec_repository_backends.rs (target/llvm-cov-target/debug/deps/spec_repository_backends-0f5522cf2157aa95)

  running 2 tests
  test remote_runtime_exposes_spec_repository_without_local_specs ... ok
  test filesystem_runtime_exposes_promoted_specs ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/spec_show_repository.rs (target/llvm-cov-target/debug/deps/spec_show_repository-a3fd34a54d1ca46e)

  running 3 tests
  test read_spec_markdown_from_repository_reads_remote_spec ... ok
  test bundle_specs_show_json_from_repository_sorts_ids ... ok
  test bundle_specs_markdown_from_repository_adds_metadata_comments ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/sqlite_archive_mirror.rs (target/llvm-cov-target/debug/deps/sqlite_archive_mirror-21bb49e25d50ce1a)

  running 1 test
  test sqlite_archive_promotes_specs_and_marks_change_archived ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/sqlite_task_mutations.rs (target/llvm-cov-target/debug/deps/sqlite_task_mutations-bed94e8d68691e36)

  running 3 tests
  test sqlite_task_mutation_service_returns_not_found_for_missing_tasks ... ok
  test sqlite_task_mutation_service_updates_existing_markdown ... ok
  test sqlite_task_mutation_service_initializes_missing_tasks ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/stats.rs (target/llvm-cov-target/debug/deps/stats-13c02e6b3173a8c6)

  running 2 tests
  test compute_command_stats_counts_command_end_events ... ok
  test collect_jsonl_files_finds_nested_jsonl_files ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/task_repository_summary.rs (target/llvm-cov-target/debug/deps/task_repository_summary-2323776095289583)

  running 1 test
  test repository_status_builds_summary_and_next_task ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks_api.rs (target/llvm-cov-target/debug/deps/tasks_api-bdc52888d6ee06b8)

  running 15 tests
  test init_tasks_creates_file_when_missing ... ok
  test list_ready_tasks_across_changes_handles_empty_repo ... ok
  test init_tasks_returns_true_when_file_already_exists ... ok
  test tasks_api_rejects_non_tasks_tracking_validator_for_schema_tracking ... ok
  test shelve_task_rejects_shelving_complete_task ... ok
  test get_next_task_returns_none_when_all_tasks_complete ... ok
  test complete_task_accepts_note_parameter ... ok
  test add_task_appends_new_task_with_next_id ... ok
  test start_task_rejects_starting_shelved_task_directly ... ok
  test add_task_creates_wave_if_not_exists ... ok
  test shelve_task_accepts_reason_parameter ... ok
  test get_next_task_returns_first_ready_task_for_enhanced_format ... ok
  test shelve_and_unshelve_task_round_trip_for_enhanced_format ... ok
  test tasks_api_operates_on_schema_apply_tracks_file ... ok
  test start_and_complete_task_enforced_by_dependencies_for_enhanced_format ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/tasks_checkbox_format.rs (target/llvm-cov-target/debug/deps/tasks_checkbox_format-1c6ee2aea31a823f)

  running 3 tests
  test checkbox_tasks_do_not_support_shelving ... ok
  test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback ... ok
  test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/tasks_orchestration.rs (target/llvm-cov-target/debug/deps/tasks_orchestration-40e739e87a79e872)

  running 26 tests
  test init_tasks_rejects_invalid_change_id ... ok
  test get_task_status_returns_error_when_file_missing ... ok
  test init_tasks_creates_file_when_missing ... ok
  test add_task_rejects_checkbox_format ... ok
  test get_next_task_returns_none_when_all_complete ... ok
  test start_task_errors_with_parse_errors ... ok
  test start_task_rejects_already_complete ... ok
  test complete_task_handles_checkbox_format ... ok
  test get_next_task_returns_current_in_progress_for_checkbox ... ok
  test add_task_assigns_next_id_in_wave ... ok
  test shelve_task_rejects_checkbox_format ... ok
  test complete_task_errors_with_parse_errors ... ok
  test shelve_task_rejects_complete_task ... ok
  test get_task_status_returns_diagnostics_for_malformed_file ... ok
  test init_tasks_does_not_overwrite_existing_file ... ok
  test add_task_errors_with_parse_errors ... ok
  test shelve_task_errors_with_parse_errors ... ok
  test complete_task_handles_enhanced_format ... ok
  test add_task_defaults_to_wave_1 ... ok
  test start_task_validates_task_is_ready ... ok
  test start_task_rejects_shelved_task ... ok
  test get_next_task_returns_first_ready_for_enhanced ... ok
  test add_task_creates_wave_when_missing ... ok
  test unshelve_task_errors_with_parse_errors ... ok
  test unshelve_task_rejects_not_shelved ... ok
  test unshelve_task_transitions_to_pending ... ok

  test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

       Running tests/templates_apply_instructions.rs (target/llvm-cov-target/debug/deps/templates_apply_instructions-50ba6c6a28999f9b)

  running 2 tests
  test compute_apply_instructions_ignores_optional_artifacts_when_apply_requires_is_omitted ... ok
  test compute_apply_instructions_reports_blocked_states_and_progress ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/templates_change_status.rs (target/llvm-cov-target/debug/deps/templates_change_status-ec1120340c1829f6)

  running 3 tests
  test compute_change_status_rejects_invalid_change_name ... ok
  test compute_change_status_treats_missing_optional_artifacts_as_non_blocking ... ok
  test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/templates_review_context.rs (target/llvm-cov-target/debug/deps/templates_review_context-22d2d577d92db76d)

  running 1 test
  test compute_review_context_collects_artifacts_validation_tasks_and_specs ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/templates_schema_resolution.rs (target/llvm-cov-target/debug/deps/templates_schema_resolution-5bb554580910cabb)

  running 9 tests
  test resolve_schema_rejects_absolute_and_backslash_names ... ok
  test resolve_schema_rejects_path_traversal_name ... ok
  test resolve_schema_uses_embedded_when_no_overrides_exist ... ok
  test resolve_instructions_reads_embedded_templates ... ok
  test resolve_templates_rejects_traversal_template_path ... ok
  test resolve_instructions_exposes_enhanced_spec_driven_templates ... ok
  test resolve_instructions_rejects_traversal_template_path ... ok
  test resolve_schema_prefers_project_over_user_override ... ok
  test export_embedded_schemas_writes_then_skips_without_force ... ok

  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/templates_schemas_listing.rs (target/llvm-cov-target/debug/deps/templates_schemas_listing-3d51a88a07065e78)

  running 10 tests
  test list_schemas_detail_entries_have_artifacts ... ok
  test list_schemas_detail_json_round_trips ... ok
  test list_schemas_detail_recommended_default_is_spec_driven ... ok
  test list_schemas_detail_spec_driven_has_expected_artifacts ... ok
  test list_schemas_detail_entries_have_descriptions ... ok
  test list_schemas_detail_all_sources_are_embedded ... ok
  test built_in_minimalist_and_event_driven_spec_templates_use_delta_shape ... ok
  test list_schemas_detail_returns_all_embedded_schemas ... ok
  test list_schemas_detail_is_sorted ... ok
  test built_in_schemas_expose_domain_discovery_template_hook ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/templates_user_guidance.rs (target/llvm-cov-target/debug/deps/templates_user_guidance-debc464456b2d117)

  running 7 tests
  test load_user_guidance_for_artifact_rejects_path_traversal_ids ... ok
  test load_user_guidance_strips_ito_internal_comment_block ... ok
  test load_user_guidance_for_artifact_strips_managed_header_block ... ok
  test load_user_guidance_for_artifact_reads_scoped_file ... ok
  test load_user_guidance_strips_managed_header_block ... ok
  test load_composed_user_guidance_combines_scoped_and_shared ... ok
  test load_user_guidance_prefers_user_prompts_guidance_file ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/traceability_e2e.rs (target/llvm-cov-target/debug/deps/traceability_e2e-2edf5959c1f0ccab)

  running 15 tests
  test legacy_checkbox_change_validate_passes_without_traceability_checks ... ok
  test legacy_checkbox_change_trace_output_is_unavailable ... ok
  test traced_change_all_covered_trace_output_is_ready ... ok
  test traced_change_uncovered_req_trace_output_shows_uncovered ... ok
  test shelved_task_leaves_requirement_uncovered ... ok
  test traced_change_uncovered_req_is_warning_in_non_strict ... ok
  test shelved_task_uncovered_req_is_warning_in_validate ... ok
  test partial_ids_trace_output_is_invalid ... ok
  test traced_change_unresolved_ref_is_error_in_validate ... ok
  test traced_change_all_covered_validate_passes ... ok
  test traced_change_uncovered_req_is_error_in_strict ... ok
  test traced_change_unresolved_ref_trace_output_shows_unresolved ... ok
  test partial_ids_validate_reports_error ... ok
  test duplicate_requirement_ids_produce_error_in_validate ... ok
  test duplicate_requirement_ids_trace_output_has_diagnostics ... ok

  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/validate.rs (target/llvm-cov-target/debug/deps/validate-38970caa58ddc3f1)

  running 23 tests
  test validate_module_reports_missing_scope_and_short_purpose ... ok
  test validate_spec_markdown_reports_missing_purpose_and_requirements ... ok
  test validate_change_uses_validation_yaml_delta_specs_validator_when_configured ... ok
  test validate_change_requires_shall_or_must_in_requirement_text ... ok
  test validate_change_skips_optional_validator_when_artifact_is_missing ... ok
  test validate_spec_markdown_strict_treats_warnings_as_invalid ... ok
  test validate_change_requires_at_least_one_delta ... ok
  test validate_module_warns_when_sub_module_purpose_too_short ... ok
  test validate_tasks_file_returns_error_for_missing_file ... ok
  test validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas ... ok
  test validate_tasks_file_returns_diagnostics_for_malformed_content ... ok
  test validate_tasks_file_issues_cite_tasks_tracking_validator_id ... ok
  test validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas ... ok
  test validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking ... ok
  test validate_tasks_file_returns_empty_for_valid_tasks ... ok
  test validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas ... ok
  test validate_module_passes_when_sub_modules_have_valid_module_md ... ok
  test validate_module_errors_when_sub_module_has_invalid_naming ... ok
  test validate_change_uses_apply_tracks_for_legacy_delta_schemas ... ok
  test validate_module_errors_when_sub_module_missing_module_md ... ok
  test validate_change_validates_apply_tracks_file_when_configured ... ok
  test validate_tasks_file_uses_apply_tracks_when_set ... ok
  test empty_tracking_file_is_warning_in_non_strict_and_error_in_strict ... ok

  test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/validate_delta_rules.rs (target/llvm-cov-target/debug/deps/validate_delta_rules-f01848586be83ad7)

  running 16 tests
  test contract_refs_rule_rejects_unknown_schemes ... ok
  test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
  test domain_documentation_consistency_rule_passes_for_matching_context_docs ... ok
  test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
  test ubiquitous_language_consistency_rule_warns_for_rejected_aliases ... ok
  test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
  test scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags ... ok
  test ubiquitous_language_consistency_rule_passes_when_aliases_are_absent ... ok
  test scenario_grammar_rule_warns_on_excessive_step_count ... ok
  test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
  test ubiquitous_language_consistency_rule_uses_term_boundaries ... ok
  test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
  test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
  test domain_documentation_consistency_rule_warns_for_conflicting_context_docs ... ok
  test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
  test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

  test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

       Running tests/validate_domain_discovery_rules.rs (target/llvm-cov-target/debug/deps/validate_domain_discovery_rules-a89046182b3ce564)

  running 5 tests
  test context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing ... ok
  test context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation ... ok
  test context_boundary_consistency_rule_is_silent_for_single_context_discovery ... ok
  test domain_rules_are_silent_without_domain_discovery_handoff ... ok
  test domain_rules_can_run_from_artifact_rules_for_event_driven_schemas ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/validate_rules_extension.rs (target/llvm-cov-target/debug/deps/validate_rules_extension-4dc5542fb1f80517)

  running 2 tests
  test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
  test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/validate_tracking_rules.rs (target/llvm-cov-target/debug/deps/validate_tracking_rules-f895e7845c382f6c)

  running 7 tests
  test task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable ... ok
  test task_quality_rule_respects_warning_floor_without_promoting_advisories ... ok
  test task_quality_rule_treats_gradle_files_as_implementation_work ... ok
  test task_quality_rule_errors_on_unknown_requirement_ids ... ok
  test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
  test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
  test task_quality_rule_errors_on_missing_status ... ok

  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

       Running tests/worktree_ensure_e2e.rs (target/llvm-cov-target/debug/deps/worktree_ensure_e2e-7fe214287cd00862)

  running 3 tests
  test ensure_worktree_disabled_returns_cwd ... ok
  test ensure_worktree_creates_and_initializes_with_include_files ... ok
  test ensure_worktree_with_setup_script ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_domain-8da9fd226a6b9493)

  running 122 tests
  test audit::context::tests::resolve_harness_session_id_returns_none_without_env ... ok
  test audit::event::tests::builder_returns_none_without_required_fields ... ok
  test audit::event::tests::entity_type_display ... ok
  test audit::event::tests::entity_type_as_str_matches_serde ... ok
  test audit::event::tests::builder_with_meta ... ok
  test audit::event::tests::actor_round_trip ... ok
  test audit::event::tests::builder_produces_valid_event ... ok
  test audit::event::tests::actor_serializes_to_lowercase ... ok
  test audit::event::tests::count_serializes_when_greater_than_one ... ok
  test audit::event::tests::audit_event_serializes_to_single_line ... ok
  test audit::event::tests::audit_event_round_trip_serialization ... ok
  test audit::event::tests::entity_type_round_trip ... ok
  test audit::event::tests::entity_type_serializes_to_lowercase ... ok
  test audit::event::tests::event_context_round_trip ... ok
  test audit::event::tests::missing_count_deserializes_as_one ... ok
  test audit::event::tests::optional_fields_omitted_when_none ... ok
  test audit::event::tests::schema_version_is_one ... ok
  test audit::materialize::tests::last_event_wins ... ok
  test audit::materialize::tests::multiple_entities_tracked_independently ... ok
  test audit::context::tests::resolve_session_id_generates_uuid ... ok
  test audit::materialize::tests::empty_events_produce_empty_state ... ok
  test audit::materialize::tests::archive_event_without_to_uses_sentinel ... ok
  test audit::materialize::tests::global_entities_have_no_scope ... ok
  test audit::materialize::tests::reconciled_event_without_to_tombstones_state ... ok
  test audit::materialize::tests::reconciled_events_update_state ... ok
  test audit::materialize::tests::single_create_event ... ok
  test audit::materialize::tests::status_change_updates_state ... ok
  test audit::reconcile::tests::detect_extra_in_log ... ok
  test audit::reconcile::tests::detect_diverged_status ... ok
  test audit::reconcile::tests::detect_missing_entity_in_log ... ok
  test audit::reconcile::tests::compensating_events_use_scope_from_drift_key ... ok
  test audit::reconcile::tests::display_drift_items ... ok
  test audit::reconcile::tests::generate_compensating_events_for_diverged ... ok
  test audit::context::tests::resolve_session_id_is_stable_across_calls ... ok
  test audit::reconcile::tests::generate_compensating_events_for_missing ... ok
  test audit::reconcile::tests::multiple_drift_types_detected ... ok
  test audit::writer::tests::noop_writer_is_object_safe ... ok
  test audit::reconcile::tests::no_drift_when_states_match ... ok
  test audit::writer::tests::noop_writer_is_send_sync ... ok
  test audit::reconcile::tests::generate_compensating_events_for_extra ... ok
  test audit::writer::tests::noop_writer_returns_ok ... ok
  test audit::writer::tests::trait_is_object_safe_for_dyn_dispatch ... ok
  test backend::tests::archive_result_roundtrip ... ok
  test backend::tests::backend_error_display_lease_conflict ... ok
  test backend::tests::backend_error_display_not_found ... ok
  test backend::tests::artifact_bundle_roundtrip ... ok
  test backend::tests::backend_error_display_revision_conflict ... ok
  test backend::tests::backend_error_display_other ... ok
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
  test errors::tests::ambiguous_target_joins_candidates_in_display_message ... ok
  test errors::tests::io_constructor_preserves_context_and_source ... ok
  test errors::tests::not_found_constructor_formats_display_message ... ok
  test modules::tests::test_module_creation ... ok
  test modules::tests::test_module_summary ... ok
  test modules::tests::test_module_summary_with_sub_modules ... ok
  test modules::tests::test_module_with_sub_modules ... ok
  test modules::tests::test_sub_module_creation ... ok
  test modules::tests::test_sub_module_summary_creation ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_accepts_valid_formats ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_handles_large_numbers ... ok
  test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_rejects_invalid_formats ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_edge_case_single_digit_with_many_dots ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_extracts_id_and_rest ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_colon_suffix ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_dot_suffix ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_leading_whitespace ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_tab_separator ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_multiple_spaces ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_unicode_in_task_name ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_preserves_trailing_whitespace_in_rest ... ok
  test tasks::checkbox::checkbox_tests::split_checkbox_task_label_returns_none_for_invalid_inputs ... ok
  test tasks::compute::tests::checkbox_mode_returns_pending_sorted_and_no_blocked ... ok
  test tasks::compute::tests::enhanced_backcompat_blocks_later_waves_and_checkpoints_until_first_incomplete_wave_done ... ok
  test discovery::tests::list_changes_skips_archive_dir ... ok
  test tasks::compute::tests::enhanced_ready_and_blocked_lists_are_sorted_by_task_id ... ok
  test tasks::compute::tests::enhanced_task_dependencies_produce_missing_crosswave_and_not_complete_blockers ... ok
  test tasks::compute::tests::enhanced_wave_dependency_blocks_by_wave_and_unblocks_when_complete ... ok
  test discovery::tests::list_module_ids_extracts_numeric_prefixes ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_empty_graph ... ok
  test discovery::tests::list_modules_only_returns_directories ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_simple_two_node_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_long_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_with_numeric_node_names ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_cycle_in_complex_graph ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_diamond_pattern_without_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_self_loop ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_acyclic_graph ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_special_characters_in_node_names ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_detects_three_node_cycle ... ok
  test tasks::cycle::cycle_tests::find_cycle_path_handles_multiple_cycles_returns_one ... ok
  test tasks::relational::relational_tests::validate_relational_detects_self_referencing_task ... ok
  test tasks::relational::relational_tests::validate_relational_marks_errors_as_error_level ... ok
  test tasks::relational::relational_tests::validate_relational_detects_missing_task_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_allows_shelved_task_depending_on_shelved_task ... ok
  test tasks::relational::relational_tests::validate_relational_detects_cross_wave_task_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_detects_duplicate_task_ids ... ok
  test tasks::relational::relational_tests::validate_relational_accepts_valid_dependency_graph ... ok
  test tasks::relational::relational_tests::validate_relational_handles_tasks_without_wave ... ok
  test tasks::relational::relational_tests::validate_relational_ignores_empty_and_checkpoint_dependencies ... ok
  test tasks::relational::relational_tests::validate_relational_detects_wave_dependency_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_detects_task_dependency_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_detects_dependency_on_shelved_task ... ok
  test tasks::relational::relational_tests::validate_relational_detects_three_node_task_cycle ... ok
  test tasks::relational::relational_tests::validate_relational_multiple_errors_for_same_task ... ok
  test tasks::relational::relational_tests::validate_relational_reports_line_numbers ... ok
  test tasks::relational::relational_tests::validate_relational_with_complex_valid_graph ... ok
  test audit::context::tests::resolve_user_identity_returns_at_prefixed_string ... ok
  test audit::context::tests::resolve_git_context_does_not_panic ... ok
  test audit::context::tests::resolve_context_populates_session_id ... ok

  test result: ok. 122 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

       Running tests/planning.rs (target/llvm-cov-target/debug/deps/planning-ead6f06c12781bf8)

  running 1 test
  test roadmap_parsing_extracts_current_progress_and_phases ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/schema_roundtrip.rs (target/llvm-cov-target/debug/deps/schema_roundtrip-cf6e9bea362a3ea4)

  running 3 tests
  test workflow_plan_json_roundtrip ... ok
  test workflow_execution_json_roundtrip ... ok
  test workflow_yaml_roundtrip ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/schema_validation.rs (target/llvm-cov-target/debug/deps/schema_validation-aedc12f021dad7e4)

  running 12 tests
  test execution_validate_rejects_invalid_fields_and_accepts_valid ... ok
  test plan_validate_rejects_empty_prompt_content ... ok
  test execution_validate_rejects_out_of_bounds_wave_index ... ok
  test task_definition_validate_accepts_optional_fields ... ok
  test task_definition_validate_rejects_invalid_fields ... ok
  test plan_validate_rejects_other_invalid_fields ... ok
  test task_execution_validate_rejects_empty_optional_strings ... ok
  test workflow_definition_validate_accepts_minimal_valid ... ok
  test workflow_definition_validate_rejects_duplicate_wave_ids ... ok
  test workflow_definition_validate_rejects_requires_and_context_files_empty_entries ... ok
  test wave_definition_validate_rejects_invalid_shapes ... ok
  test workflow_definition_validate_rejects_empty_fields ... ok

  test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/tasks.rs (target/llvm-cov-target/debug/deps/tasks-7d623ea1096366d9)

  running 2 tests
  test update_enhanced_task_status_inserts_or_replaces_status_line ... ok
  test enhanced_template_parses_and_has_checkpoint_warning ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/tasks_parsing.rs (target/llvm-cov-target/debug/deps/tasks_parsing-446b482b7a416027)

  running 32 tests
  test detect_tasks_format_enhanced_vs_checkbox ... ok
  test parse_checkbox_tasks_accepts_right_arrow_in_progress_marker ... ok
  test parse_checkbox_tasks_handles_empty_lines_and_non_checkbox_content ... ok
  test parse_checkbox_tasks_assigns_sequential_ids_when_not_explicit ... ok
  test parse_checkbox_tasks_supports_dash_and_star ... ok
  test parse_checkbox_tasks_handles_mixed_explicit_and_implicit_ids ... ok
  test parse_checkbox_tasks_uppercase_x_marks_complete ... ok
  test parse_checkbox_tasks_preserves_explicit_ids ... ok
  test parse_enhanced_tasks_extracts_requirements_field ... ok
  test tasks_path_checked_rejects_traversal_like_change_ids ... ok
  test tasks_path_uses_safe_fallback_for_invalid_change_id ... ok
  test parse_enhanced_tasks_parses_fields_and_action_block ... ok
  test parse_enhanced_tasks_accepts_all_prior_tasks_dependency_shorthand ... ok
  test update_checkbox_task_status_by_explicit_id ... ok
  test update_checkbox_task_status_preserves_bullet_style ... ok
  test enhanced_tasks_diagnostics_cover_common_errors ... ok
  test update_checkbox_task_status_sets_marker_and_preserves_text ... ok
  test parse_enhanced_tasks_requirements_single_entry ... ok
  test update_enhanced_task_status_preserves_requirements_line ... ok
  test update_enhanced_task_status_inserts_missing_fields ... ok
  test update_enhanced_task_status_preserves_existing_fields ... ok
  test parse_enhanced_tasks_handles_wave_with_comma_in_title ... ok
  test parse_enhanced_tasks_handles_multiline_action ... ok
  test parse_enhanced_tasks_handles_multiple_files ... ok
  test parse_enhanced_tasks_requirements_absent_gives_empty_vec ... ok
  test parse_enhanced_tasks_handles_empty_dependencies_field ... ok
  test parse_enhanced_tasks_handles_task_without_optional_prefix ... ok
  test parse_enhanced_tasks_requirements_not_carried_across_tasks ... ok
  test parse_enhanced_tasks_progress_counts_all_statuses ... ok
  test enhanced_tasks_cycles_and_shelved_deps_are_reported ... ok
  test parse_enhanced_tasks_accepts_wave_heading_titles ... ok
  test enhanced_tasks_wave_gating_blocks_later_waves ... ok

  test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/tasks_parsing_additional.rs (target/llvm-cov-target/debug/deps/tasks_parsing_additional-4284a6db1841f083)

  running 28 tests
  test checkbox_format_handles_special_characters_in_task_names ... ok
  test checkbox_format_handles_very_long_task_names ... ok
  test checkbox_format_handles_empty_task_text ... ok
  test checkbox_format_ignores_incomplete_checkbox_patterns ... ok
  test checkbox_format_handles_newlines_in_adjacent_lines ... ok
  test checkbox_format_progress_info_counts_correctly ... ok
  test parse_empty_file_returns_empty_result ... ok
  test parse_file_with_only_non_task_content ... ok
  test parse_file_with_only_whitespace ... ok
  test tasks_path_checked_accepts_valid_change_ids ... ok
  test tasks_path_checked_rejects_empty_change_id ... ok
  test tasks_path_checked_rejects_very_long_change_ids ... ok
  test enhanced_format_handles_checkpoints ... ok
  test enhanced_format_handles_task_without_wave ... ok
  test progress_info_calculates_remaining_correctly ... ok
  test enhanced_format_handles_very_long_file_paths ... ok
  test enhanced_format_handles_very_large_wave_numbers ... ok
  test enhanced_format_handles_multiline_action_with_code ... ok
  test enhanced_format_handles_empty_action_block ... ok
  test enhanced_format_handles_uppercase_x_in_complete_marker ... ok
  test enhanced_format_validates_date_format_strictly ... ok
  test enhanced_format_handles_status_marker_mismatch ... ok
  test enhanced_format_handles_multiple_files_with_spaces ... ok
  test wave_dependencies_detect_forward_references ... ok
  test enhanced_format_handles_duplicate_wave_numbers ... ok
  test enhanced_format_validates_missing_required_fields ... ok
  test wave_dependencies_handle_various_formats ... ok
  test enhanced_format_handles_complex_dependency_chains ... ok

  test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

       Running tests/tasks_quality_fields.rs (target/llvm-cov-target/debug/deps/tasks_quality_fields-d2e9ed9ce9aab6c1)

  running 2 tests
  test quality_fields_allow_missing_optional_metadata ... ok
  test quality_fields_round_trip_when_present ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/tasks_update.rs (target/llvm-cov-target/debug/deps/tasks_update-7d37a760f3dcc25d)

  running 19 tests
  test update_checkbox_task_status_handles_mixed_explicit_and_implicit_ids ... ok
  test update_checkbox_task_status_matches_explicit_ids_over_index ... ok
  test update_checkbox_task_status_handles_unicode_in_task_text ... ok
  test update_checkbox_task_status_handles_various_markers ... ok
  test update_checkbox_task_status_preserves_bullet_style ... ok
  test update_checkbox_task_status_errors_for_invalid_or_missing_task_id ... ok
  test update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting ... ok
  test update_checkbox_task_status_rejects_shelving ... ok
  test update_checkbox_task_status_with_id_suffix_colon ... ok
  test update_checkbox_task_status_with_id_suffix_dot ... ok
  test update_enhanced_task_status_inserts_missing_fields ... ok
  test update_enhanced_task_status_handles_task_prefix_optional ... ok
  test update_enhanced_task_status_only_updates_specified_task ... ok
  test update_enhanced_task_status_preserves_other_fields ... ok
  test update_enhanced_task_status_handles_in_progress ... ok
  test update_enhanced_task_status_handles_complex_task_ids ... ok
  test update_enhanced_task_status_updates_status_and_date ... ok
  test update_enhanced_task_status_preserves_trailing_newline ... ok
  test update_enhanced_task_status_handles_shelved ... ok

  test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/traceability.rs (target/llvm-cov-target/debug/deps/traceability-2dc94f8d312bf83e)

  running 13 tests
  test checkbox_format_gives_unavailable ... ok
  test shelved_task_does_not_count_as_coverage ... ok
  test declared_requirements_are_sorted_and_deduplicated ... ok
  test uncovered_requirement_appears_in_uncovered_list ... ok
  test no_requirement_ids_gives_unavailable ... ok
  test empty_requirements_list_gives_unavailable ... ok
  test duplicate_requirement_ids_flagged_in_diagnostics ... ok
  test in_progress_task_counts_as_coverage ... ok
  test partial_ids_gives_invalid_with_missing_titles ... ok
  test unresolved_task_reference_is_reported ... ok
  test all_requirements_covered_by_tasks ... ok
  test complete_task_counts_as_coverage ... ok
  test multiple_tasks_can_cover_same_requirement ... ok

  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_logging-919d3846765d88bd)

  running 2 tests
  test tests::unsafe_session_ids_are_rejected ... ok
  test tests::invalid_command_logger_writes_jsonl_entry ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_templates-71deaecae733736a)

  running 85 tests
  test agents::tests::default_configs_has_all_combinations ... ok
  test agents::tests::render_template_replaces_variant ... ok
  test agents::tests::render_template_replaces_model ... ok
  test agents::tests::render_template_removes_variant_line_if_not_set ... ok
  test instructions::tests::finish_template_prompts_for_archive ... ok
  test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
  test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
  test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
  test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
  test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
  test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
  test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
  test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
  test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
  test instructions::tests::orchestrate_template_renders ... ok
  test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
  test instructions::manifesto_tests::manifesto_template_renders_minimal_context ... ok
  test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
  test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
  test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
  test instructions::tests::render_template_str_is_strict_on_undefined ... ok
  test instructions::tests::render_template_str_preserves_trailing_newline ... ok
  test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
  test instructions::manifesto_tests::manifesto_template_renders_embedded_instruction_entries ... ok
  test instructions::tests::repo_sweep_template_renders ... ok
  test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
  test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
  test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
  test project_templates::tests::default_context_is_disabled ... ok
  test instructions::tests::review_template_renders_conditional_sections ... ok
  test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
  test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
  test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
  test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
  test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
  test project_templates::tests::render_project_template_passes_plain_text_through ... ok
  test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
  test project_templates::tests::render_project_template_renders_simple_variable ... ok
  test project_templates::tests::render_project_template_strict_on_undefined ... ok
  test project_templates::tests::render_project_template_renders_conditional ... ok
  test tests::default_home_files_returns_a_vec ... ok
  test tests::default_project_includes_orchestrate_user_prompt ... ok
  test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
  test tests::default_project_files_contains_expected_files ... ok
  test tests::every_shipped_command_has_ito_prefix ... ok
  test tests::agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts ... ok
  test tests::every_shipped_agent_has_ito_prefix ... ok
  test tests::every_shipped_skill_has_ito_prefix ... ok
  test tests::extract_managed_block_rejects_inline_markers ... ok
  test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
  test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
  test tests::extract_managed_block_returns_inner_content ... ok
  test tests::fix_and_feature_commands_are_embedded ... ok
  test tests::get_preset_file_returns_contents ... ok
  test tests::get_schema_file_returns_contents ... ok
  test tests::loop_command_template_uses_ito_loop_command_name ... ok
  test tests::loop_skill_template_includes_yaml_frontmatter ... ok
  test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
  test tests::every_shipped_markdown_has_managed_markers ... ok
  test tests::memory_skill_is_embedded ... ok
  test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
  test tests::normalize_ito_dir_prefixes_dot ... ok
  test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
  test tests::orchestrate_skills_and_command_are_embedded ... ok
  test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
  test tests::presets_files_contains_orchestrate_builtins ... ok
  test tests::proposal_intake_and_routing_skills_are_embedded ... ok
  test tests::render_bytes_preserves_non_utf8 ... ok
  test tests::render_bytes_returns_borrowed_when_no_rewrite_needed ... ok
  test tests::render_rel_path_rewrites_ito_prefix ... ok
  test tests::render_bytes_rewrites_dot_ito_paths ... ok
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

  test result: ok. 85 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/instructions_apply_memory.rs (target/llvm-cov-target/debug/deps/instructions_apply_memory-fd9e5ff1cdefb7cf)

  running 2 tests
  test apply_template_renders_capture_reminder_when_configured ... ok
  test apply_template_omits_capture_reminder_when_search_only_configured ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/managed_markers.rs (target/llvm-cov-target/debug/deps/managed_markers-6f2d47bfc8d49ef5)

  running 5 tests
  test schema_files_have_managed_markers ... ok
  test commands_have_managed_markers ... ok
  test default_project_files_have_managed_markers ... ok
  test agents_have_managed_markers ... ok
  test skills_have_managed_markers ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/prefix_rule.rs (target/llvm-cov-target/debug/deps/prefix_rule-c7eb9d4a41fd5ce4)

  running 3 tests
  test commands_satisfy_ito_prefix_rule ... ok
  test agents_satisfy_ito_prefix_rule ... ok
  test skills_satisfy_ito_prefix_rule ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/stamp.rs (target/llvm-cov-target/debug/deps/stamp-4e12f5d29993d962)

  running 8 tests
  test stamp_idempotent_when_same_version ... ok
  test stamp_inserts_when_no_existing_stamp ... ok
  test stamp_no_op_when_no_managed_block ... ok
  test stamp_preserves_rest_of_file ... ok
  test stamp_rewrites_spaced_stamp_to_canonical ... ok
  test stamp_rewrites_older_version_stamp ... ok
  test stamp_works_with_frontmatter_before_marker ... ok
  test stamp_round_trip_on_real_skill ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/template_markdown.rs (target/llvm-cov-target/debug/deps/template_markdown-745ef783e9c28459)

  running 1 test
  test template_markdown_is_well_formed ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/user_guidance_template.rs (target/llvm-cov-target/debug/deps/user_guidance_template-d4c93013a831e614)

  running 2 tests
  test user_guidance_template_exists_and_has_markers ... ok
  test user_prompt_stub_templates_exist ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running tests/worktree_template_rendering.rs (target/llvm-cov-target/debug/deps/worktree_template_rendering-525fcb29d447ac62)

  running 8 tests
  test skill_disabled ... ok
  test agents_md_disabled ... ok
  test skill_bare_control_siblings ... ok
  test agents_md_bare_control_siblings ... ok
  test skill_checkout_subdir ... ok
  test skill_checkout_siblings ... ok
  test agents_md_checkout_siblings ... ok
  test agents_md_checkout_subdir ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

       Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/ito_test_support-53a46c2be5c4afb9)

  running 4 tests
  test tests::normalize_replaces_home_path ... ok
  test tests::normalize_strips_ansi_and_crlf ... ok
  test tests::copy_dir_all_copies_nested_files ... ok
  test pty::tests::pty_can_echo_input_via_cat ... ok

  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

       Running tests/mock_repos_smoke.rs (target/llvm-cov-target/debug/deps/mock_repos_smoke-bc2ce31fe17a30c3)

  running 3 tests
  test mock_module_repo_resolves_by_id_or_name ... ok
  test mock_task_repo_returns_configured_tasks ... ok
  test mock_repos_basic_roundtrip ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
make: *** [check-prek] Error 1
```
