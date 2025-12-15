# Publishing mdfx to VS Code Marketplace

This guide walks through publishing the mdfx VS Code extension.

## Prerequisites

- Node.js 16+
- npm
- Microsoft account (for Azure DevOps)

## One-Time Setup

### 1. Create Azure DevOps Organization

1. Go to https://dev.azure.com
2. Sign in with your Microsoft account
3. Create a new organization if you don't have one

### 2. Create Personal Access Token (PAT)

1. In Azure DevOps, click your profile icon → **Personal access tokens**
2. Click **New Token**
3. Configure:
   - **Name**: `vscode-marketplace`
   - **Organization**: Select your organization
   - **Expiration**: Set as needed (max 1 year)
   - **Scopes**: Click **Custom defined**, then:
     - Find **Marketplace** → check **Manage**
4. Click **Create** and **copy the token immediately** (you won't see it again)

### 3. Create Publisher

1. Go to https://marketplace.visualstudio.com/manage
2. Sign in with the same Microsoft account
3. Click **Create publisher**
4. Fill in:
   - **ID**: `blackwell-systems` (must match package.json)
   - **Name**: `Blackwell Systems`
5. Click **Create**

### 4. Install vsce

```bash
npm install -g @vscode/vsce
```

### 5. Login to vsce

```bash
vsce login blackwell-systems
# Paste your PAT when prompted
```

## Before Publishing

### Add Extension Icon

Create a 128x128 PNG icon (256x256 recommended):

```bash
# Place icon in extension folder
cp /path/to/your/icon.png editors/vscode/icon.png
```

### Verify package.json

Ensure these fields are set in `editors/vscode/package.json`:

```json
{
  "name": "mdfx",
  "displayName": "mdfx",
  "description": "Language support for mdfx template syntax in Markdown files",
  "version": "1.0.0",
  "publisher": "blackwell-systems",
  "icon": "icon.png",
  "repository": {
    "type": "git",
    "url": "https://github.com/blackwell-systems/mdfx"
  },
  "license": "MIT"
}
```

### Install Dependencies

```bash
cd editors/vscode
npm install
```

## Publishing

### Package the Extension

```bash
cd editors/vscode
vsce package
```

This creates `mdfx-1.0.0.vsix`.

### Test Locally (Optional)

```bash
code --install-extension mdfx-1.0.0.vsix
```

### Publish to Marketplace

```bash
vsce publish
```

Or publish with version bump:

```bash
vsce publish minor  # 1.0.0 → 1.1.0
vsce publish patch  # 1.0.0 → 1.0.1
vsce publish major  # 1.0.0 → 2.0.0
```

### Manual Upload (Alternative)

1. Go to https://marketplace.visualstudio.com/manage
2. Click your publisher
3. Click **New extension** → **Visual Studio Code**
4. Upload the `.vsix` file

## Post-Publishing

### Verify Publication

1. Visit https://marketplace.visualstudio.com/items?itemName=blackwell-systems.mdfx
2. Extension should appear within 5-10 minutes

### Update Extension

1. Update version in `package.json`
2. Run `vsce publish`

Or update and publish in one step:

```bash
vsce publish patch -m "Fix bug in syntax highlighting"
```

## Troubleshooting

### "Personal Access Token has expired"

Generate a new PAT and run:

```bash
vsce logout blackwell-systems
vsce login blackwell-systems
```

### "publisher 'x' is not allowed"

The publisher ID in package.json must match your Marketplace publisher ID exactly.

### "Missing required field: icon"

Add an `icon.png` file (128x128 minimum) to the extension folder.

### "README.md not found"

Ensure `editors/vscode/README.md` exists and has content.

## Useful Commands

```bash
# Check what will be packaged
vsce ls

# Package without publishing
vsce package

# Show extension info
vsce show blackwell-systems.mdfx

# Unpublish (use with caution)
vsce unpublish blackwell-systems.mdfx
```

## CI/CD Publishing

For automated publishing via GitHub Actions:

```yaml
# .github/workflows/publish-vscode.yml
name: Publish VS Code Extension

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: npm install -g @vscode/vsce
      - run: cd editors/vscode && npm install
      - run: cd editors/vscode && vsce publish
        env:
          VSCE_PAT: ${{ secrets.VSCE_PAT }}
```

Add your PAT as a repository secret named `VSCE_PAT`.
