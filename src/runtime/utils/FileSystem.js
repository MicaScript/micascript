function readFile(path) {
  return Deno.core.ops.op_read_file(path);
}

function writeFile(path, contents) {
  return Deno.core.ops.op_write_file(path, contents);
}

function removeFile(path) {
  return Deno.core.ops.op_remove_file(path);
}

function listdir(dir) {
  return Deno.core.ops.op_read_dir(dir);
}

// TODO: Add other file transactions.
