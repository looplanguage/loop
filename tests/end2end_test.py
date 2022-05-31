import platform
# Imports the test library
from testlib import run_tests, add_test

def main():
    # Add your tests right here.
    # AddTest( FILE LOCATION OF LOOP FILE, EXPECTED ANSWER )
    # NOTE: The expected answer always needs to be a string
    add_test("test_comments.loop", "3")
    #add_test("test_import_export.loop", "8")
    add_test("test_function.loop", "9")
    if platform.system() == "Windows": # Windows handles "new_lines" different then Unix
        add_test("test_string.loop", "123Hello\r\nWorld!")
    else:
        add_test("test_string.loop", "123Hello\nWorld!")
    add_test("test_expression.loop", "-13")
    add_test("test_expression_precedence.loop", "-29.616")
    add_test("test_variable_declaration.loop",  "250")
    add_test("test_conditional_true.loop", "100")           # Some conditionals have parenthesis others do not
    add_test("test_conditional_false.loop", "300")          # Some conditionals have parenthesis others do not
    add_test("test_conditional_null.loop", "50")
    add_test("test_fibonacci.loop", "178")                  # Does twice, one with parenthesis other time without, than adds them
    add_test("test_closure.loop", "60")                     # 3 deep
    add_test("test_closure_variable_scope.loop", "1230")
    add_test("test_division_float.loop", "2.45")
    add_test("test_division_integer.loop", "1")
    add_test("test_array_extension_method.loop", "20")       # Contains all the different methods
    add_test("test_modulo.loop", "0.5")
    # Extension methods arent decided whether or not they should be language features or FFI related
    # add_test("test_extension_methods_variable.loop", "102")
    add_test("test_array_index.loop", "91")
    add_test("test_array_3d_assign_index.loop", "200")
    add_test("test_arrays.loop", "31")
    add_test("test_logical_operators.loop", "false")
    # Dont test hashmaps yet, as they need to be reimplemented first
    # add_test("test_hashmaps.loop", "30") # order of things change
    # add_test("test_hashmaps_nested_assign.loop", "true")
    # Does twice, one with parenthesis other time without, than adds them
    add_test("test_loop_while.loop", "20")
    add_test("test_loop_iterator.loop", "20")
    add_test("test_loop_iterator_array.loop", "46")
    add_test("test_everything_is_an_expression.loop", "110")
    add_test("test_constant.loop", "22")
    # Println is made to just be print now
    # add_test("test_println.loop", "123")
    add_test("test_expression_statements.loop", "10946")
    add_test("test_classes.loop", "2")

    # output, is the generated report of all the tests
    # has_failed, a boolean:
    #   > True: One or more test have failed.
    #   > False: All tests passed
    output, has_failed = run_tests()

    # Asserts if 'has_failed' is false, otherwise prints a red failure message
    assert not has_failed, output + "\033[91mEnd2End Tests Have Failed...\033[0m"

    # Prints a green success message
    print(output + "\033[92mEnd2End Tests Have Succeeded...\033[0m")

if __name__ == "__main__":
    main()