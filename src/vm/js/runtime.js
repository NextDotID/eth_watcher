((globalThis) => {
    const {core} = Deno;
    const {ops} = core;

    // core.initializeAsyncOps();

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(' ');
    }

    globalThis.console = {
        log: (...args) => {
            ops.op_console_log(argsToMessage(...args));
        },
        debug: (...args) => {
            ops.op_console_debug(argsToMessage(...args));
        },
        warn: (...args) => {
            ops.op_console_warn(argsToMessage(...args));
        },
        error: (...args) => {
            ops.op_console_error(argsToMessage(...args));
        },
    };
    // For inject_into_vm() use.
    globalThis.ew = {};

    // All actions triggered in script should be saved here.
    globalThis.actions = [];
})(globalThis);
