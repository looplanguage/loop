# Imports the test library
from testlib import run_tests, add_test

def main():
    # Add your tests right here.
    # AddTest( FILE LOCATION OF LOOP FILE, EXPECTED ANSWER )
    # NOTE: The expected answer always needs to be a string
    add_test("test_comments.loop", "3")
    add_test("test_import_export.loop", "8")

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