import platform
import sys
# Imports the test library
from testlib import run_tests, add_test

def main(verbose: bool):
    # Add your tests right here.
    # AddTest( FILE LOCATION OF LOOP FILE, EXPECTED ANSWER )

    # Tests relating variable and constant declaration
    add_test("test_constant.loop", "22")
    add_test("test_variable_declaration.loop",  "250")

    # Tests relating string functionality
    add_test("test_string_index.loop", "2")
    add_test("test_string_slice.loop", "o, W")
    if platform.system() == "Windows": # Windows handles "new_lines" different then Unix
        add_test("test_string.loop", "123Hello\r\nWorld!")
    else:
        add_test("test_string.loop", "123Hello\nWorld!")

    # Tests relating arrays and its extension methods
    add_test("test_arrays.loop", "31")
    add_test("test_array_index.loop", "91")
    add_test("test_array_slice.loop", "9")
    add_test("test_array_remove.loop", "20")
    add_test("test_array_3d_assign_index.loop", "200")

    # Tests relating conditionals and if-expression
    add_test("test_conditional_true.loop", "100")
    add_test("test_conditional_false.loop", "300")
    add_test("test_conditional_null.loop", "50")
    add_test("test_logical_operators.loop", "false")
    add_test("test_if_expression_return.loop", "110")

    # Tests relating operators and math
    add_test("test_division_float.loop", "2.45")
    add_test("test_division_integer.loop", "1")
    add_test("test_modulo.loop", "0.5")
    add_test("test_expression.loop", "-13")
    add_test("test_expression_precedence.loop", "-29.616")

    # Tests relating loops
    add_test("test_loop_while.loop", "20")
    add_test("test_loop_iterator.loop", "20")
    add_test("test_loop_iterator_array.loop", "46")

    # Tests relating classes and extend
    add_test("test_classes.loop", "400")
    add_test("test_extend_types.loop", "129620")
    add_test("test_classes_lazy.loop", "100")

    # Tests relating packages and FFI
    add_test("test_import_lib.loop", "hello")
    add_test("test_multi_level_import.loop", "32")
    add_test("test_import_export.loop", "430")

    # Tests relating functions
    add_test("test_function_types.loop", "20")
    add_test("test_closure.loop", "60")
    add_test("test_closure_variable_scope.loop", "1230")
    add_test("test_function.loop", "9")

    # Extra feature tests
    add_test("test_comments.loop", "3")

    # Full feature tests including many things
    add_test("test_expression_statements.loop", "10946")
    add_test("test_fibonacci.loop", "178")

    # output, is the generated report of all the tests
    # has_failed, a boolean:
    #   > True: One or more test have failed.
    #   > False: All tests passed
    output, has_failed = run_tests(verbose)

    # Asserts if 'has_failed' is false, otherwise prints a red failure message
    if has_failed:
        print(output + "\033[91mEnd2End Tests Have Failed...\033[0m")
        exit(1)

    # Prints a green success message
    print(output + "\033[92mEnd2End Tests Have Succeeded...\033[0m")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "-v":
            main(True)
        else:
            print(f"ERROR: Unknown flag '{sys.argv[1]}'")
    else:
        main(False)
