import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { spawn } from "node-pty";

const term = new Terminal({
  cursorBlink: true,
  fontFamily: "monospace",
  theme: { background: "#0e0e0e", foreground: "#00ff88" }
});

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

term.open(document.getElementById("terminal"));
fitAddon.fit();

const shell = process.env.SHELL || "zsh";
const pty = spawn(shell, [], {
  name: "xterm-color",
  cols: term.cols,
  rows: term.rows,
  cwd: process.env.HOME,
  env: process.env
});

pty.onData((data) => term.write(data));
term.onData((data) => pty.write(data));

window.addEventListener("resize", () => fitAddon.fit());