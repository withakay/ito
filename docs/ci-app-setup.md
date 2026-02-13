# Ito CI GitHub App Setup

This project uses a single GitHub App ("Ito CI") to authenticate CI workflows that need to push commits, create releases, open PRs, or write to other repositories in the org.

Using a GitHub App instead of Personal Access Tokens (PATs) gives us:

- **Scoped, short-lived tokens** generated per workflow run (not long-lived PATs)
- **Not tied to a personal account** (won't break if someone leaves)
- **Pushes trigger workflow runs** (unlike `GITHUB_TOKEN`, which suppresses re-triggers by design)
- **One identity** for all CI automation instead of multiple PATs

## Workflows that use the App

| Workflow | What it does with the token |
|---|---|
| `ci.yml` (autofix) | Pushes lint/format fixes to PR branches |
| `release-plz.yml` | Creates release PRs, pushes tags, publishes GitHub releases |
| `v-release.yml` | Pushes Homebrew formula updates to `withakay/homebrew-ito` |

## Step 1: Create the GitHub App

1. Go to **GitHub > Settings > Developer settings > GitHub Apps > New GitHub App**
   - Or directly: <https://github.com/organizations/withakay/settings/apps/new>
2. Fill in the basics:
   - **App name**: `Ito CI` (or `ito-ci` -- must be globally unique on GitHub)
   - **Homepage URL**: `https://github.com/withakay/ito` (or anything, it's just metadata)
   - **Webhook**: Uncheck "Active" (we don't need webhook events)

3. Set **Repository permissions**:

   | Permission | Access | Why |
   |---|---|---|
   | Contents | **Read & Write** | Push autofix commits, push tags/releases, push Homebrew formulae |
   | Pull requests | **Read & Write** | Create and update release PRs (release-plz) |
   | Metadata | **Read-only** | Required by GitHub for all Apps (auto-selected) |

   No organization permissions are needed.

4. Under **Where can this GitHub App be installed?**, select:
   - **Only on this account** (restricts to the `withakay` org)

5. Click **Create GitHub App**.

6. Note the **App ID** shown on the App's settings page (a numeric ID like `123456`).

## Step 2: Generate a private key

1. On the App's settings page, scroll to **Private keys**
2. Click **Generate a private key**
3. A `.pem` file downloads -- keep this safe, you'll need it in the next step

## Step 3: Install the App on repositories

1. Go to the App's **Install** page (from the App settings, click "Install App" in the sidebar)
2. Choose the `withakay` organization
3. Select **Only select repositories** and pick:
   - `withakay/ito` (CI autofix, release-plz)
   - `withakay/homebrew-ito` (Homebrew formula publishing)
4. Click **Install**

## Step 4: Add repository secrets

Go to **withakay/ito > Settings > Secrets and variables > Actions** and add:

| Secret name | Value |
|---|---|
| `ITO_CI_APP_ID` | The App ID from Step 1 (numeric, e.g. `123456`) |
| `ITO_CI_APP_PRIVATE_KEY` | The full contents of the `.pem` file from Step 2 |

Paste the entire PEM file content including the `-----BEGIN RSA PRIVATE KEY-----` and `-----END RSA PRIVATE KEY-----` lines.

## Step 5: Retire old secrets

Once the App is set up and workflows are verified working, you can delete these legacy PAT secrets:

| Secret to remove | Was used by | Replaced by |
|---|---|---|
| `RELEASE_PLZ_TOKEN` | `release-plz.yml` | App token via `ITO_CI_APP_ID` / `ITO_CI_APP_PRIVATE_KEY` |
| `HOMEBREW_TAP_TOKEN` | `v-release.yml` | App token scoped to `homebrew-ito` repo |

**Keep these secrets** (they are unrelated to GitHub auth):

| Secret | Purpose |
|---|---|
| `CARGO_REGISTRY_TOKEN` | crates.io publishing (not a GitHub token) |
| `CLAUDE_CODE_OAUTH_TOKEN` | Claude AI for release note polishing |
| `ANTHROPIC_API_KEY` | Anthropic API for PR review |

## How it works in CI

Each workflow generates a short-lived installation token at the start of the job:

```yaml
- name: Generate GitHub App token
  id: app_token
  uses: actions/create-github-app-token@v2
  with:
    app-id: ${{ secrets.ITO_CI_APP_ID }}
    private-key: ${{ secrets.ITO_CI_APP_PRIVATE_KEY }}
```

The token is then used for `actions/checkout` (so `git push` works) and passed as `GITHUB_TOKEN` to actions that need GitHub API access.

For cross-repo access (e.g., pushing to `homebrew-ito`), the token is scoped to the target repository:

```yaml
- name: Generate GitHub App token (homebrew-ito)
  id: app_token
  uses: actions/create-github-app-token@v2
  with:
    app-id: ${{ secrets.ITO_CI_APP_ID }}
    private-key: ${{ secrets.ITO_CI_APP_PRIVATE_KEY }}
    repositories: homebrew-ito
```

## Troubleshooting

### Autofix commits not triggering CI re-runs

- Verify the App is installed on `withakay/ito` with Contents write access
- Check that `ITO_CI_APP_ID` and `ITO_CI_APP_PRIVATE_KEY` secrets are set correctly
- The `actions/create-github-app-token` step will fail explicitly if the credentials are wrong

### Release-plz can't create PRs or push tags

- Verify the App has both Contents and Pull requests write permissions
- Check the App is installed on `withakay/ito`

### Homebrew formula push fails

- Verify the App is installed on `withakay/homebrew-ito`
- The `v-release.yml` workflow scopes the token with `repositories: homebrew-ito` -- if the App isn't installed on that repo, token generation fails

### Permission errors after App creation

- Go to the App's settings and verify permissions match the table in Step 1
- After changing permissions, you may need to **re-approve** the installation (GitHub prompts the org admin)

### Rotating the private key

1. Go to the App's settings > Private keys
2. Generate a new key
3. Update the `ITO_CI_APP_PRIVATE_KEY` secret in the repo
4. Revoke the old key from the App settings
