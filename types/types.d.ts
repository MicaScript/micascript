/// <reference no-default-lib="true" />;
/// <reference lib="esnext" />

declare var console: {
  write: (...args: any[]) => void;
  log: (...args: any[]) => void;
  error: (...args: any[]) => void;
};

declare var process: {
  version: string;
  versions: {
    micascript: string;
    v8: string;
  };
  env: any[];
};

declare var MicaScript: {
  getTimeInNanoseconds: () => number;
  readFile: (path: string) => Promise<string>;
  writeFile: (path: string, content: string) => Promise<void>;
  removeFile: (path: string) => void;
  listdir: (dir?: string) => string[];
};

declare var Deno: {
  core: {
    ops: {
      op_close: Function;
      op_try_close: Function;
      op_print: Function;
      op_resources: Function;
      op_wasm_streaming_feed: Function;
      op_wasm_streaming_set_url: Function;
      op_void_sync: Function;
      op_error_async: Function;
      op_error_async_deferred: Function;
      op_void_async: Function;
      op_void_async_deferred: Function;
      op_add: Function;
      op_add_async: Function;
      op_read: Function;
      op_read_all: Function;
      op_write: Function;
      op_read_sync: Function;
      op_write_sync: Function;
      op_write_all: Function;
      op_write_type_error: Function;
      op_shutdown: Function;
      op_metrics: Function;
      op_format_file_name: Function;
      op_is_proxy: Function;
      op_str_byte_length: Function;
      op_ref_op: Function;
      op_unref_op: Function;
      op_set_promise_reject_callback: Function;
      op_run_microtasks: Function;
      op_has_tick_scheduled: Function;
      op_set_has_tick_scheduled: Function;
      op_eval_context: Function;
      op_queue_microtask: Function;
      op_create_host_object: Function;
      op_encode: Function;
      op_decode: Function;
      op_serialize: Function;
      op_deserialize: Function;
      op_set_promise_hooks: Function;
      op_get_promise_details: Function;
      op_get_proxy_details: Function;
      op_get_non_index_property_names: Function;
      op_get_constructor_name: Function;
      op_memory_usage: Function;
      op_set_wasm_streaming_callback: Function;
      op_abort_wasm_streaming: Function;
      op_destructure_error: Function;
      op_dispatch_exception: Function;
      op_op_names: Function;
      op_apply_source_map: Function;
      op_set_format_exception_callback: Function;
      op_event_loop_has_more_work: Function;
      op_store_pending_promise_rejection: Function;
      op_remove_pending_promise_rejection: Function;
      op_has_pending_promise_rejection: Function;
      op_arraybuffer_was_detached: Function;
      op_get_version: Function;
      op_get_v8_version: Function;
      op_get_args: Function;
      op_get_env: Function;
      op_get_time_in_nanos: Function;
      op_read_file: Function;
      op_write_file: Function;
      op_remove_file: Function;
      op_read_dir: Function;
    };
    asyncOps: {
      op_error_async: Function;
      op_error_async_deferred: Function;
      op_void_async: Function;
      op_void_async_deferred: Function;
      op_add_async: Function;
      op_read: Function;
      op_read_all: Function;
      op_write: Function;
      op_write_all: Function;
      op_write_type_error: Function;
      op_shutdown: Function;
      op_read_file: Function;
      op_write_file: Function;
    };
    callConsole: Function;
    console: {
      debug: Function;
      error: Function;
      info: Function;
      log: Function;
      warn: Function;
      dir: Function;
      dirxml: Function;
      table: Function;
      trace: Function;
      group: Function;
      groupCollapsed: Function;
      groupEnd: Function;
      clear: Function;
      count: Function;
      countReset: Function;
      assert: Function;
      profile: Function;
      profileEnd: Function;
      time: Function;
      timeLog: Function;
      timeEnd: Function;
      timeStamp: Function;
      context: Function;
    };
    asyncStub: Function;
    ensureFastOps: Function;
    opAsync: Function;
    resources: Function;
    metrics: Function;
    registerErrorBuilder: Function;
    registerErrorClass: Function;
    buildCustomError: Function;
    eventLoopTick: Function;
    BadResource: Function;
    BadResourcePrototype: {};
    Interrupted: Function;
    InterruptedPrototype: {};
    enableOpCallTracing: Function;
    isOpCallTracingEnabled: Function;
    opCallTraces: {};
    refOp: Function;
    unrefOp: Function;
    setReportExceptionCallback: Function;
    setPromiseHooks: Function;
    close: Function;
    tryClose: Function;
    read: Function;
    readAll: Function;
    write: Function;
    writeAll: Function;
    writeTypeError: Function;
    readSync: Function;
    writeSync: Function;
    shutdown: Function;
    print: Function;
    setMacrotaskCallback: Function;
    setNextTickCallback: Function;
    runMicrotasks: Function;
    hasTickScheduled: Function;
    setHasTickScheduled: Function;
    evalContext: Function;
    createHostObject: Function;
    encode: Function;
    decode: Function;
    serialize: Function;
    deserialize: Function;
    getPromiseDetails: Function;
    getProxyDetails: Function;
    isProxy: Function;
    memoryUsage: Function;
    setWasmStreamingCallback: Function;
    abortWasmStreaming: Function;
    destructureError: Function;
    opNames: Function;
    eventLoopHasMoreWork: Function;
    setPromiseRejectCallback: Function;
    byteLength: Function;
    build: { target: "unknown"; arch: "unknown"; os: "unknown"; vendor: "unknown"; env: undefined };
    setBuildInfo: Function;
    prepareStackTrace: Function;
  };
  __op__: Function;
};
