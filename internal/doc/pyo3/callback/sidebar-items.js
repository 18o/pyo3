initSidebarItems({"fn":[["abort_on_traverse_panic","Aborts if panic has occurred. Used inside `__traverse__` implementations, where panicking is not possible."],["convert",""],["handle_panic","Variant of the above which does not perform the callback conversion. This allows the callback conversion to be done manually in the case where lifetimes might otherwise cause issue."],["panic_result_into_callback_output","Converts the output of std::panic::catch_unwind into a Python function output, either by raising a Python exception or by unwrapping the contained success output."]],"macro":[["wrapping_cast",""]],"struct":[["HashCallbackOutput",""]],"trait":[["IntoPyCallbackOutput","Convert the result of callback function into the appropriate return value."],["PyCallbackOutput","A type which can be the return type of a python C-API callback"],["WrappingCastTo",""]]});