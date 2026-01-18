import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';

function runGit(args, cwd) {
  return execFileSync('git', args, {
    cwd,
    stdio: 'pipe',
    encoding: 'utf8',
  });
}

function canApplyPatch({ cwd, patchPath, reverse }) {
  try {
    runGit(['apply', '--check', ...(reverse ? ['--reverse'] : []), patchPath], cwd);
    return true;
  } catch {
    return false;
  }
}

const repoRoot = process.cwd();
const cinnyDir = path.join(repoRoot, 'cinny');
const patchPath = path.join(repoRoot, 'patches', 'cinny-wayland-clipboard-image-paste.patch');

if (!existsSync(cinnyDir)) {
  console.error('Missing cinny submodule at:', cinnyDir);
  console.error('Run: git submodule update --init --recursive');
  process.exit(1);
}

if (!existsSync(patchPath)) {
  console.error('Missing patch file at:', patchPath);
  process.exit(1);
}

// If patch is already applied, reverse-check will succeed.
if (canApplyPatch({ cwd: cinnyDir, patchPath, reverse: true })) {
  console.log('Cinny patch already applied:', path.basename(patchPath));
  process.exit(0);
}

if (!canApplyPatch({ cwd: cinnyDir, patchPath, reverse: false })) {
  console.error('Cinny patch cannot be applied cleanly:', patchPath);
  console.error('If you updated the cinny submodule, the patch may need a refresh.');
  process.exit(1);
}

runGit(['apply', patchPath], cinnyDir);
console.log('Applied cinny patch:', path.basename(patchPath));
