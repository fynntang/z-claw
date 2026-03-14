/**
 * Cross-platform runner for ensure-zeroclaw-binary script.
 * Spawns .ps1 on Windows, .sh otherwise; exits with same code.
 */
import { spawnSync } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';
const __dirname = path.dirname(fileURLToPath(import.meta.url));

const repoRoot = path.resolve(__dirname, '..');
const isWin = process.platform === 'win32';
const script = isWin
  ? path.join(repoRoot, 'scripts', 'ensure-zeroclaw-binary.ps1')
  : path.join(repoRoot, 'scripts', 'ensure-zeroclaw-binary.sh');

const result = spawnSync(
  isWin ? 'powershell' : 'bash',
  isWin ? ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', script] : ['-e', script],
  { cwd: repoRoot, stdio: 'inherit', shell: !isWin }
);
process.exit(result.status ?? 1);
