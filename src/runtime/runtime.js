// import _process from "./process.js";

((globalThis) => {
  function stringify(object, indent) {
    const has_indent = typeof indent === "number" && indent >= 0;

    if (Object.keys(object).length === 0) return "{}";

    return (
      `{${has_indent ? "\n" + " ".repeat(indent) : " "}` +
      Object.entries(object)
        .map(
          ([key, value]) =>
            `${key}: ${typeof value === "string" ? `'${value}'` : toString(value, false)}`
        )
        .join(`,${has_indent ? "\n" + " ".repeat(indent) : " "}`) +
      `${has_indent ? "\n" : " "}}`
    );
  }

  function toString(variable, show_object = true) {
    if (variable === undefined) return "undefined";
    if (variable === null) return "null";

    const isArray = Array.isArray(variable);
    const isNativeClass =
      typeof variable === "function" && /^\s*class\s+/.test(variable.toString());
    const isFunction = !isNativeClass && typeof variable === "function";
    const isClass =
      !isNativeClass &&
      !isArray &&
      typeof variable === "object" &&
      variable.constructor &&
      variable.constructor.name !== "Object";

    if (isNativeClass) return `[class ${variable.name}]`;
    if (isFunction) return `[Function: ${variable.name}]`;
    if (isClass) return `${variable.constructor.name} ${stringify(variable, 2)}`;

    if (Array.isArray(variable))
      return (
        "[" + variable.map((v) => (typeof v === "string" ? `'${v}'` : toString(v))).join(", ") + "]"
      );

    if (typeof variable === "object") return show_object ? stringify(variable) : "[object Object]";

    return String(variable);
  }

  function toStringAll(...args) {
    return args.map(toString).join(" ");
  }

  // https://console.spec.whatwg.org/
  globalThis.console = {
    write: (...args) => {
      Deno.core.print(toStringAll(...args), false);
    },
    log: (...args) => {
      Deno.core.print(`${toStringAll(...args)}\n`, false);
    },
    error: (...args) => {
      Deno.core.print(`[err]: ${toStringAll(...args)}\n`, true);
    },
  };

  globalThis.process = {
    version: Deno.core.ops.op_get_version(),
    versions: {
      micascript: Deno.core.ops.op_get_version(),
      v8: Deno.core.ops.op_get_v8_version(),
    },
    argv: Deno.core.ops.op_get_args(),
    env: Deno.core.ops.op_get_env(),
  };

  globalThis.MicaScript = {
    readFile: (path) => {
      return Deno.core.ops.op_read_file(path);
    },
    writeFile: (path, contents) => {
      return Deno.core.ops.op_write_file(path, contents);
    },
    removeFile: (path) => {
      return Deno.core.ops.op_remove_file(path);
    },
    listdir: (dir) => {
      return Deno.core.ops.op_read_dir(dir);
    },
    getTimeInNanoseconds: () => {
      return Deno.core.ops.op_get_time_in_nanos();
    },
  };
})(globalThis);
