import EventEmitter from "./utils/EventEmitter.js";

class Process extends EventEmitter {}

export default globalThis["process"] || new Process();
