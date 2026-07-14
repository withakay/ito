//! Exact fingerprints for generated surfaces retired by lifecycle consolidation.

pub(super) const EMPTY_SHA256: &str =
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub(super) fn retired_skill_prefix_sha256(name: &str) -> Option<&'static str> {
    Some(match name {
        "ito-brainstorming" => "14887eea417eeb820ac29f82f367c2f6af80006acee5242a12c93a1fa92c8269",
        "ito-cleanup" => "d9d55560d65ab1bf338c2716e5a7728374514c44e01a59f9448419a13e9233d2",
        "ito-commit" => "402cddb7d3a4400641d0a83da041d0ed9e804a850122214bf7f21350e3a2a842",
        "ito-feature" => "b452c1aa9fe1a9b68f834d4b45070355f5b9c3bca2d06d6f6d406d71f5b35aa6",
        "ito-finish" => "5003dff6fdf9d7403aa902a9b72f9d3b9a39fc38fee247acb18f2c51e50b7bdd",
        "ito-fix" => "093d44454a1e0c692bfdd6f8aada7ef24361003b442a7bd49fcc1a3feff7fc0c",
        "ito-list" => "366a228145fc19e6d7c94eb8ab0f7c93a48114afbeb82ae0f6c8a02d7bc8a96f",
        "ito-memory" => "14c7573e1a1ee54b3b2ea8a473748831ab3bd163f2bd66738a7b75a4887a21f7",
        "ito-orchestrate" => "186e8f470e64b7a8b1177881b08c1a200d3f2b084b44a303b62c49b6d6502ebc",
        "ito-orchestrate-setup" => {
            "90f1503b0c088a9b6fdb12d29f6ffaae10970a51e049166917e3479baf3df2da"
        }
        "ito-orchestrator-workflow" => {
            "f25c1efc3ff617ce1613ede9c910eb43476f7bc99e1e8d8acf8e52b39b685b43"
        }
        "ito-path" => "a1f8354fc5577d4f0996daa6da10dad1524b9dbf1e9840d6c404cabd50b7f0fc",
        "ito-plan" => "62a58c13179c0953c2943cb1fa7f56f751e35153f29992ad2f2dc89fa777f351",
        "ito-proposal-intake" => "590d58c9fd45e0a8300ca2b05cdadb567021768f5407ec6b7a97d3239231e94e",
        "ito-subagent-driven-development" => {
            "885fb4c15552edff8c0226d9cd353c8ddb7e4af998890d96ced2f7e478e61e16"
        }
        "ito-tasks" => "eb1eae51179468c67ba816c427bf5e0ffe3948b6e2a932d672ca9a9208dcdf36",
        "ito-test-with-subagent" => {
            "e3218c47a8601d6e410256095e6d82c584a880bf45f57474684f77cf45f43310"
        }
        "ito-tmux" => "93cb725faa0c36842e26e1106a9c85736b28c96cda0679ff15a931413edcbd32",
        "ito-update-repo" => "301da2598f33340a516d05b10e0a998f191b04840bbcd0f67f6bfee0b16cb21e",
        "ito-using-git-worktrees" => {
            "441255265a143006e2682cd474ad9e68aa85317438e5803e1ce370a97e6354a7"
        }
        "ito-using-ito-skills" => {
            "0636a1e8c5775c042adb6fb3d30d2d63b5e7f15d0d4321fb0588ec87da55089d"
        }
        "ito-verification-before-completion" => {
            "301c90bcaa7f90a2c73877b41d1b6e4046ed84cb26bb437a5f8da7ca8bdf95af"
        }
        "ito-wiki" => "5b82875af05d87b7a9e2a092d9618440923e769afc138bad847a097b65f6b7f0",
        "ito-wiki-search" => "1b0aea2edaa27d7f4d1ff36df46c07c580ed880e4d2eb845fcca3e3c4c986237",
        "ito-workflow" => "db4707c02944af35aa4a2736d8159116de2c5af6b2082e02fc9dbcdbd60997ca",
        "ito-apply-change-proposal" => {
            "67c5afcc744392dc049b0e1061298af7d3206a3f6cb6393bc4b43915fa5a5ec8"
        }
        "ito-write-change-proposal" => {
            "f687ae6472d26026e856958c56eb22971308a177925a4f2b8b3ceebe5aa97cf7"
        }
        "ito-dispatching-parallel-agents" => {
            "bc979b7f209a12bc15b8504cff31a7c69edac630a256955ff404029c38565729"
        }
        "ito-finishing-a-development-branch" => {
            "2a89f5c5f85e5adfbb27913a56321704e918f6f8dbe5780cf32942259e32b009"
        }
        "ito-receiving-code-review" => {
            "cbf580a8b2b65fba03646a47223264654d7037c8b1901236c1f485a005fb64df"
        }
        "ito-requesting-code-review" => {
            "6ebec50e93a3561485283ba55441c0c7f6cc8e4da1dc8bf23c71ed40cc69a10d"
        }
        "ito-systematic-debugging" => {
            "ea4a5379cb1f624017756053e90e49e1297a16b402cdecbbd15a572d20a49f90"
        }
        "ito-test-driven-development" => {
            "d11666ee4a93cc0b2ca8b9f1e47f64508c455a55d1f210d61a81c30740fd62d9"
        }
        "ito-writing-skills" => "04466cd910a375f9b958fcbfbf69a6074d101765d1d616cc367def51e47d793e",
        "tmux" => "937ecd0571aea7733779ebb91ac4d48ae30ef8001e8fa360ad6260b543916f6b",
        "test-with-subagent" => "2791141daf4feada3cf57fa3bd1ead04e73cde1b014118317e097389a8cc0568",
        "using-ito-skills" => "607ce3b3e7773419a11791e12daf6ab7b4fc8a913a3e9bea4a95d398e7a95011",
        _ => return None,
    })
}

pub(super) fn retired_command_prefix_sha256(name: &str) -> Option<&'static str> {
    Some(match name {
        "ito-feature" => "d329c71931a17624896b131a74345e4f53df1263a6589d8d862284508ba4e54e",
        "ito-fix" => "06e1eca9a6c205c866803f8e6f3a0dfedeaab44a86b11935d11c21099506b231",
        "ito-list" => "1acac04f0a1d452f41ceece87c019922ee3d2be162c13d739bcbfd22ea4c0c2b",
        "loop" => "086af3d2dfce5af0b6ffa7d61081811c90434934596524926b43e0870ceb1885",
        "ito-orchestrate" => "b9d83af3f627042cd7a8a6c4d5d996c9f24d58e309d95bf982018bcb5e18d40f",
        "ito-plan" => "3bfe0386fd79158f64df118fcd94eeb32eaef38bde3f3d9a51e0ace468eb8684",
        "ito-proposal-intake" => "68b1bf13489a1ef72c25127d4733c3a9daa05babd7fe3b5b921071ac9517fa6f",
        "ito-project-setup" => EMPTY_SHA256,
        "ito-update-repo" => "f40cc8b911d748ad54be25992fda81835d8cfc2e375494540d5cb002cc387e09",
        _ => return None,
    })
}

pub(super) fn retired_codex_role_prefix_sha256(name: &str) -> Option<&'static str> {
    Some(match name {
        "ito-general" => "e98f2999cd5c37a40d1d6fa843b617bee57b0d39842c4f7e784d79c52f61f884",
        "ito-thinking" => "2775a7727d731993a95a84b04fb1f06a5998205140e57708e7f687761b74010d",
        "ito-orchestrator" => "c75d00f4ecc4746e5c8b9205933cd0a075bba6cac42167d46e9ada87ea023853",
        "ito-quick" => "2d6b6827bb7877d0ae747a2fac4db4c604fee3bacf2f3ea64da059aa40b5de43",
        "ito-planner" => "1eff4ce0b8994584aa34e16dae8001e5e05aad0306e720f1a8f75b0559a5a60a",
        "ito-researcher" => "55021d9f4d9d59f71e5ada5499e060d76b16fc166cd6b542e08246c299592b5f",
        "ito-reviewer" => "8c0caa8cea0b564efd74d852f5cd4e64690af83c14ad7e1cc7a31f225c876d39",
        "ito-worker" => "6592096a974eb81b616105815b92c322909f390dbf76a6b35d1c4cc98508f86e",
        "ito-orchestrator-planner" => {
            "95c778c5c73faf76847edbc4d24e0cfbc136bf1d8886104e2be4b1b9f68f30c1"
        }
        "ito-orchestrator-researcher" => {
            "5928a7b7f94d5d736208cba99d43a7b9190738e92fdb54c2d1b4cabd86ea7fb4"
        }
        "ito-orchestrator-reviewer" => {
            "d04ae0f62d3f7ee6c1d3d594f69d8b17a25d8baca503258f3417a20ddafdba1b"
        }
        "ito-orchestrator-worker" => {
            "1fd206b5ae2be2f3246c410f34b94d30437d006650f1e6688f321ad495e648a5"
        }
        _ => return None,
    })
}
